use std::{path::PathBuf, sync::Arc};

use log::info;
use pingquery::{
    actor::ListenActor, diagnostics::Diagnostics, persistence::Persistence,
    proto::api::ping_query_server::PingQueryServer, server::PingQueryService,
};
use r2d2_sqlite::SqliteConnectionManager;

use structopt::StructOpt;

#[tokio::main]
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
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(pingquery::proto::FILE_DESCRIPTOR_SET)
        .build()?;

    let addr = "[::]:50051".parse().unwrap();
    info!("listening @ {}", addr);
    tonic::transport::Server::builder()
        .add_service(PingQueryServer::new(service))
        .add_service(reflection)
        .serve(addr)
        .await?;
    Ok(())
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
