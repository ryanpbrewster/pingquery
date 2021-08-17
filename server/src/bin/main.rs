use std::{path::PathBuf, sync::Arc};

use actix::{Actor, AsyncContext, Handler, StreamHandler};
use actix_web::{
    middleware,
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};

use actix_web_actors::ws;
use log::{debug, info, warn};
use pingquery::{
    actor::{ClientHandle, ClientMsg, ListenActor},
    diagnostics::Diagnostics,
    persistence::Persistence,
    proto::api::{ExecRequest, InitializeRequest, InteractRequest, SetConfigRequest},
    server::{PQResult, PingQueryService},
};
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
    let session = InteractSession {
        service: service.into_inner(),
        client: None,
    };
    let (_addr, resp) = ws::start_with_addr(session, &r, stream)?;
    Ok(resp)
}

struct InteractSession {
    service: Arc<PingQueryService>,
    client: Option<ClientHandle>,
}

impl Actor for InteractSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.client = Some(self.service.interact(ctx.address().recipient()));
    }
    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        if let Some(client) = &self.client {
            client.sender.send(ClientMsg::End).unwrap();
        }
        actix::Running::Stop
    }
}

impl Handler<PQResult> for InteractSession {
    type Result = ();

    fn handle(&mut self, msg: PQResult, ctx: &mut Self::Context) -> Self::Result {
        match msg.0 {
            Ok(v) => ctx.text(serde_json::to_string(&v).unwrap()),
            Err(_e) => ctx.close(None),
        }
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for InteractSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        info!("[WS] incoming: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let req: InteractRequest = match serde_json::from_slice(text.as_bytes()) {
                    Ok(v) => v,
                    Err(e) => {
                        ctx.text(e.to_string());
                        ctx.close(None);
                        return;
                    }
                };
                info!("[WS] req: {:?}", req);
                self.client
                    .as_ref()
                    .unwrap()
                    .sender
                    .send(ClientMsg::User(req))
                    .unwrap();
            }
            Ok(ws::Message::Close(r)) => {
                ctx.close(r);
            }
            Err(e) => {
                warn!("[WS] closing because of {}", e);
                ctx.close(None);
            }
            Ok(_) => {}
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
