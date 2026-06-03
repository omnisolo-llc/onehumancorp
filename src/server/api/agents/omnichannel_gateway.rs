use axum::{
    extract::{State, Json, Request},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    middleware::{self, Next},
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;

#[derive(Deserialize)]
pub struct OmnichannelPayload {
    pub tenant_id: String,
    pub platform: String,
    pub original_message: String,
    #[serde(default)]
    pub attachments: Vec<AttachmentPayload>,
}

#[derive(Deserialize, Default)]
pub struct AttachmentPayload {
    pub media_type: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct GatewayResponse {
    pub success: bool,
    pub message_id: Option<String>,
}

// Simple signature validation middleware to ensure we aren't blinding trusting incoming webhooks
async fn validate_omnichannel_signature(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let headers = req.headers();

    // In a real production system, this would check HMAC signatures against the platform secret (e.g. Meta Graph API secret).
    // For this implementation scope, we verify the presence of a signature header that our test/system will send.
    if !headers.contains_key("X-OHC-Omni-Signature") {
        tracing::warn!("Omnichannel webhook rejected: Missing signature");

        // Ensure this ONLY works if compiled in debug AND explicitly set via env var
        #[cfg(debug_assertions)]
        {
            if std::env::var("ENABLE_TEST_AUTH_BYPASS").unwrap_or_default() == "true" {
                if headers.contains_key("X-Test-Bypass") {
                    tracing::info!("Bypassing signature validation for test environment");
                    return Ok(next.run(req).await);
                }
            }
        }

        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/receive", post(handle_incoming_message))
        .route_layer(middleware::from_fn(validate_omnichannel_signature))
        .with_state(orchestrator)
}

async fn handle_incoming_message(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Json(payload): Json<OmnichannelPayload>,
) -> impl IntoResponse {
    let event_id = uuid::Uuid::new_v4().to_string();

    // Normalize to the unified "tenant.message.received" event format for the CSAgent
    let event = DepartmentEvent {
        id: event_id.clone(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.message.received".to_string(),
        payload: serde_json::json!({
            "platform": payload.platform,
            "message": payload.original_message,
            "attachments": payload.attachments,
        }),
    };

    match orchestrator.dispatch_event(event).await {
        Ok(_) => (StatusCode::OK, Json(GatewayResponse { success: true, message_id: Some(event_id) })).into_response(),
        Err(e) => {
            tracing::error!("Omnichannel gateway dispatch failed: {}", e);
            if e.contains("AI Budget exhausted") {
                (StatusCode::TOO_MANY_REQUESTS, Json(GatewayResponse { success: false, message_id: None })).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(GatewayResponse { success: false, message_id: None })).into_response()
            }
        }
    }
}
