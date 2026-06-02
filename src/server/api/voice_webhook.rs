use axum::{
    extract::{Form, State, Extension},
    response::IntoResponse,
};
use std::sync::Arc;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TwilioIncomingCallPayload {
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: String,
    #[serde(rename = "CallSid")]
    pub call_sid: String,
}

pub async fn voice_webhook_handler(
    State(hub): State<Arc<crate::hub::Hub>>,
    Form(payload): Form<TwilioIncomingCallPayload>,
) -> axum::response::Response {
    let merchant_phone = payload.to.clone();
    let caller_phone = payload.from.clone();

    // Look up the tenant based on the merchant phone number.
    let pool = crate::db::get_pool();
    let row = sqlx::query!("SELECT tenant_id FROM voice_agent_config WHERE phone_number = $1 AND is_enabled = true", merchant_phone)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

    let merchant_id = match row {
        Some(r) => r.tenant_id,
        None => "system".to_string(), // Fallback if no specific config
    };

    // For a real implementation, we would bridge to VoiceAIEdgeEngine
    let engine = Arc::new(crate::voice::VoiceAIEdgeEngine::new());
    let _session_id = engine.handle_incoming_call(&merchant_id, &caller_phone).await;

    // Twilio requires a TwiML response. For the purpose of starting the stream.
    let twiml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
        <Response>
            <Say>Connecting you to the AI Receptionist.</Say>
            <Connect>
                <Stream url=\"wss://{}/api/v1/webhooks/voice/stream\" />
            </Connect>
        </Response>",
        "api.onehumancorp.com" // Usually host is dynamic but mocked for webhook response
    );

    (
        axum::http::StatusCode::OK,
        [("content-type", "text/xml")],
        twiml,
    )
        .into_response()
}
