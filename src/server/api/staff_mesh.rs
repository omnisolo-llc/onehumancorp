use axum::{
    extract::{Path, State},
    response::IntoResponse,
    http::HeaderMap,
    routing::post,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateStaffRequest {
    pub name: String,
    pub phone_number: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct CreateStaffResponse {
    pub id: String,
    pub invite_token: String,
}

#[derive(Deserialize)]
pub struct SetPinRequest {
    pub pin: String,
}

#[derive(Serialize)]
pub struct SetPinResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct StaffMember {
    pub id: String,
    pub name: String,
    pub phone_number: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct GetStaffResponse {
    pub staff: Vec<StaffMember>,
}

#[derive(Deserialize)]
pub struct SyncTimecardRequest {
    pub events: Vec<TimecardEventInput>,
}

#[derive(Deserialize)]
pub struct TimecardEventInput {
    pub id: String,
    pub staff_id: String,
    pub event_type: String,
    pub offline_timestamp: String,
}

#[derive(Serialize)]
pub struct SyncTimecardResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct GetTimecardResponse {
    pub events: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub staff_id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub id: String,
}

#[derive(Deserialize)]
pub struct UpdateTaskRequest {
    pub status: Option<String>,
    pub title: Option<String>,
}

#[derive(Serialize)]
pub struct GetTasksResponse {
    pub tasks: Vec<serde_json::Value>,
}

#[derive(Serialize)]
pub struct GetSummariesResponse {
    pub summaries: Vec<serde_json::Value>,
}

fn get_tenant_id(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-spiffe-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|val| ::server_auth::parse_spiffe_id(val).ok())
        .map(|(t, _)| t)
}

pub async fn create_staff_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
    Json(payload): Json<CreateStaffRequest>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };
    let staff_id = format!("staff_{}", Uuid::new_v4());

    // In a real implementation, we'd create a token in a store. Here we just use a dummy token pattern for demonstration.
    let invite_token = format!("invite_{}", Uuid::new_v4());

    match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "INSERT INTO ohc_staff_member (id, tenant_id, name, phone_number, role) VALUES (?, ?, ?, ?, ?)",
            )
            .bind(&staff_id)
            .bind(&tenant_id)
            .bind(&payload.name)
            .bind(&payload.phone_number)
            .bind(&payload.role)
            .execute(pool)
            .await;
            if res.is_err() {
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "db_error"})),
                ).into_response();
            }
        }
        crate::db::DbStore::Postgres => {
             let mut tx = match db.pool.begin().await {
                 Ok(tx) => tx,
                 Err(e) => {
                     tracing::error!("Failed to begin transaction: {:?}", e);
                     return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
                 }
             };
             if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                 tracing::error!("Failed to set org context: {:?}", e);
                 return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
             }
             let res = sqlx::query(
                "INSERT INTO ohc_staff_member (id, tenant_id, name, phone_number, role) VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(&staff_id)
            .bind(&tenant_id)
            .bind(&payload.name)
            .bind(&payload.phone_number)
            .bind(&payload.role)
            .execute(&mut *tx)
            .await;
             if let Err(e) = res {
                tracing::error!("Failed to insert staff member: {:?}", e);
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "db_error"})),
                ).into_response();
            }
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "db_error"})),
                ).into_response();
            }
        }
    }

    (axum::http::StatusCode::OK, Json(CreateStaffResponse { id: staff_id, invite_token })).into_response()
}

pub async fn set_staff_pin_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    State(db): State<Arc<DB>>,
    Json(payload): Json<SetPinRequest>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    // In a real app, hash the pin here (e.g. using bcrypt)
    let pin_hash = format!("hashed_{}", payload.pin);

    match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "UPDATE ohc_staff_member SET pin_hash = ? WHERE id = ? AND tenant_id = ?",
            )
            .bind(&pin_hash)
            .bind(&id)
            .bind(&tenant_id)
            .execute(pool)
            .await;
            if res.is_err() {
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "db_error"})),
                ).into_response();
            }
        }
        crate::db::DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin transaction: {:?}", e);
                    return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
                }
            };
            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                tracing::error!("Failed to set org context: {:?}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
            let res = sqlx::query(
                "UPDATE ohc_staff_member SET pin_hash = $1 WHERE id = $2 AND tenant_id = $3",
            )
            .bind(&pin_hash)
            .bind(&id)
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await;
            if let Err(e) = res {
                tracing::error!("Failed to set staff pin: {:?}", e);
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "db_error"})),
                ).into_response();
            }
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "db_error"})),
                ).into_response();
            }
        }
    }

    (axum::http::StatusCode::OK, Json(SetPinResponse { success: true })).into_response()
}

