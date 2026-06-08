pub mod workspaces;
pub mod tasks;
pub mod messages;
pub mod artifacts;
pub mod approvals;
#[cfg(test)]
pub mod tests;

use axum::Router;
use std::sync::Arc;
use crate::db::DB;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;

#[derive(Clone)]
pub struct AssistantState {
    pub db: Arc<DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

pub fn router<S>(db: Arc<DB>, orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = AssistantState { db, orchestrator };

    Router::new()
        .nest("/workspaces", workspaces::router(state.clone()))
        .nest("/tasks", tasks::router(state.clone()))
        .nest("/approvals", approvals::router(state.clone()))
        .nest("/artifacts", artifacts::router(state.clone()))
        .nest("/messages", messages::router(state.clone()))
}
