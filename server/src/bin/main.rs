use std::{path::PathBuf, sync::Arc};

use actix::{Actor, AsyncContext, StreamHandler, WrapFuture};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, middleware, web::{self, Data}};

use actix_web_actors::ws;
use log::{debug, info};
use pingquery::{actor::{ClientHandle, ListenActor}, diagnostics::Diagnostics, persistence::Persistence, proto::api::{DiagnosticsRequest, ExecRequest, InitializeRequest, InteractResponse, SetConfigRequest}, server::PingQueryService};
use r2d2_sqlite::SqliteConnectionManager;

use structopt::StructOpt;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tonic::Status;

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
            .service(
                web::resource("/diagnostics")
                    .app_data(Data::new(service.clone()))
                    .route(web::post().to(diagnostics_handler)),
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

async fn diagnostics_handler(
    service: web::Data<PingQueryService>,
    request: web::Json<DiagnosticsRequest>,
) -> HttpResponse {
    match service.get_ref().diagnostics(request.into_inner()).await {
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
    let (tx, rx) = mpsc::unbounded_channel();
    let handle = service.get_ref().interact(tx);
    let session = InteractSession {
        outputs: rx,
        client: handle,
    };
    ws::start(session, &r, stream)
}


struct InteractSession {
    outputs: UnboundedReceiver<Result<InteractResponse, Status>>,
    client: ClientHandle,
}

impl Actor for InteractSession {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for InteractSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        info!("[WS] incoming: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            _ => (),
        }
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
