use axum::{
    extract::{Form, State},
    response::{IntoResponse, Response},
    http::StatusCode,
    routing::post,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::voice::engine::VoiceAIEdgeEngine;

#[derive(Deserialize, Debug)]
pub struct TwilioVoiceWebhookPayload {
    #[serde(rename = "From")]
    pub from: Option<String>,
    #[serde(rename = "To")]
    pub to: Option<String>,
}

#[derive(Clone)]
pub struct WebhookState {
    pub engine: Arc<VoiceAIEdgeEngine>,
}

pub fn router() -> Router {
    let state = WebhookState {
        engine: Arc::new(VoiceAIEdgeEngine::new()),
    };

    Router::new()
        .route("/incoming", post(handle_incoming_call))
        .with_state(state)
}

async fn handle_incoming_call(
    State(state): State<WebhookState>,
    Form(payload): Form<TwilioVoiceWebhookPayload>,
) -> impl IntoResponse {
    let caller_phone = payload.from.unwrap_or_else(|| "Unknown".to_string());

    // Minimal webhook implementation, mock Twilio signature verification
    let merchant_id = "merchant_default";

    let _session_id = state.engine.handle_incoming_call(merchant_id, &caller_phone).await;

    let twiml = "<Response><Say>AI Receptionist connected.</Say></Response>";

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/xml")
        .body(axum::body::Body::from(twiml))
        .unwrap()
}