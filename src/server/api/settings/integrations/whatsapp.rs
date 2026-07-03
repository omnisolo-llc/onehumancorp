use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use ::server_common::Claims;
use axum::extract::Extension;

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectWhatsAppRequest {
    pub bot_token: Option<String>,
    pub api_token: Option<String>,
    pub from_phone: Option<String>,
}

pub async fn connect_whatsapp_cloud_api(
    State(hub): State<Arc<Hub>>,
    Extension(user): Extension<Claims>,
    Json(payload): Json<ConnectWhatsAppRequest>,
) -> impl IntoResponse {
    let tenant_id = match user.organization_id {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, "Missing tenant context").into_response(),
    };

    let api_token = payload.api_token.unwrap_or_default();
    let from_phone = payload.from_phone.unwrap_or_default();

    let integration_code = serde_json::json!({
        "api_token": api_token,
        "from_phone": from_phone,
    }).to_string();

    let id = format!("{}_whatsapp_cloud_api", tenant_id);

    let db_pool = &hub.pool;
    let res = sqlx::query(
        "INSERT INTO tool_integrations (id, tenant_id, name, status, integration_code)
         VALUES ($1, $2, 'whatsapp_cloud_api', 'connected', $3)
         ON CONFLICT (id) DO UPDATE SET status = 'connected', integration_code = $3"
    )
    .bind(&id)
    .bind(&tenant_id)
    .bind(&integration_code)
    .execute(db_pool)
    .await;

    if let Err(e) = res {
        tracing::error!("Failed to save WhatsApp Cloud API integration: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    let id_uuid = uuid::Uuid::new_v4().to_string();
    let creds_res = sqlx::query(
        "INSERT INTO integration_credentials (id, tenant_id, integration_id, bot_token, api_token, from_phone)
         VALUES ($1, $2, 'whatsapp_cloud_api', '', $3, $4)
         ON CONFLICT (tenant_id, integration_id) DO UPDATE SET bot_token = '', api_token = $3, from_phone = $4"
    )
    .bind(&id_uuid)
    .bind(&tenant_id)
    .bind(&api_token)
    .bind(&from_phone)
    .execute(db_pool)
    .await;

    if let Err(e) = creds_res {
        tracing::error!("Failed to save WhatsApp Cloud API credentials: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    let _creds = ::server_ohc::orchestration::ConnectIntegrationRequest {
        integration_id: "whatsapp_cloud_api".to_string(),
        base_url: "https://graph.facebook.com/v19.0".to_string(),
        bot_token: "".to_string(),
        chat_id: "".to_string(),
        webhook_url: "".to_string(),
        api_token: api_token.clone(),
        from_phone: from_phone.clone(),
    };



    (StatusCode::OK, axum::Json(serde_json::json!({"success": true}))).into_response()
}

pub async fn connect_whatsapp_twilio(
    State(hub): State<Arc<Hub>>,
    Extension(user): Extension<Claims>,
    Json(payload): Json<ConnectWhatsAppRequest>,
) -> impl IntoResponse {
    let tenant_id = match user.organization_id {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, "Missing tenant context").into_response(),
    };

    let bot_token = payload.bot_token.unwrap_or_default();
    let api_token = payload.api_token.unwrap_or_default();
    let from_phone = payload.from_phone.unwrap_or_default();

    let integration_code = serde_json::json!({
        "bot_token": bot_token,
        "api_token": api_token,
        "from_phone": from_phone,
    }).to_string();

    let id = format!("{}_whatsapp", tenant_id);

    let db_pool = &hub.pool;
    let res = sqlx::query(
        "INSERT INTO tool_integrations (id, tenant_id, name, status, integration_code)
         VALUES ($1, $2, 'whatsapp', 'connected', $3)
         ON CONFLICT (id) DO UPDATE SET status = 'connected', integration_code = $3"
    )
    .bind(&id)
    .bind(&tenant_id)
    .bind(&integration_code)
    .execute(db_pool)
    .await;

    if let Err(e) = res {
        tracing::error!("Failed to save WhatsApp Twilio integration: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    let id_uuid = uuid::Uuid::new_v4().to_string();
    let creds_res = sqlx::query(
        "INSERT INTO integration_credentials (id, tenant_id, integration_id, bot_token, api_token, from_phone)
         VALUES ($1, $2, 'whatsapp', $3, $4, $5)
         ON CONFLICT (tenant_id, integration_id) DO UPDATE SET bot_token = $3, api_token = $4, from_phone = $5"
    )
    .bind(&id_uuid)
    .bind(&tenant_id)
    .bind(&bot_token)
    .bind(&api_token)
    .bind(&from_phone)
    .execute(db_pool)
    .await;

    if let Err(e) = creds_res {
        tracing::error!("Failed to save WhatsApp Twilio credentials: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    let _creds = ::server_ohc::orchestration::ConnectIntegrationRequest {
        integration_id: "whatsapp".to_string(),
        base_url: "https://api.twilio.com".to_string(),
        bot_token: bot_token.clone(),
        chat_id: "".to_string(),
        webhook_url: "".to_string(),
        api_token: api_token.clone(),
        from_phone: from_phone.clone(),
    };



    (StatusCode::OK, axum::Json(serde_json::json!({"success": true}))).into_response()
}
