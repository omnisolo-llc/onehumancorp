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

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    Router::new()
        .route("/", post(create_staff_handler).get(get_staff_handler))
        .route("/{id}/pin", post(set_staff_pin_handler))
        .route("/timecard", post(sync_timecard_handler).get(get_timecard_handler))
        .route("/tasks", axum::routing::post(create_staff_task_handler).get(get_staff_tasks_handler))
        .route("/tasks/sync", axum::routing::post(sync_staff_tasks_handler))
        .route("/summaries", axum::routing::get(get_shift_summaries_handler))
        .route("/summaries/generate", axum::routing::post(generate_shift_summary_handler))
        .with_state(db)
}


#[derive(Deserialize, Debug)]
pub struct CreateStaffTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub staff_id: Option<String>,
    pub shift_id: Option<String>,
}

#[derive(Serialize)]
pub struct StaffTaskResponse {
    pub success: bool,
    pub id: String,
}

pub async fn create_staff_task_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
    Json(payload): Json<CreateStaffTaskRequest>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };
    let task_id = format!("task_{}", Uuid::new_v4());

    let priority = payload.priority.unwrap_or_else(|| "medium".to_string());

    match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "INSERT INTO ohc_staff_tasks (id, tenant_id, staff_id, shift_id, title, description, priority, status) VALUES (?, ?, ?, ?, ?, ?, ?, 'pending')",
            )
            .bind(&task_id)
            .bind(&tenant_id)
            .bind(&payload.staff_id)
            .bind(&payload.shift_id)
            .bind(&payload.title)
            .bind(&payload.description)
            .bind(&priority)
            .execute(pool)
            .await;
            if let Err(e) = res {
                tracing::error!("Failed to insert staff task: {:?}", e);
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
                "INSERT INTO ohc_staff_tasks (id, tenant_id, staff_id, shift_id, title, description, priority, status) VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending')",
            )
            .bind(&task_id)
            .bind(&tenant_id)
            .bind(&payload.staff_id)
            .bind(&payload.shift_id)
            .bind(&payload.title)
            .bind(&payload.description)
            .bind(&priority)
            .execute(&mut *tx)
            .await;
            if let Err(e) = res {
                tracing::error!("Failed to insert staff task: {:?}", e);
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

    (axum::http::StatusCode::OK, Json(StaffTaskResponse { success: true, id: task_id })).into_response()
}

pub async fn get_staff_tasks_handler(
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
                "SELECT id, title, description, priority, status, staff_id, shift_id, escalated_to, CAST(created_at AS TEXT) AS created_at FROM ohc_staff_tasks WHERE tenant_id = ? AND status != 'completed' ORDER BY created_at DESC",
            )
            .bind(&tenant_id)
            .fetch_all(pool)
            .await;
            rows.map(|rows| rows.into_iter().map(|row| {
                use sqlx::Row;
                serde_json::json!({
                    "id": row.get::<String, _>("id"),
                    "title": row.get::<String, _>("title"),
                    "description": row.get::<Option<String>, _>("description"),
                    "priority": row.get::<String, _>("priority"),
                    "status": row.get::<String, _>("status"),
                    "staff_id": row.get::<Option<String>, _>("staff_id"),
                    "shift_id": row.get::<Option<String>, _>("shift_id"),
                    "escalated_to": row.get::<Option<String>, _>("escalated_to"),
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
                "SELECT id, title, description, priority, status, staff_id, shift_id, escalated_to, created_at::text AS created_at FROM ohc_staff_tasks WHERE tenant_id = $1 AND status != 'completed' ORDER BY created_at DESC",
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
                    "title": row.get::<String, _>("title"),
                    "description": row.get::<Option<String>, _>("description"),
                    "priority": row.get::<String, _>("priority"),
                    "status": row.get::<String, _>("status"),
                    "staff_id": row.get::<Option<String>, _>("staff_id"),
                    "shift_id": row.get::<Option<String>, _>("shift_id"),
                    "escalated_to": row.get::<Option<String>, _>("escalated_to"),
                    "created_at": row.get::<String, _>("created_at"),
                })
            }).collect::<Vec<_>>()).unwrap_or_default()
        }
    };

    (axum::http::StatusCode::OK, Json(serde_json::json!({ "tasks": tasks }))).into_response()
}

#[derive(Deserialize, Debug)]
pub struct SyncStaffTasksRequest {
    pub mutations: Vec<serde_json::Value>,
}

