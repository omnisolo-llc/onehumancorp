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
pub struct CreateStaffTaskRequest {
    pub description: String,
    pub priority: Option<String>,
    pub staff_id: Option<String>,
}

#[derive(Serialize)]
pub struct CreateStaffTaskResponse {
    pub id: String,
}

#[derive(Serialize)]
pub struct StaffTask {
    pub id: String,
    pub staff_id: Option<String>,
    pub description: String,
    pub priority: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct GetTasksResponse {
    pub tasks: Vec<StaffTask>,
}

#[derive(Deserialize)]
pub struct MarkTaskRequest {
    pub status: String,
    pub offline_timestamp: Option<String>,
}

#[derive(Deserialize)]
pub struct ReportEscalationRequest {
    pub escalation_text: String,
    pub staff_id: Option<String>,
}

#[derive(Serialize)]
pub struct ReportEscalationResponse {
    pub id: String,
}

#[derive(Serialize)]
pub struct ShiftSummary {
    pub id: String,
    pub shift_date: String,
    pub summary_text: String,
    pub escalations: Option<String>,
    pub supply_needs: Option<String>,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct GetShiftSummariesResponse {
    pub summaries: Vec<ShiftSummary>,
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
    Json(payload): Json<CreateStaffTaskRequest>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };
    let task_id = format!("stask_{}", Uuid::new_v4());
    let priority = payload.priority.unwrap_or_else(|| "normal".to_string());

    match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "INSERT INTO staff_tasks (id, tenant_id, description, priority, staff_id) VALUES (?, ?, ?, ?, ?)",
            )
            .bind(&task_id)
            .bind(&tenant_id)
            .bind(&payload.description)
            .bind(&priority)
            .bind(&payload.staff_id)
            .execute(pool)
            .await;
            if let Err(e) = res {
                tracing::error!("Failed to create staff task: {:?}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
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
                "INSERT INTO staff_tasks (id, tenant_id, description, priority, staff_id) VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(&task_id)
            .bind(&tenant_id)
            .bind(&payload.description)
            .bind(&priority)
            .bind(&payload.staff_id)
            .execute(&mut *tx)
            .await;
            if let Err(e) = res {
                tracing::error!("Failed to create staff task: {:?}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
        }
    }

    (axum::http::StatusCode::OK, Json(CreateStaffTaskResponse { id: task_id })).into_response()
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
            let rows = sqlx::query(
                "SELECT id, staff_id, description, priority, status, CAST(created_at AS TEXT) AS created_at FROM staff_tasks WHERE tenant_id = ? ORDER BY created_at DESC LIMIT 100",
            )
            .bind(&tenant_id)
            .fetch_all(pool)
            .await;
            rows.map(|rows| rows.into_iter().map(|row| {
                use sqlx::Row;
                StaffTask {
                    id: row.get("id"),
                    staff_id: row.get("staff_id"),
                    description: row.get("description"),
                    priority: row.get("priority"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                }
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
                "SELECT id, staff_id, description, priority, status, created_at::text AS created_at FROM staff_tasks WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 100",
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
                StaffTask {
                    id: row.get("id"),
                    staff_id: row.get("staff_id"),
                    description: row.get("description"),
                    priority: row.get("priority"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                }
            }).collect::<Vec<_>>()).unwrap_or_default()
        }
    };

    (axum::http::StatusCode::OK, Json(GetTasksResponse { tasks })).into_response()
}

pub async fn mark_task_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
    Path(task_id): Path<String>,
    Json(payload): Json<MarkTaskRequest>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "UPDATE staff_tasks SET status = ?, offline_timestamp = COALESCE(?, offline_timestamp), updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?",
            )
            .bind(&payload.status)
            .bind(&payload.offline_timestamp)
            .bind(&task_id)
            .bind(&tenant_id)
            .execute(pool)
            .await;
            if let Err(e) = res {
                tracing::error!("Failed to update staff task: {:?}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
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
                "UPDATE staff_tasks SET status = $1, offline_timestamp = COALESCE($2::timestamptz, offline_timestamp), updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND tenant_id = $4",
            )
            .bind(&payload.status)
            .bind(&payload.offline_timestamp)
            .bind(&task_id)
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await;
            if let Err(e) = res {
                tracing::error!("Failed to update staff task: {:?}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
        }
    }

    (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}

pub async fn report_escalation_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
    Json(payload): Json<ReportEscalationRequest>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };
    let escalation_id = format!("escalation_{}", Uuid::new_v4());

