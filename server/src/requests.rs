use std::convert::{TryFrom, TryInto};

use tonic::Status;

use crate::{
    proto::api::{self, interact_request},
    value::Row,
};

pub enum Interaction {
    Query { name: String, params: Row },
    Mutate { name: String, params: Row },
}

impl TryFrom<api::InteractRequest> for Interaction {
    type Error = Status;

    fn try_from(value: api::InteractRequest) -> Result<Self, Self::Error> {
        match value.r#type {
            None => Err(Status::invalid_argument("missing type for InteractRequest")),
            Some(interact_request::Type::Query(stmt)) => Ok(Interaction::Query {
                name: stmt.name,
                params: stmt.params.unwrap_or_default().try_into()?,
            }),
            Some(interact_request::Type::Mutate(stmt)) => Ok(Interaction::Mutate {
                name: stmt.name,
                params: stmt.params.unwrap_or_default().try_into()?,
            }),
            Some(interact_request::Type::Listen(_stmt)) => {
                Err(Status::unimplemented("listen interactions"))
            }
        }
    }
}
