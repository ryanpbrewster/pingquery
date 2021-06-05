use std::{collections::BTreeMap, convert::TryInto, sync::Arc};

use log::trace;
use tokio::sync::mpsc;
use tonic::Status;

use crate::{
    persistence::Persistence,
    proto::api::{self, InteractResponse},
    requests::Interaction,
    value::Row,
};

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
    outputs: mpsc::UnboundedSender<Result<InteractResponse, Status>>,
    persistence: Arc<Persistence>,
    listener: ListenHandle,
}
impl ClientActor {
    pub fn start(
        outputs: mpsc::UnboundedSender<Result<InteractResponse, Status>>,
        persistence: Arc<Persistence>,
        listener: ListenHandle,
    ) -> ClientHandle {
        let (sender, receiver) = mpsc::unbounded_channel();
        let handle = ClientHandle { sender };
        let actor = ClientActor {
            handle: handle.clone(),
            inputs: receiver,
            outputs,
            persistence,
            listener,
        };
        tokio::spawn(async move { actor.run().await });
        handle
    }
    async fn run(mut self) {
        trace!("[ACTOR] starting...");
        while let Some(msg) = self.inputs.recv().await {
            trace!("[ACTOR] recv {:?}", msg);
            let resp = match msg {
                ClientMsg::End => break,
                ClientMsg::User(req) => self.handle_user(req),
                ClientMsg::Requery { id, name, params } => self.handle_requery(id, name, params),
            };
            self.outputs.send(resp).unwrap();
        }
        trace!("[ACTOR] exiting...");
    }

    fn handle_user(&mut self, req: api::InteractRequest) -> Result<api::InteractResponse, Status> {
        let id = req.id;
        let req: Interaction = req.try_into()?;
        let rows = match req {
            Interaction::Query { name, params } => self.persistence.do_query(&name, &params)?,
            Interaction::Mutate { name, params } => {
                let rows = self.persistence.do_query(&name, &params)?;
                self.listener
                    .sender
                    .send(ListenMsg::Ping {
                        paths: vec![String::new()],
                    })
                    .unwrap();
                rows
            }
            Interaction::Listen { name, params } => {
                self.listener
                    .sender
                    .send(ListenMsg::Register {
                        handle: self.handle.clone(),
                        path: String::new(),
                        id,
                        name: name.clone(),
                        params: params.clone(),
                    })
                    .unwrap();
                self.persistence.do_query(&name, &params)?
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
        let rows = self.persistence.do_query(&name, &params)?;
        Ok(api::InteractResponse {
            id,
            rows: rows.into_iter().map(|row| row.into()).collect(),
        })
    }
}

#[derive(Debug)]
pub enum ListenMsg {
    Register {
        path: String,
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
                    path,
                    handle,
                    id,
                    name,
                    params,
                } => self.handle_register(path, handle, id, name, params),
                ListenMsg::Ping { paths } => self.handle_ping(paths),
            }
        }
        trace!("[LISTEN] exiting...");
    }

    fn handle_register(
        &mut self,
        path: String,
        handle: ClientHandle,
        id: i32,
        name: String,
        params: Row,
    ) {
        self.registry.entry(path).or_default().push(Listen {
            handle,
            id,
            name,
            params,
        });
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