    match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "INSERT INTO ohc_location_escalation (id, tenant_id, escalation_text, staff_id, status) VALUES (?, ?, ?, ?, 'pending')",
            )
            .bind(&escalation_id)
            .bind(&tenant_id)
            .bind(&payload.escalation_text)
            .bind(&payload.staff_id)
            .execute(pool)
            .await;
            if let Err(e) = res {
                tracing::error!("Failed to create escalation: {:?}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
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
                "INSERT INTO ohc_location_escalation (id, tenant_id, escalation_text, staff_id, status) VALUES ($1, $2, $3, $4, 'pending')",
            )
            .bind(&escalation_id)
            .bind(&tenant_id)
            .bind(&payload.escalation_text)
            .bind(&payload.staff_id)
            .execute(&mut *tx)
            .await;
            if let Err(e) = res {
                tracing::error!("Failed to create escalation: {:?}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db_error"}))).into_response();
            }
        }
    }

    (axum::http::StatusCode::OK, Json(ReportEscalationResponse { id: escalation_id })).into_response()
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
                "SELECT id, CAST(shift_date AS TEXT) AS shift_date, summary_text, escalations, supply_needs, CAST(created_at AS TEXT) AS created_at FROM shift_summaries WHERE tenant_id = ? ORDER BY created_at DESC LIMIT 50",
            )
            .bind(&tenant_id)
            .fetch_all(pool)
            .await;
            rows.map(|rows| rows.into_iter().map(|row| {
                use sqlx::Row;
                ShiftSummary {
                    id: row.get("id"),
                    shift_date: row.get("shift_date"),
                    summary_text: row.get("summary_text"),
                    escalations: row.get("escalations"),
                    supply_needs: row.get("supply_needs"),
                    created_at: row.get("created_at"),
                }
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
                "SELECT id, shift_date::text AS shift_date, summary_text, escalations, supply_needs, created_at::text AS created_at FROM shift_summaries WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 50",
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
                ShiftSummary {
                    id: row.get("id"),
                    shift_date: row.get("shift_date"),
                    summary_text: row.get("summary_text"),
                    escalations: row.get("escalations"),
                    supply_needs: row.get("supply_needs"),
                    created_at: row.get("created_at"),
                }
            }).collect::<Vec<_>>()).unwrap_or_default()
        }
    };

    (axum::http::StatusCode::OK, Json(GetShiftSummariesResponse { summaries })).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {

    Router::new()
        .route("/", post(create_staff_handler).get(get_staff_handler))
        .route("/{id}/pin", post(set_staff_pin_handler))
        .route("/timecard", post(sync_timecard_handler).get(get_timecard_handler))
        .route("/tasks", post(create_task_handler).get(get_tasks_handler))
        .route("/tasks/{id}", post(mark_task_handler))
        .route("/escalations", post(report_escalation_handler))
        .route("/summaries", axum::routing::get(get_shift_summaries_handler))
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
            "CREATE TABLE staff_tasks (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                staff_id TEXT,
                description TEXT NOT NULL,
                priority TEXT NOT NULL DEFAULT 'normal',
                status TEXT NOT NULL DEFAULT 'pending',
                created_by TEXT NOT NULL DEFAULT 'system',
                offline_timestamp TIMESTAMP,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );"
        ).execute(&sqlite_pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE shift_summaries (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                shift_date DATE NOT NULL,
                summary_text TEXT NOT NULL,
                escalations TEXT,
                supply_needs TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );"
        ).execute(&sqlite_pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE ohc_location_escalation (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                staff_id TEXT,
                escalation_text TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );"
        ).execute(&sqlite_pool).await.unwrap();

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
            .route("/tasks", axum::routing::post(create_task_handler).get(get_tasks_handler))
            .route("/tasks/{id}", axum::routing::post(mark_task_handler))
            .route("/escalations", axum::routing::post(report_escalation_handler))
            .route("/summaries", axum::routing::get(get_shift_summaries_handler))
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

        // 5. Create and Complete Task
        let create_task_payload = serde_json::json!({
            "description": "Clean front counter",
            "priority": "high",
            "staff_id": staff_id
        });

        let request = Request::builder()
            .method("POST")
            .uri("/tasks")
            .header("content-type", "application/json")
            .header("x-spiffe-id", "spiffe://ohc/org/test_tenant/agent/test_agent")
            .body(Body::from(create_task_payload.to_string()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let task_id = body_json.get("id").unwrap().as_str().unwrap().to_string();

        let mark_task_payload = serde_json::json!({
            "status": "completed",
            "offline_timestamp": "2024-01-01T12:05:00Z"
        });

        let request = Request::builder()
            .method("POST")
            .uri(format!("/tasks/{}", task_id))
            .header("content-type", "application/json")
            .header("x-spiffe-id", "spiffe://ohc/org/test_tenant/agent/test_agent")
            .body(Body::from(mark_task_payload.to_string()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // 6. Get Tasks
        let request = Request::builder()
            .method("GET")
            .uri("/tasks")
            .header("x-spiffe-id", "spiffe://ohc/org/test_tenant/agent/test_agent")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        let tasks_array = body_json.get("tasks").unwrap().as_array().unwrap();
        assert_eq!(tasks_array.len(), 1);
        assert_eq!(tasks_array[0].get("description").unwrap().as_str().unwrap(), "Clean front counter");
        assert_eq!(tasks_array[0].get("status").unwrap().as_str().unwrap(), "completed");

        // 7. Test Summaries (empty list initially)
        let request = Request::builder()
            .method("GET")
            .uri("/summaries")
            .header("x-spiffe-id", "spiffe://ohc/org/test_tenant/agent/test_agent")
            .body(Body::empty())
            .unwrap();
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
