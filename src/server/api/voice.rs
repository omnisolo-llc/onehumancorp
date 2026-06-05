use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    Json,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use ::server_common::Claims;

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct VoiceConfig {
    pub id: String,
    pub tenant_id: String,
    pub greeting: String,
    pub transfer_number: Option<String>,
    pub voice_type: String,
    pub multi_lingual_enabled: bool,
    pub twilio_number: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct ConfigureVoiceRequest {
    pub greeting: String,
    pub transfer_number: Option<String>,
    pub voice_type: String,
    pub multi_lingual_enabled: bool,
}

pub async fn provision_voice_receptionist(
    State(db): State<std::sync::Arc<crate::db::DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ConfigureVoiceRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mock_twilio_number = "+15551234567".to_string();
    let config_id = Uuid::new_v4().to_string();

    let mut tx = db.pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let record = sqlx::query_as::<_, VoiceConfig>(
        r#"
        INSERT INTO voice_config (id, tenant_id, greeting, transfer_number, voice_type, multi_lingual_enabled, twilio_number)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (tenant_id) DO UPDATE SET
            greeting = EXCLUDED.greeting,
            transfer_number = EXCLUDED.transfer_number,
            voice_type = EXCLUDED.voice_type,
            multi_lingual_enabled = EXCLUDED.multi_lingual_enabled,
            updated_at = CURRENT_TIMESTAMP
        RETURNING id, tenant_id, greeting, transfer_number, voice_type, multi_lingual_enabled, twilio_number, created_at, updated_at
        "#
    )
    .bind(config_id)
    .bind(&tenant_id)
    .bind(payload.greeting)
    .bind(payload.transfer_number)
    .bind(payload.voice_type)
    .bind(payload.multi_lingual_enabled)
    .bind(Some(mock_twilio_number))
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, Json(record)))
}

pub async fn get_voice_config(
    State(db): State<std::sync::Arc<crate::db::DB>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut tx = db.pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let record = sqlx::query_as::<_, VoiceConfig>(
        r#"
        SELECT id, tenant_id, greeting, transfer_number, voice_type, multi_lingual_enabled, twilio_number, created_at, updated_at
        FROM voice_config
        WHERE tenant_id = $1
        ORDER BY updated_at DESC
        LIMIT 1
        "#
    )
    .bind(&tenant_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match record {
        Some(config) => Ok((StatusCode::OK, Json(config))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