pub async fn get_staff_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let staff: Vec<StaffMember> = match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let rows: Result<Vec<(String, String, String, String)>, _> = sqlx::query_as(
                "SELECT id, name, phone_number, role FROM ohc_staff_member WHERE tenant_id = ?",
            )
            .bind(&tenant_id)
            .fetch_all(pool)
            .await;

            rows.unwrap_or_default().into_iter().map(|(id, name, phone_number, role)| {
                StaffMember { id, name, phone_number, role }
            }).collect()
        }
        crate::db::DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin transaction: {:?}", e);
                    return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
                }
            };
            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                tracing::error!("Failed to set org context: {:?}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
            let rows: Result<Vec<(String, String, String, String)>, _> = sqlx::query_as(
                "SELECT id, name, phone_number, role FROM ohc_staff_member WHERE tenant_id = $1",
            )
            .bind(&tenant_id)
            .fetch_all(&mut *tx)
            .await;
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "db_error"})),
                ).into_response();
            }

            rows.unwrap_or_default().into_iter().map(|(id, name, phone_number, role)| {
                StaffMember { id, name, phone_number, role }
            }).collect()
        }
    };

    (axum::http::StatusCode::OK, Json(GetStaffResponse { staff })).into_response()
}

pub async fn sync_timecard_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
    Json(payload): Json<SyncTimecardRequest>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    for event in payload.events {
        match &db.store {
            crate::db::DbStore::Sqlite(pool) => {
                let _ = sqlx::query(
                    "INSERT INTO ohc_timecard_event (id, tenant_id, staff_id, event_type, event_time) VALUES (?, ?, ?, ?, ?)",
                )
                .bind(&event.id)
                .bind(&tenant_id)
                .bind(&event.staff_id)
                .bind(&event.event_type)
                .bind(&event.offline_timestamp)
                .execute(pool)
                .await;
            }
            crate::db::DbStore::Postgres => {
                let mut tx = match db.pool.begin().await {
                    Ok(tx) => tx,
                    Err(e) => {
                        tracing::error!("Failed to begin transaction: {:?}", e);
                        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
                    }
                };
                if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                    tracing::error!("Failed to set org context: {:?}", e);
                    return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
                }
                let res = sqlx::query(
                    "INSERT INTO ohc_timecard_event (id, tenant_id, staff_id, event_type, event_time) VALUES ($1, $2, $3, $4, $5::timestamp)",
                )
                .bind(&event.id)
                .bind(&tenant_id)
                .bind(&event.staff_id)
                .bind(&event.event_type)
                .bind(&event.offline_timestamp)
                .execute(&mut *tx)
                .await;
                if let Err(e) = res {
                    tracing::error!("Failed to insert timecard event: {:?}", e);
                    return (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error": "db_error"})),
                    ).into_response();
                }
                if let Err(e) = tx.commit().await {
                    tracing::error!("Failed to commit transaction: {:?}", e);
                    return (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error": "db_error"})),
                    ).into_response();
                }
            }
        }
    }

    (axum::http::StatusCode::OK, Json(SyncTimecardResponse { success: true })).into_response()
}

