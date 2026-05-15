use axum::{
    extract::{State, Json},
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use crate::services::onboarding::onboarding_agent::OnboardingAgent;
use crate::services::onboarding::conversational::{ChatSession, ConversationalOnboardingService};
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};

#[derive(Clone)]
pub struct OnboardingStateWrapper {
    pub agent: Arc<OnboardingAgent>,
    pub conversational_service: Arc<ConversationalOnboardingService>,
}

pub fn router(agent: Arc<OnboardingAgent>) -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let conversational_service = Arc::new(ConversationalOnboardingService::new());
    let state = OnboardingStateWrapper {
        agent,
        conversational_service,
    };

    let r = Router::new()
        .route("/start", post(start_onboarding))
        .route("/state", get(get_state))
        .route("/state", post(save_state))
        .route("/chat/start", post(start_chat))
        .route("/chat/message", post(handle_chat_message))
        .with_state(state);

    // Convert to accept MeshTransport state
    Router::new().merge(r)
}

async fn start_onboarding(
    State(state): State<OnboardingStateWrapper>,
    Json(payload): Json<StartOnboardingRequest>,
) -> Result<Json<StartOnboardingResponse>, axum::http::StatusCode> {
    match state.agent.start_onboarding(payload).await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_state(
    State(_state): State<OnboardingStateWrapper>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    Ok(Json(serde_json::json!({
        "state": "{}"
    })))
}

async fn save_state(
    State(_state): State<OnboardingStateWrapper>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn start_chat(
    State(state): State<OnboardingStateWrapper>,
) -> Result<Json<ChatSession>, axum::http::StatusCode> {
    let session = state.conversational_service.start_session();
    Ok(Json(session))
}

#[derive(serde::Deserialize)]
pub struct ChatMessageRequest {
    pub session: ChatSession,
    pub message: String,
}

async fn handle_chat_message(
    State(state): State<OnboardingStateWrapper>,
    Json(payload): Json<ChatMessageRequest>,
) -> Result<Json<ChatSession>, axum::http::StatusCode> {
    let new_session = state.conversational_service.handle_message(payload.session, &payload.message);
    Ok(Json(new_session))
}
