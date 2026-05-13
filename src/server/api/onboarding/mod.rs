use axum::{
    extract::{State, Json},
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use crate::services::onboarding::onboarding_agent::OnboardingAgent;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};

pub fn router(agent: Arc<OnboardingAgent>) -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let r = Router::new()
        .route("/start", post(start_onboarding))
        .route("/state", get(get_state))
        .route("/state", post(save_state))
        .with_state(agent);
    Router::new().merge(r)
}

async fn start_onboarding(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<StartOnboardingRequest>,
) -> Result<Json<StartOnboardingResponse>, axum::http::StatusCode> {
    match agent.start_onboarding(payload).await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_state(
    State(agent): State<Arc<OnboardingAgent>>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let token = headers.get("authorization").and_then(|h| h.to_str().ok()).unwrap_or("").replace("Bearer ", "");
    if token.is_empty() { return Ok(Json(serde_json::json!({ "state": "{}" }))); }
    let auth_info = match ::server_auth::parse_spiffe_id(&token) { Ok(i) => i, Err(_) => return Ok(Json(serde_json::json!({ "state": "{}" }))) };
    let user_id = auth_info.1;
    let tenant_id = auth_info.0;
    let pool = &agent.db.pool;
    use sqlx::Row;
    let row = sqlx::query("SELECT state_json FROM onboarding_state WHERE user_id = $1")
        .bind(&user_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Some(r) = row {
        let state_json: serde_json::Value = r.get("state_json");
        Ok(Json(serde_json::json!({ "state": state_json.to_string() })))
    } else {
        Ok(Json(serde_json::json!({ "state": "{}" })))
    }
}

async fn save_state(
    State(agent): State<Arc<OnboardingAgent>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let token = headers.get("authorization").and_then(|h| h.to_str().ok()).unwrap_or("").replace("Bearer ", "");
    if token.is_empty() { return Ok(axum::http::StatusCode::NO_CONTENT); }
    let auth_info = match ::server_auth::parse_spiffe_id(&token) { Ok(i) => i, Err(_) => return Ok(axum::http::StatusCode::NO_CONTENT) };
    let user_id = auth_info.1;
    let tenant_id = auth_info.0;
    let pool = &agent.db.pool;
    let current_step = payload.get("step").and_then(|s| s.as_i64()).unwrap_or(0) as i32;
    sqlx::query("INSERT INTO onboarding_state (tenant_id, user_id, current_step, state_json) VALUES ($1, $2, $3, $4) ON CONFLICT (tenant_id, user_id) DO UPDATE SET state_json = onboarding_state.state_json || EXCLUDED.state_json, current_step = EXCLUDED.current_step, updated_at = CURRENT_TIMESTAMP").bind(tenant_id).bind(user_id).bind(current_step).bind(payload).execute(pool).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}
