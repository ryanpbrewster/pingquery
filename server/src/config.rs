use crate::{
    proto::api,
    value::{Row, Value},
};
use anyhow::anyhow;
use core::fmt;
use lazy_static::lazy_static;
use regex::Regex;
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
    pub listen: Vec<Path>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Path {
    segments: Vec<Segment>,
}
impl Path {
    pub fn resolve(self, params: &Row) -> anyhow::Result<Vec<String>> {
        self.segments
            .into_iter()
            .map(|s| match s {
                Segment::Lit(lit) => Ok(lit),
                Segment::Var(name) => match params.columns.get(&name) {
                    Some(Value::Text(v)) => Ok(v.clone()),
                    None => Err(anyhow!(
                        "could not resolve var segment for {}: no such var",
                        name
                    )),
                    Some(_) => Err(anyhow!(
                        "could not resolve var segment for {}: non-string value",
                        name
                    )),
                },
            })
            .collect()
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Segment {
    Lit(String),
    Var(String),
}
impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/")?;
        for segment in &self.segments {
            write!(f, "{}/", segment)?;
        }
        Ok(())
    }
}
impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Segment::Lit(s) => write!(f, "{}", s),
            Segment::Var(name) => write!(f, "{{{}}}", name),
        }
    }
}

#[derive(Debug)]
pub struct MutateConfig {
    pub name: String,
    pub sql_template: String,
    pub notify: Vec<Path>,
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
            listen: value.listen.into_iter().map(|p| p.into()).collect(),
        }
    }
}
impl From<Path> for api::Path {
    fn from(value: Path) -> Self {
        Self {
            segments: value.segments.into_iter().map(|s| s.to_string()).collect(),
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
            listen: value
                .listen
                .into_iter()
                .map(|p| p.try_into())
                .collect::<anyhow::Result<_>>()?,
        })
    }
}
impl TryFrom<api::Path> for Path {
    type Error = anyhow::Error;

    fn try_from(proto: api::Path) -> Result<Self, Self::Error> {
        let segments = proto
            .segments
            .into_iter()
            .map(|s| segment_from_proto(&s))
            .collect::<anyhow::Result<_>>()?;
        Ok(Path { segments })
    }
}

lazy_static! {
    static ref SEGMENT_LIT: Regex = Regex::new("^([A-Za-z-_]+)$").unwrap();
    static ref SEGMENT_VAR: Regex = Regex::new("^\\{([A-Za-z_:]+)\\}$").unwrap();
}
fn segment_from_proto(raw: &str) -> anyhow::Result<Segment> {
    if let Some(m) = SEGMENT_LIT.captures(raw).and_then(|m| m.get(1)) {
        return Ok(Segment::Lit(m.as_str().to_owned()));
    }
    if let Some(m) = SEGMENT_VAR.captures(raw).and_then(|m| m.get(1)) {
        return Ok(Segment::Var(m.as_str().to_owned()));
    }
    Err(anyhow!("invalid path segment: '{}'", raw))
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
            notify: value
                .notify
                .into_iter()
                .map(|p| p.try_into())
                .collect::<anyhow::Result<_>>()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke_test_segment() {
        let input = vec!["my-lit".to_owned(), "{:my_name}".to_owned()];
        let parsed: Path = api::Path {
            segments: input.clone(),
        }
        .try_into()
        .unwrap();
        assert_eq!(
            parsed.segments,
            vec![
                Segment::Lit("my-lit".to_owned()),
                Segment::Var(":my_name".to_owned())
            ]
        );
    }
}
