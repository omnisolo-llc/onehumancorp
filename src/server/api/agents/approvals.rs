use axum::{
    extract::{Extension, State, Path, Query},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::ApprovalRequest;
use ::server_common::Claims;

#[derive(Serialize)]
pub struct ApprovalsResponse {
    pub pending_approvals: Vec<ApprovalRequest>,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub cursor: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Deserialize)]
pub struct DecisionRequest {
    pub approved: bool,
}

#[derive(Serialize)]
pub struct DecisionResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct BudgetResponse {
    pub budget: i32,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_approvals))
        .route("/budget", get(get_budget))
        .route("/{id}", post(decide_approval))
        .with_state(orchestrator)
}

async fn get_budget(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(BudgetResponse { budget: 0 })).into_response(),
    };

    match orchestrator.get_ai_budget(&tenant_id).await {
        Ok(budget) => (StatusCode::OK, Json(BudgetResponse { budget })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(BudgetResponse { budget: 0 })).into_response(),
    }
}

async fn list_approvals(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(ApprovalsResponse { pending_approvals: vec![], next_cursor: None })).into_response(),
    };

    let limit = query.limit.unwrap_or(20);

    // Fetch from DB using cursor pagination
    let approvals = orchestrator.get_pending_approvals(&tenant_id, query.cursor.clone(), limit as i64).await;

    let next_cursor = if approvals.len() == limit {
        approvals.last().map(|a| a.id.clone())
    } else {
        None
    };

    (StatusCode::OK, Json(ApprovalsResponse {
        pending_approvals: approvals,
        next_cursor,
    })).into_response()
}

async fn decide_approval(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Path(id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<DecisionRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    match orchestrator.decide_approval(&id, &tenant_id, payload.approved).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response(),
    }
}
// Support for AI Agent Department Architecture
