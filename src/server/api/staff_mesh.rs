use axum::{
    extract::{Path, State},
    response::IntoResponse,
    http::HeaderMap,
    Json,
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
    pub status: String,
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

fn get_tenant_id(headers: &HeaderMap) -> String {
    headers
        .get("x-spiffe-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|val| ::server_auth::parse_spiffe_id(val).ok())
        .map(|(t, _)| t)
        .unwrap_or_else(|| "default".to_string())
}

pub async fn create_staff_handler(
    headers: HeaderMap,
    State(db): State<Arc<DB>>,
    Json(payload): Json<CreateStaffRequest>,
) -> impl IntoResponse {
    let tenant_id = get_tenant_id(&headers);
    let staff_id = format!("staff_{}", Uuid::new_v4());

    // In a real implementation, we'd create a token in a store. Here we just use a dummy token pattern for demonstration.
    let invite_token = format!("invite_{}", Uuid::new_v4());

    match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "INSERT INTO staff_members (id, tenant_id, name, phone_number, role) VALUES (?, ?, ?, ?, ?)",
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
             let res = sqlx::query(
                "INSERT INTO staff_members (id, tenant_id, name, phone_number, role) VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(&staff_id)
            .bind(&tenant_id)
            .bind(&payload.name)
            .bind(&payload.phone_number)
            .bind(&payload.role)
            .execute(&db.pool)
            .await;
             if res.is_err() {
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
    let tenant_id = get_tenant_id(&headers);

    // In a real app, hash the pin here (e.g. using bcrypt)
    let pin_hash = format!("hashed_{}", payload.pin);

    match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "UPDATE staff_members SET pin_hash = ? WHERE id = ? AND tenant_id = ?",
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
            let res = sqlx::query(
                "UPDATE staff_members SET pin_hash = $1 WHERE id = $2 AND tenant_id = $3",
            )
            .bind(&pin_hash)
            .bind(&id)
            .bind(&tenant_id)
            .execute(&db.pool)
            .await;
            if res.is_err() {
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
    let tenant_id = get_tenant_id(&headers);

    let staff: Vec<StaffMember> = match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let rows: Result<Vec<(String, String, String, String, String)>, _> = sqlx::query_as(
                "SELECT id, name, phone_number, role, status FROM staff_members WHERE tenant_id = ?",
            )
            .bind(&tenant_id)
            .fetch_all(pool)
            .await;

            rows.unwrap_or_default().into_iter().map(|(id, name, phone_number, role, status)| {
                StaffMember { id, name, phone_number, role, status }
            }).collect()
        }
        crate::db::DbStore::Postgres => {
            let rows: Result<Vec<(String, String, String, String, String)>, _> = sqlx::query_as(
                "SELECT id, name, phone_number, role, status FROM staff_members WHERE tenant_id = $1",
            )
            .bind(&tenant_id)
            .fetch_all(&db.pool)
            .await;

            rows.unwrap_or_default().into_iter().map(|(id, name, phone_number, role, status)| {
                StaffMember { id, name, phone_number, role, status }
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
    let tenant_id = get_tenant_id(&headers);

    for event in payload.events {
        match &db.store {
            crate::db::DbStore::Sqlite(pool) => {
                let _ = sqlx::query(
                    "INSERT INTO timecard_events (id, tenant_id, staff_id, event_type, offline_timestamp) VALUES (?, ?, ?, ?, ?)",
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
                let _ = sqlx::query(
                    "INSERT INTO timecard_events (id, tenant_id, staff_id, event_type, offline_timestamp) VALUES ($1, $2, $3, $4, $5::timestamp)",
                )
                .bind(&event.id)
                .bind(&tenant_id)
                .bind(&event.staff_id)
                .bind(&event.event_type)
                .bind(&event.offline_timestamp)
                .execute(&db.pool)
                .await;
            }
        }
    }

    (axum::http::StatusCode::OK, Json(SyncTimecardResponse { success: true })).into_response()
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
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(sqlite_pool.clone()),
        };

        // Setup schema
        sqlx::query(
            "CREATE TABLE staff_members (
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
            "CREATE TABLE timecard_events (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                staff_id TEXT NOT NULL,
                event_type TEXT NOT NULL,
                offline_timestamp TIMESTAMP NOT NULL,
                synced_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                _sync_status TEXT DEFAULT 'pending',
                version INTEGER DEFAULT 1
            );"
        ).execute(&sqlite_pool).await.unwrap();

        let db_arc = Arc::new(db);

        let app = axum::Router::new()
            .route("/staff", axum::routing::post(create_staff_handler).get(get_staff_handler))
            .route("/staff/:id/pin", axum::routing::post(set_staff_pin_handler))
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
            .header("x-spiffe-id", "spiffe://onehumancorp.io/test_org/test_tenant")
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
            .header("x-spiffe-id", "spiffe://onehumancorp.io/test_org/test_tenant")
            .body(Body::from(pin_payload.to_string()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // 3. Get Staff
        let request = Request::builder()
            .method("GET")
            .uri("/staff")
            .header("x-spiffe-id", "spiffe://onehumancorp.io/test_org/test_tenant")
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
            .header("x-spiffe-id", "spiffe://onehumancorp.io/test_org/test_tenant")
            .body(Body::from(timecard_payload.to_string()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
