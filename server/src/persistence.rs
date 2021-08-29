use std::{convert::TryInto, sync::Arc};

use crate::{
    config::{Config, MutateConfig, Path, QueryConfig},
    diagnostics::{Diagnostics, DiagnosticsReport},
    proto::api::{self, ExecRequest, ExecResponse},
    value::{Row, Value},
};

use log::trace;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{
    types::{FromSql, FromSqlError, ToSqlOutput, ValueRef},
    ToSql,
};

use anyhow::{anyhow, Result};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug)]
pub struct Persistence {
    pub metadata: Pool<SqliteConnectionManager>,
    pub data: Pool<SqliteConnectionManager>,
    pub diagnostics: Arc<Diagnostics>,
}

impl Persistence {
    pub async fn init(&self) -> Result<()> {
        trace!("init");
        let mut conn = self.metadata.get()?;
        let txn = conn.transaction()?;
        init_tables(&txn)?;
        txn.commit()?;
        Ok(())
    }
    pub async fn diagnostics(&self) -> Result<DiagnosticsReport> {
        trace!("diagnostics");
        Ok(self.diagnostics.report())
    }

    pub fn get_config(&self) -> Result<Config> {
        trace!("get_config");
        let mut conn = self.metadata.get()?;
        let txn = conn.transaction()?;
        let (queries, mutates) = read_config(&txn)?;
        txn.commit()?;
        let config = Config {
            queries: queries.into_iter().map(|c| (c.name.clone(), c)).collect(),
            mutates: mutates.into_iter().map(|c| (c.name.clone(), c)).collect(),
        };
        Ok(config)
    }

    pub fn set_config(&self, config: Config) -> Result<()> {
        trace!("set_config: {:?}", config);
        let mut conn = self.metadata.get()?;
        let txn = conn.transaction()?;
        clear_tables(&txn)?;
        write_tables(&txn, &config)?;
        txn.commit()?;
        Ok(())
    }

    pub fn exec(&self, request: ExecRequest) -> Result<ExecResponse> {
        trace!("exec: {:?}", request);
        let raw_sql = request.raw_sql;
        let conn = self.data.get()?;
        let mut stmt = conn
            .prepare(&raw_sql)
            .map_err(|e| anyhow!("invalid sql: {}", e))?;
        let rows: Vec<Row> = stmt
            .query_map([], |row| Ok(row_from_sql(row)))?
            .collect::<Result<_, _>>()?;
        Ok(ExecResponse {
            rows: rows.into_iter().map(|r| r.into()).collect(),
        })
    }

    pub fn do_query(&self, name: String, sql_template: &str, params: &Row) -> Result<Vec<Row>> {
        self.diagnostics
            .queries
            .entry(name)
            .or_default()
            .num_executions
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut conn = self.data.get()?;
        let txn = conn.transaction()?;
        let rows = do_stmt(&txn, sql_template, params)?;
        txn.commit()?;
        Ok(rows)
    }
}

fn init_tables(txn: &rusqlite::Transaction) -> Result<()> {
    txn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS queries (
            name TEXT NOT NULL PRIMARY KEY,
            sql_template TEXT NOT NULL,
            listen TEXT NOT NULL
        )
    "#,
        [],
    )?;
    txn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS mutates (
            name TEXT NOT NULL PRIMARY KEY,
            sql_template TEXT NOT NULL,
            notify TEXT NOT NULL
        )
    "#,
        [],
    )?;
    Ok(())
}

fn clear_tables(txn: &rusqlite::Transaction) -> Result<()> {
    txn.execute("DELETE FROM queries", [])?;
    txn.execute("DELETE FROM mutates", [])?;
    Ok(())
}

fn write_tables(txn: &rusqlite::Transaction, config: &Config) -> Result<()> {
    let mut qstmt =
        txn.prepare("INSERT INTO queries (name, sql_template, listen) VALUES (?, ?, ?)")?;
    for query in config.queries.values() {
        let params: &[&dyn ToSql] = &[
            &query.name,
            &query.sql_template,
            &wrap_paths(query.listen.clone()),
        ];
        qstmt.execute(params)?;
    }
    let mut mstmt =
        txn.prepare("INSERT INTO mutates (name, sql_template, notify) VALUES (?, ?, ?)")?;
    for mutate in config.mutates.values() {
        let params: &[&dyn ToSql] = &[
            &mutate.name,
            &mutate.sql_template,
            &wrap_paths(mutate.notify.clone()),
        ];
        mstmt.execute(params)?;
    }
    Ok(())
}

