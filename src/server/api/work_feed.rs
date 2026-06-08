use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;
use tracing::info;

// For this test environment, we'll extract the Tenant-Id from headers directly
// In a full environment, this would use a proper Axum extractor over JWT claims.
use axum::http::HeaderMap;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct WorkItem {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub type_: String,
    pub status: String,
    pub title: String,
    pub preview: Option<String>,
    pub draft_response: Option<String>,
    pub payload: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct CreateWorkItemRequest {
    pub tenant_id: Uuid,
    pub type_: String,
    pub title: String,
    pub preview: Option<String>,
    pub payload: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct UpdateWorkItemRequest {
    pub status: Option<String>,
    pub draft_response: Option<String>,
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/work-feed", get(list_work_items).post(create_work_item))
        .route("/api/v1/work-feed/{id}", put(update_work_item))
        .with_state(state)
}

fn extract_tenant_id(headers: &HeaderMap) -> Result<Uuid, StatusCode> {
    if let Some(tenant_header) = headers.get("x-tenant-id") {
        if let Ok(tenant_str) = tenant_header.to_str() {
            if let Ok(tenant_uuid) = Uuid::parse_str(tenant_str) {
                return Ok(tenant_uuid);
            }
        }
    }

    // Fallback for dev/CI
    if std::env::var("CI").is_ok() || cfg!(test) || std::env::var("OHC_DEFAULT_TENANT_ID").is_ok() {
        let default_id = std::env::var("OHC_DEFAULT_TENANT_ID")
            .unwrap_or_else(|_| "00000000-0000-0000-0000-000000000000".to_string());
        Ok(Uuid::parse_str(&default_id).unwrap_or_else(|_| Uuid::nil()))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn list_work_items(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let tenant_id = match extract_tenant_id(&headers) {
        Ok(id) => id,
        Err(status) => return (status, Json(serde_json::json!([]))),
    };

    let mut tx = match state.db_pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!([]))),
    };

    let _ = sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
        .bind(tenant_id.to_string())
        .execute(&mut *tx)
        .await;

    let items = sqlx::query_as::<_, WorkItem>(
        r#"
        SELECT id, tenant_id, type as type_, status, title, preview, draft_response, payload, created_at, updated_at
        FROM work_items
        WHERE status != 'archived' AND status != 'completed' AND tenant_id = $1
        ORDER BY created_at DESC
        "#
    )
    .bind(tenant_id)
    .fetch_all(&mut *tx)
    .await
    .unwrap_or_default();

    let _ = tx.commit().await;

    (StatusCode::OK, Json(serde_json::to_value(items).unwrap_or_else(|_| serde_json::json!([]))))
}

async fn create_work_item(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateWorkItemRequest>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let payload_json = payload.payload.unwrap_or_else(|| serde_json::json!({}));

    let draft = if payload.type_ == "message" {
        Some(format!("Drafted response for: {}", payload.title))
    } else {
        None
    };

    let status = if draft.is_some() { "drafted" } else { "pending" };

    let mut tx = match state.db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to start transaction: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to create"})));
        }
    };

    let _ = sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
        .bind(payload.tenant_id.to_string())
        .execute(&mut *tx)
        .await;

    let result = sqlx::query(
        r#"
        INSERT INTO work_items (id, tenant_id, type, status, title, preview, draft_response, payload)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#
    )
    .bind(id)
    .bind(payload.tenant_id)
    .bind(payload.type_)
    .bind(status)
    .bind(payload.title)
    .bind(payload.preview)
    .bind(draft)
    .bind(payload_json)
    .fetch_one(&mut *tx)
    .await;

    match result {
        Ok(record) => {
            let record_id: Uuid = record.get("id");
            let _ = tx.commit().await;
            info!("Created work item: {}", record_id);
            (StatusCode::CREATED, Json(serde_json::json!({"id": record_id})))
        }
        Err(e) => {
            let _ = tx.rollback().await;
            tracing::error!("Failed to create work item: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to create"})))
        }
    }
}

async fn update_work_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<UpdateWorkItemRequest>,
) -> impl IntoResponse {
    let tenant_id = match extract_tenant_id(&headers) {
        Ok(id) => id,
        Err(status) => return (status, Json(serde_json::json!({"error": "Unauthorized"}))),
    };

    let mut tx = match state.db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to start transaction: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to update"})));
        }
    };

    let _ = sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
        .bind(tenant_id.to_string())
        .execute(&mut *tx)
        .await;

    let result = sqlx::query(
        r#"
        UPDATE work_items
        SET status = COALESCE($1, status),
            draft_response = COALESCE($2, draft_response),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $3 AND tenant_id = $4
        "#
    )
    .bind(payload.status)
    .bind(payload.draft_response)
    .bind(id)
    .bind(tenant_id)
    .execute(&mut *tx)
    .await;

    match result {
        Ok(_) => {
            let _ = tx.commit().await;
            (StatusCode::OK, Json(serde_json::json!({"success": true})))
        }
        Err(e) => {
            let _ = tx.rollback().await;
            tracing::error!("Failed to update work item: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to update"})))
        }
    }
}