pub async fn get_timecard_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let events = match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let rows = sqlx::query(
                "SELECT id, staff_id, event_type, CAST(event_time AS TEXT) AS offline_timestamp, CAST(created_at AS TEXT) AS created_at FROM ohc_timecard_event WHERE tenant_id = ? ORDER BY created_at DESC LIMIT 100",
            )
            .bind(&tenant_id)
            .fetch_all(pool)
            .await;
            rows.map(|rows| rows.into_iter().map(|row| {
                use sqlx::Row;
                serde_json::json!({
                    "id": row.get::<String, _>("id"),
                    "staff_id": row.get::<String, _>("staff_id"),
                    "event_type": row.get::<String, _>("event_type"),
                    "offline_timestamp": row.get::<String, _>("offline_timestamp"),
                    "created_at": row.get::<String, _>("created_at"),
                })
            }).collect::<Vec<_>>()).unwrap_or_default()
        }
        crate::db::DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin transaction: {:?}", e);
                    return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
                }
            };
            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                tracing::error!("Failed to set org context: {:?}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
            let rows = sqlx::query(
                "SELECT id, staff_id, event_type, event_time::text AS offline_timestamp, created_at::text AS created_at FROM ohc_timecard_event WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 100",
            )
            .bind(&tenant_id)
            .fetch_all(&mut *tx)
            .await;
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "db_error"})),
                ).into_response();
            }

            rows.map(|rows| rows.into_iter().map(|row| {
                use sqlx::Row;
                serde_json::json!({
                    "id": row.get::<String, _>("id"),
                    "staff_id": row.get::<String, _>("staff_id"),
                    "event_type": row.get::<String, _>("event_type"),
                    "offline_timestamp": row.get::<String, _>("offline_timestamp"),
                    "created_at": row.get::<String, _>("created_at"),
                })
            }).collect::<Vec<_>>()).unwrap_or_default()
        }
    };

    (axum::http::StatusCode::OK, Json(GetTimecardResponse { events })).into_response()
}

pub async fn create_task_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
    Json(payload): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };
    let task_id = format!("task_{}", Uuid::new_v4());

    match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "INSERT INTO staff_tasks (id, tenant_id, staff_id, title, description, priority) VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(&task_id)
            .bind(&tenant_id)
            .bind(&payload.staff_id)
            .bind(&payload.title)
            .bind(&payload.description.clone().unwrap_or_default())
            .bind(&payload.priority.clone().unwrap_or_else(|| "normal".to_string()))
            .execute(pool)
            .await;
            if res.is_err() {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
        }
        crate::db::DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(_) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response(),
            };
            if let Err(_) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
            let res = sqlx::query(
                "INSERT INTO staff_tasks (id, tenant_id, staff_id, title, description, priority) VALUES ($1, $2, $3, $4, $5, $6)",
            )
            .bind(&task_id)
            .bind(&tenant_id)
            .bind(&payload.staff_id)
            .bind(&payload.title)
            .bind(&payload.description.clone().unwrap_or_default())
            .bind(&payload.priority.clone().unwrap_or_else(|| "normal".to_string()))
            .execute(&mut *tx)
            .await;
            if res.is_err() || tx.commit().await.is_err() {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
        }
    }
    (axum::http::StatusCode::OK, Json(TaskResponse { id: task_id })).into_response()
}

pub async fn get_tasks_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let tasks = match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let rows = sqlx::query("SELECT id, staff_id, title, description, status, priority FROM staff_tasks WHERE tenant_id = ? ORDER BY created_at DESC")
                .bind(&tenant_id)
                .fetch_all(pool)
                .await;
            rows.map(|rows| rows.into_iter().map(|row| {
                use sqlx::Row;
                serde_json::json!({
                    "id": row.get::<String, _>("id"),
                    "staff_id": row.get::<String, _>("staff_id"),
                    "title": row.get::<String, _>("title"),
                    "description": row.get::<String, _>("description"),
                    "status": row.get::<String, _>("status"),
                    "priority": row.get::<String, _>("priority"),
                })
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
            let rows = sqlx::query("SELECT id, staff_id, title, description, status, priority FROM staff_tasks WHERE tenant_id = $1 ORDER BY created_at DESC")
                .bind(&tenant_id)
                .fetch_all(&mut *tx)
                .await;
            if tx.commit().await.is_err() {
                 return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
            rows.map(|rows| rows.into_iter().map(|row| {
                use sqlx::Row;
                serde_json::json!({
                    "id": row.get::<String, _>("id"),
                    "staff_id": row.get::<String, _>("staff_id"),
                    "title": row.get::<String, _>("title"),
                    "description": row.get::<String, _>("description"),
                    "status": row.get::<String, _>("status"),
                    "priority": row.get::<String, _>("priority"),
                })
            }).collect::<Vec<_>>()).unwrap_or_default()
        }
    };
    (axum::http::StatusCode::OK, Json(GetTasksResponse { tasks })).into_response()
}

pub async fn update_task_handler(
    headers: HeaderMap,
    axum::extract::Path(task_id): axum::extract::Path<String>,
    State(db): State<Arc<DB>>,
    Json(payload): Json<UpdateTaskRequest>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };
    match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let mut query = String::from("UPDATE staff_tasks SET updated_at = CURRENT_TIMESTAMP");
            if payload.status.is_some() { query.push_str(", status = ?"); }
            if payload.title.is_some() { query.push_str(", title = ?"); }
            query.push_str(" WHERE id = ? AND tenant_id = ?");

            let mut builder = sqlx::query(&query);
            if let Some(s) = &payload.status { builder = builder.bind(s); }
            if let Some(t) = &payload.title { builder = builder.bind(t); }
            builder = builder.bind(&task_id).bind(&tenant_id);

            let res = builder.execute(pool).await;
            if res.is_err() {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
        }
        crate::db::DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(_) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response(),
            };
            if let Err(_) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }

            let mut count = 1;
            let mut query = String::from("UPDATE staff_tasks SET updated_at = CURRENT_TIMESTAMP");
            if payload.status.is_some() { query.push_str(&format!(", status = ${}", count)); count += 1; }
            if payload.title.is_some() { query.push_str(&format!(", title = ${}", count)); count += 1; }
            query.push_str(&format!(" WHERE id = ${} AND tenant_id = ${}", count, count + 1));

            let mut builder = sqlx::query(&query);
            if let Some(s) = &payload.status { builder = builder.bind(s); }
            if let Some(t) = &payload.title { builder = builder.bind(t); }
            builder = builder.bind(&task_id).bind(&tenant_id);

            let res = builder.execute(&mut *tx).await;
            if res.is_err() || tx.commit().await.is_err() {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
        }
    }
    (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}


