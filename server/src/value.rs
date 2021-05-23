use std::{collections::BTreeMap, convert::TryFrom};

use rusqlite::types::{FromSql, FromSqlError};
use tonic::Status;

use crate::proto::api;

pub enum Value {
    Integer(i64),
    Text(String),
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

impl From<BTreeMap<String, Value>> for api::Row {
    fn from(row: BTreeMap<String, Value>) -> Self {
        api::Row {
            columns: row.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}
