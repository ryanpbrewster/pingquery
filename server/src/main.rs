use std::{path::PathBuf, sync::Arc};

use log::info;
use pingquery::{
    persistence::Persistence, proto::api::ping_query_server::PingQueryServer,
    server::PingQueryService,
};
use r2d2_sqlite::SqliteConnectionManager;

use structopt::StructOpt;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let flags = CliFlags::from_args();

    let persistence = Persistence {
        metadata: sqlite(flags.metadata)?,
        data: sqlite(flags.data)?,
    };
    let service = PingQueryService {
        persistence: Arc::new(persistence),
    };

    let addr = "[::1]:50051".parse().unwrap();
    info!("listening @ {}", addr);
    Server::builder()
        .add_service(PingQueryServer::new(service))
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
