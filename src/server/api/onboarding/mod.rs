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

    // Convert to accept MeshTransport state
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
    let user_id = headers
        .get("x-user-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("anonymous");

    match agent.get_state(user_id).await {
        Ok(state) => Ok(Json(state)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn save_state(
    State(agent): State<Arc<OnboardingAgent>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let user_id = headers
        .get("x-user-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("anonymous");

    match agent.save_state(user_id, payload).await {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
