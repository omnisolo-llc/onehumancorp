use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use ::server_common::Claims;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct StaffMember {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub phone_number: Option<String>,
    pub role: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct AddStaffRequest {
    pub name: String,
    pub phone_number: Option<String>,
    pub role: String,
}

#[derive(Deserialize)]
pub struct SetPinRequest {
    pub staff_id: String,
    pub pin: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimecardEvent {
    pub id: String,
    pub tenant_id: String,
    pub staff_member_id: String,
    pub event_type: String,
    pub client_timestamp: chrono::DateTime<chrono::Utc>,
    pub sync_id: Option<String>,
}

#[derive(Deserialize)]
pub struct TerminalClockEvent {
    pub staff_member_id: String,
    pub pin: String, // We use PIN to auth offline and here for simple server auth
    pub event_type: String, // 'CLOCK_IN', 'CLOCK_OUT'
    pub client_timestamp: chrono::DateTime<chrono::Utc>,
    pub sync_id: Option<String>,
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(list_staff).post(add_staff))
        .route("/pin", post(set_pin))
        .route("/terminal/clock-in", post(terminal_clock_event))
        .route("/terminal/clock-out", post(terminal_clock_event))
}

async fn list_staff(
    Extension(user): Extension<Claims>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    let _ = pool; // For now we just return a stub to unblock UI while we implement DB

    // Note: Since we are not strictly required to use a real DB backend for the mesh in this prototype,
    // we'll return an empty list or mock list here, but in a real app we'd query `ohc_staff_members`.
    let mock_staff = vec![];
    (StatusCode::OK, Json(mock_staff)).into_response()
}

async fn add_staff(
    Extension(user): Extension<Claims>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<AddStaffRequest>,
) -> impl IntoResponse {
    let staff = StaffMember {
        id: Uuid::new_v4().to_string(),
        tenant_id: user.tenant_id.unwrap_or_else(|| "default".to_string()),
        name: payload.name,
        phone_number: payload.phone_number,
        role: payload.role,
        status: "ACTIVE".to_string(),
    };
    // Mock save
    (StatusCode::CREATED, Json(staff)).into_response()
}

async fn set_pin(
    Extension(user): Extension<Claims>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<SetPinRequest>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({"status": "ok"}))).into_response()
}

async fn terminal_clock_event(
    Extension(user): Extension<Claims>, // Wait, terminal might be unauthenticated if it relies on PIN? Let's assume tenant is passed in header/JWT.
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<TerminalClockEvent>,
) -> impl IntoResponse {
    let event = TimecardEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: user.tenant_id.unwrap_or_else(|| "default".to_string()),
        staff_member_id: payload.staff_member_id,
        event_type: payload.event_type,
        client_timestamp: payload.client_timestamp,
        sync_id: payload.sync_id,
    };
    (StatusCode::CREATED, Json(event)).into_response()
}
