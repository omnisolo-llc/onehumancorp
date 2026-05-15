use axum::{
    extract::{State, Json, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::msgbus::{Message, Bus};
use serde_json::Value;

#[derive(Clone)]
pub struct WhatsappState {
    pub bus: Arc<dyn Bus>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyQuery {
    #[serde(rename = "hub.mode")]
    pub mode: String,
    #[serde(rename = "hub.challenge")]
    pub challenge: String,
    #[serde(rename = "hub.verify_token")]
    pub verify_token: String,
}

#[derive(Debug, Deserialize)]
pub struct WhatsappWebhookPayload {
    pub object: String,
    pub entry: Vec<WhatsappEntry>,
}

#[derive(Debug, Deserialize)]
pub struct WhatsappEntry {
    pub id: String,
    pub changes: Vec<WhatsappChange>,
}

#[derive(Debug, Deserialize)]
pub struct WhatsappChange {
    pub field: String,
    pub value: Value, // Complex value, containing messages
}

pub fn router<B: Bus + 'static>(bus: Arc<B>) -> Router<()> {
    let state = WhatsappState {
        bus: bus as Arc<dyn Bus>,
    };

    Router::new()
        .route("/webhook", get(verify_webhook))
        .route("/webhook", post(handle_webhook))
        .with_state(state)
}

async fn verify_webhook(Query(query): Query<VerifyQuery>) -> impl IntoResponse {
    let expected_token = std::env::var("WHATSAPP_VERIFY_TOKEN").unwrap_or_else(|_| "ohc_whatsapp_token".to_string());

    if query.mode == "subscribe" && query.verify_token == expected_token {
        (StatusCode::OK, query.challenge).into_response()
    } else {
        StatusCode::FORBIDDEN.into_response()
    }
}

async fn handle_webhook(
    State(state): State<WhatsappState>,
    Json(payload): Json<WhatsappWebhookPayload>,
) -> impl IntoResponse {
    if payload.object != "whatsapp_business_account" {
        return StatusCode::NOT_FOUND.into_response();
    }

    for entry in payload.entry {
        for change in entry.changes {
            if change.field == "messages" {
                if let Some(messages) = change.value.get("messages").and_then(|m| m.as_array()) {
                    for message in messages {
                        let from = message.get("from").and_then(|f| f.as_str()).unwrap_or_default();
                        let body = message.get("text")
                                          .and_then(|t| t.get("body"))
                                          .and_then(|b| b.as_str())
                                          .unwrap_or_default();

                        let msg_payload = serde_json::json!({
                            "source": "whatsapp",
                            "from": from,
                            "body": body,
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        });

                        let _ = state.bus.publish(Message {
                            topic: "tenant.message.received".to_string(),
                            payload: serde_json::to_vec(&msg_payload).unwrap_or_default(),
                        }).await;
                    }
                }
            }
        }
    }

    StatusCode::OK.into_response()
}
