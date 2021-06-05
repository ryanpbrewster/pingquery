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
    pub listen: Vec<Path>,
}

#[derive(Debug)]
pub struct MutateConfig {
    pub name: String,
    pub sql_template: String,
    pub notify: Vec<Path>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Path {
    pub segments: Vec<String>,
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

impl From<QueryConfig> for api::QueryConfig {
    fn from(value: QueryConfig) -> Self {
        Self {
            name: value.name,
            sql_template: value.sql_template,
            listen: value.listen.into_iter().map(|p| p.into()).collect(),
        }
    }
}
impl TryFrom<api::QueryConfig> for QueryConfig {
    type Error = Status;

    fn try_from(value: api::QueryConfig) -> Result<Self, Self::Error> {
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
            listen: value
                .listen
                .into_iter()
                .map(|p| p.try_into())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFrom<api::Path> for Path {
    type Error = Status;

    fn try_from(value: api::Path) -> Result<Self, Self::Error> {
        let segments: Vec<String> = value.segments;
        Ok(Path { segments })
    }
}
impl From<Path> for api::Path {
    fn from(value: Path) -> Self {
        Self {
            segments: value.segments,
        }
    }
}

impl From<MutateConfig> for api::MutateConfig {
    fn from(value: MutateConfig) -> Self {
        Self {
            name: value.name,
            sql_template: value.sql_template,
            notify: value.notify.into_iter().map(|p| p.into()).collect(),
        }
    }
}
impl TryFrom<api::MutateConfig> for MutateConfig {
    type Error = Status;

    fn try_from(value: api::MutateConfig) -> Result<Self, Self::Error> {
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
            notify: value
                .notify
                .into_iter()
                .map(|p| p.try_into())
                .collect::<Result<_, _>>()?,
        })
    }
}
