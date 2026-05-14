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
        .route("/generate-description", post(generate_description))
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
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    // In a real app we'd get the tenant_id from auth context
    let tenant_id = "default-session";
    match agent.get_state(tenant_id).await {
        Ok(Some(state)) => Ok(Json(serde_json::json!({ "state": state.to_string() }))),
        Ok(None) => Ok(Json(serde_json::json!({ "state": "{}" }))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn save_state(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = "default-session";
    let user_id = "default-user";
    let step = payload.get("step").and_then(|s| s.as_i64()).unwrap_or(1) as i32;

    match agent.save_state(tenant_id, user_id, step, payload).await {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(serde::Deserialize)]
struct GenDescRequest {
    name: String,
}

#[derive(serde::Serialize)]
struct GenDescResponse {
    description: String,
}

async fn generate_description(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<GenDescRequest>,
) -> Result<Json<GenDescResponse>, axum::http::StatusCode> {
    match agent.generate_ai_description(&payload.name).await {
        Ok(description) => Ok(Json(GenDescResponse { description })),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
