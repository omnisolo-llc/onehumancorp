use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub description: String,
    pub priority: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTaskRequest {
    pub status: String,
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub tenant_id: String,
    pub staff_id: Option<String>,
    pub description: String,
    pub status: String,
    pub priority: String,
}

fn get_tenant_id(headers: &HeaderMap) -> Option<String> {
    headers.get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

pub async fn list_tasks(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let tasks = match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let rows = sqlx::query(
                "SELECT id, tenant_id, staff_id, description, status, priority FROM staff_tasks WHERE tenant_id = ?"
            )
            .bind(&tenant_id)
            .fetch_all(pool)
            .await;

            rows.map(|rows| rows.into_iter().map(|row| {
                use sqlx::Row;
                TaskResponse {
                    id: row.get::<String, _>("id"),
                    tenant_id: row.get::<String, _>("tenant_id"),
                    staff_id: row.get::<Option<String>, _>("staff_id"),
                    description: row.get::<String, _>("description"),
                    status: row.get::<String, _>("status"),
                    priority: row.get::<String, _>("priority"),
                }
            }).collect::<Vec<_>>()).unwrap_or_default()
        }
        crate::db::DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(_) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response(),
            };
            if let Err(_) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
            let rows = sqlx::query(
                "SELECT id, tenant_id, staff_id, description, status, priority FROM staff_tasks WHERE tenant_id = $1"
            )
            .bind(&tenant_id)
            .fetch_all(&mut *tx)
            .await;
            let _ = tx.commit().await;

            rows.map(|rows| rows.into_iter().map(|row| {
                use sqlx::Row;
                TaskResponse {
                    id: row.get::<String, _>("id"),
                    tenant_id: row.get::<String, _>("tenant_id"),
                    staff_id: row.get::<Option<String>, _>("staff_id"),
                    description: row.get::<String, _>("description"),
                    status: row.get::<String, _>("status"),
                    priority: row.get::<String, _>("priority"),
                }
            }).collect::<Vec<_>>()).unwrap_or_default()
        }
    };

    (axum::http::StatusCode::OK, Json(tasks)).into_response()
}

pub async fn create_task(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
    Json(payload): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let task_id = uuid::Uuid::new_v4().to_string();
    let priority = payload.priority.unwrap_or_else(|| "NORMAL".to_string());

    match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let _ = sqlx::query(
                "INSERT INTO staff_tasks (id, tenant_id, description, status, priority) VALUES (?, ?, ?, 'PENDING', ?)"
            )
            .bind(&task_id)
            .bind(&tenant_id)
            .bind(&payload.description)
            .bind(&priority)
            .execute(pool)
            .await;
        }
        crate::db::DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(_) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response(),
            };
            if let Err(_) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
            let _ = sqlx::query(
                "INSERT INTO staff_tasks (id, tenant_id, description, status, priority) VALUES ($1, $2, $3, 'PENDING', $4)"
            )
            .bind(&task_id)
            .bind(&tenant_id)
            .bind(&payload.description)
            .bind(&priority)
            .execute(&mut *tx)
            .await;
            let _ = tx.commit().await;
        }
    }

    let response = TaskResponse {
        id: task_id,
        tenant_id,
        staff_id: None,
        description: payload.description,
        status: "PENDING".to_string(),
        priority,
    };

    (axum::http::StatusCode::CREATED, Json(response)).into_response()
}

pub async fn update_task(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTaskRequest>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let _ = sqlx::query(
                "UPDATE staff_tasks SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?"
            )
            .bind(&payload.status)
            .bind(&id)
            .bind(&tenant_id)
            .execute(pool)
            .await;
        }
        crate::db::DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(_) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response(),
            };
            if let Err(_) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
            let _ = sqlx::query(
                "UPDATE staff_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3"
            )
            .bind(&payload.status)
            .bind(&id)
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await;
            let _ = tx.commit().await;
        }
    }

    (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    Router::new()
        .route("/", get(list_tasks).post(create_task))
        .route("/{id}", put(update_task))
        .with_state(db)
}
