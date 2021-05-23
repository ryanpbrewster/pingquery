use std::{pin::Pin, sync::{Arc, Mutex}};

use crate::{config::{MutateConfig, QueryConfig}, proto::api::{Config, ExecRequest, ExecResponse, GetConfigRequest, GetConfigResponse, InteractRequest, InteractResponse, SetConfigRequest, SetConfigResponse, ping_query_server::PingQuery}};
use futures_core::Stream;
use log::trace;

use tonic::{Request, Response, Status, Streaming};

pub struct PingQueryService {
    pub metadata: Arc<Mutex<rusqlite::Connection>>,
}

#[tonic::async_trait]
impl PingQuery for PingQueryService {
    async fn get_config(
        &self,
        request: Request<GetConfigRequest>,
    ) -> Result<Response<GetConfigResponse>, Status> {
        trace!("get_config: {:?}", request.into_inner());
        let metadata = self.metadata.lock().unwrap();
        metadata.execute(r#"
            CREATE TABLE IF NOT EXISTS queries (
                name TEXT NOT NULL PRIMARY KEY,
                sql_template TEXT NOT NULL
            )
        "#, []).unwrap();
        metadata.execute(r#"
            CREATE TABLE IF NOT EXISTS mutates (
                name TEXT NOT NULL PRIMARY KEY,
                sql_template TEXT NOT NULL
            )
        "#, []).unwrap();
        let queries: Vec<QueryConfig> = {
            let mut stmt = metadata.prepare("SELECT * FROM queries").unwrap();
            stmt.query_map([], |row| {
                Ok(QueryConfig {
                    name: row.get_unwrap("name"),
                    sql_template: row.get_unwrap("sql_template"),
                })
            }).unwrap().collect::<Result<_, _>>().unwrap()
        };
        let mutates: Vec<MutateConfig> = {
            let mut stmt = metadata.prepare("SELECT * FROM mutates").unwrap();
            stmt.query_map([], |row| {
                Ok(MutateConfig {
                    name: row.get_unwrap("name"),
                    sql_template: row.get_unwrap("sql_template"),
                })
            }).unwrap().collect::<Result<_, _>>().unwrap()
        };
        let config = crate::config::Config {
            queries: queries.into_iter().map(|c| (c.name.clone(), c)).collect(),
            mutates: mutates.into_iter().map(|c| (c.name.clone(), c)).collect(),
        };
        Ok(Response::new(GetConfigResponse { config: Some(config.into()) }))
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
