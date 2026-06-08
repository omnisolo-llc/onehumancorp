use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use crate::api::assistant::AssistantState;
use ::server_common::Claims;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Approval {
    pub id: String,
    pub task_id: String,
    pub tool_name: String,
    pub args: Option<serde_json::Value>,
    pub status: Option<String>,
    pub risk_level: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct DecideApprovalRequest {
    pub status: String, // 'approved' or 'denied'
}

pub fn router<S>(state: AssistantState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_approvals))
        .route("/task/:task_id", get(list_task_approvals))
        .route("/:id/decide", post(decide_approval))
        .with_state(state)
}

async fn list_approvals(
    State(state): State<AssistantState>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json::<Vec<Approval>>(vec![])).into_response(),
    };

    let approvals = match sqlx::query_as::<_, Approval>(
        "SELECT id, task_id, tool_name, args, status, risk_level, created_at, updated_at FROM assistant_approvals WHERE tenant_id = $1 ORDER BY created_at DESC"
    )
    .bind(tenant_id)
    .fetch_all(&state.db.pool)
    .await {
        Ok(a) => a,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<Approval>>(vec![])).into_response(),
    };

    (StatusCode::OK, Json(approvals)).into_response()
}

async fn list_task_approvals(
    State(state): State<AssistantState>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json::<Vec<Approval>>(vec![])).into_response(),
    };

    let approvals = match sqlx::query_as::<_, Approval>(
        "SELECT id, task_id, tool_name, args, status, risk_level, created_at, updated_at FROM assistant_approvals WHERE task_id = $1 AND tenant_id = $2 ORDER BY created_at DESC"
    )
    .bind(task_id)
    .bind(tenant_id)
    .fetch_all(&state.db.pool)
    .await {
        Ok(a) => a,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<Approval>>(vec![])).into_response(),
    };

    (StatusCode::OK, Json(approvals)).into_response()
}

async fn decide_approval(
    State(state): State<AssistantState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<DecideApprovalRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response(),
    };

    match sqlx::query(
        "UPDATE assistant_approvals SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3"
    )
    .bind(payload.status)
    .bind(&id)
    .bind(tenant_id)
    .execute(&state.db.pool)
    .await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to update approval"}))).into_response(),
    }
}
