use ping_query::{proto::api::ping_query_server::PingQueryServer, server::PingQueryService};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    let service = PingQueryService;
    Server::builder()
        .add_service(PingQueryServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
