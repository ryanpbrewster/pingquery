use std::{convert::TryInto, sync::Arc};

use crate::{
    actor::{ClientActor, ClientHandle, ListenHandle},
    config::Config,
    persistence::Persistence,
    proto::api::{
        DiagnosticsResponse, ExecRequest, ExecResponse, GetConfigResponse, InitializeRequest,
        InitializeResponse, InteractResponse, SetConfigRequest, SetConfigResponse,
    },
};
use anyhow::anyhow;

use actix::{Message, Recipient};

#[derive(Clone)]
pub struct PingQueryService {
    pub persistence: Arc<Persistence>,
    pub listener: ListenHandle,
}

impl PingQueryService {
    pub async fn initialize(
        &self,
        _request: InitializeRequest,
    ) -> anyhow::Result<InitializeResponse> {
        self.persistence.init().await?;
        Ok(InitializeResponse::default())
    }
    pub async fn diagnostics(&self) -> anyhow::Result<DiagnosticsResponse> {
        let report = self.persistence.diagnostics().await?;
        Ok(report.into())
    }
    pub async fn get_config(&self) -> anyhow::Result<GetConfigResponse> {
        let config = self.persistence.get_config()?;
        Ok(GetConfigResponse {
            config: Some(config.into()),
        })
    }

    pub async fn set_config(&self, request: SetConfigRequest) -> anyhow::Result<SetConfigResponse> {
        let config: Config = request
            .config
            .ok_or_else(|| anyhow!("missing config"))?
            .try_into()?;
        self.persistence.set_config(config)?;
        Ok(SetConfigResponse::default())
    }

    pub async fn exec(&self, request: ExecRequest) -> anyhow::Result<ExecResponse> {
        self.persistence.exec(request)
    }

    pub fn interact(&self, addr: Recipient<PQResult>) -> ClientHandle {
        let persistence = self.persistence.clone();
        let listener = self.listener.clone();
        ClientActor::start(addr, persistence, listener)
    }
}

pub struct PQResult(pub anyhow::Result<InteractResponse>);
impl Message for PQResult {
    type Result = ();
}
