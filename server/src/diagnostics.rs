use crate::proto::api;
use std::{
    collections::HashMap,
    sync::atomic::{AtomicI32, AtomicI64, Ordering},
};

use dashmap::DashMap;

#[derive(Debug, Default)]
pub struct Diagnostics {
    pub num_connected_clients: AtomicI32,
    pub queries: DashMap<String, QueryDiagnostics>,
}

#[derive(Debug, Default)]
pub struct QueryDiagnostics {
    pub num_executions: AtomicI64,
}

#[derive(Debug)]
pub struct DiagnosticsReport {
    pub num_connected_clients: i32,
    pub queries: HashMap<String, QueryDiagnosticsReport>,
}
#[derive(Debug)]
pub struct QueryDiagnosticsReport {
    pub num_executions: i64,
}

impl Diagnostics {
    pub fn report(&self) -> DiagnosticsReport {
        DiagnosticsReport {
            num_connected_clients: self.num_connected_clients.load(Ordering::SeqCst),
            queries: self
                .queries
                .iter()
                .map(|kv| (kv.key().clone(), kv.value().report()))
                .collect(),
        }
    }
}
impl QueryDiagnostics {
    pub fn report(&self) -> QueryDiagnosticsReport {
        QueryDiagnosticsReport {
            num_executions: self.num_executions.load(Ordering::SeqCst),
        }
    }
}

impl From<DiagnosticsReport> for api::DiagnosticsResponse {
    fn from(report: DiagnosticsReport) -> Self {
        let queries: Vec<api::QueryDiagnostics> = report
            .queries
            .into_iter()
            .map(|(name, query)| api::QueryDiagnostics {
                name,
                num_executions: query.num_executions,
            })
            .collect();
        api::DiagnosticsResponse {
            num_connected_clients: report.num_connected_clients,
            queries,
        }
    }
}
