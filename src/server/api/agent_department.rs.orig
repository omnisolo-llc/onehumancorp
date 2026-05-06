use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::domain::agent_department::models::{AgentDepartment, TaskApproval};
use crate::domain::agent_department::service::AgentDepartmentService;

pub fn router(service: Arc<AgentDepartmentService>) -> Router {
    Router::new()
        .route("/provision", post(provision_handler))
        .route("/approvals", get(list_approvals_handler))
        .route("/approvals/:id/review", post(review_approval_handler))
        .with_state(service)
}

async fn provision_handler(
    State(service): State<Arc<AgentDepartmentService>>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<AgentDepartment>>, String> {
    let tenant_id = headers.get("x-tenant-id").and_then(|h| h.to_str().ok()).unwrap_or("00000000-0000-0000-0000-000000000000");
    let depts = service.provision_default_departments(tenant_id).await?;
    Ok(Json(depts))
}

async fn list_approvals_handler(
    State(service): State<Arc<AgentDepartmentService>>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<TaskApproval>>, String> {
    let tenant_id = headers.get("x-tenant-id").and_then(|h| h.to_str().ok()).unwrap_or("00000000-0000-0000-0000-000000000000");
    let approvals = service.get_pending_approvals(tenant_id).await?;
    Ok(Json(approvals))
}

#[derive(Deserialize)]
pub struct ReviewRequest {
    action: String, // APPROVE, REJECT, MODIFY
}

async fn review_approval_handler(
    State(service): State<Arc<AgentDepartmentService>>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
    Json(payload): Json<ReviewRequest>,
) -> Result<Json<()>, String> {
    let tenant_id = headers.get("x-tenant-id").and_then(|h| h.to_str().ok()).unwrap_or("00000000-0000-0000-0000-000000000000");
    service.review_approval(tenant_id, &id, &payload.action).await?;
    Ok(Json(()))
}
