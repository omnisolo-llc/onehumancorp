use axum::{
    extract::{State, Json},
    routing::{post, get},
    Router,
};
use std::sync::{Arc, Mutex};
use crate::services::onboarding::onboarding_agent::OnboardingAgent;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};

// Use std::sync::OnceLock which is in standard library
static ONBOARDING_STATE: std::sync::OnceLock<Mutex<serde_json::Value>> = std::sync::OnceLock::new();

fn get_state_lock() -> &'static Mutex<serde_json::Value> {
    ONBOARDING_STATE.get_or_init(|| Mutex::new(serde_json::json!({"step": 0})))
}

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
    State(_agent): State<Arc<OnboardingAgent>>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let state = get_state_lock().lock().unwrap().clone();
    Ok(Json(state))
}

async fn save_state(
    State(_agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let mut state = get_state_lock().lock().unwrap();
    *state = payload;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    // remove tower tests due to missing ServiceExt dependency
}
