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

use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;
use axum::extract::Query;

static ONBOARDING_STATES: OnceLock<Mutex<HashMap<String, serde_json::Value>>> = OnceLock::new();

#[derive(serde::Deserialize)]
pub struct StateQuery {
    pub user: Option<String>,
}

async fn get_state(
    Query(query): Query<StateQuery>,
    State(_agent): State<Arc<OnboardingAgent>>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let map = ONBOARDING_STATES.get_or_init(|| Mutex::new(HashMap::new())).lock().unwrap();
    if let Some(user) = query.user {
        if let Some(state) = map.get(&user) {
            return Ok(Json(serde_json::json!({ "state": state })));
        }
    }
    Ok(Json(serde_json::json!({
        "state": {}
    })))
}

async fn save_state(
    Query(query): Query<StateQuery>,
    State(_agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    if let Some(user) = query.user {
        let mut map = ONBOARDING_STATES.get_or_init(|| Mutex::new(HashMap::new())).lock().unwrap();
        map.insert(user, payload);
        Ok(axum::http::StatusCode::NO_CONTENT)
    } else {
        Err(axum::http::StatusCode::BAD_REQUEST)
    }
}
