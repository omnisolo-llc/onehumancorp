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
        .route("/intake", post(process_intake_handler))
        .route("/state", get(get_state))
        .route("/state", post(save_state))
        .with_state(agent);

    // Convert to accept MeshTransport state
    Router::new().merge(r)
}

#[derive(serde::Deserialize)]
pub struct IntakeRequest {
    pub description: String,
}

async fn process_intake_handler(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<IntakeRequest>,
) -> Result<Json<crate::services::onboarding::onboarding_agent::IntakeData>, axum::http::StatusCode> {
    match agent.process_intake(&payload.description).await {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
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
    State(_agent): State<Arc<OnboardingAgent>>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    Ok(Json(serde_json::json!({
        "state": "{}"
    })))
}

async fn save_state(
    State(_agent): State<Arc<OnboardingAgent>>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    Ok(axum::http::StatusCode::NO_CONTENT)
}
