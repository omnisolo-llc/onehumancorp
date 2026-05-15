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
pub struct MetaState {
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
pub struct MetaWebhookPayload {
    pub object: String,
    pub entry: Vec<MetaEntry>,
}

#[derive(Debug, Deserialize)]
pub struct MetaEntry {
    pub id: String,
    pub time: i64,
    pub messaging: Option<Vec<MetaMessaging>>,
}

#[derive(Debug, Deserialize)]
pub struct MetaMessaging {
    pub sender: MetaId,
    pub recipient: MetaId,
    pub message: Option<MetaMessageData>,
}

#[derive(Debug, Deserialize)]
pub struct MetaId {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct MetaMessageData {
    pub mid: String,
    pub text: Option<String>,
}

pub fn router<B: Bus + 'static>(bus: Arc<B>) -> Router<()> {
    let state = MetaState {
        bus: bus as Arc<dyn Bus>,
    };

    Router::new()
        .route("/webhook", get(verify_webhook))
        .route("/webhook", post(handle_webhook))
        .with_state(state)
}

async fn verify_webhook(Query(query): Query<VerifyQuery>) -> impl IntoResponse {
    let expected_token = std::env::var("META_VERIFY_TOKEN").unwrap_or_else(|_| "ohc_meta_token".to_string());

    if query.mode == "subscribe" && query.verify_token == expected_token {
        (StatusCode::OK, query.challenge).into_response()
    } else {
        StatusCode::FORBIDDEN.into_response()
    }
}

async fn handle_webhook(
    State(state): State<MetaState>,
    Json(payload): Json<MetaWebhookPayload>,
) -> impl IntoResponse {
    if payload.object != "page" && payload.object != "instagram" {
        return StatusCode::NOT_FOUND.into_response();
    }

    for entry in payload.entry {
        if let Some(messaging) = entry.messaging {
            for event in messaging {
                if let Some(message) = event.message {
                    if let Some(text) = message.text {
                        let msg_payload = serde_json::json!({
                            "source": payload.object,
                            "from": event.sender.id,
                            "to": event.recipient.id,
                            "body": text,
                            "timestamp": entry.time
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
