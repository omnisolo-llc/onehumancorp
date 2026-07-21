use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;
use crate::db::DB;

pub struct AppState {
    pub db: Arc<DB>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub tenant_id: String,
    pub customer_id: String,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionResponse {
    pub id: String,
    pub magic_token: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApprovalRequest {
    pub session_id: String,
    pub request_type: String, // 'Quote', 'Design', 'ChangeOrder'
    pub description: String,
    pub reference_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalResponse {
    pub id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitApprovalRequest {
    pub status: String, // 'Approved', 'Rejected'
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    let state = AppState { db };
    Router::new()
        .route("/sessions", post(create_session))
        .route("/sessions/:token", get(get_session_by_token))
        .route("/sessions/:session_id/approvals", post(create_approval_request))
        .route("/approvals/:approval_id", post(submit_approval))
        .with_state(std::sync::Arc::new(state))
}

async fn create_session(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<impl IntoResponse, (axum::http::StatusCode, String)> {
    let id = Uuid::new_v4().to_string();
    let magic_token = format!("cp_{}", Uuid::new_v4().to_string().replace("-", ""));
    let expires_at = payload.expires_at.unwrap_or_else(|| Utc::now() + chrono::Duration::days(7));

    let pool = state.db.pool.clone();
    let mut tx = pool.begin().await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    // Simulate RLS by passing the tenant ID as app.current_tenant
    sqlx::query("SET LOCAL app.current_tenant = $1")
        .bind(&payload.tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    sqlx::query(
        r#"
        INSERT INTO client_portal_sessions (id, tenant_id, customer_id, magic_token, expires_at)
        VALUES ($1, $2, $3, $4, $5)
        "#)
        .bind(&id)
        .bind(&payload.tenant_id)
        .bind(&payload.customer_id)
        .bind(&magic_token)
        .bind(&expires_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert client portal session: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string())
        })?;

    tx.commit().await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    let url = format!("https://portal.onehumancorp.com/{}", magic_token);

    Ok((
        StatusCode::CREATED,
        Json(CreateSessionResponse {
            id,
            magic_token,
            url,
        }),
    ))
}

#[derive(Debug, Serialize)]
pub struct SessionStateResponse {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub expires_at: DateTime<Utc>,
    pub pending_approvals: Vec<ApprovalState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalState {
    pub id: String,
    pub status: String,
    pub request_type: String,
    pub description: String,
    pub reference_id: Option<String>,
}

async fn get_session_by_token(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
) -> Result<impl IntoResponse, (axum::http::StatusCode, String)> {
    let pool = state.db.pool.clone();

    // Find the session (needs admin privileges as we don't know the tenant yet)
    let row: Option<(String, String, String, DateTime<Utc>)> = sqlx::query_as(
        r#"
        SELECT id, tenant_id, customer_id, expires_at
        FROM client_portal_sessions
        WHERE magic_token = $1 AND expires_at > NOW()
        "#)
        .bind(&token)
        .fetch_optional(&pool)
        .await
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    let row = row.ok_or((axum::http::StatusCode::NOT_FOUND, "Not Found".to_string()))?;
    let session_id = row.0;
    let tenant_id = row.1;
    let customer_id = row.2;
    let expires_at = row.3;

    // We fetch the approvals for this session. We can use the tenant context now.
    let mut tx = pool.begin().await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    sqlx::query("SET LOCAL app.current_tenant = $1")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    let approval_rows: Vec<(String, String, String, String, Option<String>)> = sqlx::query_as(
        r#"
        SELECT id, status, type as request_type, description, reference_id
        FROM client_approval_requests
        WHERE session_id = $1 AND status = 'Pending'
        "#)
        .bind(&session_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    let mut pending_approvals = Vec::new();
    for r in approval_rows {
        pending_approvals.push(ApprovalState {
            id: r.0,
            status: r.1,
            request_type: r.2,
            description: r.3,
            reference_id: r.4,
        });
    }

    tx.commit().await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    Ok(Json(SessionStateResponse {
        id: session_id,
        tenant_id,
        customer_id,
        expires_at,
        pending_approvals,
    }))
}


async fn create_approval_request(
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<String>,
    Json(payload): Json<CreateApprovalRequest>,
) -> Result<impl IntoResponse, (axum::http::StatusCode, String)> {
    let id = Uuid::new_v4().to_string();

    let pool = state.db.pool.clone();

    // We must find the tenant for this session to set RLS correctly
    let row: Option<(String,)> = sqlx::query_as(
        r#"SELECT tenant_id FROM client_portal_sessions WHERE id = $1"#)
        .bind(&session_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    let row = row.ok_or((axum::http::StatusCode::NOT_FOUND, "Not Found".to_string()))?;
    let tenant_id = row.0;

    let mut tx = pool.begin().await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    sqlx::query("SET LOCAL app.current_tenant = $1")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    sqlx::query(
        r#"
        INSERT INTO client_approval_requests (id, session_id, type, description, reference_id)
        VALUES ($1, $2, $3, $4, $5)
        "#)
        .bind(&id)
        .bind(&session_id)
        .bind(&payload.request_type)
        .bind(&payload.description)
        .bind(&payload.reference_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
             tracing::error!("Failed to insert client approval request: {:?}", e);
             (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string())
        })?;

    tx.commit().await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(ApprovalResponse {
            id,
            status: "Pending".to_string(),
        }),
    ))
}


async fn submit_approval(
    State(state): State<Arc<AppState>>,
    Path(approval_id): Path<String>,
    Json(payload): Json<SubmitApprovalRequest>,
) -> Result<impl IntoResponse, (axum::http::StatusCode, String)> {
    if payload.status != "Approved" && payload.status != "Rejected" {
         return Err((axum::http::StatusCode::BAD_REQUEST, "Status must be 'Approved' or 'Rejected'".to_string()));
    }

    let pool = state.db.pool.clone();

    // Look up the session ID from the approval to get the tenant ID
    let row: Option<(String,)> = sqlx::query_as(
        r#"
        SELECT s.tenant_id
        FROM client_approval_requests a
        JOIN client_portal_sessions s ON a.session_id = s.id
        WHERE a.id = $1
        "#)
        .bind(&approval_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    let row = row.ok_or((axum::http::StatusCode::NOT_FOUND, "Not Found".to_string()))?;
    let tenant_id = row.0;

    let mut tx = pool.begin().await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    sqlx::query("SET LOCAL app.current_tenant = $1")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    let rows_affected = sqlx::query(
        r#"
        UPDATE client_approval_requests
        SET status = $1, updated_at = NOW()
        WHERE id = $2 AND status = 'Pending'
        "#)
        .bind(&payload.status)
        .bind(&approval_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?
        .rows_affected();

    if rows_affected == 0 {
        return Err((axum::http::StatusCode::NOT_FOUND, "Not Found".to_string())); // Or already processed
    }

    tx.commit().await.map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error".to_string()))?;

    Ok(Json(ApprovalResponse {
        id: approval_id,
        status: payload.status,
    }))
}
