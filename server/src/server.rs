use std::pin::Pin;

use crate::proto::api::{ping_query_server::PingQuery, InteractResponse};
use futures_core::Stream;
use log::trace;

use tonic::Status;

pub struct PingQueryService;

#[tonic::async_trait]
impl PingQuery for PingQueryService {
    async fn get_config(
        &self,
        request: tonic::Request<crate::proto::api::GetConfigRequest>,
    ) -> Result<tonic::Response<crate::proto::api::GetConfigResponse>, tonic::Status> {
        trace!("get_config: {:?}", request);
        Err(tonic::Status::unimplemented("get_config unimplemented"))
    }

    async fn set_config(
        &self,
        request: tonic::Request<crate::proto::api::SetConfigRequest>,
    ) -> Result<tonic::Response<crate::proto::api::SetConfigResponse>, tonic::Status> {
        trace!("set_config: {:?}", request);
        Err(tonic::Status::unimplemented("set_config unimplemented"))
    }

    async fn exec(
        &self,
        request: tonic::Request<crate::proto::api::ExecRequest>,
    ) -> Result<tonic::Response<crate::proto::api::ExecRequest>, tonic::Status> {
        trace!("exec: {:?}", request);
        Err(tonic::Status::unimplemented("exec unimplemented"))
    }

    type InteractStream =
        Pin<Box<dyn Stream<Item = Result<InteractResponse, Status>> + Send + Sync + 'static>>;

    async fn interact(
        &self,
        request: tonic::Request<tonic::Streaming<crate::proto::api::InteractRequest>>,
    ) -> Result<tonic::Response<Self::InteractStream>, tonic::Status> {
        trace!("interact: {:?}", request);
        Err(tonic::Status::unimplemented("interact unimplemented"))
    }
}
