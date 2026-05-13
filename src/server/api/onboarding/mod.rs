use axum::{
    extract::{State, Json},
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use crate::services::onboarding::onboarding_agent::OnboardingAgent;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};
use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;

static ONBOARDING_STATE: OnceLock<Mutex<HashMap<String, serde_json::Value>>> = OnceLock::new();

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

async fn get_state() -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let state_map = ONBOARDING_STATE.get_or_init(|| Mutex::new(HashMap::new())).lock().unwrap();
    if let Some(state) = state_map.get("default") {
        Ok(Json(state.clone()))
    } else {
        Ok(Json(serde_json::json!({})))
    }
}

async fn save_state(
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let mut state_map = ONBOARDING_STATE.get_or_init(|| Mutex::new(HashMap::new())).lock().unwrap();
    state_map.insert("default".to_string(), payload);
    Ok(axum::http::StatusCode::NO_CONTENT)
}
