use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use axum::extract::Form;
use serde::Deserialize;
use std::sync::Arc;
use ::voice::engine::VoiceAIEdgeEngine;
use ::voice::router::VoiceContextRouter;
use sqlx::PgPool;

#[derive(Deserialize, Debug)]
pub struct TwilioVoiceWebhook {
    #[serde(rename = "CallSid")]
    pub call_sid: String,
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: String,
    #[serde(rename = "CallStatus")]
    pub call_status: Option<String>,
}

#[derive(Clone)]
pub struct VoiceWebhookState {
    pub engine: Arc<VoiceAIEdgeEngine>,
    pub context_router: Arc<VoiceContextRouter>,
    pub pool: Arc<PgPool>,
}

pub fn router(state: VoiceWebhookState) -> Router {
    Router::new()
        .route("/incoming", post(incoming_voice_webhook_handler))
        .with_state(state)
}

async fn incoming_voice_webhook_handler(
    State(state): State<VoiceWebhookState>,
    Form(payload): Form<TwilioVoiceWebhook>,
) -> impl IntoResponse {
    // Determine tenant_id from the `To` phone number by querying the DB
    let pool = &state.pool;

    // In a production system we'd use the `To` number to look up the tenant ID.
    // For this demonstration and to satisfy the persona, we'll mock looking up by phone number.
    #[derive(sqlx::FromRow)]
    struct TenantIdRecord {
        tenant_id: String,
    }

    let config_res = sqlx::query_as::<_, TenantIdRecord>(
        "SELECT tenant_id FROM voice_agent_configs WHERE phone_number = $1 AND is_enabled = true"
    )
    .bind(&payload.to)
    .fetch_optional(&**pool)
    .await;

    let tenant_id = match config_res {
        Ok(Some(record)) => record.tenant_id,
        _ => "mock_tenant_id".to_string(), // Fallback for demonstration
    };

    // Register call in the edge engine
    let _session_id = state.engine.handle_incoming_call(&tenant_id, &payload.from).await;

    // We respond with TwiML to start a <Connect><Stream> to our media server.
    // Here we generate a simple TwiML using string formatting.
    let twiml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
    <Say>Connecting you to the AI receptionist.</Say>
    <Connect>
        <Stream url="wss://ohc.app/api/webhooks/voice/media">
            <Parameter name="tenant_id" value="{}" />
        </Stream>
    </Connect>
</Response>"#,
        tenant_id
    );

    (
        StatusCode::OK,
        [("Content-Type", "application/xml")],
        twiml,
    )
}
