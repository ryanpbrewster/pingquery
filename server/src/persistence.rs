use std::{collections::BTreeMap, convert::TryInto};

use crate::{
    config::{Config, MutateConfig, QueryConfig},
    proto::api::{self, ExecRequest, ExecResponse, InteractRequest, InteractResponse},
    requests::Interaction,
    value::{Row, Value},
};

use log::trace;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{
    types::{ToSqlOutput, ValueRef},
    ToSql,
};
use tokio::sync::{
    mpsc::{self, Sender},
    watch,
};
use tonic::{Status, Streaming};

pub struct Persistence {
    pub metadata: Pool<SqliteConnectionManager>,
    pub data: Pool<SqliteConnectionManager>,
}

impl Persistence {
    pub async fn init(&self) -> Result<(), Status> {
        trace!("init");
        let mut conn = self.metadata.get().unwrap();
        let txn = conn.transaction().unwrap();
        init_tables(&txn);
        txn.commit().unwrap();
        Ok(())
    }
    pub async fn get_config(&self) -> Result<Config, Status> {
        trace!("get_config");
        let mut conn = self.metadata.get().unwrap();
        let txn = conn.transaction().unwrap();
        let (queries, mutates) = read_config(&txn);
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
        clear_tables(&txn);
        write_tables(&txn, &config);
        txn.commit().unwrap();
        trace!("committed config {:?}", config);
        Ok(())
    }

    pub async fn exec(&self, request: ExecRequest) -> Result<ExecResponse, Status> {
        trace!("exec: {:?}", request);
        let raw_sql = request.raw_sql;
        let conn = self.data.get().unwrap();
        let mut stmt = conn
            .prepare(&raw_sql)
            .map_err(|_| Status::invalid_argument("invalid sql"))?;
        let rows: Vec<Row> = stmt
            .query_map([], |row| Ok(row_from_sql(row)))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        Ok(ExecResponse {
            rows: rows.into_iter().map(|r| r.into()).collect(),
        })
    }

    pub async fn do_interact(
        &self,
        req: api::InteractRequest,
    ) -> Result<api::InteractResponse, Status> {
        let id = req.id;
        let req: Result<Interaction, Status> = req.try_into();
        let rows = match req {
            Err(err) => {
                return Err(err);
            }
            Ok(Interaction::Query { name, params }) => self.do_query(&name, &params).await?,
            Ok(Interaction::Mutate { name, params }) => self.do_query(&name, &params).await?,
            Ok(Interaction::Listen { name, params }) => self.do_query(&name, &params).await?,
        };
        return Ok(api::InteractResponse {
            id,
            rows: rows.into_iter().map(|row| row.into()).collect(),
        });
    }

    async fn do_query(&self, name: &str, params: &Row) -> Result<Vec<Row>, Status> {
        let sql_template = {
            let mut conn = self.metadata.get().unwrap();
            let txn = conn.transaction().unwrap();
            find_template(&txn, name)
                .ok_or_else(|| Status::invalid_argument(&format!("no such statement: {}", name)))?
        };
        let rows = {
            let mut conn = self.data.get().unwrap();
            let txn = conn.transaction().unwrap();
            let rows = do_stmt(&txn, &sql_template, params)?;
            txn.commit().unwrap();
            rows
        };
        Ok(rows)
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

fn write_tables(txn: &rusqlite::Transaction, config: &Config) {
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

fn read_config(txn: &rusqlite::Transaction) -> (Vec<QueryConfig>, Vec<MutateConfig>) {
    let queries: Vec<QueryConfig> = {
        let mut stmt = txn.prepare("SELECT * FROM queries").unwrap();
        stmt.query_map([], |row| Ok(query_from_sql(row)))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap()
    };
    let mutates: Vec<MutateConfig> = {
        let mut stmt = txn.prepare("SELECT * FROM mutates").unwrap();
        stmt.query_map([], |row| Ok(mutate_from_sql(row)))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap()
    };
    (queries, mutates)
}

fn find_template(txn: &rusqlite::Transaction, name: &str) -> Option<String> {
    let (queries, mutates) = read_config(txn);
    if let Some(q) = queries.into_iter().find(|q| q.name == name) {
        return Some(q.sql_template);
    };
    if let Some(m) = mutates.into_iter().find(|q| q.name == name) {
        return Some(m.sql_template);
    };
    None
}

fn do_stmt(
    txn: &rusqlite::Transaction,
    sql_template: &str,
    params: &Row,
) -> Result<Vec<Row>, Status> {
    let mut stmt = txn
        .prepare(sql_template)
        .map_err(|e| Status::invalid_argument(&format!("invalid sql: {}", e)))?;
    let params: Vec<(&str, &dyn ToSql)> = params
        .columns
        .iter()
        .map(|(name, value)| (name.as_ref(), value as &dyn ToSql))
        .collect();
    let rows: Vec<Row> = stmt
        .query_map(params.as_slice(), |row| Ok(row_from_sql(row)))
        .map_err(|_| Status::invalid_argument("failed to query"))?
        .collect::<Result<_, _>>()
        .map_err(|_| Status::invalid_argument("failed to collect rows"))?;
    Ok(rows)
}

fn query_from_sql(row: &rusqlite::Row) -> QueryConfig {
    QueryConfig {
        name: row.get_unwrap("name"),
        sql_template: row.get_unwrap("sql_template"),
        listen: vec![],
    }
}
fn mutate_from_sql(row: &rusqlite::Row) -> MutateConfig {
    MutateConfig {
        name: row.get_unwrap("name"),
        sql_template: row.get_unwrap("sql_template"),
        notify: vec![],
    }
}
fn row_from_sql(row: &rusqlite::Row) -> Row {
    let columns = row
        .column_names()
        .into_iter()
        .map(|s| (s.to_owned(), row.get_unwrap(s)))
        .collect();
    Row { columns }
}
impl ToSql for Value {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let vref: ValueRef = self.into();
        Ok(ToSqlOutput::Borrowed(vref))
    }
}
