use std::collections::BTreeMap;

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

impl From<Config> for crate::proto::api::Config {
    fn from(value: Config) -> Self {
        Self {
            queries: value.queries.into_iter().map(|(_, v)| v.into()).collect(),
            mutates: value.mutates.into_iter().map(|(_, v)| v.into()).collect(),
        }
    }
}

impl From<QueryConfig> for crate::proto::api::StatementConfig {
    fn from(value: QueryConfig) -> Self {
        Self {
            name: value.name,
            sql_template: value.sql_template,
        }
    }
}

impl From<MutateConfig> for crate::proto::api::StatementConfig {
    fn from(value: MutateConfig) -> Self {
        Self {
            name: value.name,
            sql_template: value.sql_template,
        }
    }
}