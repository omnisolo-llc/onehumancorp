use axum::{
    extract::{Extension, State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use ::server_common::Claims;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone)]
pub struct PendingAction {
    pub id: String,
    pub tenant_id: String,
    pub agent_id: String,
    pub risk_level: String,
    pub payload: serde_json::Value,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct PendingActionsResponse {
    pub actions: Vec<PendingAction>,
}

#[derive(Deserialize)]
pub struct TaskDecisionRequest {
    pub task_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct DecisionResponse {
    pub success: bool,
}

pub fn router(pool: PgPool) -> Router<PgPool> {
    Router::new()
        .route("/actions/pending", get(handle_get_pending_actions))
        .route("/actions/approve", post(handle_approve_action))
        .route("/actions/reject", post(handle_reject_action))
        .with_state(pool)
}

pub async fn get_pending_actions(pool: &PgPool, tenant_id: &str) -> Vec<PendingAction> {
    let mut results = Vec::new();
    let query = "SELECT id, tenant_id, agent_id, risk_level, payload, status, created_at, updated_at FROM agent_pending_actions WHERE tenant_id = $1 AND status = 'PENDING'";
    if let Ok(rows) = sqlx::query(query).bind(tenant_id).fetch_all(pool).await {
        for row in rows {
            let payload_str: Option<String> = row.get("payload");
            let payload = if let Some(s) = payload_str {
                serde_json::from_str(&s).unwrap_or(serde_json::json!({}))
            } else {
                let payload_json: Option<serde_json::Value> = row.try_get("payload").ok();
                payload_json.unwrap_or(serde_json::json!({}))
            };
            results.push(PendingAction {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                agent_id: row.get("agent_id"),
                risk_level: row.get("risk_level"),
                payload,
                status: row.get("status"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }
    }
    results
}

pub async fn create_pending_action(pool: &PgPool, action: PendingAction) -> Result<(), String> {
    let query = "INSERT INTO agent_pending_actions (id, tenant_id, agent_id, risk_level, payload, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";
    let payload_val = action.payload.to_string();
    sqlx::query(query)
        .bind(&action.id)
        .bind(&action.tenant_id)
        .bind(&action.agent_id)
        .bind(&action.risk_level)
        .bind(&payload_val)
        .bind(&action.status)
        .bind(action.created_at)
        .bind(action.updated_at)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn approve_action(pool: &PgPool, tenant_id: &str, task_id: &str) -> Result<(), String> {
    let query = "UPDATE agent_pending_actions SET status = 'APPROVED', updated_at = $1 WHERE id = $2 AND tenant_id = $3";
    let now = Utc::now();
    let res = sqlx::query(query)
        .bind(now)
        .bind(task_id)
        .bind(tenant_id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    if res.rows_affected() > 0 {
        Ok(())
    } else {
        Err("Action not found or unauthorized".to_string())
    }
}

pub async fn reject_action(pool: &PgPool, tenant_id: &str, task_id: &str) -> Result<(), String> {
    let query = "UPDATE agent_pending_actions SET status = 'REJECTED', updated_at = $1 WHERE id = $2 AND tenant_id = $3";
    let now = Utc::now();
    let res = sqlx::query(query)
        .bind(now)
        .bind(task_id)
        .bind(tenant_id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    if res.rows_affected() > 0 {
        Ok(())
    } else {
        Err("Action not found or unauthorized".to_string())
    }
}

async fn handle_get_pending_actions(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(PendingActionsResponse { actions: vec![] })).into_response(),
    };

    let actions = get_pending_actions(&pool, &tenant_id).await;
    (StatusCode::OK, Json(PendingActionsResponse { actions })).into_response()
}

async fn handle_approve_action(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<TaskDecisionRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    match approve_action(&pool, &tenant_id, &payload.task_id).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response(),
    }
}

async fn handle_reject_action(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<TaskDecisionRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    match reject_action(&pool, &tenant_id, &payload.task_id).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response(),
    }
}
