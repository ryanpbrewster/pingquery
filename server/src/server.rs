use std::{convert::TryInto, sync::Arc};

use crate::{
    actor::{ClientActor, ClientMsg, ListenHandle},
    config::Config,
    persistence::Persistence,
    proto::api::{
        ping_query_server::PingQuery, DiagnosticsRequest, DiagnosticsResponse, ExecRequest,
        ExecResponse, GetConfigRequest, GetConfigResponse, InitializeRequest, InitializeResponse,
        InteractRequest, InteractResponse, SetConfigRequest, SetConfigResponse,
    },
};

use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status, Streaming};

pub struct PingQueryService {
    pub persistence: Arc<Persistence>,
    pub listener: ListenHandle,
}

#[tonic::async_trait]
impl PingQuery for PingQueryService {
    async fn initialize(
        &self,
        _request: Request<InitializeRequest>,
    ) -> Result<Response<InitializeResponse>, Status> {
        self.persistence.init().await?;
        Ok(Response::new(InitializeResponse::default()))
    }
    async fn diagnostics(
        &self,
        _request: Request<DiagnosticsRequest>,
    ) -> Result<Response<DiagnosticsResponse>, Status> {
        Err(Status::unimplemented("diagnostics unimplemented"))
    }
    async fn get_config(
        &self,
        _request: Request<GetConfigRequest>,
    ) -> Result<Response<GetConfigResponse>, Status> {
        let config = self.persistence.get_config()?;
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
            .ok_or_else(|| Status::invalid_argument("missing config"))?
            .try_into()?;
        self.persistence.set_config(config)?;
        Ok(Response::new(SetConfigResponse::default()))
    }

    async fn exec(&self, request: Request<ExecRequest>) -> Result<Response<ExecResponse>, Status> {
        let resp = self.persistence.exec(request.into_inner())?;
        Ok(Response::new(resp))
    }

    type InteractStream = UnboundedReceiverStream<Result<InteractResponse, Status>>;

    async fn interact(
        &self,
        request: Request<Streaming<InteractRequest>>,
    ) -> Result<Response<Self::InteractStream>, Status> {
        let persistence = self.persistence.clone();
        let listener = self.listener.clone();
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        let handle = ClientActor::start(tx, persistence, listener);
        let mut inputs = request.into_inner();
        tokio::spawn(async move {
            while let Ok(Some(msg)) = inputs.message().await {
                handle
                    .sender
                    .send(ClientMsg::User(msg))
                    .map_err(|_| Status::internal("failed to pipe InteractRequest to actor"))
                    .unwrap();
            }
            handle.sender.send(ClientMsg::End).unwrap();
        });
        Ok(Response::new(UnboundedReceiverStream::new(rx)))
    }
}
