use std::{
    collections::BTreeMap,
    convert::TryInto,
    pin::Pin,
    sync::{Arc, Mutex},
};

use crate::{
    config::{Config, MutateConfig, QueryConfig},
    proto::api::{
        ping_query_server::PingQuery, ExecRequest, ExecResponse, GetConfigRequest,
        GetConfigResponse, InteractRequest, InteractResponse, SetConfigRequest, SetConfigResponse,
    },
    value::Value,
};
use futures_core::Stream;
use log::trace;

use tonic::{Request, Response, Status, Streaming};

pub struct PingQueryService {
    pub metadata: Arc<Mutex<rusqlite::Connection>>,
    pub data: Arc<Mutex<rusqlite::Connection>>,
}

#[tonic::async_trait]
impl PingQuery for PingQueryService {
    async fn get_config(
        &self,
        request: Request<GetConfigRequest>,
    ) -> Result<Response<GetConfigResponse>, Status> {
        trace!("get_config: {:?}", request.get_ref());
        let mut lock = self.metadata.lock().unwrap();
        let txn = lock.transaction().unwrap();
        init_tables(&txn);
        let queries: Vec<QueryConfig> = {
            let mut stmt = txn.prepare("SELECT * FROM queries").unwrap();
            stmt.query_map([], |row| {
                Ok(QueryConfig {
                    name: row.get_unwrap("name"),
                    sql_template: row.get_unwrap("sql_template"),
                })
            })
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap()
        };
        let mutates: Vec<MutateConfig> = {
            let mut stmt = txn.prepare("SELECT * FROM mutates").unwrap();
            stmt.query_map([], |row| {
                Ok(MutateConfig {
                    name: row.get_unwrap("name"),
                    sql_template: row.get_unwrap("sql_template"),
                })
            })
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap()
        };
        txn.commit().unwrap();
        let config = Config {
            queries: queries.into_iter().map(|c| (c.name.clone(), c)).collect(),
            mutates: mutates.into_iter().map(|c| (c.name.clone(), c)).collect(),
        };
        Ok(Response::new(GetConfigResponse {
            config: Some(config.into()),
        }))
    }

    async fn set_config(
        &self,
        request: Request<SetConfigRequest>,
    ) -> Result<Response<SetConfigResponse>, Status> {
        trace!("set_config: {:?}", request.get_ref());
        let config: Config = request
            .into_inner()
            .config
            .ok_or(Status::invalid_argument("missing config"))?
            .try_into()?;

        let mut lock = self.metadata.lock().unwrap();
        let txn = lock.transaction().unwrap();
        init_tables(&txn);
        clear_tables(&txn);
        write_tables(&txn, config);
        txn.commit().unwrap();
        Ok(Response::new(SetConfigResponse::default()))
    }

    async fn exec(&self, request: Request<ExecRequest>) -> Result<Response<ExecResponse>, Status> {
        trace!("exec: {:?}", request.get_ref());
        let raw_sql = request.into_inner().raw_sql;
        let lock = self.data.lock().unwrap();
        let mut stmt = lock.prepare(&raw_sql).unwrap();
        let rows: Vec<BTreeMap<String, Value>> = stmt
            .query_map([], |row| {
                let row = row
                    .column_names()
                    .into_iter()
                    .map(|s| (s.to_owned(), row.get_unwrap(s)))
                    .collect();
                trace!("row = {:?}", row);
                Ok(row)
            })
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        Ok(Response::new(ExecResponse {
            rows: rows.into_iter().map(|r| r.into()).collect(),
        }))
    }

    type InteractStream =
        Pin<Box<dyn Stream<Item = Result<InteractResponse, Status>> + Send + Sync + 'static>>;

    async fn interact(
        &self,
        request: Request<Streaming<InteractRequest>>,
    ) -> Result<Response<Self::InteractStream>, Status> {
        trace!("interact: {:?}", request.get_ref());
        Err(Status::unimplemented("interact unimplemented"))
    }
}

fn init_tables(txn: &rusqlite::Transaction) {
    txn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS queries (
            name TEXT NOT NULL PRIMARY KEY,
            sql_template TEXT NOT NULL
        )
    "#,
        [],
    )
    .unwrap();
    txn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS mutates (
            name TEXT NOT NULL PRIMARY KEY,
            sql_template TEXT NOT NULL
        )
    "#,
        [],
    )
    .unwrap();
}

fn clear_tables(txn: &rusqlite::Transaction) {
    txn.execute("DELETE FROM queries", []).unwrap();
    txn.execute("DELETE FROM mutates", []).unwrap();
}

fn write_tables(txn: &rusqlite::Transaction, config: Config) {
    let mut qstmt = txn
        .prepare("INSERT INTO queries (name, sql_template) VALUES (?, ?)")
        .unwrap();
    for query in config.queries.values() {
        qstmt.execute([&query.name, &query.sql_template]).unwrap();
    }
    let mut mstmt = txn
        .prepare("INSERT INTO mutates (name, sql_template) VALUES (?, ?)")
        .unwrap();
    for mutate in config.mutates.values() {
        mstmt.execute([&mutate.name, &mutate.sql_template]).unwrap();
    }
}
