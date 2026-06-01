use axum::{
    extract::{State, Json, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use crate::db::get_pool;
use crate::common::auth_utils::set_org_context;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct VoiceAgentConfig {
    pub tenant_id: String,
    pub phone_number: String,
    pub is_enabled: bool,
    pub primary_language: String,
    pub custom_instructions: String,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/:tenant_id", get(get_config).post(update_config))
}

async fn get_config(
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    let pool = get_pool();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    };

    if let Err(_) = set_org_context(&mut *tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Context error").into_response();
    }

    let config = sqlx::query_as::<_, VoiceAgentConfig>(
        "SELECT tenant_id, phone_number, is_enabled, primary_language, custom_instructions FROM voice_agent_configs WHERE tenant_id = $1"
    )
    .bind(tenant_id)
    .fetch_optional(&mut *tx)
    .await;

    let _ = tx.commit().await;

    match config {
        Ok(Some(cfg)) => (StatusCode::OK, Json(cfg)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Config not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response(),
    }
}

async fn update_config(
    Path(tenant_id): Path<String>,
    Json(payload): Json<VoiceAgentConfig>,
) -> impl IntoResponse {
    let pool = get_pool();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    };

    if let Err(_) = set_org_context(&mut *tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Context error").into_response();
    }

    let res = sqlx::query(
        "INSERT INTO voice_agent_configs (tenant_id, phone_number, is_enabled, primary_language, custom_instructions, updated_at) VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP) ON CONFLICT(tenant_id) DO UPDATE SET phone_number = excluded.phone_number, is_enabled = excluded.is_enabled, primary_language = excluded.primary_language, custom_instructions = excluded.custom_instructions, updated_at = CURRENT_TIMESTAMP"
    )
    .bind(tenant_id)
    .bind(payload.phone_number)
    .bind(payload.is_enabled as bool)
    .bind(payload.primary_language)
    .bind(payload.custom_instructions)
    .execute(&mut *tx)
    .await;

    let _ = tx.commit().await;

    match res {
        Ok(_) => (StatusCode::OK, "Updated").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response(),
    }
}
