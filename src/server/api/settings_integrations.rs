use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use ::server_common::Claims;
use sqlx::PgPool;

#[derive(Deserialize, Serialize, Clone)]
pub struct ConnectIntegrationRequest {
    #[serde(default)]
    pub integration_id: String,
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub bot_token: String,
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub webhook_url: String,
    #[serde(default)]
    pub api_token: String,
    #[serde(default)]
    pub from_phone: String,
}

#[derive(Serialize)]
pub struct SettingResponse {
    pub success: bool,
}

pub async fn whatsapp_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ConnectIntegrationRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(SettingResponse { success: false })).into_response(),
    };

    let result = sqlx::query(
        "INSERT INTO settings (tenant_id, twilio_whatsapp_config) VALUES ($1, $2) ON CONFLICT (tenant_id) DO UPDATE SET twilio_whatsapp_config = $2"
    )
    .bind(&tenant_id)
    .bind(sqlx::types::Json(&payload))
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(SettingResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to save Twilio WhatsApp config: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(SettingResponse { success: false })).into_response()
        }
    }
}

pub async fn whatsapp_cloud_api_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ConnectIntegrationRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(SettingResponse { success: false })).into_response(),
    };

    let result = sqlx::query(
        "INSERT INTO settings (tenant_id, meta_whatsapp_config) VALUES ($1, $2) ON CONFLICT (tenant_id) DO UPDATE SET meta_whatsapp_config = $2"
    )
    .bind(&tenant_id)
    .bind(sqlx::types::Json(&payload))
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(SettingResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to save Meta WhatsApp Cloud API config: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(SettingResponse { success: false })).into_response()
        }
    }
}
