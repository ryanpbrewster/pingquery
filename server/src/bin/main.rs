use std::{path::PathBuf, sync::Arc};

use actix::Actor;
use actix_web::{
    middleware,
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};

use actix_web_actors::ws;
use log::{debug, info};
use pingquery::{
    actor::{ClientActor, ListenActor},
    diagnostics::Diagnostics,
    persistence::Persistence,
    proto::api::{ExecRequest, InitializeRequest, SetConfigRequest},
    server::PingQueryService,
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use structopt::StructOpt;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let flags = CliFlags::from_args();

    let persistence = Persistence {
        metadata: sqlite(flags.metadata)?,
        data: sqlite(flags.data)?,
        diagnostics: Arc::new(Diagnostics::default()),
    };
    let service = PingQueryService {
        persistence: Arc::new(persistence),
        listener: ListenActor::default().start(),
    };
    let addr = "[::]:8080";
    info!("listening @ {}", addr);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(actix_cors::Cors::permissive())
            .service(web::resource("/").route(web::get().to(health_check_handler)))
            .service(
                web::resource("/diagnostics")
                    .app_data(Data::new(service.clone()))
                    .route(web::get().to(diagnostics_handler)),
            )
            .service(
                web::resource("/exec")
                    .app_data(Data::new(service.clone()))
                    .route(web::post().to(exec_handler)),
            )
            .service(
                web::resource("/initialize")
                    .app_data(Data::new(service.clone()))
                    .route(web::post().to(initialize_handler)),
            )
            .service(
                web::resource("/config")
                    .app_data(Data::new(service.clone()))
                    .route(web::get().to(get_config_handler))
                    .route(web::post().to(set_config_handler)),
            )
            .service(
                web::resource("/interact")
                    .app_data(Data::new(service.clone()))
                    .route(web::get().to(interact_handler)),
            )
            .default_service(web::route().to(p404))
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}

async fn health_check_handler() -> HttpResponse {
    HttpResponse::Ok().body("ok\n")
}

async fn initialize_handler(
    service: web::Data<PingQueryService>,
    request: web::Json<InitializeRequest>,
) -> HttpResponse {
    match service.get_ref().initialize(request.into_inner()).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => {
            debug!("error: {:?}", e);
            HttpResponse::BadRequest().body(e.to_string())
        }
    }
}

async fn exec_handler(
    service: web::Data<PingQueryService>,
    request: web::Json<ExecRequest>,
) -> HttpResponse {
    match service.get_ref().exec(request.into_inner()).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => {
            debug!("error: {:?}", e);
            HttpResponse::BadRequest().body(e.to_string())
        }
    }
}

async fn diagnostics_handler(service: web::Data<PingQueryService>) -> HttpResponse {
    match service.get_ref().diagnostics().await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => {
            debug!("error: {:?}", e);
            HttpResponse::BadRequest().body(e.to_string())
        }
    }
}

async fn get_config_handler(service: web::Data<PingQueryService>) -> HttpResponse {
    match service.get_ref().get_config().await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => {
            debug!("error: {:?}", e);
            HttpResponse::BadRequest().body(e.to_string())
        }
    }
}

async fn set_config_handler(
    service: web::Data<PingQueryService>,
    request: web::Json<SetConfigRequest>,
) -> HttpResponse {
    let request = request.into_inner();
    debug!("set config: {:?}", request);
    match service.get_ref().set_config(request).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => {
            debug!("error: {:?}", e);
            HttpResponse::BadRequest().body(e.to_string())
        }
    }
}

async fn interact_handler(
    service: web::Data<PingQueryService>,
    r: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let service = service.into_inner();
    let session = ClientActor {
        persistence: service.persistence.clone(),
        listener: service.listener.clone(),
    };
    ws::start(session, &r, stream)
}

async fn p404() -> HttpResponse {
    HttpResponse::NotFound().body("Not Found")
}

#[derive(StructOpt)]
struct CliFlags {
    #[structopt(long)]
    data: Option<PathBuf>,
    #[structopt(long)]
    metadata: Option<PathBuf>,
}

fn sqlite(path: Option<PathBuf>) -> anyhow::Result<Pool<SqliteConnectionManager>> {
    let manager = match path {
        None => SqliteConnectionManager::memory(),
        Some(path) => SqliteConnectionManager::file(path),
    };
    let pool = Pool::new(manager)?;
    Ok(pool)
}