pub async fn delete_task_handler(
    headers: HeaderMap,
    axum::extract::Path(task_id): axum::extract::Path<String>,
    State(db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };
    match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query("DELETE FROM staff_tasks WHERE id = ? AND tenant_id = ?")
                .bind(&task_id)
                .bind(&tenant_id)
                .execute(pool)
                .await;
            if res.is_err() {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
        }
        crate::db::DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(_) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response(),
            };
            if let Err(_) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
            let res = sqlx::query("DELETE FROM staff_tasks WHERE id = $1 AND tenant_id = $2")
                .bind(&task_id)
                .bind(&tenant_id)
                .execute(&mut *tx)
                .await;
            if res.is_err() || tx.commit().await.is_err() {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
        }
    }
    (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}


pub async fn get_summaries_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let summaries = match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let rows = sqlx::query("SELECT id, summary_text, escalations, CAST(created_at AS TEXT) AS created_at FROM shift_summaries WHERE tenant_id = ? ORDER BY created_at DESC LIMIT 10")
                .bind(&tenant_id)
                .fetch_all(pool)
                .await;
            rows.map(|rows| rows.into_iter().map(|row| {
                use sqlx::Row;
                serde_json::json!({
                    "id": row.get::<String, _>("id"),
                    "summary_text": row.get::<String, _>("summary_text"),
                    "escalations": row.get::<Option<String>, _>("escalations"),
                    "created_at": row.get::<String, _>("created_at"),
                })
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
            let rows = sqlx::query("SELECT id, summary_text, escalations, created_at::text AS created_at FROM shift_summaries WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 10")
                .bind(&tenant_id)
                .fetch_all(&mut *tx)
                .await;
            if tx.commit().await.is_err() {
                 return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
            rows.map(|rows| rows.into_iter().map(|row| {
                use sqlx::Row;
                serde_json::json!({
                    "id": row.get::<String, _>("id"),
                    "summary_text": row.get::<String, _>("summary_text"),
                    "escalations": row.get::<Option<String>, _>("escalations"),
                    "created_at": row.get::<String, _>("created_at"),
                })
            }).collect::<Vec<_>>()).unwrap_or_default()
        }
    };
    (axum::http::StatusCode::OK, Json(GetSummariesResponse { summaries })).into_response()
}


#[derive(Serialize)]
pub struct GetShiftsResponse {
    pub shifts: Vec<serde_json::Value>,
}

pub async fn get_shifts_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let pool = crate::db::get_pool();
    let rows = sqlx::query("SELECT id, start_time, end_time, role, status, staff_id FROM shifts WHERE tenant_id = $1 ORDER BY start_time DESC")
        .bind(&tenant_id)
        .fetch_all(&pool)
        .await;

    let shifts = rows.map(|rows| rows.into_iter().map(|row| {
        use sqlx::Row;
        serde_json::json!({
            "id": row.get::<String, _>("id"),
            "start_time": match row.try_get::<String, _>("start_time") { Ok(s) => crate::db::parse_sqlite_datetime(&s).unwrap_or_else(|_| chrono::Utc::now()), Err(_) => row.try_get("start_time").unwrap_or_else(|_| chrono::Utc::now()) },
            "end_time": match row.try_get::<String, _>("end_time") { Ok(s) => crate::db::parse_sqlite_datetime(&s).unwrap_or_else(|_| chrono::Utc::now()), Err(_) => row.try_get("end_time").unwrap_or_else(|_| chrono::Utc::now()) },
            "role": row.get::<String, _>("role"),
            "status": row.get::<String, _>("status"),
            "staff_id": row.get::<String, _>("staff_id"),
        })
    }).collect::<Vec<_>>()).unwrap_or_default();

    (axum::http::StatusCode::OK, Json(GetShiftsResponse { shifts })).into_response()
}

