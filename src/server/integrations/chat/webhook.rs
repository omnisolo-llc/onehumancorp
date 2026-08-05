use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::models::{WebhookPayload, Message};

#[derive(Deserialize)]
pub struct WebhookVerifyQuery {
    #[serde(rename = "hub.mode")]
    pub mode: Option<String>,
    #[serde(rename = "hub.verify_token")]
    pub verify_token: Option<String>,
    #[serde(rename = "hub.challenge")]
    pub challenge: Option<String>,
}

pub struct ChatState {
    pub verify_token: String,
    // Channel for emitting events to the event mesh/agent
    pub event_tx: mpsc::Sender<Message>,
}

pub async fn verify_webhook(
    State(state): State<Arc<ChatState>>,
    Query(query): Query<WebhookVerifyQuery>,
) -> impl IntoResponse {
    if let (Some(mode), Some(token), Some(challenge)) = (query.mode, query.verify_token, query.challenge) {
        if mode == "subscribe" && token == state.verify_token {
            return (StatusCode::OK, challenge).into_response();
        }
    }
    (StatusCode::FORBIDDEN, "Verification failed".to_string()).into_response()
}

pub async fn handle_webhook(
    State(state): State<Arc<ChatState>>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    // Basic processing of the webhook
    if payload.object == "whatsapp_business_account" {
        for entry in payload.entry {
            for change in entry.changes {
                if let Some(messages) = change.value.messages {
                    for msg in messages {
                        if let Some(text) = msg.text {
                            let chat_message = Message {
                                id: uuid::Uuid::new_v4().to_string(),
                                tenant_id: "default_tenant".to_string(), // In reality, map phone_number_id to tenant
                                conversation_id: msg.from.clone(), // In reality, find or create conversation
                                sender_type: "contact".to_string(),
                                sender_id: Some(msg.from),
                                content: text.body,
                                created_at: chrono::Utc::now(),
                            };
                            let _ = state.event_tx.send(chat_message).await;
                        }
                    }
                }
            }
        }
        return StatusCode::OK;
    }
    StatusCode::NOT_FOUND
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;
    use axum::Json;
    use crate::models::{WebhookPayload, WebhookEntry, WebhookChange, WebhookValue, WebhookMessage, WebhookText};

    #[tokio::test]
    async fn test_handle_webhook_parses_whatsapp_message() {
        let (tx, mut rx) = mpsc::channel(100);
        let state = Arc::new(ChatState {
            verify_token: "test".to_string(),
            event_tx: tx,
        });

        let payload = WebhookPayload {
            object: "whatsapp_business_account".to_string(),
            entry: vec![WebhookEntry {
                id: "123".to_string(),
                changes: vec![WebhookChange {
                    field: "messages".to_string(),
                    value: WebhookValue {
                        messaging_product: "whatsapp".to_string(),
                        metadata: crate::models::WebhookMetadata {
                            display_phone_number: "123".to_string(),
                            phone_number_id: "456".to_string(),
                        },
                        contacts: None,
                        messages: Some(vec![WebhookMessage {
                            from: "user1".to_string(),
                            id: "msg1".to_string(),
                            timestamp: "0".to_string(),
                            message_type: "text".to_string(),
                            text: Some(WebhookText {
                                body: "Hello OHC!".to_string(),
                            }),
                        }]),
                    },
                }],
            }],
        };

        handle_webhook(State(state), Json(payload)).await;

        let msg = rx.recv().await.expect("Expected a message to be sent to the channel");
        assert_eq!(msg.content, "Hello OHC!");
        assert_eq!(msg.conversation_id, "user1");
        assert_eq!(msg.sender_type, "contact");
    }
}
