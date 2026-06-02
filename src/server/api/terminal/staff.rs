use axum::{
    extract::{Extension, Path, Json},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use axum::http::StatusCode;
use crate::hub::Hub;
use uuid::Uuid;

#[derive(Serialize)]
pub struct StaffMember {
    pub id: String,
    pub name: String,
    pub phone_number: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct CreateStaffRequest {
    pub name: String,
    pub phone_number: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct CreateStaffResponse {
    pub success: bool,
    pub staff_id: String,
    pub magic_link: String,
}

#[derive(Deserialize)]
pub struct SetPinRequest {
    pub pin: String,
}

#[derive(Serialize)]
pub struct SetPinResponse {
    pub success: bool,
}

#[derive(Deserialize, Debug)]
pub struct TimecardEventPayload {
    pub id: String,
    pub staff_member_id: String,
    pub event_type: String, // "clock_in" or "clock_out"
    pub timestamp: String,
}

#[derive(Deserialize)]
pub struct SyncRequest {
    pub timecards: Vec<TimecardEventPayload>,
}

#[derive(Serialize)]
pub struct SyncResponse {
    pub success: bool,
    pub synced_count: usize,
}

#[derive(Serialize)]
pub struct ListStaffResponse {
    pub staff_members: Vec<StaffMember>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

async fn create_staff(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_auth::common::Claims>,
    Json(payload): Json<CreateStaffRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());
    let staff_id = Uuid::new_v4().to_string();

    let query = "INSERT INTO staff_members (id, tenant_id, name, phone_number, role) VALUES ($1, $2, $3, $4, $5)";

    let result = sqlx::query(query)
        .bind(&staff_id)
        .bind(&tenant_id)
        .bind(&payload.name)
        .bind(&payload.phone_number)
        .bind(&payload.role)
        .execute(&hub.pool)
        .await;

    match result {
        Ok(_) => {
            let magic_link = format!("https://ohc.app/terminal/setup?id={}", staff_id);
            (
                StatusCode::OK,
                Json(CreateStaffResponse {
                    success: true,
                    staff_id,
                    magic_link,
                }),
            ).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create staff member: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DB_ERROR".to_string(),
                    message: "Failed to create staff member".to_string(),
                }),
            ).into_response()
        }
    }
}

async fn set_pin(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_auth::common::Claims>,
    Path(staff_id): Path<String>,
    Json(payload): Json<SetPinRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    let hashed_pin = bcrypt::hash(payload.pin, bcrypt::DEFAULT_COST).unwrap_or_else(|_| "".to_string());

    let query = "UPDATE staff_members SET pin_hash = $1 WHERE id = $2 AND tenant_id = $3";

    let result = sqlx::query(query)
        .bind(&hashed_pin)
        .bind(&staff_id)
        .bind(&tenant_id)
        .execute(&hub.pool)
        .await;

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(SetPinResponse { success: true }),
        ).into_response(),
        Err(e) => {
            tracing::error!("Failed to set PIN: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DB_ERROR".to_string(),
                    message: "Failed to set PIN".to_string(),
                }),
            ).into_response()
        }
    }
}

async fn list_staff(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_auth::common::Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    let query = "SELECT id, name, phone_number, role FROM staff_members WHERE tenant_id = $1";

    let result = sqlx::query_as::<_, (String, String, String, String)>(query)
        .bind(&tenant_id)
        .fetch_all(&hub.pool)
        .await;

    match result {
        Ok(rows) => {
            let staff_members = rows.into_iter().map(|(id, name, phone_number, role)| {
                StaffMember { id, name, phone_number, role }
            }).collect();
            (
                StatusCode::OK,
                Json(ListStaffResponse { staff_members }),
            ).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list staff members: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DB_ERROR".to_string(),
                    message: "Failed to fetch staff members".to_string(),
                }),
            ).into_response()
        }
    }
}

async fn sync_terminal(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_auth::common::Claims>,
    Json(payload): Json<SyncRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());
    let mut synced_count = 0;

    for event in payload.timecards {
        let query = "INSERT INTO timecard_events (id, tenant_id, staff_member_id, event_type, timestamp)
                     VALUES ($1, $2, $3, $4, $5)
                     ON CONFLICT (id) DO NOTHING";

        let timestamp = chrono::DateTime::parse_from_rfc3339(&event.timestamp)
            .unwrap_or_else(|_| chrono::Utc::now().into());

        let result = sqlx::query(query)
            .bind(&event.id)
            .bind(&tenant_id)
            .bind(&event.staff_member_id)
            .bind(&event.event_type)
            .bind(timestamp.with_timezone(&chrono::Utc))
            .execute(&hub.pool)
            .await;

        if result.is_ok() {
            synced_count += 1;
        }
    }

    (
        StatusCode::OK,
        Json(SyncResponse { success: true, synced_count }),
    ).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/staff", post(create_staff).get(list_staff))
        .route("/staff/:id/pin", post(set_pin))
        .route("/sync", post(sync_terminal))
        .layer(Extension(hub))
}
