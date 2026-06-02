use axum::{Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AddStaffRequest {
    pub name: String,
    pub role: String,
    pub phone_number: String,
}

#[derive(Serialize)]
pub struct AddStaffResponse {
    pub success: bool,
    pub pin_setup_link: String,
}

pub async fn add_staff_handler(
    headers: axum::http::HeaderMap,
    Json(payload): Json<AddStaffRequest>,
) -> impl IntoResponse {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json(AddStaffResponse { success: false, pin_setup_link: "".to_string() }));
    }

    // Logic to add staff to database goes here

    (
        StatusCode::OK,
        Json(AddStaffResponse { success: true, pin_setup_link: format!("https://ohc.app/setup-pin?token={}", uuid::Uuid::new_v4()) }),
    )
}

#[derive(Deserialize, Debug)]
pub struct ClockEventRequest {
    pub team_member_id: String,
    pub event_type: String, // "CLOCK_IN" or "CLOCK_OUT"
    pub client_timestamp: String,
    pub device_id: String,
}

#[derive(Serialize)]
pub struct ClockEventResponse {
    pub success: bool,
}

pub async fn clock_event_handler(
    headers: axum::http::HeaderMap,
    Json(payload): Json<ClockEventRequest>,
) -> impl IntoResponse {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json(ClockEventResponse { success: false }));
    }

    // Logic to process clock event

    (
        StatusCode::OK,
        Json(ClockEventResponse { success: true }),
    )
}
