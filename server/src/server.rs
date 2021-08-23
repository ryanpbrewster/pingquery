use std::{convert::TryInto, sync::Arc};

use crate::{
    actor::ListenActor,
    config::Config,
    persistence::Persistence,
    proto::api::{
        DiagnosticsResponse, ExecRequest, ExecResponse, GetConfigResponse, InitializeRequest,
        InitializeResponse, SetConfigRequest, SetConfigResponse,
    },
};
use anyhow::{anyhow, Result};

use actix::Addr;

#[derive(Clone)]
pub struct PingQueryService {
    pub persistence: Arc<Persistence>,
    pub listener: Addr<ListenActor>,
}

impl PingQueryService {
    pub async fn initialize(&self, _request: InitializeRequest) -> Result<InitializeResponse> {
        self.persistence.init().await?;
        Ok(InitializeResponse::default())
    }
    pub async fn diagnostics(&self) -> Result<DiagnosticsResponse> {
        let report = self.persistence.diagnostics().await?;
        Ok(report.into())
    }
    pub async fn get_config(&self) -> Result<GetConfigResponse> {
        let config = self.persistence.get_config()?;
        Ok(GetConfigResponse {
            config: Some(config.into()),
        })
    }

    pub async fn set_config(&self, request: SetConfigRequest) -> Result<SetConfigResponse> {
        let config: Config = request
            .config
            .ok_or_else(|| anyhow!("missing config"))?
            .try_into()?;
        self.persistence.set_config(config)?;
        Ok(SetConfigResponse::default())
    }

    pub async fn exec(&self, request: ExecRequest) -> Result<ExecResponse> {
        self.persistence.exec(request)
    }
}
