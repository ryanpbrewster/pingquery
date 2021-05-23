use log::info;
use pingquery::{proto::api::ping_query_server::PingQueryServer, server::PingQueryService};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let service = PingQueryService;

    let addr = "[::1]:50051".parse().unwrap();
    info!("listening @ {}", addr);
    Server::builder()
        .add_service(PingQueryServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
