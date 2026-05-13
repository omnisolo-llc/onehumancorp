use axum::{
    extract::{State, Json},
    routing::{post, get},
    Router,
};
use std::sync::{Arc, Mutex, OnceLock};
use std::collections::HashMap;
use crate::services::onboarding::onboarding_agent::OnboardingAgent;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};

static WIZARD_STATE: OnceLock<Mutex<HashMap<String, serde_json::Value>>> = OnceLock::new();

fn get_wizard_state() -> &'static Mutex<HashMap<String, serde_json::Value>> {
    WIZARD_STATE.get_or_init(|| Mutex::new(HashMap::new()))
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
    let map = get_wizard_state().lock().unwrap();
    if let Some(state) = map.get("test_user") {
        Ok(Json(state.clone()))
    } else {
        Ok(Json(serde_json::json!({
            "step": 1
        })))
    }
}

async fn save_state(
    State(_agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let mut map = get_wizard_state().lock().unwrap();
    map.insert("test_user".to_string(), payload);
    Ok(axum::http::StatusCode::NO_CONTENT)
}