pub async fn get_escalations_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let pool = crate::db::get_pool();
    let rows = sqlx::query("SELECT id, summary, status FROM escalations WHERE tenant_id = $1 ORDER BY created_at DESC")
        .bind(&tenant_id)
        .fetch_all(&pool)
        .await;

    let escalations = rows.map(|rows| rows.into_iter().map(|row| {
        use sqlx::Row;
        serde_json::json!({
            "id": row.get::<String, _>("id"),
            "summary": row.get::<String, _>("summary"),
            "status": row.get::<String, _>("status"),
        })
    }).collect::<Vec<_>>()).unwrap_or_default();

    (axum::http::StatusCode::OK, Json(serde_json::json!({ "escalations": escalations }))).into_response()
}

pub async fn simulate_event_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };
    let task_id = format!("task_{}", Uuid::new_v4());
    let staff_id = "unassigned";

    let pool = crate::db::get_pool();
    let res = sqlx::query(
        "INSERT INTO staff_tasks (id, tenant_id, staff_id, description, status, priority) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(&task_id)
    .bind(&tenant_id)
    .bind(&staff_id)
    .bind("Simulated Event: Low Inventory")
    .bind("pending")
    .bind("high")
    .execute(&pool)
    .await;

    if res.is_ok() {
        (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
    } else {
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "failed to simulate event"}))).into_response()
    }
}

