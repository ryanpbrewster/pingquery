use std::{
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
};
use tonic::Status;

use crate::proto::api;

#[derive(Debug)]
pub struct Config {
    pub queries: BTreeMap<String, QueryConfig>,
    pub mutates: BTreeMap<String, MutateConfig>,
}

#[derive(Debug)]
pub struct QueryConfig {
    pub name: String,
    pub sql_template: String,
}

#[derive(Debug)]
pub struct MutateConfig {
    pub name: String,
    pub sql_template: String,
}

impl From<Config> for api::Config {
    fn from(value: Config) -> Self {
        Self {
            queries: value.queries.into_iter().map(|(_, v)| v.into()).collect(),
            mutates: value.mutates.into_iter().map(|(_, v)| v.into()).collect(),
        }
    }
}
impl TryFrom<api::Config> for Config {
    type Error = Status;

    fn try_from(value: api::Config) -> Result<Self, Self::Error> {
        let queries: Vec<QueryConfig> = value
            .queries
            .into_iter()
            .map(|p| p.try_into())
            .collect::<Result<_, _>>()?;
        let mutates: Vec<MutateConfig> = value
            .mutates
            .into_iter()
            .map(|p| p.try_into())
            .collect::<Result<_, _>>()?;
        Ok(Config {
            queries: queries.into_iter().map(|c| (c.name.clone(), c)).collect(),
            mutates: mutates.into_iter().map(|c| (c.name.clone(), c)).collect(),
        })
    }
}

impl From<QueryConfig> for api::StatementConfig {
    fn from(value: QueryConfig) -> Self {
        Self {
            name: value.name,
            sql_template: value.sql_template,
        }
    }
}
impl TryFrom<api::StatementConfig> for QueryConfig {
    type Error = Status;

    fn try_from(value: api::StatementConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            name: if value.name.is_empty() {
                return Err(Status::invalid_argument("missing name"));
            } else {
                value.name
            },
            sql_template: if value.sql_template.is_empty() {
                return Err(Status::invalid_argument("missing sql_template"));
            } else {
                value.sql_template
            },
        })
    }
}

impl From<MutateConfig> for api::StatementConfig {
    fn from(value: MutateConfig) -> Self {
        Self {
            name: value.name,
            sql_template: value.sql_template,
        }
    }
}
impl TryFrom<api::StatementConfig> for MutateConfig {
    type Error = Status;

    fn try_from(value: api::StatementConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            name: if value.name.is_empty() {
                return Err(Status::invalid_argument("missing name"));
            } else {
                value.name
            },
            sql_template: if value.sql_template.is_empty() {
                return Err(Status::invalid_argument("missing sql_template"));
            } else {
                value.sql_template
            },
        })
    }
}
