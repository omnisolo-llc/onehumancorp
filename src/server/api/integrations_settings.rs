use crate::integrations::registry::IntegrationsRegistry;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ConnectWhatsAppCloudApiReq {
    pub api_token: Option<String>,
    pub phone_number_id: Option<String>,
    pub display_phone_number: Option<String>,
}

#[derive(Serialize)]
pub struct ConnectIntegrationRes {
    pub success: bool,
    pub message: String,
    pub status: String,
    pub usable: bool,
}

pub(crate) fn verification_unavailable() -> impl IntoResponse {
    // Registry construction is configuration, not provider verification. This
    // route has no verified-connection receipt or durable credential flow yet.
    (
        StatusCode::NOT_IMPLEMENTED,
        [(axum::http::header::CACHE_CONTROL, "no-store")],
        Json(ConnectIntegrationRes {
            success: false,
            message: "Secure provider verification is unavailable. No connection was established."
                .to_string(),
            status: "pending_verification".to_string(),
            usable: false,
        }),
    )
}

pub async fn connect_whatsapp_cloud_api(
    State(_registry): State<Arc<IntegrationsRegistry>>,
    Json(_payload): Json<ConnectWhatsAppCloudApiReq>,
) -> impl IntoResponse {
    verification_unavailable()
}

#[derive(Deserialize)]
pub struct ConnectWhatsAppReq {
    pub bot_token: Option<String>,
    pub api_token: Option<String>,
    pub from_phone: Option<String>,
    pub integration_id: Option<String>,
    pub base_url: Option<String>,
}

pub async fn connect_whatsapp(
    State(_registry): State<Arc<IntegrationsRegistry>>,
    Json(_payload): Json<ConnectWhatsAppReq>,
) -> impl IntoResponse {
    verification_unavailable()
}
