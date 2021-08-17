use crate::proto::api;
use anyhow::anyhow;
use std::{
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
};

#[derive(Debug)]
pub struct Config {
    pub queries: BTreeMap<String, QueryConfig>,
    pub mutates: BTreeMap<String, MutateConfig>,
}

#[derive(Debug)]
pub struct QueryConfig {
    pub name: String,
    pub sql_template: String,
    pub listen: Vec<String>,
}

#[derive(Debug)]
pub struct MutateConfig {
    pub name: String,
    pub sql_template: String,
    pub notify: Vec<String>,
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
    type Error = anyhow::Error;

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
            listen: value.listen,
        }
    }
}
impl TryFrom<api::QueryConfig> for QueryConfig {
    type Error = anyhow::Error;

    fn try_from(value: api::QueryConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            name: if value.name.is_empty() {
                return Err(anyhow!("missing name"));
            } else {
                value.name
            },
            sql_template: if value.sql_template.is_empty() {
                return Err(anyhow!("missing sql_template"));
            } else {
                value.sql_template
            },
            listen: value.listen,
        })
    }
}

impl From<MutateConfig> for api::MutateConfig {
    fn from(value: MutateConfig) -> Self {
        Self {
            name: value.name,
            sql_template: value.sql_template,
            notify: value.notify,
        }
    }
}
impl TryFrom<api::MutateConfig> for MutateConfig {
    type Error = anyhow::Error;

    fn try_from(value: api::MutateConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            name: if value.name.is_empty() {
                return Err(anyhow!("missing name"));
            } else {
                value.name
            },
            sql_template: if value.sql_template.is_empty() {
                return Err(anyhow!("missing sql_template"));
            } else {
                value.sql_template
            },
            notify: value.notify,
        })
    }
}

pub fn encode_strings(xs: &[String]) -> String {
    serde_json::to_string(xs).unwrap()
}
pub fn decode_strings(raw: String) -> Vec<String> {
    serde_json::from_str(&raw).unwrap()
}

#[cfg(test)]
mod test {
    use crate::config::{decode_strings, encode_strings};

    #[test]
    fn string_list_encoding_is_bijective() {
        let xs = vec![];
        assert_eq!(xs, decode_strings(encode_strings(&xs)));
    }
}
