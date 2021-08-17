use std::{
    collections::BTreeMap,
    convert::TryInto,
    sync::{atomic::Ordering, Arc},
};

use actix::Recipient;
use log::trace;
use tokio::sync::mpsc;
use tonic::Status;

use crate::{persistence::Persistence, proto::api::{self, InteractResponse}, requests::Interaction, server::PQResult, value::Row};

#[derive(Debug)]
pub enum ClientMsg {
    User(api::InteractRequest),
    Requery { id: i32, name: String, params: Row },
    End,
}
#[derive(Clone, Debug)]
pub struct ClientHandle {
    pub sender: mpsc::UnboundedSender<ClientMsg>,
}
pub struct ClientActor {
    handle: ClientHandle,
    inputs: mpsc::UnboundedReceiver<ClientMsg>,
    addr: Recipient<PQResult>,
    persistence: Arc<Persistence>,
    listener: ListenHandle,
}
impl ClientActor {
    pub fn start(
        addr: Recipient<PQResult>,
        persistence: Arc<Persistence>,
        listener: ListenHandle,
    ) -> ClientHandle {
        let (sender, receiver) = mpsc::unbounded_channel();
        let handle = ClientHandle { sender };
        let actor = ClientActor {
            handle: handle.clone(),
            inputs: receiver,
            addr,
            persistence,
            listener,
        };
        tokio::spawn(async move { actor.run().await });
        handle
    }
    async fn run(mut self) {
        self.persistence
            .diagnostics
            .num_connected_clients
            .fetch_add(1, Ordering::SeqCst);
        trace!("[ACTOR] starting...");
        while let Some(msg) = self.inputs.recv().await {
            trace!("[ACTOR] recv {:?}", msg);
            let resp = match msg {
                ClientMsg::End => break,
                ClientMsg::User(req) => self.handle_user(req),
                ClientMsg::Requery { id, name, params } => self.handle_requery(id, name, params),
            };
            if self.addr.try_send(PQResult(resp)).is_err() {
                break;
            }
        }
        trace!("[ACTOR] exiting...");
        self.persistence
            .diagnostics
            .num_connected_clients
            .fetch_sub(1, Ordering::SeqCst);
    }

    fn handle_user(&mut self, req: api::InteractRequest) -> Result<api::InteractResponse, Status> {
        let id = req.id;
        let req: Interaction = req.try_into()?;
        let rows =
            match req {
                Interaction::Query { name, params } => {
                    let config = self.persistence.get_config()?;
                    let query = config.queries.get(&name).ok_or_else(|| {
                        Status::invalid_argument(&format!("no such query: {}", name))
                    })?;
                    self.persistence
                        .do_query(name, &query.sql_template, &params)?
                }
                Interaction::Mutate { name, params } => {
                    let config = self.persistence.get_config()?;
                    let mutate = config.mutates.get(&name).ok_or_else(|| {
                        Status::invalid_argument(&format!("no such mutate: {}", name))
                    })?;
                    let rows = self
                        .persistence
                        .do_query(name, &mutate.sql_template, &params)?;
                    if !mutate.notify.is_empty() {
                        self.listener
                            .sender
                            .send(ListenMsg::Ping {
                                paths: mutate.notify.clone(),
                            })
                            .unwrap();
                    }
                    rows
                }
                Interaction::Listen { name, params } => {
                    let config = self.persistence.get_config()?;
                    let query = config.queries.get(&name).ok_or_else(|| {
                        Status::invalid_argument(&format!("no such query: {}", name))
                    })?;
                    if !query.listen.is_empty() {
                        self.listener
                            .sender
                            .send(ListenMsg::Register {
                                paths: query.listen.clone(),
                                handle: self.handle.clone(),
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
    ) -> Result<api::InteractResponse, Status> {
        let config = self.persistence.get_config()?;
        let query = config
            .queries
            .get(&name)
            .ok_or_else(|| Status::invalid_argument(&format!("no such query: {}", name)))?;
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
        handle: ClientHandle,
        id: i32,
        name: String,
        params: Row,
    },
    Ping {
        paths: Vec<String>,
    },
}

#[derive(Clone)]
pub struct ListenHandle {
    pub sender: mpsc::UnboundedSender<ListenMsg>,
}
pub struct ListenActor {
    handle: ListenHandle,
    inputs: mpsc::UnboundedReceiver<ListenMsg>,
    registry: BTreeMap<String, Vec<Listen>>,
}

pub struct Listen {
    handle: ClientHandle,
    id: i32,
    name: String,
    params: Row,
}

impl ListenActor {
    pub fn start() -> ListenHandle {
        let (sender, receiver) = mpsc::unbounded_channel();
        let handle = ListenHandle { sender };
        let actor = ListenActor {
            handle: handle.clone(),
            inputs: receiver,
            registry: BTreeMap::new(),
        };
        tokio::spawn(async move { actor.run().await });
        handle
    }
    async fn run(mut self) {
        trace!("[LISTEN] starting...");
        while let Some(msg) = self.inputs.recv().await {
            trace!("[LISTEN] recv {:?}", msg);
            match msg {
                ListenMsg::Register {
                    paths,
                    handle,
                    id,
                    name,
                    params,
                } => self.handle_register(paths, handle, id, name, params),
                ListenMsg::Ping { paths } => self.handle_ping(paths),
            }
        }
        trace!("[LISTEN] exiting...");
    }

    fn handle_register(
        &mut self,
        paths: Vec<String>,
        handle: ClientHandle,
        id: i32,
        name: String,
        params: Row,
    ) {
        for path in paths {
            self.registry.entry(path).or_default().push(Listen {
                handle: handle.clone(),
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
                        .handle
                        .sender
                        .send(ClientMsg::Requery {
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
