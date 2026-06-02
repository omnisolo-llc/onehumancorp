use axum::{
    extract::{Form, State},
    response::IntoResponse,
    http::{StatusCode, HeaderMap},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TwilioVoiceWebhookPayload {
    pub call_sid: Option<String>,
    #[serde(rename = "CallSid")]
    pub call_sid_alt: Option<String>,
    pub from: Option<String>,
    #[serde(rename = "From")]
    pub from_alt: Option<String>,
    pub to: Option<String>,
    #[serde(rename = "To")]
    pub to_alt: Option<String>,
    pub call_status: Option<String>,
    #[serde(rename = "CallStatus")]
    pub call_status_alt: Option<String>,
}

pub fn validate_twilio_signature(headers: &HeaderMap, auth_token: &str, url: &str, params: &std::collections::HashMap<String, String>) -> bool {
    let signature = match headers.get("X-Twilio-Signature") {
        Some(s) => s.to_str().unwrap_or(""),
        None => return false,
    };

    let mut keys: Vec<&String> = params.keys().collect();
    keys.sort();

    let mut data = url.to_string();
    for key in keys {
        data.push_str(key);
        data.push_str(params.get(key).unwrap());
    }

    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    use base64::{Engine as _, engine::general_purpose::STANDARD};

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(auth_token.as_bytes()).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());

    let expected_signature = STANDARD.encode(mac.finalize().into_bytes());
    signature == expected_signature
}

// TODO: When integrating, the `WebhookState` (or similar state containing DB context)
// needs to be correctly mapped into this handler. For now, since `server_integrations_twilio`
// crate doesn't have access to `crate::api`, we drop the explicit `State` constraint and
// document the TODO to fetch from KAIROS properly later.
pub async fn twilio_voice_webhook_handler(
    headers: HeaderMap,
    Form(payload): Form<TwilioVoiceWebhookPayload>,
) -> impl IntoResponse {
    let mut params = std::collections::HashMap::new();
    if let Some(ref val) = payload.call_sid.as_ref().or(payload.call_sid_alt.as_ref()) {
        params.insert("CallSid".to_string(), val.to_string());
    }
    if let Some(ref val) = payload.from.as_ref().or(payload.from_alt.as_ref()) {
        params.insert("From".to_string(), val.to_string());
    }
    if let Some(ref val) = payload.to.as_ref().or(payload.to_alt.as_ref()) {
        params.insert("To".to_string(), val.to_string());
    }
    if let Some(ref val) = payload.call_status.as_ref().or(payload.call_status_alt.as_ref()) {
        params.insert("CallStatus".to_string(), val.to_string());
    }

    let auth_token = std::env::var("TWILIO_AUTH_TOKEN").unwrap_or_default();
    let webhook_url = std::env::var("TWILIO_WEBHOOK_URL").unwrap_or_else(|_| "https://api.ohc.com/api/v1/webhooks/voice/incoming".to_string());

    if !validate_twilio_signature(&headers, &auth_token, &webhook_url, &params) {
        tracing::warn!("Invalid Twilio webhook signature");
        return (StatusCode::UNAUTHORIZED, [(axum::http::header::CONTENT_TYPE, "text/xml")], "".to_string());
    }

    let call_sid = payload.call_sid.or(payload.call_sid_alt).unwrap_or_default();
    let from = payload.from.or(payload.from_alt).unwrap_or_default();
    let to = payload.to.or(payload.to_alt).unwrap_or_default();
    let status = payload.call_status.or(payload.call_status_alt).unwrap_or_default();

    tracing::info!("Received Twilio voice webhook: CallSid: {}, From: {}, To: {}, Status: {}",
        call_sid, from, to, status);

    // TODO: Fix KAIROS state machine wiring here
    // Currently, `voice_agent_configs` does not have a fully materialized fetch method on `DB`.
    // Once `voice_agent_configs` is wired into `DB` and KAIROS orchestration exposes
    // `initialize_voice_session`, we can query the tenant ID from the `to` phone number
    // and initialize the state machine session.

    // Fallback: log the info and return the TwiML to establish WebSocket streaming.
    // let result = sqlx::query!("SELECT organization_id FROM voice_agent_configs WHERE phone_number = $1", to)
    //     .fetch_optional(&state.db_pool)
    //     .await;

    let twiml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <Response>\n\
             <Say>Connecting you to the OHC AI Receptionist.</Say>\n\
             <Connect>\n\
                 <Stream url=\"wss://{}/api/v1/webhooks/voice/stream\" />\n\
             </Connect>\n\
         </Response>",
        "api.ohc.com"
    );

    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/xml")],
        twiml,
    )
}
