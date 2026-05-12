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
        .route("/suggest", post(suggest_info))
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

#[derive(serde::Deserialize)]
struct SuggestRequest {
    name: String,
    business_type: String,
}

async fn suggest_info(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<SuggestRequest>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    match agent.suggest_business_info(&payload.name, &payload.business_type).await {
        Ok((tagline, description)) => Ok(Json(serde_json::json!({
            "tagline": tagline,
            "description": description
        }))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_state(
    State(_agent): State<Arc<OnboardingAgent>>,
    Extension(claims): Extension<::server_common::Claims>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let org_id = claims.organization_id.ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
    // Use dummy for now, but ensured claims are present
    Ok(Json(serde_json::json!({
        "state": "{}",
        "org_id": org_id
    })))
}

async fn save_state(
    State(_agent): State<Arc<OnboardingAgent>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let _org_id = claims.organization_id.ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}
