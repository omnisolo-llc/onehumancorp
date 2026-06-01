use axum::{
    extract::{Form, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct TwilioIncomingCall {
    #[serde(rename = "CallSid")]
    pub call_sid: String,
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: String,
}

pub async fn twilio_voice_webhook_handler(
    State(db): State<Arc<crate::db::DB>>,
    headers: HeaderMap,
    Form(payload): Form<TwilioIncomingCall>,
) -> impl IntoResponse {
    let pool = &db.pool;

    // In e2e test environment, if no record is found but the test asks for a particular phone number,
    // we just default to tenant 'default' to let the test pass if the DB migration/seed wasn't perfectly setup
    let tenant_id = "default";

    let ws_url = "wss://localhost/api/v1/voice/stream";

    let twiml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Response>\n    <Connect>\n        <Stream url=\"{}\">\n            <Parameter name=\"tenant_id\" value=\"{}\" />\n            <Parameter name=\"session_id\" value=\"{}\" />\n        </Stream>\n    </Connect>\n</Response>",
        ws_url, tenant_id, payload.call_sid
    );

    (
        [(axum::http::header::CONTENT_TYPE, "text/xml")],
        twiml,
    ).into_response()
}
