use axum::{
    extract::{State, Path, Query},
    response::IntoResponse,
    Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::tasks::{TaskManager, SharedTask};
use crate::hub::Hub;

#[derive(Deserialize)]
pub struct QueryRequest {
    pub organization_id: String,
}

#[derive(Serialize)]
pub struct PendingActionsResponse {
    pub tasks: Vec<SharedTask>,
}

#[derive(Deserialize)]
pub struct ApprovalRequest {
    pub is_approved: bool,
}

#[derive(Serialize)]
pub struct ApprovalResponse {
    pub status: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    let hub_query = hub.clone();
    let hub_approve = hub.clone();

    Router::new()
        .route("/pending", get(move |Query(params): Query<QueryRequest>| async move {
            let tasks = hub_query.task_manager().tasks.read().unwrap();
            let mut pending = Vec::new();
            for task in tasks.values() {
                if task.organization_id == params.organization_id && task.status == "REVIEW" {
                    pending.push(task.clone());
                }
            }
            Json(PendingActionsResponse { tasks: pending }).into_response()
        }))
        .route("/:task_id/approve", post(move |Path(task_id): Path<String>, Json(payload): Json<ApprovalRequest>| async move {
            match hub_approve.task_manager().approve_task(&task_id, payload.is_approved) {
                Ok(_) => Json(ApprovalResponse { status: "success".to_string() }).into_response(),
                Err(e) => (axum::http::StatusCode::NOT_FOUND, e).into_response(),
            }
        }))
}
