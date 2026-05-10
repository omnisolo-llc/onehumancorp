use axum::{
    extract::{Path, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde_json::{Value, json};
use std::sync::Arc;
use crate::integrations::registry::IntegrationsRegistry;
use crate::hub::Hub;
use crate::ohc::orchestration::Message;
use chrono::Utc;

pub fn router<S: Clone + Send + Sync + 'static>(registry: Arc<IntegrationsRegistry>, hub: Arc<Hub>) -> Router<S> {
    let registry_oauth = registry.clone();
    let registry_webhook = registry.clone();
    let hub_clone = hub.clone();

    Router::new()
        .route("/:integration_id/oauth", get(move |Path(integration_id): Path<String>| async move {
            let redirect_uri = "https://app.onehumancorp.com/integrations/callback";
            match registry_oauth.initiate_oauth(&integration_id, redirect_uri) {
                Ok(url) => (StatusCode::OK, Json(json!({ "url": url }))).into_response(),
                Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "error": e }))).into_response(),
            }
        }))
        .route("/:integration_id/webhook", post(move |Path(integration_id): Path<String>, Json(payload): Json<Value>| async move {
            let payload_str = payload.to_string();

            // Optionally persist the webhook data to the database based on integration type
            if integration_id == "manychat" {
                if let Err(e) = hub_clone.publish(
                    Message {
                        id: format!("msg-webhook-{}", Utc::now().timestamp()),
                        from_agent: "customer_via_manychat".to_string(),
                        to_agent: "manychat_agent".to_string(),
                        content: payload_str.clone(),
                        occurred_at_unix: Utc::now().timestamp(),
                        meeting_id: "manychat_inbox".to_string(),
                        r#type: "webhook".to_string(),
                    }
                ) {
                    tracing::error!("Failed to broadcast Manychat webhook event to mesh: {}", e);
                }
            } else if integration_id == "calendly" {
                if let Err(e) = hub_clone.publish(
                    Message {
                        id: format!("msg-webhook-{}", Utc::now().timestamp()),
                        from_agent: "system".to_string(),
                        to_agent: "calendly_agent".to_string(),
                        content: "Calendly Booking Received".to_string(),
                        occurred_at_unix: Utc::now().timestamp(),
                        meeting_id: "calendly_inbox".to_string(),
                        r#type: "webhook".to_string(),
                    }
                ) {
                    tracing::error!("Failed to broadcast Calendly webhook event to mesh: {}", e);
                }
            }

            match registry_webhook.handle_webhook(&integration_id, &payload_str) {
                Ok(_) => (StatusCode::OK, Json(json!({ "status": "success" }))).into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e }))).into_response(),
            }
        }))
}
