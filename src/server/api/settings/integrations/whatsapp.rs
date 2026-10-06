use ::server_common::Claims;
use axum::{Json, extract::Extension, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectWhatsAppRequest {
    pub bot_token: Option<String>,
    pub api_token: Option<String>,
    pub from_phone: Option<String>,
}

fn unavailable_for_owner(user: &Claims) -> axum::response::Response {
    if user
        .organization_id
        .as_deref()
        .is_none_or(|id| id.trim().is_empty())
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"success": false, "usable": false, "status": "unavailable", "message": "Authenticated organization required"})),
        )
            .into_response();
    }
    if !user
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("owner") || role.eq_ignore_ascii_case("admin"))
    {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"success": false, "usable": false, "status": "unavailable", "message": "Owner approval is required"})),
        )
            .into_response();
    }
    // Neither provider has a verified encrypted connection flow on this route.
    // In particular, do not store submitted strings or label them connected.
    crate::api::integrations_settings::verification_unavailable().into_response()
}

pub async fn connect_whatsapp_cloud_api(
    Extension(user): Extension<Claims>,
    Json(_payload): Json<ConnectWhatsAppRequest>,
) -> impl IntoResponse {
    unavailable_for_owner(&user)
}

pub async fn connect_whatsapp_twilio(
    Extension(user): Extension<Claims>,
    Json(_payload): Json<ConnectWhatsAppRequest>,
) -> impl IntoResponse {
    unavailable_for_owner(&user)
}