pub async fn generate_summary_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let summary_id = format!("sum_{}", Uuid::new_v4());

    // Simulate LLM summary generation
    let summary_text = "Shift Summary: Staff completed 3 inventory tasks and handled 2 orders smoothly. No escalations reported. (Simulated AI Summary)";

    let pool = crate::db::get_pool();
    let res = sqlx::query(
        "INSERT INTO shift_summaries (id, tenant_id, shift_date, summary_text) VALUES ($1, $2, CURRENT_DATE, $3)",
    )
    .bind(&summary_id)
    .bind(&tenant_id)
    .bind(&summary_text)
    .execute(&pool)
    .await;

    if res.is_ok() {
        (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
    } else {
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "failed to generate summary"}))).into_response()
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    Router::new()
        .route("/", post(create_staff_handler).get(get_staff_handler))
        .route("/{id}/pin", post(set_staff_pin_handler))
        .route("/timecard", post(sync_timecard_handler).get(get_timecard_handler))
        .route("/tasks", post(create_task_handler).get(get_tasks_handler))
        .route("/tasks/{id}", post(update_task_handler).delete(delete_task_handler))
        .route("/summaries", axum::routing::get(get_summaries_handler))
        .route("/shifts", axum::routing::get(get_shifts_handler))
        .route("/escalations", axum::routing::get(get_escalations_handler))
        .route("/simulate-event", axum::routing::post(simulate_event_handler))
        .route("/generate-summary", axum::routing::post(generate_summary_handler))
        .with_state(db)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use crate::db::{DB, DbStore};

    #[tokio::test]
    async fn test_staff_mesh_flow() {
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let db = DB {
            pool: crate::db::secure_pg_pool_options().acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(sqlite_pool.clone()),
        };

        // Setup schema
        sqlx::query(
            "CREATE TABLE ohc_staff_member (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                phone_number TEXT NOT NULL,
                role TEXT NOT NULL,
                pin_hash TEXT,
                status TEXT NOT NULL DEFAULT 'ACTIVE',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                _sync_status TEXT DEFAULT 'pending',
                version INTEGER DEFAULT 1
            );"
        ).execute(&sqlite_pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE ohc_timecard_event (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                staff_id TEXT NOT NULL,
                event_type TEXT NOT NULL,
                event_time TIMESTAMP NOT NULL,
                synced_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                _sync_status TEXT DEFAULT 'pending',
                version INTEGER DEFAULT 1
            );"
        ).execute(&sqlite_pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE staff_tasks (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                staff_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'pending',
                priority TEXT NOT NULL DEFAULT 'normal',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );"
        ).execute(&sqlite_pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE shift_summaries (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                shift_id TEXT,
                summary_text TEXT NOT NULL,
                escalations TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );"
        ).execute(&sqlite_pool).await.unwrap();

        let db_arc = Arc::new(db);

        let app = axum::Router::new()
            .route("/staff", axum::routing::post(create_staff_handler).get(get_staff_handler))
            .route("/staff/{id}/pin", axum::routing::post(set_staff_pin_handler))
            .route("/timecard", axum::routing::post(sync_timecard_handler))
            .with_state(db_arc);

        // 1. Create Staff
        let create_payload = serde_json::json!({
            "name": "Sarah Smith",
            "phone_number": "555-0199",
            "role": "Cashier"
        });

        let request = Request::builder()
            .method("POST")
            .uri("/staff")
            .header("content-type", "application/json")
            .header("x-spiffe-id", "spiffe://ohc/org/test_tenant/agent/test_agent")
            .body(Body::from(create_payload.to_string()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let staff_id = body_json.get("id").unwrap().as_str().unwrap().to_string();

        // 2. Set PIN
        let pin_payload = serde_json::json!({
            "pin": "1234"
        });

        let request = Request::builder()
            .method("POST")
            .uri(format!("/staff/{}/pin", staff_id))
            .header("content-type", "application/json")
            .header("x-spiffe-id", "spiffe://ohc/org/test_tenant/agent/test_agent")
            .body(Body::from(pin_payload.to_string()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // 3. Get Staff
        let request = Request::builder()
            .method("GET")
            .uri("/staff")
            .header("x-spiffe-id", "spiffe://ohc/org/test_tenant/agent/test_agent")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        let staff_array = body_json.get("staff").unwrap().as_array().unwrap();
        assert_eq!(staff_array.len(), 1);
        assert_eq!(staff_array[0].get("name").unwrap().as_str().unwrap(), "Sarah Smith");

        // 4. Sync Timecard
        let timecard_payload = serde_json::json!({
            "events": [{
                "id": "evt_123",
                "staff_id": staff_id,
                "event_type": "CLOCK_IN",
                "offline_timestamp": "2024-01-01T12:00:00Z"
            }]
        });

        let request = Request::builder()
            .method("POST")
            .uri("/timecard")
            .header("content-type", "application/json")
            .header("x-spiffe-id", "spiffe://ohc/org/test_tenant/agent/test_agent")
            .body(Body::from(timecard_payload.to_string()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}

#[derive(Deserialize)]
pub struct StaffEscalationRequest {
    pub alert_id: Option<String>,
    pub draft: String,
}

#[derive(Serialize)]
pub struct StaffEscalationResponse {
    pub success: bool,
}

pub async fn escalate_issue_handler(
    headers: HeaderMap,
    State(_db): State<Arc<DB>>,
    Json(payload): Json<StaffEscalationRequest>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let triage_id = uuid::Uuid::new_v4().to_string();

    let context_json = serde_json::json!({
        "message": payload.draft,
        "alert_id": payload.alert_id,
        "source": "Staff Escalation"
    }).to_string();

    let pool = crate::db::get_pool();
    if let Err(e) = sqlx::query(
        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, 'staff_escalation', $3, $4, 'PENDING_APPROVAL', NOW(), NOW())"
    )
    .bind(&triage_id)
    .bind(&tenant_id)
    .bind(&context_json)
    .bind(serde_json::json!({ "action_type": "Review Escalation" }).to_string())
    .execute(&pool)
    .await {
        tracing::error!("Failed to insert triage item for staff escalation: {}", e);
        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "db_error"})),
        ).into_response();
    }

    (axum::http::StatusCode::OK, Json(StaffEscalationResponse { success: true })).into_response()
}

