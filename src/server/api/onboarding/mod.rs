use axum::{
    extract::{State, Json},
    routing::{post, get},
    Router,
};
use std::sync::{Arc, Mutex, OnceLock};
use std::collections::HashMap;
use crate::services::onboarding::onboarding_agent::OnboardingAgent;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};

// State per tenant. Note: in production this uses Redis/PostgreSQL
fn wizard_state() -> &'static Mutex<HashMap<String, String>> {
    static WIZARD_STATE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
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
    headers: axum::http::HeaderMap,
    State(_agent): State<Arc<OnboardingAgent>>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");
    let map = wizard_state().lock().unwrap();
    let state = map.get(tenant_id).cloned().unwrap_or_else(|| "{}".to_string());
    Ok(Json(serde_json::json!({
        "state": state
    })))
}

async fn save_state(
    headers: axum::http::HeaderMap,
    State(_agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");
    let mut map = wizard_state().lock().unwrap();
    map.insert(tenant_id.to_string(), payload.to_string());
    Ok(axum::http::StatusCode::NO_CONTENT)
}
