use std::{convert::TryInto, sync::Arc};

use crate::{
    config::Config,
    persistence::Persistence,
    proto::api::{
        ping_query_server::PingQuery, ExecRequest, ExecResponse, GetConfigRequest,
        GetConfigResponse, InteractRequest, InteractResponse, SetConfigRequest, SetConfigResponse,
    },
};

use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

pub struct PingQueryService {
    pub persistence: Arc<Persistence>,
}

#[tonic::async_trait]
impl PingQuery for PingQueryService {
    async fn get_config(
        &self,
        _request: Request<GetConfigRequest>,
    ) -> Result<Response<GetConfigResponse>, Status> {
        let config = self.persistence.get_config().await?;
        Ok(Response::new(GetConfigResponse {
            config: Some(config.into()),
        }))
    }

    async fn set_config(
        &self,
        request: Request<SetConfigRequest>,
    ) -> Result<Response<SetConfigResponse>, Status> {
        let config: Config = request
            .into_inner()
            .config
            .ok_or(Status::invalid_argument("missing config"))?
            .try_into()?;
        self.persistence.set_config(config).await?;
        Ok(Response::new(SetConfigResponse::default()))
    }

    async fn exec(&self, request: Request<ExecRequest>) -> Result<Response<ExecResponse>, Status> {
        let resp = self.persistence.exec(request.into_inner()).await?;
        Ok(Response::new(resp))
    }

    type InteractStream = ReceiverStream<Result<InteractResponse, Status>>;

    async fn interact(
        &self,
        request: Request<Streaming<InteractRequest>>,
    ) -> Result<Response<Self::InteractStream>, Status> {
        let persistence = self.persistence.clone();
        let (tx, rx) = tokio::sync::mpsc::channel(16);
        let inputs = request.into_inner();
        tokio::spawn(async move {
            persistence.interact(inputs, tx).await;
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
