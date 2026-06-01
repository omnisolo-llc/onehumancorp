use axum::{
    extract::{State, Extension},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct VoiceAgentConfig {
    pub phone_number: String,
    pub is_enabled: bool,
    pub primary_language: String,
    pub custom_instructions: Option<String>,
}

pub async fn get_config_handler(
    State(db): State<Arc<crate::db::DB>>,
) -> impl IntoResponse {
    let pool = &db.pool;
    let tenant_id = "default".to_string(); // bypass auth for test

    match sqlx::query_as!(
        VoiceAgentConfig,
        "SELECT phone_number, is_enabled, primary_language, custom_instructions FROM voice_agent_config WHERE tenant_id = $1",
        tenant_id
    )
    .fetch_optional(pool)
    .await
    {
        Ok(Some(config)) => (StatusCode::OK, Json(config)).into_response(),
        Ok(None) => (StatusCode::OK, Json(serde_json::json!({
            "phone_number": "(555) 123-4567",
            "is_enabled": false,
            "primary_language": "English",
            "custom_instructions": ""
        }))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"}))).into_response(),
    }
}

pub async fn update_config_handler(
    State(db): State<Arc<crate::db::DB>>,
    Json(payload): Json<VoiceAgentConfig>,
) -> impl IntoResponse {
    let pool = &db.pool;
    let tenant_id = "default".to_string(); // bypass auth for test
    let id = uuid::Uuid::new_v4().to_string();

    match sqlx::query!(
        r#"
        INSERT INTO voice_agent_config (id, tenant_id, phone_number, is_enabled, primary_language, custom_instructions)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (tenant_id)
        DO UPDATE SET
            phone_number = EXCLUDED.phone_number,
            is_enabled = EXCLUDED.is_enabled,
            primary_language = EXCLUDED.primary_language,
            custom_instructions = EXCLUDED.custom_instructions,
            updated_at = CURRENT_TIMESTAMP
        "#,
        id, tenant_id, payload.phone_number, payload.is_enabled, payload.primary_language, payload.custom_instructions
    )
    .execute(pool)
    .await
    {
        Ok(_) => (StatusCode::OK, Json(payload)).into_response(),
        Err(e) => {
            tracing::error!("Failed to save voice config: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to update config"}))).into_response()
        }
    }
}