pub async fn get_staff_tasks_handler(
    headers: HeaderMap,
    axum::extract::Query(query): axum::extract::Query<crate::common::auth_utils::UiTenantQuery>,
    State(_db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    let pool = crate::db::get_pool();
    let rows = sqlx::query(
        "SELECT id, tenant_id, staff_id, description, status, priority, created_at, updated_at FROM staff_tasks WHERE tenant_id = $1 ORDER BY created_at DESC"
    )
    .bind(&tenant_id)
    .fetch_all(&pool)
    .await;

    let tasks = rows.map(|rows| rows.into_iter().map(|row| {
        use sqlx::Row;
        if mobile_optimized {
            serde_json::json!({
                "id": row.get::<String, _>("id"),
                "tenant_id": row.get::<String, _>("tenant_id"),
                "staff_id": row.get::<String, _>("staff_id"),
                "status": row.get::<String, _>("status"),
                "priority": row.get::<String, _>("priority"),
                "created_at": match row.try_get::<String, _>("created_at") { Ok(s) => crate::db::parse_sqlite_datetime(&s).unwrap_or_else(|_| chrono::Utc::now()), Err(_) => row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()) },
            })
        } else {
            serde_json::json!({
                "id": row.get::<String, _>("id"),
                "tenant_id": row.get::<String, _>("tenant_id"),
                "staff_id": row.get::<String, _>("staff_id"),
                "description": row.get::<String, _>("description"),
                "status": row.get::<String, _>("status"),
                "priority": row.get::<String, _>("priority"),
                "created_at": match row.try_get::<String, _>("created_at") { Ok(s) => crate::db::parse_sqlite_datetime(&s).unwrap_or_else(|_| chrono::Utc::now()), Err(_) => row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()) },
                "updated_at": match row.try_get::<String, _>("updated_at") { Ok(s) => crate::db::parse_sqlite_datetime(&s).unwrap_or_else(|_| chrono::Utc::now()), Err(_) => row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()) },
            })
        }
    }).collect::<Vec<_>>()).unwrap_or_default();

    (axum::http::StatusCode::OK, Json(serde_json::json!({ "tasks": tasks }))).into_response()
}

pub async fn get_shift_summaries_handler(
    headers: HeaderMap,
    axum::extract::Query(query): axum::extract::Query<crate::common::auth_utils::UiTenantQuery>,
    State(_db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    let pool = crate::db::get_pool();
    let rows = sqlx::query(
        "SELECT id, tenant_id, shift_date, summary_text, metrics, created_at, updated_at FROM shift_summaries WHERE tenant_id = $1 ORDER BY shift_date DESC LIMIT 30"
    )
    .bind(&tenant_id)
    .fetch_all(&pool)
    .await;

    let summaries = rows.map(|rows| rows.into_iter().map(|row| {
        use sqlx::Row;
        if mobile_optimized {
            serde_json::json!({
                "id": row.get::<String, _>("id"),
                "tenant_id": row.get::<String, _>("tenant_id"),
                "shift_date": row.get::<chrono::NaiveDate, _>("shift_date"),
                "summary_text": row.get::<String, _>("summary_text"),
                "created_at": match row.try_get::<String, _>("created_at") { Ok(s) => crate::db::parse_sqlite_datetime(&s).unwrap_or_else(|_| chrono::Utc::now()), Err(_) => row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()) },
                "updated_at": match row.try_get::<String, _>("updated_at") { Ok(s) => crate::db::parse_sqlite_datetime(&s).unwrap_or_else(|_| chrono::Utc::now()), Err(_) => row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()) },
            })
        } else {
            serde_json::json!({
                "id": row.get::<String, _>("id"),
                "tenant_id": row.get::<String, _>("tenant_id"),
                "shift_date": row.get::<chrono::NaiveDate, _>("shift_date"),
                "summary_text": row.get::<String, _>("summary_text"),
                "metrics": row.get::<Option<sqlx::types::Json<serde_json::Value>>, _>("metrics"),
                "created_at": match row.try_get::<String, _>("created_at") { Ok(s) => crate::db::parse_sqlite_datetime(&s).unwrap_or_else(|_| chrono::Utc::now()), Err(_) => row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()) },
                "updated_at": match row.try_get::<String, _>("updated_at") { Ok(s) => crate::db::parse_sqlite_datetime(&s).unwrap_or_else(|_| chrono::Utc::now()), Err(_) => row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()) },
            })
        }
    }).collect::<Vec<_>>()).unwrap_or_default();

    (axum::http::StatusCode::OK, Json(serde_json::json!({ "summaries": summaries }))).into_response()
}
