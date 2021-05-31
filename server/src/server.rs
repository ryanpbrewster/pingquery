use std::sync::Arc;

use crate::{
    persistence::Persistence,
    proto::api::{
        ping_query_server::PingQuery, ExecRequest, ExecResponse, GetConfigRequest,
        GetConfigResponse, InteractRequest, InteractResponse, SetConfigRequest, SetConfigResponse,
    },
};

use log::trace;

use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

pub struct PingQueryService {
    pub persistence: Arc<Persistence>,
}

#[tonic::async_trait]
impl PingQuery for PingQueryService {
    async fn get_config(
        &self,
        request: Request<GetConfigRequest>,
    ) -> Result<Response<GetConfigResponse>, Status> {
        trace!("get_config: {:?}", request.get_ref());
        let resp = self.persistence.get_config(request.into_inner()).await?;
        Ok(Response::new(resp))
    }

    async fn set_config(
        &self,
        request: Request<SetConfigRequest>,
    ) -> Result<Response<SetConfigResponse>, Status> {
        trace!("set_config: {:?}", request.get_ref());
        self.persistence.set_config(request.into_inner()).await?;
        Ok(Response::new(SetConfigResponse::default()))
    }

    async fn exec(&self, request: Request<ExecRequest>) -> Result<Response<ExecResponse>, Status> {
        trace!("exec: {:?}", request.get_ref());
        let resp = self.persistence.exec(request.into_inner()).await?;
        Ok(Response::new(resp))
    }

    type InteractStream = ReceiverStream<Result<InteractResponse, Status>>;

    async fn interact(
        &self,
        request: Request<Streaming<InteractRequest>>,
    ) -> Result<Response<Self::InteractStream>, Status> {
        trace!("interact: [START]");
        let persistence = self.persistence.clone();
        let (tx, rx) = tokio::sync::mpsc::channel(16);
        let inputs = request.into_inner();
        tokio::spawn(async move {
            persistence.interact(inputs, tx).await;
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
