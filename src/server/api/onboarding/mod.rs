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
        .route("/suggest", post(suggest_description))
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

async fn suggest_description(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let product_name = payload.get("product_name").and_then(|v| v.as_str()).unwrap_or_default();
    let business_type = payload.get("business_type").and_then(|v| v.as_str()).unwrap_or_default();

    match agent.suggest_product_description(product_name, business_type).await {
        Ok(description) => Ok(Json(serde_json::json!({ "description": description }))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_state(
    State(agent): State<Arc<OnboardingAgent>>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = params.get("tenant_id").cloned().unwrap_or_else(|| "default".to_string());
    let org_id = params.get("org_id").cloned().unwrap_or_else(|| "default".to_string());

    match agent.get_onboarding_state(&tenant_id, &org_id).await {
        Ok(state) => Ok(Json(state)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn save_state(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = payload.get("tenant_id").and_then(|v| v.as_str()).unwrap_or("default").to_string();
    let org_id = payload.get("org_id").and_then(|v| v.as_str()).unwrap_or("default").to_string();
    let user_id = payload.get("user_id").and_then(|v| v.as_str()).unwrap_or("default").to_string();
    let step = payload.get("step").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let state = payload.get("state").cloned().unwrap_or(serde_json::json!({}));

    match agent.save_onboarding_state(&tenant_id, &org_id, &user_id, step, state).await {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
