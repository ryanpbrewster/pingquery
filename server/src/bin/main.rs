use std::{path::PathBuf, sync::Arc, time::Instant};

use actix::{Actor, StreamHandler};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, middleware, web::{self, Data}};
use actix_web_actors::ws;

use log::info;
use pingquery::{actor::ListenActor, diagnostics::Diagnostics, persistence::Persistence, proto::api::{DiagnosticsRequest, ExecRequest, ExecResponse, InitializeRequest, InitializeResponse}, server::PingQueryService};
use r2d2_sqlite::SqliteConnectionManager;

use structopt::StructOpt;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let flags = CliFlags::from_args();

    let persistence = Persistence {
        metadata: sqlite(flags.metadata)?,
        data: sqlite(flags.data)?,
        diagnostics: Arc::new(Diagnostics::default()),
    };
    let service = PingQueryService {
        persistence: Arc::new(persistence),
        listener: ListenActor::start(),
    };
    let addr = "[::]:8080";
    info!("listening @ {}", addr);
    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/diagnostics").app_data(Data::new(service.clone())).route(web::post().to(diagnostics_handler)))
            .service(web::resource("/exec").app_data(Data::new(service.clone())).route(web::post().to(exec_handler)))
            .service(web::resource("/initialize").app_data(Data::new(service.clone())).route(web::post().to(initialize_handler)))
            .default_service(web::route().to(p404))
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}

async fn initialize_handler(service: web::Data<PingQueryService>, request: web::Json<InitializeRequest>) -> HttpResponse {
    match service.get_ref().initialize(request.into_inner()).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

async fn exec_handler(service: web::Data<PingQueryService>, request: web::Json<ExecRequest>) -> HttpResponse {
    match service.get_ref().exec(request.into_inner()).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

async fn diagnostics_handler(service: web::Data<PingQueryService>, request: web::Json<DiagnosticsRequest>) -> HttpResponse {
    match service.get_ref().diagnostics(request.into_inner()).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
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

fn sqlite(path: Option<PathBuf>) -> Result<r2d2::Pool<SqliteConnectionManager>, r2d2::Error> {
    let manager = match path {
        None => SqliteConnectionManager::memory(),
        Some(path) => SqliteConnectionManager::file(path),
    };
    r2d2::Pool::new(manager)
}
