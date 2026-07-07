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
    State(db): State<Arc<DB>>,
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
    State(_db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let pool = crate::db::get_pool();
    let rows = sqlx::query(
        "SELECT id, tenant_id, staff_id, description, status, priority, created_at, updated_at FROM staff_tasks WHERE tenant_id = $1 ORDER BY created_at DESC"
    )
    .bind(&tenant_id)
    .fetch_all(&pool)
    .await;

    let tasks = rows.map(|rows| rows.into_iter().map(|row| {
        use sqlx::Row;
        serde_json::json!({
            "id": row.get::<String, _>("id"),
            "tenant_id": row.get::<String, _>("tenant_id"),
            "staff_id": row.get::<String, _>("staff_id"),
            "description": row.get::<String, _>("description"),
            "status": row.get::<String, _>("status"),
            "priority": row.get::<String, _>("priority"),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at"),
            "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at"),
        })
    }).collect::<Vec<_>>()).unwrap_or_default();

    (axum::http::StatusCode::OK, Json(serde_json::json!({ "tasks": tasks }))).into_response()
}

pub async fn get_shift_summaries_handler(
    headers: HeaderMap,
    State(_db): State<Arc<DB>>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&headers) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let pool = crate::db::get_pool();
    let rows = sqlx::query(
        "SELECT id, tenant_id, shift_date, summary_text, metrics, created_at, updated_at FROM shift_summaries WHERE tenant_id = $1 ORDER BY shift_date DESC LIMIT 30"
    )
    .bind(&tenant_id)
    .fetch_all(&pool)
    .await;

    let summaries = rows.map(|rows| rows.into_iter().map(|row| {
        use sqlx::Row;
        serde_json::json!({
            "id": row.get::<String, _>("id"),
            "tenant_id": row.get::<String, _>("tenant_id"),
            "shift_date": row.get::<chrono::NaiveDate, _>("shift_date"),
            "summary_text": row.get::<String, _>("summary_text"),
            "metrics": row.get::<Option<sqlx::types::Json<serde_json::Value>>, _>("metrics"),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at"),
            "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at"),
        })
    }).collect::<Vec<_>>()).unwrap_or_default();

    (axum::http::StatusCode::OK, Json(serde_json::json!({ "summaries": summaries }))).into_response()
}
