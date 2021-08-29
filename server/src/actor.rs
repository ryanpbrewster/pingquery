use actix_web_actors::ws;
use anyhow::{anyhow, Result};
use std::{
    convert::TryInto,
    sync::{atomic::Ordering, Arc},
};

use crate::{
    config::Path,
    listen::Registree,
    persistence::Persistence,
    proto::api::{self, InteractRequest},
    requests::Interaction,
    value::Row,
};
use actix::{Actor, ActorContext, Addr, AsyncContext, Context, Handler, Message, StreamHandler};
use log::{info, trace, warn};

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
    pub persistence: Arc<Persistence>,
    pub listener: Addr<ListenActor>,
}
impl Actor for ClientActor {
    type Context = ws::WebsocketContext<Self>;

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
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        info!("[WS] incoming: {:?}", msg);
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                warn!("[WS] closing because of {}", e);
                return ctx.close(None);
            }
        };
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Close(r) => ctx.close(r),
            ws::Message::Text(text) => {
                let req: InteractRequest = match serde_json::from_slice(text.as_bytes()) {
                    Ok(v) => v,
                    Err(e) => {
                        ctx.text(e.to_string());
                        ctx.close(None);
                        return;
                    }
                };
                info!("[WS] req: {:?}", req);
                let resp = match self.handle_user(req, ctx.address()) {
                    Ok(resp) => resp,
                    Err(e) => {
                        ctx.text(e.to_string());
                        ctx.close(None);
                        return;
                    }
                };
                ctx.text(serde_json::to_string(&resp).unwrap());
            }
            ws::Message::Binary(_)
            | ws::Message::Continuation(_)
            | ws::Message::Pong(_)
            | ws::Message::Nop => {}
        }
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
        match resp {
            Ok(v) => ctx.text(serde_json::to_string(&v).unwrap()),
            Err(_e) => ctx.close(None),
        }
    }
}
impl ClientActor {
    fn handle_user(
        &mut self,
        req: api::InteractRequest,
        addr: Addr<ClientActor>,
    ) -> Result<api::InteractResponse> {
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
                let mut config = self.persistence.get_config()?;
                let mutate = config
                    .mutates
                    .remove(&name)
                    .ok_or_else(|| anyhow!("no such mutate: {}", name))?;
                let rows = self
                    .persistence
                    .do_query(name, &mutate.sql_template, &params)?;
                if !mutate.notify.is_empty() {
                    let paths: Vec<Vec<String>> = mutate
                        .notify
                        .into_iter()
                        .map(|p| p.resolve(&params))
                        .collect::<anyhow::Result<_>>()?;
                    self.listener.try_send(ListenMsg::Ping { paths }).unwrap();
                }
                rows
            }
            Interaction::Listen { name, params } => {
                let mut config = self.persistence.get_config()?;
                let query = config
                    .queries
                    .remove(&name)
                    .ok_or_else(|| anyhow!("no such query: {}", name))?;
                if !query.listen.is_empty() {
                    let paths: Vec<Vec<String>> = query
                        .listen
                        .into_iter()
                        .map(|p| p.resolve(&params))
                        .collect::<anyhow::Result<_>>()?;
                    self.listener.try_send(ListenMsg::Register {
                        paths,
                        addr,
                        id,
                        name: name.clone(),
                        params: params.clone(),
                    })?;
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
        paths: Vec<Vec<String>>,
        addr: Addr<ClientActor>,
        id: i32,
        name: String,
        params: Row,
    },
    Ping {
        paths: Vec<Vec<String>>,
    },
}
impl Message for ListenMsg {
    type Result = ();
}

#[derive(Debug, Default)]
pub struct ListenActor {
    registry: Registree<Listen>,
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
        paths: Vec<Vec<String>>,
        addr: Addr<ClientActor>,
        id: i32,
        name: String,
        params: Row,
    ) {
        for path in paths {
            self.registry.insert(
                path,
                Listen {
                    addr: addr.clone(),
                    id,
                    name: name.clone(),
                    params: params.clone(),
                },
            );
        }
    }
    fn handle_ping(&mut self, paths: Vec<Vec<String>>) {
        for path in paths {
            self.registry.traverse(&path, |listens| {
                trace!("[LISTEN] notifying {} listens @ {:?}", listens.len(), path);
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
            });
        }
    }
}
