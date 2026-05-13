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

#[derive(serde::Deserialize)]
pub struct CustomOnboardingReq {
    name: String,
    r#type: String,
    goal: String,
}

#[derive(serde::Serialize)]
pub struct CustomOnboardingRes {
    public_link: String,
}

async fn start_onboarding(
    State(_agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<CustomOnboardingReq>,
) -> Result<Json<CustomOnboardingRes>, axum::http::StatusCode> {
    let slug = payload.name.to_lowercase().replace(" ", "-");
    let link = format!("ohc.com/{}", slug);
    Ok(Json(CustomOnboardingRes { public_link: link }))
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
