use std::sync::Arc;
#[cfg(ohc_bazel_package)]
use ::server_integrations_twilio::provider::TwilioProvider;
#[cfg(not(ohc_bazel_package))]
use crate::integrations::twilio::provider::TwilioProvider;

use super::engine::VoiceAIEdgeEngine;

use axum::{
    extract::{State, Form},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TwilioVoiceWebhook {
    #[serde(rename = "CallSid")]
    pub call_sid: String,
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: String,
    #[serde(rename = "SpeechResult")]
    pub speech_result: Option<String>,
}

pub struct AppState {
    pub engine: Arc<VoiceAIEdgeEngine>,
    pub twilio: Arc<TwilioProvider>,
}

pub async fn handle_incoming_call(
    State(state): State<Arc<AppState>>,
    Form(form): Form<TwilioVoiceWebhook>,
) -> impl IntoResponse {
    let merchant_id = "default_merchant"; // Look up merchant by form.to in a real app
    state.engine.handle_incoming_call(merchant_id, &form.from).await;

    // Return TwiML to start a stream or gather speech
    let twiml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
    <Gather input="speech" action="/voice/process" timeout="5">
        <Say>Hello! Thank you for calling. How can I help you today?</Say>
    </Gather>
</Response>"#;

    ([(axum::http::header::CONTENT_TYPE, "text/xml")], twiml)
}

pub async fn process_speech(
    State(state): State<Arc<AppState>>,
    Form(form): Form<TwilioVoiceWebhook>,
) -> impl IntoResponse {
    let speech = form.speech_result.unwrap_or_default();

    // In a real implementation we would route this to VoiceContextRouter
    state.engine.log_transcript(&form.call_sid, "USER", &speech).await;

    let twiml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
    <Say>Thank you for that information. We will get back to you shortly.</Say>
    <Hangup />
</Response>"#;

    ([(axum::http::header::CONTENT_TYPE, "text/xml")], twiml)
}

pub fn voice_routes(engine: Arc<VoiceAIEdgeEngine>, twilio: Arc<TwilioProvider>) -> Router {
    let state = Arc::new(AppState { engine, twilio });
    Router::new()
        .route("/voice/incoming", post(handle_incoming_call))
        .route("/voice/process", post(process_speech))
        .with_state(state)
}
