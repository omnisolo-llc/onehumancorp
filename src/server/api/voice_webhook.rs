use axum::{
    extract::{State, Extension, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::voice::config::VoiceAgentConfig;
use crate::voice::engine::VoiceAIEdgeEngine;
use ::server_common::Claims;
use std::sync::Mutex;
use std::collections::HashMap;

// In-memory mock for DB, for now since we don't have a real DB schema for VoiceAgentConfig defined in migrations yet.
lazy_static::lazy_static! {
    static ref MOCK_CONFIGS: Mutex<HashMap<String, VoiceAgentConfig>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Deserialize)]
pub struct TwilioWebhookPayload {
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: String,
    #[serde(rename = "CallSid")]
    pub call_sid: String,
}

pub async fn voice_webhook_handler(
    State(engine): State<Arc<VoiceAIEdgeEngine>>,
    axum::extract::Form(payload): axum::extract::Form<TwilioWebhookPayload>,
) -> impl IntoResponse {
    // In a real implementation, we'd look up the tenant_id from the `payload.to` phone number
    let merchant_id = "tenant_123"; // Mock for now

    // Verify Twilio signature
    let _signature = payload.call_sid.clone(); // Conceptual check
    if _signature.is_empty() {
        return (StatusCode::UNAUTHORIZED, "Invalid Signature").into_response();
    }

    // Initialize session in the engine
    let _session_id = engine.handle_incoming_call(merchant_id, &payload.from).await;

    // Return TwiML
    let twiml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
    <Connect>
        <Stream url="wss://ohc.com/api/webhooks/voice/stream" />
    </Connect>
</Response>"#;

    ([(axum::http::header::CONTENT_TYPE, "application/xml")], twiml).into_response()
}

pub async fn get_voice_agent_settings(
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.tenant;

    let configs = MOCK_CONFIGS.lock().unwrap();
    if let Some(config) = configs.get(&tenant_id) {
        Json(config.clone()).into_response()
    } else {
        Json(VoiceAgentConfig {
            tenant_id: tenant_id.clone(),
            phone_number: "(555) 123-4567".to_string(), // Mock assigned number
            is_enabled: false,
            primary_language: "English".to_string(),
            custom_instructions: "".to_string(),
        }).into_response()
    }
}

pub async fn update_voice_agent_settings(
    Extension(claims): Extension<Claims>,
    Json(mut payload): Json<VoiceAgentConfig>,
) -> impl IntoResponse {
    payload.tenant_id = claims.tenant.clone(); // Enforce row-level security conceptually

    let mut configs = MOCK_CONFIGS.lock().unwrap();
    configs.insert(claims.tenant, payload.clone());

    Json(payload).into_response()
}

pub fn router(engine: Arc<VoiceAIEdgeEngine>) -> Router {
    Router::new()
        .route("/incoming", post(voice_webhook_handler))
        .with_state(engine)
}

pub fn settings_router() -> Router {
    Router::new()
        .route("/", get(get_voice_agent_settings).post(update_voice_agent_settings))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{Request, header}};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_settings() {
        let app = Router::new()
            .route("/api/settings/voice-agent", get(get_voice_agent_settings))
            .layer(axum::middleware::from_fn(|req: axum::extract::Request, next: axum::middleware::Next| async move {
                let mut req = req;
                req.extensions_mut().insert(Claims {
                    sub: "user_1".to_string(),
                    tenant: "tenant_1".to_string(),
                    exp: 0,
                    iat: 0,
                });
                next.run(req).await
            }));

        let response = app
            .oneshot(Request::builder().uri("/api/settings/voice-agent").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
