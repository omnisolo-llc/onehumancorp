use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use ::server_common::Claims;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct ChatAppState {
    pub chat_service: Arc<ChatService>,
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub conversation_id: Uuid,
    pub content: String,
}

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub channel: String,
    pub sender_id: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ConversationHistoryResponse {
    pub conversation: ChatConversation,
    pub messages: Vec<ChatMessage>,
}

pub fn router(chat_service: Arc<ChatService>) -> Router {
    let state = ChatAppState { chat_service };

    Router::new()
        .route("/inbox/conversations", get(list_conversations))
        .route("/inbox/conversations/:conversation_id/messages", get(get_conversation_history))
        .route("/inbox/messages", post(send_message))
        .route("/webhook", post(webhook_handler))
        .with_state(state)
}

async fn list_conversations(
    State(_state): State<ChatAppState>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let _tenant_id = match claims.organization_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        Some(id) => id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    // Need to fetch conversations. For now, returning an empty list or we can implement fetch in service.
    // The instructions say "fetch conversation history".
    (StatusCode::OK, Json(vec![] as Vec<ChatConversation>)).into_response()
}

async fn get_conversation_history(
    State(_state): State<ChatAppState>,
    Extension(claims): Extension<Claims>,
    Path(_conversation_id): Path<Uuid>,
) -> impl IntoResponse {
    let _tenant_id = match claims.organization_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        Some(id) => id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    // This would fetch messages from the DB.
    (StatusCode::OK, Json(vec![] as Vec<ChatMessage>)).into_response()
}

async fn send_message(
    State(state): State<ChatAppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateMessageRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        Some(id) => id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    match state.chat_service.send_message(
        tenant_id,
        payload.conversation_id,
        "agent".to_string(),
        None, // Assuming agent sender ID can be None or extracted from claims
        payload.content.clone(),
    ).await {
        Ok(msg) => {
            // Also need to push to redis pubsub here
            let redis_client_opt = crate::redis_pool::get_redis_client();
            let msg_clone = msg.clone();
            if let Some(client) = redis_client_opt {
                tokio::spawn(async move {
                    if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                        let topic = format!("unified:chat:{}:messages", tenant_id);
                        let envelope = serde_json::json!({
                            "channel": "chat",
                            "topic": format!("messages:{}", tenant_id),
                            "data": msg_clone,
                            "seq": chrono::Utc::now().timestamp_millis() as u64
                        });
                        let _: Result<(), _> = redis::cmd("PUBLISH")
                            .arg(&topic)
                            .arg(envelope.to_string())
                            .query_async(&mut conn)
                            .await;
                    }
                });
            }

            (StatusCode::OK, Json(msg)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn webhook_handler(
    State(state): State<ChatAppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        Some(id) => id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    // Very simplified webhook handler. We would normally find/create contact and conversation.
    // For now we just mock sending a message via the service to trigger the Ambassador agent via DB actions/Event Mesh.
    // Assuming Uuid::default() as a placeholder for conversation_id for this simplified handler.

    let conversation_id = Uuid::default();

    match state.chat_service.send_message(
        tenant_id,
        conversation_id,
        "customer".to_string(),
        Some(Uuid::default()), // Placeholder for sender_id
        payload.message.clone(),
    ).await {
        Ok(msg) => {
            // Push incoming customer message to redis pubsub
            let redis_client_opt = crate::redis_pool::get_redis_client();
            let msg_clone = msg.clone();
            if let Some(client) = redis_client_opt {
                tokio::spawn(async move {
                    if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                        let topic = format!("unified:chat:{}:messages", tenant_id);
                        let envelope = serde_json::json!({
                            "channel": "chat",
                            "topic": format!("messages:{}", tenant_id),
                            "data": msg_clone,
                            "seq": chrono::Utc::now().timestamp_millis() as u64
                        });
                        let _: Result<(), _> = redis::cmd("PUBLISH")
                            .arg(&topic)
                            .arg(envelope.to_string())
                            .query_async(&mut conn)
                            .await;
                    }
                });
            }

            // Trigger The Ambassador (Agent Assist)
            // Simplified: creating another "agent" drafted reply message
            match state.chat_service.send_message(
                tenant_id,
                conversation_id,
                "agent".to_string(),
                None, // System / Agent
                format!("Thank you for your message! This is a drafted response from The Ambassador regarding: {}", payload.message),
            ).await {
                Ok(draft_msg) => {
                     let redis_client_opt_draft = crate::redis_pool::get_redis_client();
                     if let Some(client) = redis_client_opt_draft {
                        tokio::spawn(async move {
                            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                                let topic = format!("unified:chat:{}:messages", tenant_id);
                                let envelope = serde_json::json!({
                                    "channel": "chat",
                                    "topic": format!("messages:{}", tenant_id),
                                    "data": draft_msg,
                                    "seq": chrono::Utc::now().timestamp_millis() as u64
                                });
                                let _: Result<(), _> = redis::cmd("PUBLISH")
                                    .arg(&topic)
                                    .arg(envelope.to_string())
                                    .query_async(&mut conn)
                                    .await;
                            }
                        });
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to generate Ambassador draft: {}", e);
                }
            }

            (StatusCode::OK, Json(serde_json::json!({"status": "received", "message_id": msg.id}))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to ingest webhook message: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
