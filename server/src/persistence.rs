use crate::{
    config::{Config, MutateConfig, QueryConfig},
    proto::api::{ExecRequest, ExecResponse, InteractRequest, InteractResponse},
    value::Row,
};

use log::trace;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::sync::mpsc::Sender;
use tonic::{Status, Streaming};

pub struct Persistence {
    pub metadata: Pool<SqliteConnectionManager>,
    pub data: Pool<SqliteConnectionManager>,
}

impl Persistence {
    pub async fn get_config(&self) -> Result<Config, Status> {
        trace!("get_config");
        let mut conn = self.metadata.get().unwrap();
        let txn = conn.transaction().unwrap();
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
        Ok(config)
    }

    pub async fn set_config(&self, config: Config) -> Result<(), Status> {
        trace!("set_config: {:?}", config);

        let mut conn = self.metadata.get().unwrap();
        let txn = conn.transaction().unwrap();
        init_tables(&txn);
        clear_tables(&txn);
        write_tables(&txn, config);
        txn.commit().unwrap();
        Ok(())
    }

    pub async fn exec(&self, request: ExecRequest) -> Result<ExecResponse, Status> {
        trace!("exec: {:?}", request);
        let raw_sql = request.raw_sql;
        let conn = self.data.get().unwrap();
        let mut stmt = conn.prepare(&raw_sql).unwrap();
        let rows: Vec<Row> = stmt
            .query_map([], |row| {
                let columns = row
                    .column_names()
                    .into_iter()
                    .map(|s| (s.to_owned(), row.get_unwrap(s)))
                    .collect();
                trace!("row = {:?}", columns);
                Ok(Row { columns })
            })
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        Ok(ExecResponse {
            rows: rows.into_iter().map(|r| r.into()).collect(),
        })
    }

    pub async fn interact(
        &self,
        mut inputs: Streaming<InteractRequest>,
        outputs: Sender<Result<InteractResponse, Status>>,
    ) {
        trace!("interact: [START]");
        while let Some(req) = inputs.message().await.unwrap() {
            trace!("interact: [RECV] {:?}", req);
            outputs.send(Ok(InteractResponse::default())).await.unwrap();
        }
        trace!("interact: [DONE]");
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
