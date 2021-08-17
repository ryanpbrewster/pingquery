use std::convert::{TryFrom, TryInto};

use crate::{proto::api, value::Row};
use anyhow::anyhow;

pub enum Interaction {
    Query { name: String, params: Row },
    Mutate { name: String, params: Row },
    Listen { name: String, params: Row },
}

impl TryFrom<api::InteractRequest> for Interaction {
    type Error = anyhow::Error;

    fn try_from(value: api::InteractRequest) -> Result<Self, Self::Error> {
        if let Some(stmt) = value.query {
            return Ok(Interaction::Query {
                name: stmt.name,
                params: stmt.params.unwrap_or_default().try_into()?,
            });
        }
        if let Some(stmt) = value.mutate {
            return Ok(Interaction::Mutate {
                name: stmt.name,
                params: stmt.params.unwrap_or_default().try_into()?,
            });
        }
        if let Some(stmt) = value.listen {
            return Ok(Interaction::Listen {
                name: stmt.name,
                params: stmt.params.unwrap_or_default().try_into()?,
            });
        }
        Err(anyhow!("missing type for InteractRequest"))
    }
}
