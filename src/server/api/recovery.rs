use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sqlx::{PgPool, Row};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<PgPool>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/campaigns", get(list_campaigns))
        .route("/attempts", get(list_attempts))
        .route("/attempts/:id/approve", post(approve_attempt))
}

async fn list_campaigns(
    State(state): State<AppState>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
) -> impl IntoResponse {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthenticated" }))).into_response();
            } else {
                auth.org_id.clone()
            }
        },
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthenticated" }))).into_response()
    };

    let rows = match sqlx::query("SELECT id, name, auto_send, delay_minutes FROM recovery_campaigns WHERE tenant_id = $1")
        .bind(&tenant_id)
        .fetch_all(&*state.pool)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            tracing::error!("Failed to fetch recovery campaigns: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Database error" }))).into_response();
        }
    };

    let campaigns: Vec<serde_json::Value> = rows.into_iter().map(|row| {
        serde_json::json!({
            "id": row.get::<String, _>("id"),
            "name": row.get::<String, _>("name"),
            "auto_send": row.get::<bool, _>("auto_send"),
            "delay_minutes": row.get::<i32, _>("delay_minutes"),
        })
    }).collect();

    (StatusCode::OK, Json(campaigns)).into_response()
}

async fn list_attempts(
    State(state): State<AppState>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
) -> impl IntoResponse {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthenticated" }))).into_response();
            } else {
                auth.org_id.clone()
            }
        },
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthenticated" }))).into_response()
    };

    let rows = match sqlx::query("SELECT id, customer_id, source_event_id, assistant_message_id, status FROM recovery_attempts WHERE tenant_id = $1")
        .bind(&tenant_id)
        .fetch_all(&*state.pool)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            tracing::error!("Failed to fetch recovery attempts: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Database error" }))).into_response();
        }
    };

    let attempts: Vec<serde_json::Value> = rows.into_iter().map(|row| {
        serde_json::json!({
            "id": row.get::<String, _>("id"),
            "customer_id": row.try_get::<Option<String>, _>("customer_id").unwrap_or(None),
            "source_event_id": row.get::<String, _>("source_event_id"),
            "assistant_message_id": row.try_get::<Option<String>, _>("assistant_message_id").unwrap_or(None),
            "status": row.get::<String, _>("status"),
        })
    }).collect();

    (StatusCode::OK, Json(attempts)).into_response()
}

async fn approve_attempt(
    Path(id): Path<String>,
    State(state): State<AppState>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
) -> impl IntoResponse {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthenticated" }))).into_response();
            } else {
                auth.org_id.clone()
            }
        },
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthenticated" }))).into_response()
    };

    let res = sqlx::query("UPDATE recovery_attempts SET status = 'APPROVED' WHERE id = $1 AND tenant_id = $2 AND status = 'PENDING'")
        .bind(&id)
        .bind(&tenant_id)
        .execute(&*state.pool)
        .await;

    match res {
        Ok(result) => {
            if result.rows_affected() > 0 {
                (StatusCode::OK, Json(serde_json::json!({ "success": true, "message": format!("Attempt {} approved", id) }))).into_response()
            } else {
                (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Attempt not found or not in PENDING state" }))).into_response()
            }
        }
        Err(e) => {
            tracing::error!("Failed to approve attempt: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Failed to approve attempt" }))).into_response()
        }
    }
}
