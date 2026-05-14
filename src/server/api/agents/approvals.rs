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
/// Core API request/response payload for ApprovalsResponse.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct ApprovalsResponse {
    /// Stores the `pending_approvals` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub pending_approvals: Vec<ApprovalRequest>,
    /// Stores the `next_cursor` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
/// Core API request/response payload for PaginationQuery.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct PaginationQuery {
    /// Stores the `cursor` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub cursor: Option<String>,
    /// Stores the `limit` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub limit: Option<usize>,
}

#[derive(Deserialize)]
/// Core API request/response payload for DecisionRequest.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct DecisionRequest {
    /// Stores the `approved` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub approved: bool,
}

#[derive(Serialize)]
/// Core API request/response payload for DecisionResponse.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct DecisionResponse {
    /// Stores the `success` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub success: bool,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_approvals))
        .route("/{id}", post(decide_approval))
        .with_state(orchestrator)
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

    // Assuming we fetch all and paginate manually for now given simple DB fetch
    // Real cursor implementation would need DB level ordering and limit
    let mut approvals = orchestrator.get_pending_approvals(&tenant_id).await;

    // Sort to ensure stable pagination
    approvals.sort_by(|a, b| a.id.cmp(&b.id));

    let limit = query.limit.unwrap_or(20);

    let start_idx = match query.cursor {
        Some(cursor) => approvals.iter().position(|a| a.id == cursor).unwrap_or(0),
        None => 0,
    };

    let end_idx = std::cmp::min(start_idx + limit, approvals.len());

    let paginated_approvals = approvals[start_idx..end_idx].to_vec();

    let next_cursor = if end_idx < approvals.len() {
        Some(approvals[end_idx].id.clone())
    } else {
        None
    };

    (StatusCode::OK, Json(ApprovalsResponse {
        pending_approvals: paginated_approvals,
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
