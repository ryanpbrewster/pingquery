use std::{
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
};

use rusqlite::types::{FromSql, FromSqlError};
use tonic::Status;

use crate::proto::api;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct Row {
    pub columns: BTreeMap<String, Value>,
}

impl TryFrom<api::Value> for Value {
    type Error = Status;

    fn try_from(v: api::Value) -> Result<Self, Self::Error> {
        match v.r#type {
            Some(api::value::Type::Integer(n)) => Ok(Value::Integer(n)),
            Some(api::value::Type::Text(s)) => Ok(Value::Text(s)),
            None => Err(Status::invalid_argument("missing value")),
        }
    }
}
impl From<Value> for api::Value {
    fn from(v: Value) -> Self {
        let inner = match v {
            Value::Integer(n) => api::value::Type::Integer(n),
            Value::Text(s) => api::value::Type::Text(s),
        };
        api::Value {
            r#type: Some(inner),
        }
    }
}

impl TryFrom<rusqlite::types::Value> for Value {
    type Error = Status;

    fn try_from(v: rusqlite::types::Value) -> Result<Self, Self::Error> {
        match v {
            rusqlite::types::Value::Integer(n) => Ok(Value::Integer(n)),
            rusqlite::types::Value::Text(s) => Ok(Value::Text(s)),
            rusqlite::types::Value::Null
            | rusqlite::types::Value::Real(_)
            | rusqlite::types::Value::Blob(_) => {
                Err(Status::invalid_argument("unknown sqlite value"))
            }
        }
    }
}
impl From<Value> for rusqlite::types::Value {
    fn from(v: Value) -> Self {
        match v {
            Value::Integer(n) => rusqlite::types::Value::Integer(n),
            Value::Text(s) => rusqlite::types::Value::Text(s),
        }
    }
}
impl<'a> From<&'a Value> for rusqlite::types::ValueRef<'a> {
    fn from(v: &'a Value) -> Self {
        match v {
            Value::Integer(n) => rusqlite::types::ValueRef::Integer(*n),
            Value::Text(s) => rusqlite::types::ValueRef::Text(s.as_bytes()),
        }
    }
}
impl FromSql for Value {
    fn column_result(v: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match v {
            rusqlite::types::ValueRef::Integer(n) => Ok(Value::Integer(n)),
            rusqlite::types::ValueRef::Text(s) => {
                Ok(Value::Text(String::from_utf8(s.to_vec()).unwrap()))
            }
            rusqlite::types::ValueRef::Real(_)
            | rusqlite::types::ValueRef::Blob(_)
            | rusqlite::types::ValueRef::Null => Err(FromSqlError::InvalidType),
        }
    }
}

impl From<Row> for api::Row {
    fn from(row: Row) -> Self {
        api::Row {
            columns: row
                .columns
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl TryFrom<api::Row> for Row {
    type Error = Status;

    fn try_from(v: api::Row) -> Result<Self, Self::Error> {
        let columns: BTreeMap<String, Value> = v
            .columns
            .into_iter()
            .map(|(k, v)| {
                let v: Value = v.try_into()?;
                Ok((k, v))
            })
            .collect::<Result<BTreeMap<String, Value>, Status>>()?;
        Ok(Row { columns })
    }
}
