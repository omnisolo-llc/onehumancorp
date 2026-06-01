use axum::{
    routing::{post, get},
    Router, Json, extract::State, response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ::voice::engine::VoiceAIEdgeEngine;
use std::sync::Mutex;

#[derive(Clone)]
pub struct VoiceWebhookState {
    pub engine: Arc<VoiceAIEdgeEngine>,
}

#[derive(Debug, Deserialize)]
pub struct TwilioIncomingCall {
    #[serde(rename = "CallSid")]
    pub call_sid: String,
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigUpdatePayload {
    pub is_enabled: bool,
    pub custom_instructions: String,
}

// Temporary in-memory config for testing
lazy_static::lazy_static! {
    static ref GLOBAL_VOICE_CONFIG: Mutex<ConfigUpdatePayload> = Mutex::new(ConfigUpdatePayload {
        is_enabled: false,
        custom_instructions: "".to_string(),
    });
}

pub async fn handle_incoming_voice(
    State(state): State<VoiceWebhookState>,
) -> impl IntoResponse {
    let session_id = state.engine.handle_incoming_call("merchant_id", "+1234567890").await;

    // Return standard TwiML connecting the call to our Voice Stream (mocked)
    let twiml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <Response>
            <Connect>
                <Stream url="wss://our-domain.com/api/webhooks/voice/stream/{}">
                    <Parameter name="hello" value="world"/>
                </Stream>
            </Connect>
        </Response>"#,
        session_id
    );

    ([(axum::http::header::CONTENT_TYPE, "text/xml")], twiml)
}

pub async fn get_config() -> impl IntoResponse {
    let config = GLOBAL_VOICE_CONFIG.lock().unwrap();
    (StatusCode::OK, Json(serde_json::json!({
        "is_enabled": config.is_enabled,
        "custom_instructions": config.custom_instructions,
    })))
}

pub async fn update_config(
    Json(payload): Json<ConfigUpdatePayload>,
) -> impl IntoResponse {
    let mut config = GLOBAL_VOICE_CONFIG.lock().unwrap();
    config.is_enabled = payload.is_enabled;
    config.custom_instructions = payload.custom_instructions;

    (StatusCode::OK, Json(serde_json::json!({"status": "success"})))
}

pub fn router(state: VoiceWebhookState) -> Router {
    Router::new()
        .route("/incoming", post(handle_incoming_voice))
        .route("/config", get(get_config).post(update_config))
        .with_state(state)
}
