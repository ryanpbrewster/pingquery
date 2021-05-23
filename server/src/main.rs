use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use log::info;
use pingquery::{proto::api::ping_query_server::PingQueryServer, server::PingQueryService};
use rusqlite::OpenFlags;
use structopt::StructOpt;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let flags = CliFlags::from_args();

    let metadata_connection =
        rusqlite::Connection::open_with_flags(flags.metadata, OpenFlags::default())?;
    let service = PingQueryService {
        metadata: Arc::new(Mutex::new(metadata_connection)),
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
    metadata: PathBuf,
}
