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
        .route("/chat", post(process_chat_handler))
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
    pub image_url: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<crate::services::onboarding::onboarding_agent::ChatMessage>,
}

async fn process_intake_handler(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<IntakeRequest>,
) -> Result<Json<crate::services::onboarding::onboarding_agent::IntakeData>, axum::http::StatusCode> {
    let mut combined_input = payload.description.clone();
    if let Some(image_url) = &payload.image_url {
        combined_input.push_str(&format!("\nImage provided: {}", image_url));
    }
    match agent.process_intake(&combined_input).await {
        Ok(data) => Ok(Json(data)),
        Err(error) => {
            tracing::error!("onboarding intake agent error: {}", error);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn process_chat_handler(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<crate::services::onboarding::onboarding_agent::ChatResponse>, axum::http::StatusCode> {
    match agent.process_chat(payload.messages).await {
        Ok(data) => Ok(Json(data)),
        Err(error) => {
            tracing::error!("onboarding chat agent error: {}", error);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_draft(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone(); // In this context agent_id refers to the user

    let tid = if tenant_id.is_empty() { "default".to_string() } else { tenant_id };
    let uid = if user_id.is_empty() { "default".to_string() } else { user_id };

    match agent.get_onboarding_state(&tid, &uid).await {
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

    let tid = if tenant_id.is_empty() { "default".to_string() } else { tenant_id };
    let uid = if user_id.is_empty() { "default".to_string() } else { user_id };

    let step = payload.get("wizardState")
        .and_then(|w| w.get("step"))
        .or_else(|| payload.get("step"))
        .and_then(|s| s.as_i64())
        .unwrap_or(0) as i32;

    match agent.save_onboarding_state(&tid, &uid, step, &payload).await {
        Ok(_) => Ok(axum::http::StatusCode::OK),
        Err(e) => {
            tracing::error!("Failed to save onboarding draft: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

async fn start_onboarding(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<StartOnboardingRequest>,
) -> Result<Json<StartOnboardingResponse>, axum::http::StatusCode> {
    match agent.start_onboarding(payload).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            tracing::error!("Failed to start onboarding: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

async fn launch_onboarding(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone();
    let current_step = 5; // Launch step

    let tid = if tenant_id.is_empty() { "default".to_string() } else { tenant_id };
    let uid = if user_id.is_empty() { "default".to_string() } else { user_id };

    let state = serde_json::json!({
        "status": "launched"
    });
    match agent.save_onboarding_state(&tid, &uid, current_step, &state).await {
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

    // Support X- headers if auth_info is empty (for setup phase)
    let tid = if tenant_id.is_empty() { "default".to_string() } else { tenant_id };
    let uid = if user_id.is_empty() { "default".to_string() } else { user_id };

    match agent.get_onboarding_state(&tid, &uid).await {
        Ok(state) => Ok(Json(state)),
        Err(e) => {
            tracing::error!("Failed to get onboarding state: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

async fn save_state(
    State(agent): State<Arc<OnboardingAgent>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = auth_info.org_id.clone();
    let user_id = auth_info.agent_id.clone();

    let tid = if tenant_id.is_empty() { "default".to_string() } else { tenant_id };
    let uid = if user_id.is_empty() { "default".to_string() } else { user_id };

    let step = payload.get("wizardState")
        .and_then(|w| w.get("step"))
        .or_else(|| payload.get("step"))
        .and_then(|s| s.as_i64())
        .unwrap_or(0) as i32;

    match agent.save_onboarding_state(&tid, &uid, step, &payload).await {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Failed to save onboarding state: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}
