use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::ApprovalRequest;
use ::server_common::Claims;
use axum::{
    Json, Router,
    extract::{Extension, Path, Query, State},
    http::{StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[cfg(test)]
#[path = "approvals_readback_test.rs"]
mod readback_tests;

#[derive(Serialize, Deserialize, Clone)]
pub struct ApprovalsResponse {
    pub pending_approvals: Vec<ApprovalRequest>,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub cursor: Option<String>,
    pub limit: Option<usize>,
    pub mobile_optimized: Option<bool>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionRequest {
    pub approved: bool,
    pub edited_payload: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct DecisionResponse {
    pub success: bool,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_approvals))
        .route("/activity", get(list_activity_feed))
        .route("/ledger", get(list_ledger_entries))
        .route("/{id}", post(decide_approval))
        .route_layer(axum::middleware::from_fn(
            crate::api::fixture_boundary::reject_retired_fixture_paths,
        ))
        .with_state(orchestrator)
}

fn private_approvals(status: StatusCode, body: ApprovalsResponse) -> axum::response::Response {
    (
        status,
        [(header::CACHE_CONTROL, "private, no-store")],
        Json(body),
    )
        .into_response()
}

async fn list_approvals(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => {
            return private_approvals(
                StatusCode::UNAUTHORIZED,
                ApprovalsResponse {
                    pending_approvals: vec![],
                    next_cursor: None,
                },
            );
        }
    };

    let limit = query.limit.unwrap_or(20);
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    // Approval decisions are mutable across writers and server instances. Read
    // committed state directly; SWR can resurrect a decision after its HTTP ACK.
    let mut approvals = orchestrator
        .get_pending_approvals(&tenant_id, query.cursor.clone(), limit as i64)
        .await;
    if mobile_optimized {
        for a in &mut approvals {
            a.payload = None;
        }
    }

    let next_cursor = if approvals.len() == limit {
        approvals.last().map(|a| a.id.clone())
    } else {
        None
    };

    let response = ApprovalsResponse {
        pending_approvals: approvals,
        next_cursor,
    };
    private_approvals(StatusCode::OK, response)
}

async fn list_activity_feed(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => {
            return private_approvals(
                StatusCode::UNAUTHORIZED,
                ApprovalsResponse {
                    pending_approvals: vec![],
                    next_cursor: None,
                },
            );
        }
    };

    let limit = query.limit.unwrap_or(20);
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    // Approval decisions are mutable across writers and server instances. Read
    // committed state directly; SWR can resurrect a decision after its HTTP ACK.
    let mut activities = orchestrator
        .get_activity_feed(&tenant_id, query.cursor.clone(), limit as i64)
        .await;
    if mobile_optimized {
        for a in &mut activities {
            a.payload = None;
        }
    }

    let next_cursor = if activities.len() == limit {
        activities.last().map(|a| a.id.clone())
    } else {
        None
    };

    let response = ApprovalsResponse {
        pending_approvals: activities,
        next_cursor,
    };
    private_approvals(StatusCode::OK, response)
}

async fn decide_approval(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Path(id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<DecisionRequest>,
) -> impl IntoResponse {
    if !claims
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("owner") || role.eq_ignore_ascii_case("admin"))
    {
        return (
            StatusCode::FORBIDDEN,
            Json(DecisionResponse { success: false }),
        )
            .into_response();
    }
    if id.len() > 255
        || payload
            .edited_payload
            .as_ref()
            .is_some_and(|value| value.to_string().len() > 65536)
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(DecisionResponse { success: false }),
        )
            .into_response();
    }
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(DecisionResponse { success: false }),
            )
                .into_response();
        }
    };

    match orchestrator
        .decide_approval(&id, &tenant_id, payload.approved, payload.edited_payload)
        .await
    {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DecisionResponse { success: false }),
        )
            .into_response(),
    }
}
// Support for AI Agent Department Architecture

async fn list_ledger_entries(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "entries": [] })),
            )
                .into_response();
        }
    };

    let limit = query.limit.unwrap_or(50);

    match orchestrator
        .get_ledger_entries(&tenant_id, limit as i64)
        .await
    {
        Ok(entries) => (
            StatusCode::OK,
            Json(serde_json::json!({ "entries": entries })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )
            .into_response(),
    }
}
