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

#[derive(Deserialize)]
pub struct SendTemplateMessageRequest {
    pub to: String,
    pub template_name: String,
    pub language_code: String,
    pub components: Option<Vec<serde_json::Value>>,
}

async fn get_whatsapp_creds(tenant_id: &str) -> Option<(String, String)> {
    let pool = crate::db::get_pool();
    let row = sqlx::query_as::<_, (String, String)>(
        "SELECT COALESCE(api_token, ''), COALESCE(from_phone, '') \
         FROM integration_credentials \
         WHERE tenant_id = $1 AND integration_id = 'whatsapp_cloud_api' LIMIT 1"
    )
    .bind(tenant_id)
    .fetch_optional(&pool)
    .await
    .ok()??;

    if row.0.is_empty() || row.1.is_empty() {
        None
    } else {
        Some(row)
    }
}

pub async fn get_whatsapp_templates(
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match user.organization_id {
        Some(ref t) => t,
        None => return (StatusCode::UNAUTHORIZED, "Missing tenant context").into_response(),
    };

    let (api_token, phone_number_id) = match get_whatsapp_creds(tenant_id).await {
        Some(creds) => creds,
        None => return (StatusCode::BAD_REQUEST, "WhatsApp Cloud API is not connected").into_response(),
    };

    let client = crate::integrations::whatsapp::client::WhatsAppClient::new(api_token, phone_number_id);
    match client.sync_templates().await {
        Ok(templates) => (StatusCode::OK, Json(templates)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub async fn get_whatsapp_health(
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match user.organization_id {
        Some(ref t) => t,
        None => return (StatusCode::UNAUTHORIZED, "Missing tenant context").into_response(),
    };

    let (api_token, phone_number_id) = match get_whatsapp_creds(tenant_id).await {
        Some(creds) => creds,
        None => return (StatusCode::BAD_REQUEST, "WhatsApp Cloud API is not connected").into_response(),
    };

    let client = crate::integrations::whatsapp::client::WhatsAppClient::new(api_token, phone_number_id);
    match client.get_phone_number_health().await {
        Ok(health) => (StatusCode::OK, Json(health)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub async fn send_whatsapp_template(
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<SendTemplateMessageRequest>,
) -> impl IntoResponse {
    let tenant_id = match user.organization_id {
        Some(ref t) => t,
        None => return (StatusCode::UNAUTHORIZED, "Missing tenant context").into_response(),
    };

    let (api_token, phone_number_id) = match get_whatsapp_creds(tenant_id).await {
        Some(creds) => creds,
        None => return (StatusCode::BAD_REQUEST, "WhatsApp Cloud API is not connected").into_response(),
    };

    let client = crate::integrations::whatsapp::client::WhatsAppClient::new(api_token, phone_number_id);
    let components = payload.components.unwrap_or_default();
    match client.send_template_message(&payload.to, &payload.template_name, &payload.language_code, components).await {
        Ok(msg_id) => (StatusCode::OK, Json(serde_json::json!({ "success": true, "message_id": msg_id }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
