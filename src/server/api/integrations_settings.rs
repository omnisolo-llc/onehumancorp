use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::integrations::registry::IntegrationsRegistry;

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
}

pub async fn connect_whatsapp_cloud_api(
    State(registry): State<Arc<IntegrationsRegistry>>,
    Json(payload): Json<ConnectWhatsAppCloudApiReq>,
) -> impl IntoResponse {
    let creds = ::server_ohc::orchestration::ConnectIntegrationRequest {
        integration_id: "whatsapp_cloud_api".to_string(),
        base_url: "".to_string(),
        bot_token: "".to_string(),
        chat_id: payload.phone_number_id.unwrap_or_default(),
        webhook_url: "".to_string(),
        api_token: payload.api_token.unwrap_or_default(),
        from_phone: payload.display_phone_number.unwrap_or_default(),
    };

    match registry.connect("whatsapp_cloud_api", "", creds) {
        Ok(_) => Json(ConnectIntegrationRes {
            success: true,
            message: "WhatsApp Cloud API connected successfully".to_string(),
        }),
        Err(e) => Json(ConnectIntegrationRes {
            success: false,
            message: format!("Failed to connect: {}", e),
        }),
    }
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
    State(registry): State<Arc<IntegrationsRegistry>>,
    Json(payload): Json<ConnectWhatsAppReq>,
) -> impl IntoResponse {
    let integration_id = payload.integration_id.unwrap_or_else(|| "whatsapp".to_string());
    let creds = ::server_ohc::orchestration::ConnectIntegrationRequest {
        integration_id: integration_id.clone(),
        base_url: payload.base_url.unwrap_or_default(),
        bot_token: payload.bot_token.unwrap_or_default(),
        chat_id: "".to_string(),
        webhook_url: "".to_string(),
        api_token: payload.api_token.unwrap_or_default(),
        from_phone: payload.from_phone.unwrap_or_default(),
    };

    match registry.connect(&integration_id, "", creds) {
        Ok(_) => Json(ConnectIntegrationRes {
            success: true,
            message: "WhatsApp connected successfully".to_string(),
        }),
        Err(e) => Json(ConnectIntegrationRes {
            success: false,
            message: format!("Failed to connect: {}", e),
        }),
    }
}
