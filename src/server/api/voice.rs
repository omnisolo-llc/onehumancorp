use axum::{
    extract::{State, Form},
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use std::sync::Arc;
use crate::orchestration::mesh::TeammateMesh;
use crate::common::Claims;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct VoiceAgentConfig {
    pub id: String,
    pub tenant_id: String,
    pub phone_number: Option<String>,
    pub is_enabled: bool,
    pub primary_language: String,
    pub custom_instructions: Option<String>,
    pub allow_booking: bool,
    pub allow_texting: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct VoiceAgentConfigRequest {
    pub is_enabled: bool,
    pub allow_booking: bool,
    pub allow_texting: bool,
    pub primary_language: Option<String>,
    pub custom_instructions: Option<String>,
}

pub async fn get_voice_settings(
    State(pool): State<PgPool>,
    axum::extract::Extension(user): axum::extract::Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = user.organization_id.unwrap_or_default();

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to start transaction").into_response(),
    };

    if let Err(_) = sqlx::query(&format!("SET app.current_tenant = '{}'", tenant_id))
        .execute(&mut *tx)
        .await
    {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to set tenant context").into_response();
    }

    let config = sqlx::query_as::<_, VoiceAgentConfig>(
        "SELECT * FROM voice_agent_config WHERE tenant_id = $1"
    )
    .bind(&tenant_id)
    .fetch_optional(&mut *tx)
    .await;

    let _ = tx.commit().await;

    match config {
        Ok(Some(cfg)) => Json(cfg).into_response(),
        Ok(None) => {
            let default_cfg = VoiceAgentConfig {
                id: "".to_string(),
                tenant_id: tenant_id,
                phone_number: None,
                is_enabled: false,
                primary_language: "english".to_string(),
                custom_instructions: None,
                allow_booking: false,
                allow_texting: false,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            Json(default_cfg).into_response()
        },
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {}", e)).into_response(),
    }
}

pub async fn update_voice_settings(
    State(pool): State<PgPool>,
    axum::extract::Extension(user): axum::extract::Extension<Claims>,
    Json(payload): Json<VoiceAgentConfigRequest>,
) -> impl IntoResponse {
    let tenant_id = user.organization_id.unwrap_or_default();

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to start transaction").into_response(),
    };

    if let Err(_) = sqlx::query(&format!("SET app.current_tenant = '{}'", tenant_id))
        .execute(&mut *tx)
        .await
    {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to set tenant context").into_response();
    }

    let id = Uuid::new_v4().to_string();
    let primary_lang = payload.primary_language.unwrap_or_else(|| "english".to_string());

    let result = sqlx::query(
        r#"
        INSERT INTO voice_agent_config
        (id, tenant_id, is_enabled, primary_language, custom_instructions, allow_booking, allow_texting, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
        ON CONFLICT (tenant_id) DO UPDATE SET
            is_enabled = EXCLUDED.is_enabled,
            primary_language = EXCLUDED.primary_language,
            custom_instructions = EXCLUDED.custom_instructions,
            allow_booking = EXCLUDED.allow_booking,
            allow_texting = EXCLUDED.allow_texting,
            updated_at = NOW()
        "#
    )
    .bind(&id)
    .bind(&tenant_id)
    .bind(payload.is_enabled)
    .bind(primary_lang)
    .bind(payload.custom_instructions)
    .bind(payload.allow_booking)
    .bind(payload.allow_texting)
    .execute(&mut *tx)
    .await;

    match result {
        Ok(_) => {
            if let Err(_) = tx.commit().await {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to commit transaction").into_response();
            }
            Json(serde_json::json!({"status": "success"})).into_response()
        },
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {}", e)).into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct TwilioIncomingCall {
    #[serde(rename = "From")]
    pub from: Option<String>,
    #[serde(rename = "To")]
    pub to: Option<String>,
    #[serde(rename = "CallSid")]
    pub call_sid: Option<String>,
}

pub async fn incoming_voice_webhook(
    State(_hub): State<Arc<TeammateMesh>>,
    Form(payload): Form<TwilioIncomingCall>,
) -> impl IntoResponse {
    let from = payload.from.unwrap_or_default();
    let to = payload.to.unwrap_or_default();
    let _call_sid = payload.call_sid.unwrap_or_default();

    tracing::info!("Incoming Twilio Voice Webhook: from {} to {}", from, to);

    let twiml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
    <Say>Hello! Thank you for calling. How can I help you today?</Say>
    <Record timeout="10" />
</Response>"#;

    ([(axum::http::header::CONTENT_TYPE, "text/xml")], twiml).into_response()
}