pub async fn sync_staff_tasks_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
    Json(payload): Json<SyncStaffTasksRequest>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    for mutation in payload.mutations {
        let task_id = mutation.get("id").and_then(|v| v.as_str()).unwrap_or("");
        let action = mutation.get("action").and_then(|v| v.as_str()).unwrap_or("");

        if task_id.is_empty() || action.is_empty() {
            continue;
        }

        match &db.store {
            crate::db::DbStore::Sqlite(pool) => {
                if action == "complete" {
                    let _ = sqlx::query("UPDATE ohc_staff_tasks SET status = 'completed', completed_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
                        .bind(task_id).bind(&tenant_id).execute(pool).await;
                } else if action == "escalate" {
                    let _ = sqlx::query("UPDATE ohc_staff_tasks SET status = 'escalated' WHERE id = ? AND tenant_id = ?")
                        .bind(task_id).bind(&tenant_id).execute(pool).await;
                } else if action == "create_alert" { // low supply
                     let title = mutation.get("title").and_then(|v| v.as_str()).unwrap_or("Alert");
                     let new_task_id = format!("task_{}", Uuid::new_v4());
                     let _ = sqlx::query("INSERT INTO ohc_staff_tasks (id, tenant_id, title, priority, status) VALUES (?, ?, ?, 'urgent', 'escalated')")
                        .bind(&new_task_id).bind(&tenant_id).bind(title).execute(pool).await;
                }
            }
            crate::db::DbStore::Postgres => {
                let mut tx = match db.pool.begin().await {
                    Ok(tx) => tx,
                    Err(_) => continue,
                };
                if let Err(_) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                    continue;
                }
                if action == "complete" {
                    let _ = sqlx::query("UPDATE ohc_staff_tasks SET status = 'completed', completed_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
                        .bind(task_id).bind(&tenant_id).execute(&mut *tx).await;
                } else if action == "escalate" {
                    let _ = sqlx::query("UPDATE ohc_staff_tasks SET status = 'escalated' WHERE id = $1 AND tenant_id = $2")
                        .bind(task_id).bind(&tenant_id).execute(&mut *tx).await;
                } else if action == "create_alert" {
                     let title = mutation.get("title").and_then(|v| v.as_str()).unwrap_or("Alert");
                     let new_task_id = format!("task_{}", Uuid::new_v4());
                     let _ = sqlx::query("INSERT INTO ohc_staff_tasks (id, tenant_id, title, priority, status) VALUES ($1, $2, $3, 'urgent', 'escalated')")
                        .bind(&new_task_id).bind(&tenant_id).bind(title).execute(&mut *tx).await;
                }
                let _ = tx.commit().await;
            }
        }
    }

    (axum::http::StatusCode::OK, Json(serde_json::json!({ "success": true }))).into_response()
}

pub async fn get_shift_summaries_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let summaries = match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let rows = sqlx::query(
                "SELECT id, shift_id, summary_text, issues_escalated, tasks_completed, CAST(created_at AS TEXT) AS created_at FROM ohc_shift_summaries WHERE tenant_id = ? ORDER BY created_at DESC LIMIT 10",
            )
            .bind(&tenant_id)
            .fetch_all(pool)
            .await;
            rows.map(|rows| rows.into_iter().map(|row| {
                use sqlx::Row;
                serde_json::json!({
                    "id": row.get::<String, _>("id"),
                    "shift_id": row.get::<String, _>("shift_id"),
                    "summary_text": row.get::<String, _>("summary_text"),
                    "issues_escalated": row.get::<i32, _>("issues_escalated"),
                    "tasks_completed": row.get::<i32, _>("tasks_completed"),
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
                "SELECT id, shift_id, summary_text, issues_escalated, tasks_completed, created_at::text AS created_at FROM ohc_shift_summaries WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 10",
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
                    "shift_id": row.get::<String, _>("shift_id"),
                    "summary_text": row.get::<String, _>("summary_text"),
                    "issues_escalated": row.get::<i32, _>("issues_escalated"),
                    "tasks_completed": row.get::<i32, _>("tasks_completed"),
                    "created_at": row.get::<String, _>("created_at"),
                })
            }).collect::<Vec<_>>()).unwrap_or_default()
        }
    };

    (axum::http::StatusCode::OK, Json(serde_json::json!({ "summaries": summaries }))).into_response()
}

#[derive(Deserialize, Debug)]
pub struct GenerateShiftSummaryRequest {
    pub shift_id: String,
}

pub async fn generate_shift_summary_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
    Json(payload): Json<GenerateShiftSummaryRequest>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let summary_id = format!("summary_{}", Uuid::new_v4());

    // Generate a basic summary string internally (simulating the Business Advisory agent for now)
    let summary_text = "Shift completed smoothly. Checked bathrooms and restocked front shelves.";

    match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let _ = sqlx::query(
                "INSERT INTO ohc_shift_summaries (id, tenant_id, shift_id, summary_text, issues_escalated, tasks_completed) VALUES (?, ?, ?, ?, 0, 1)",
            )
            .bind(&summary_id)
            .bind(&tenant_id)
            .bind(&payload.shift_id)
            .bind(summary_text)
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
            if let Err(_) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
            let _ = sqlx::query(
                "INSERT INTO ohc_shift_summaries (id, tenant_id, shift_id, summary_text, issues_escalated, tasks_completed) VALUES ($1, $2, $3, $4, 0, 1)",
            )
            .bind(&summary_id)
            .bind(&tenant_id)
            .bind(&payload.shift_id)
            .bind(summary_text)
            .execute(&mut *tx)
            .await;
            let _ = tx.commit().await;
        }
    }

    (axum::http::StatusCode::OK, Json(serde_json::json!({ "success": true, "id": summary_id }))).into_response()
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
