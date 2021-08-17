use std::{convert::TryInto, sync::Arc};

use crate::{
    actor::{ClientActor, ClientHandle, ClientMsg, ListenHandle},
    config::Config,
    persistence::Persistence,
    proto::api::{
        DiagnosticsRequest, DiagnosticsResponse, ExecRequest, ExecResponse, GetConfigResponse,
        InitializeRequest, InitializeResponse, InteractRequest, InteractResponse, SetConfigRequest,
        SetConfigResponse,
    },
};

use actix::{Message, Recipient};
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status, Streaming};

#[derive(Clone)]
pub struct PingQueryService {
    pub persistence: Arc<Persistence>,
    pub listener: ListenHandle,
}

impl PingQueryService {
    pub async fn initialize(
        &self,
        _request: InitializeRequest,
    ) -> Result<InitializeResponse, Status> {
        self.persistence.init().await?;
        Ok(InitializeResponse::default())
    }
    pub async fn diagnostics(
        &self,
    ) -> Result<DiagnosticsResponse, Status> {
        let report = self.persistence.diagnostics().await?;
        Ok(report.into())
    }
    pub async fn get_config(&self) -> Result<GetConfigResponse, Status> {
        let config = self.persistence.get_config()?;
        Ok(GetConfigResponse {
            config: Some(config.into()),
        })
    }

    pub async fn set_config(&self, request: SetConfigRequest) -> Result<SetConfigResponse, Status> {
        let config: Config = request
            .config
            .ok_or_else(|| Status::invalid_argument("missing config"))?
            .try_into()?;
        self.persistence.set_config(config)?;
        Ok(SetConfigResponse::default())
    }

    pub async fn exec(&self, request: ExecRequest) -> Result<ExecResponse, Status> {
        self.persistence.exec(request)
    }

    pub fn interact(
        &self,
        addr: Recipient<PQResult>,
    ) -> ClientHandle {
        let persistence = self.persistence.clone();
        let listener = self.listener.clone();
        ClientActor::start(addr, persistence, listener)
    }
}

pub struct PQResult(pub Result<InteractResponse, Status>);
impl Message for PQResult {
    type Result = ();
}