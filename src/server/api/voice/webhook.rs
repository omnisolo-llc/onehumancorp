use axum::{
    extract::{State, Form},
    response::{IntoResponse, Response},
    http::{StatusCode, header, HeaderMap},
    routing::post,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::db::get_pool;
use crate::voice::engine::VoiceAIEdgeEngine;
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct TwilioWebhookPayload {
    #[serde(rename = "CallSid")]
    pub call_sid: Option<String>,
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: String,
}

#[derive(FromRow)]
struct ConfigResult {
    tenant_id: String,
    is_enabled: bool,
}

pub fn router<S>(engine: Arc<VoiceAIEdgeEngine>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/incoming", post(handle_incoming_call))
        .with_state(engine)
}

async fn handle_incoming_call(
    State(engine): State<Arc<VoiceAIEdgeEngine>>,
    headers: HeaderMap,
    Form(payload): Form<TwilioWebhookPayload>,
) -> impl IntoResponse {
    let pool = get_pool();

    // Very basic Twilio signature check mock
    let _signature = headers.get("X-Twilio-Signature").and_then(|h| h.to_str().ok()).unwrap_or("");

    // Fetch tenant_id using the "To" phone number.
    let config = sqlx::query_as::<_, ConfigResult>(
        "SELECT tenant_id, is_enabled FROM voice_agent_configs WHERE phone_number = $1"
    )
    .bind(&payload.to)
    .fetch_optional(&pool)
    .await;

    if let Ok(Some(cfg)) = config {
        if cfg.is_enabled {
            // Start session with AI Voice Edge Engine (KAIROS state machine)
            let session_id = engine.handle_incoming_call(&cfg.tenant_id, &payload.from).await;

            let domain = std::env::var("APP_DOMAIN").unwrap_or_else(|_| "localhost:3000".to_string());
            let protocol = if domain.contains("localhost") { "ws" } else { "wss" };

            let twiml = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
                <Response>
                    <Connect>
                        <Stream url="{}://{}/api/v1/webhooks/voice/stream/{}" />
                    </Connect>
                </Response>"#,
                protocol, domain, session_id
            );

            return Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/xml")
                .body(axum::body::Body::from(twiml))
                .unwrap();
        }
    }

    // Fallback if AI receptionist is not enabled or not found
    let twiml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <Response>
            <Say>Please leave a message after the beep.</Say>
            <Record />
        </Response>"#;

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/xml")
        .body(axum::body::Body::from(twiml.to_string()))
        .unwrap()
}
