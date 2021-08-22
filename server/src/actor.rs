use anyhow::{anyhow, Result};
use std::{
    collections::BTreeMap,
    convert::TryInto,
    sync::{atomic::Ordering, Arc},
};

use actix::{Actor, ActorContext, Addr, AsyncContext, Context, Handler, Message, Recipient};
use log::trace;
use crate::{
    persistence::Persistence, proto::api, requests::Interaction, server::PQResult, value::Row,
};

#[derive(Debug)]
pub enum ClientMsg {
    User(api::InteractRequest),
    Requery { id: i32, name: String, params: Row },
    End,
}
impl Message for ClientMsg {
    type Result = ();
}

#[derive(Clone, Debug)]
pub struct ClientActor {
    pub addr: Recipient<PQResult>,
    pub persistence: Arc<Persistence>,
    pub listener: Addr<ListenActor>,
}
impl Actor for ClientActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        trace!("[ACTOR] starting...");
        self.persistence
            .diagnostics
            .num_connected_clients
            .fetch_add(1, Ordering::SeqCst);
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        trace!("[ACTOR] exiting...");
        self.persistence
            .diagnostics
            .num_connected_clients
            .fetch_sub(1, Ordering::SeqCst);
    }
}
impl Handler<ClientMsg> for ClientActor {
    type Result = ();

    fn handle(&mut self, msg: ClientMsg, ctx: &mut Self::Context) {
        trace!("[ACTOR] recv {:?}", msg);
        let resp = match msg {
            ClientMsg::End => return ctx.stop(),
            ClientMsg::User(req) => self.handle_user(req, ctx.address()),
            ClientMsg::Requery { id, name, params } => self.handle_requery(id, name, params),
        };
        if self.addr.try_send(PQResult(resp)).is_err() {
            ctx.stop()
        }
    }
}
impl ClientActor {
    fn handle_user(&mut self, req: api::InteractRequest, addr: Addr<ClientActor>) -> Result<api::InteractResponse> {
        let id = req.id;
        let req: Interaction = req.try_into()?;
        let rows = match req {
            Interaction::Query { name, params } => {
                let config = self.persistence.get_config()?;
                let query = config
                    .queries
                    .get(&name)
                    .ok_or_else(|| anyhow!("no such query: {}", name))?;
                self.persistence
                    .do_query(name, &query.sql_template, &params)?
            }
            Interaction::Mutate { name, params } => {
                let config = self.persistence.get_config()?;
                let mutate = config
                    .mutates
                    .get(&name)
                    .ok_or_else(|| anyhow!("no such mutate: {}", name))?;
                let rows = self
                    .persistence
                    .do_query(name, &mutate.sql_template, &params)?;
                if !mutate.notify.is_empty() {
                    self.listener
                        .try_send(ListenMsg::Ping {
                            paths: mutate.notify.clone(),
                        }).unwrap();
                }
                rows
            }
            Interaction::Listen { name, params } => {
                let config = self.persistence.get_config()?;
                let query = config
                    .queries
                    .get(&name)
                    .ok_or_else(|| anyhow!("no such query: {}", name))?;
                if !query.listen.is_empty() {
                    self.listener
                        .try_send(ListenMsg::Register {
                            paths: query.listen.clone(),
                            addr,
                            id,
                            name: name.clone(),
                            params: params.clone(),
                        })
                        .unwrap();
                }
                self.persistence
                    .do_query(name, &query.sql_template, &params)?
            }
        };
        Ok(api::InteractResponse {
            id,
            rows: rows.into_iter().map(|row| row.into()).collect(),
        })
    }

    fn handle_requery(
        &mut self,
        id: i32,
        name: String,
        params: Row,
    ) -> Result<api::InteractResponse> {
        let config = self.persistence.get_config()?;
        let query = config
            .queries
            .get(&name)
            .ok_or_else(|| anyhow!("no such query: {}", name))?;
        let rows = self
            .persistence
            .do_query(name, &query.sql_template, &params)?;
        Ok(api::InteractResponse {
            id,
            rows: rows.into_iter().map(|row| row.into()).collect(),
        })
    }
}

#[derive(Debug)]
pub enum ListenMsg {
    Register {
        paths: Vec<String>,
        addr: Addr<ClientActor>,
        id: i32,
        name: String,
        params: Row,
    },
    Ping {
        paths: Vec<String>,
    },
}
impl Message for ListenMsg {
    type Result = ();
}

#[derive(Debug, Default)]
pub struct ListenActor {
    registry: BTreeMap<String, Vec<Listen>>,
}
impl Actor for ListenActor {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        trace!("[LISTEN] starting...");
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        trace!("[LISTEN] exiting...");
    }
}
impl Handler<ListenMsg> for ListenActor {
    type Result = ();

    fn handle(&mut self, msg: ListenMsg, _ctx: &mut Self::Context) -> Self::Result {
            trace!("[LISTEN] recv {:?}", msg);
            match msg {
                ListenMsg::Register {
                    paths,
                    addr,
                    id,
                    name,
                    params,
                } => self.handle_register(paths, addr, id, name, params),
                ListenMsg::Ping { paths } => self.handle_ping(paths),
            }
    }
}

#[derive(Debug)]
pub struct Listen {
    addr: Addr<ClientActor>,
    id: i32,
    name: String,
    params: Row,
}

impl ListenActor {
    fn handle_register(
        &mut self,
        paths: Vec<String>,
        addr: Addr<ClientActor>,
        id: i32,
        name: String,
        params: Row,
    ) {
        for path in paths {
            self.registry.entry(path).or_default().push(Listen {
                addr: addr.clone(),
                id,
                name: name.clone(),
                params: params.clone(),
            });
        }
    }
    fn handle_ping(&mut self, paths: Vec<String>) {
        for path in paths {
            if let Some(listens) = self.registry.get_mut(&path) {
                trace!("[LISTEN] notifying {} listens @ {}", listens.len(), path);
                listens.retain(|listen| {
                    listen
                        .addr
                        .try_send(ClientMsg::Requery {
                            id: listen.id,
                            name: listen.name.clone(),
                            params: listen.params.clone(),
                        })
                        .is_ok()
                });
                trace!("[LISTEN] successfully notified {}", listens.len());
            }
        }
    }
}
