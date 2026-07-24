use axum::{
    extract::{State, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::{DB, DbStore};
use sqlx::Row;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
}

#[derive(Serialize, Deserialize)]
pub struct ResourcePayload {
    pub name: String,
    pub description: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AvailabilityBlockPayload {
    pub resource_id: String,
    pub start_time: String,
    pub end_time: String,
    pub is_recurring: Option<bool>,
    pub recurrence_rule: Option<String>,
}

pub fn router<S>(db: Arc<DB>) -> Router<S> where S: Clone + Send + Sync + 'static, {
    let state = AppState { db };
    Router::new()
        .route("/resources", get(list_resources).post(create_resource))
        .route("/resources/:id", put(update_resource).delete(delete_resource))
        .route("/availability", get(list_availability).post(create_availability))
        .route("/availability/:id", delete(delete_availability))
        .with_state(state)
}

async fn list_resources(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let rows = match &state.db.store {
        DbStore::Sqlite(pool) => {
            sqlx::query("SELECT id, name, description, type, CAST(created_at AS TEXT) AS created_at FROM resources WHERE tenant_id = ?")
                .bind(&tenant_id)
                .fetch_all(pool)
                .await
        }
        DbStore::Postgres(pool) => {
            let mut tx = pool.begin().await.unwrap();
            let _ = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;
            sqlx::query("SELECT id, name, description, type, created_at::text AS created_at FROM resources WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_all(&mut *tx)
                .await
        }
    };

    match rows {
        Ok(records) => {
            let res: Vec<serde_json::Value> = records.into_iter().map(|r| {
                serde_json::json!({
                    "id": r.get::<String, _>("id"),
                    "name": r.get::<String, _>("name"),
                    "description": r.get::<Option<String>, _>("description"),
                    "type": r.get::<Option<String>, _>("type"),
                    "created_at": r.get::<Option<String>, _>("created_at"),
                })
            }).collect();
            (StatusCode::OK, Json(res)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list resources: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response()
        }
    }
}

async fn create_resource(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<ResourcePayload>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let r_type = payload.r#type.unwrap_or_else(|| "provider".to_string());

    let res = match &state.db.store {
        DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO resources (id, tenant_id, name, description, type) VALUES (?, ?, ?, ?, ?)")
                .bind(&id).bind(&tenant_id).bind(&payload.name).bind(&payload.description).bind(&r_type)
                .execute(pool)
                .await
        }
        DbStore::Postgres(pool) => {
            let mut tx = pool.begin().await.unwrap();
            let _ = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;
            let result = sqlx::query("INSERT INTO resources (id, tenant_id, name, description, type) VALUES ($1, $2, $3, $4, $5)")
                .bind(&id).bind(&tenant_id).bind(&payload.name).bind(&payload.description).bind(&r_type)
                .execute(&mut *tx)
                .await;
            let _ = tx.commit().await;
            result
        }
    };

    match res {
        Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({"id": id}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to create resource: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response()
        }
    }
}

async fn update_resource(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
    Json(payload): Json<ResourcePayload>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let res = match &state.db.store {
        DbStore::Sqlite(pool) => {
            sqlx::query("UPDATE resources SET name = ?, description = ?, type = COALESCE(?, type), updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
                .bind(&payload.name).bind(&payload.description).bind(&payload.r#type).bind(&id).bind(&tenant_id)
                .execute(pool)
                .await
        }
        DbStore::Postgres(pool) => {
            let mut tx = pool.begin().await.unwrap();
            let _ = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;
            let result = sqlx::query("UPDATE resources SET name = $1, description = $2, type = COALESCE($3, type), updated_at = CURRENT_TIMESTAMP WHERE id = $4 AND tenant_id = $5")
                .bind(&payload.name).bind(&payload.description).bind(&payload.r#type).bind(&id).bind(&tenant_id)
                .execute(&mut *tx)
                .await;
            let _ = tx.commit().await;
            result
        }
    };

    match res {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to update resource: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response()
        }
    }
}

async fn delete_resource(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let res = match &state.db.store {
        DbStore::Sqlite(pool) => {
            sqlx::query("DELETE FROM resources WHERE id = ? AND tenant_id = ?")
                .bind(&id).bind(&tenant_id)
                .execute(pool)
                .await
        }
        DbStore::Postgres(pool) => {
            let mut tx = pool.begin().await.unwrap();
            let _ = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;
            let result = sqlx::query("DELETE FROM resources WHERE id = $1 AND tenant_id = $2")
                .bind(&id).bind(&tenant_id)
                .execute(&mut *tx)
                .await;
            let _ = tx.commit().await;
            result
        }
    };

    match res {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to delete resource: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response()
        }
    }
}


async fn list_availability(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let rows = match &state.db.store {
        DbStore::Sqlite(pool) => {
            sqlx::query("SELECT id, resource_id, CAST(start_time AS TEXT) AS start_time, CAST(end_time AS TEXT) AS end_time, is_recurring, recurrence_rule FROM availability_blocks WHERE tenant_id = ?")
                .bind(&tenant_id)
                .fetch_all(pool)
                .await
        }
        DbStore::Postgres(pool) => {
            let mut tx = pool.begin().await.unwrap();
            let _ = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;
            sqlx::query("SELECT id, resource_id, start_time::text AS start_time, end_time::text AS end_time, is_recurring, recurrence_rule FROM availability_blocks WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_all(&mut *tx)
                .await
        }
    };

    match rows {
        Ok(records) => {
            let res: Vec<serde_json::Value> = records.into_iter().map(|r| {
                serde_json::json!({
                    "id": r.get::<String, _>("id"),
                    "resource_id": r.get::<String, _>("resource_id"),
                    "start_time": r.get::<String, _>("start_time"),
                    "end_time": r.get::<String, _>("end_time"),
                    "is_recurring": r.get::<Option<bool>, _>("is_recurring"),
                    "recurrence_rule": r.get::<Option<String>, _>("recurrence_rule"),
                })
            }).collect();
            (StatusCode::OK, Json(res)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list availability: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response()
        }
    }
}

async fn create_availability(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<AvailabilityBlockPayload>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let is_recurring = payload.is_recurring.unwrap_or(false);

    // Parse times
    let st = chrono::DateTime::parse_from_rfc3339(&payload.start_time).unwrap();
    let et = chrono::DateTime::parse_from_rfc3339(&payload.end_time).unwrap();

    let res = match &state.db.store {
        DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO availability_blocks (id, tenant_id, resource_id, start_time, end_time, is_recurring, recurrence_rule) VALUES (?, ?, ?, ?, ?, ?, ?)")
                .bind(&id).bind(&tenant_id).bind(&payload.resource_id).bind(&st.to_rfc3339()).bind(&et.to_rfc3339()).bind(&is_recurring).bind(&payload.recurrence_rule)
                .execute(pool)
                .await
        }
        DbStore::Postgres(pool) => {
            let mut tx = pool.begin().await.unwrap();
            let _ = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;
            let result = sqlx::query("INSERT INTO availability_blocks (id, tenant_id, resource_id, start_time, end_time, is_recurring, recurrence_rule) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                .bind(&id).bind(&tenant_id).bind(&payload.resource_id).bind(st).bind(et).bind(&is_recurring).bind(&payload.recurrence_rule)
                .execute(&mut *tx)
                .await;
            let _ = tx.commit().await;
            result
        }
    };

    match res {
        Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({"id": id}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to create availability: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response()
        }
    }
}

async fn delete_availability(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let res = match &state.db.store {
        DbStore::Sqlite(pool) => {
            sqlx::query("DELETE FROM availability_blocks WHERE id = ? AND tenant_id = ?")
                .bind(&id).bind(&tenant_id)
                .execute(pool)
                .await
        }
        DbStore::Postgres(pool) => {
            let mut tx = pool.begin().await.unwrap();
            let _ = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;
            let result = sqlx::query("DELETE FROM availability_blocks WHERE id = $1 AND tenant_id = $2")
                .bind(&id).bind(&tenant_id)
                .execute(&mut *tx)
                .await;
            let _ = tx.commit().await;
            result
        }
    };

    match res {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to delete availability: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response()
        }
    }
}
