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

    let metadata_connection = match flags.metadata {
        None => rusqlite::Connection::open_in_memory()?,
        Some(path) => rusqlite::Connection::open_with_flags(path, OpenFlags::default())?,
    };
    let data_connection = match flags.data {
        None => rusqlite::Connection::open_in_memory()?,
        Some(path) => rusqlite::Connection::open_with_flags(path, OpenFlags::default())?,
    };
    let service = PingQueryService {
        metadata: Arc::new(Mutex::new(metadata_connection)),
        data: Arc::new(Mutex::new(data_connection)),
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
