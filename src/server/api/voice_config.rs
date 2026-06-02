use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceConfigPayload {
    pub is_enabled: bool,
    pub primary_language: String,
    pub custom_instructions: String,
}

pub async fn get_voice_config(
    Extension(claims): Extension<::server_common::Claims>,
) -> axum::response::Response {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());
    let pool = crate::db::get_pool();

    let row = sqlx::query!(
        "SELECT phone_number, is_enabled, primary_language, custom_instructions FROM voice_agent_config WHERE tenant_id = $1",
        tenant_id
    )
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);

    if let Some(r) = row {
        (
            axum::http::StatusCode::OK,
            Json(serde_json::json!({
                "phone_number": r.phone_number,
                "is_enabled": r.is_enabled,
                "primary_language": r.primary_language,
                "custom_instructions": r.custom_instructions,
            })),
        )
            .into_response()
    } else {
        (
            axum::http::StatusCode::OK,
            Json(serde_json::json!({
                "phone_number": "(555) 123-4567", // Mock provisioned number
                "is_enabled": false,
                "primary_language": "English",
                "custom_instructions": "",
            })),
        )
            .into_response()
    }
}

pub async fn update_voice_config(
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<VoiceConfigPayload>,
) -> axum::response::Response {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());
    let pool = crate::db::get_pool();

    let result = sqlx::query!(
        "INSERT INTO voice_agent_config (tenant_id, phone_number, is_enabled, primary_language, custom_instructions)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (tenant_id) DO UPDATE SET
         is_enabled = EXCLUDED.is_enabled,
         primary_language = EXCLUDED.primary_language,
         custom_instructions = EXCLUDED.custom_instructions",
        tenant_id,
        "(555) 123-4567", // Mock provisioned number
        payload.is_enabled,
        payload.primary_language,
        payload.custom_instructions
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (
            axum::http::StatusCode::OK,
            Json(serde_json::json!({"status": "success"})),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to update voice config: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Database error"})),
            )
                .into_response()
        }
    }
}
