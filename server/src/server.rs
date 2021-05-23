use std::pin::Pin;

use crate::proto::api::{
    ping_query_server::PingQuery, ExecRequest, ExecResponse, GetConfigRequest, GetConfigResponse,
    InteractRequest, InteractResponse, SetConfigRequest, SetConfigResponse,
};
use futures_core::Stream;
use log::trace;

use tonic::{Request, Response, Status, Streaming};

pub struct PingQueryService;

#[tonic::async_trait]
impl PingQuery for PingQueryService {
    async fn get_config(
        &self,
        request: Request<GetConfigRequest>,
    ) -> Result<Response<GetConfigResponse>, Status> {
        trace!("get_config: {:?}", request.into_inner());
        Err(Status::unimplemented("get_config unimplemented"))
    }

    async fn set_config(
        &self,
        request: Request<SetConfigRequest>,
    ) -> Result<Response<SetConfigResponse>, Status> {
        trace!("set_config: {:?}", request.into_inner());
        Err(Status::unimplemented("set_config unimplemented"))
    }

    async fn exec(&self, request: Request<ExecRequest>) -> Result<Response<ExecResponse>, Status> {
        trace!("exec: {:?}", request.into_inner());
        Err(Status::unimplemented("exec unimplemented"))
    }

    type InteractStream =
        Pin<Box<dyn Stream<Item = Result<InteractResponse, Status>> + Send + Sync + 'static>>;

    async fn interact(
        &self,
        request: Request<Streaming<InteractRequest>>,
    ) -> Result<Response<Self::InteractStream>, Status> {
        trace!("interact: {:?}", request.into_inner());
        Err(Status::unimplemented("interact unimplemented"))
    }
}
