use axum::{
    extract::{State, Json, Extension},
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
        .route("/generate", post(generate_onboarding_handler))
        .route("/state", get(get_state).post(save_state))
        .route("/launch", post(launch_onboarding))
        .route("/draft", get(get_draft).post(save_draft))
        .layer(axum::middleware::from_fn(::server_auth::guest_auth_middleware))
        .with_state(agent);

    // Convert to accept MeshTransport state
    Router::new().merge(r)
}

#[derive(serde::Deserialize)]
pub struct IntakeRequest {
    pub description: String,
}

#[derive(serde::Deserialize)]
pub struct ZeroClickRequest {
    pub prompt: String,
}

async fn generate_onboarding_handler(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<ZeroClickRequest>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone();

    match agent.enqueue_zero_click(&tenant_id, &user_id, &payload.prompt).await {
        Ok(_) => {
            Ok(Json(serde_json::json!({
                "tenant_id": tenant_id,
                "status": "enqueued"
            })))
        },
        Err(error) => {
            tracing::error!("zero click enqueue error: {}", error);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn process_intake_handler(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<IntakeRequest>,
) -> Result<Json<crate::services::onboarding::onboarding_agent::IntakeData>, axum::http::StatusCode> {
    match agent.process_intake(&payload.description).await {
        Ok(data) => Ok(Json(data)),
        Err(error) => {
            tracing::warn!("onboarding intake fallback used after agent error: {}", error);
            Ok(Json(crate::services::onboarding::onboarding_agent::IntakeData {
                business_name: {
                    let desc = payload.description.trim();
                    let char_count = desc.chars().count();
                    if char_count > 30 {
                        let truncated: String = desc.chars().take(27).collect();
                        format!("{}...", truncated)
                    } else {
                        desc.to_string()
                    }
                },
                business_type: "Local Business".to_string(),
                categories: vec!["services".to_string()],
                initial_products: Vec::new(),
                location: None,
                target_audience: None,
            }))
        }
    }
}

async fn get_draft(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone(); // In this context agent_id refers to the user
    match agent.get_onboarding_state(&tenant_id, &user_id).await {
        Ok(state) => Ok(Json(state)),
        Err(_) => Ok(Json(serde_json::json!({}))), // fallback
    }
}

async fn save_draft(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone();

    let step = payload.get("wizardState")
        .and_then(|w| w.get("step"))
        .or_else(|| payload.get("step"))
        .and_then(|s| s.as_i64())
        .unwrap_or(0) as i32;

    match agent.save_onboarding_state(&tenant_id, &user_id, step, &payload).await {
        Ok(_) => Ok(axum::http::StatusCode::OK),
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

async fn launch_onboarding(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone();
    let current_step = 5; // Launch step

    let state = serde_json::json!({
        "status": "launched"
    });
    match agent.save_onboarding_state(&tenant_id, &user_id, current_step, &state).await {
        Ok(_) => Ok(Json(state)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn get_state(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone();
    match agent.get_onboarding_state(&tenant_id, &user_id).await {
        Ok(state) => Ok(Json(state)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn save_state(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone();

    let step = payload.get("wizardState")
        .and_then(|w| w.get("step"))
        .or_else(|| payload.get("step"))
        .and_then(|s| s.as_i64())
        .unwrap_or(0) as i32;

    match agent.save_onboarding_state(&tenant_id, &user_id, step, &payload).await {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