fn read_config(txn: &rusqlite::Transaction) -> Result<(Vec<QueryConfig>, Vec<MutateConfig>)> {
    let queries: Vec<QueryConfig> = {
        let mut stmt = txn.prepare("SELECT * FROM queries")?;
        let rows = stmt
            .query_map([], |row| Ok(query_from_sql(row)))?
            .collect::<Result<_, _>>()?;
        rows
    };
    let mutates: Vec<MutateConfig> = {
        let mut stmt = txn.prepare("SELECT * FROM mutates")?;
        let rows = stmt
            .query_map([], |row| Ok(mutate_from_sql(row)))?
            .collect::<Result<_, _>>()?;
        rows
    };
    Ok((queries, mutates))
}

fn do_stmt(txn: &rusqlite::Transaction, sql_template: &str, params: &Row) -> Result<Vec<Row>> {
    let mut stmt = txn
        .prepare(sql_template)
        .map_err(|e| anyhow!("invalid sql: {}", e))?;
    let params: Vec<(&str, &dyn ToSql)> = params
        .columns
        .iter()
        .map(|(name, value)| (name.as_ref(), value as &dyn ToSql))
        .collect();
    let rows: Vec<Row> = stmt
        .query_map(params.as_slice(), |row| Ok(row_from_sql(row)))
        .map_err(|e| anyhow!("failed to query: {}", e))?
        .collect::<Result<_, _>>()
        .map_err(|_| anyhow!("failed to collect rows"))?;
    Ok(rows)
}

fn query_from_sql(row: &rusqlite::Row) -> QueryConfig {
    QueryConfig {
        name: row.get_unwrap("name"),
        sql_template: row.get_unwrap("sql_template"),
        listen: unwrap_paths(row.get_unwrap("listen")),
    }
}
fn mutate_from_sql(row: &rusqlite::Row) -> MutateConfig {
    MutateConfig {
        name: row.get_unwrap("name"),
        sql_template: row.get_unwrap("sql_template"),
        notify: unwrap_paths(row.get_unwrap("notify")),
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

struct JsonWrapper<T>(T);
impl<T: Serialize> ToSql for JsonWrapper<T> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(rusqlite::types::Value::Text(
            serde_json::to_string(&self.0).unwrap(),
        )))
    }
}
impl<T> FromSql for JsonWrapper<T>
where
    T: DeserializeOwned,
{
    fn column_result(value: ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        if let ValueRef::Text(raw) = value {
            return match serde_json::from_slice(raw) {
                Ok(parsed) => Ok(JsonWrapper(parsed)),
                Err(err) => Err(FromSqlError::Other(Box::new(err))),
            };
        }
        Err(FromSqlError::InvalidType)
    }
}

fn wrap_paths(paths: Vec<Path>) -> JsonWrapper<Vec<api::Path>> {
    let protos: Vec<api::Path> = paths.into_iter().map(|p| p.into()).collect();
    JsonWrapper(protos)
}
fn unwrap_paths(wrapper: JsonWrapper<Vec<api::Path>>) -> Vec<Path> {
    wrapper
        .0
        .into_iter()
        .map(|p| p.try_into())
        .collect::<anyhow::Result<Vec<Path>>>()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::persistence::JsonWrapper;

    #[test]
    fn json_serde_survives_round_trips() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE json_wrapper (raw TEXT)", [])
            .unwrap();

        let input: JsonWrapper<Vec<u32>> = JsonWrapper(vec![3, 1, 4]);
        conn.execute("INSERT INTO json_wrapper (raw) VALUES (?)", [&input])
            .unwrap();
        let output: JsonWrapper<Vec<u32>> = conn
            .query_row("SELECT raw FROM json_wrapper", [], |row| row.get("raw"))
            .unwrap();
        assert_eq!(output.0, input.0);
    }
}
