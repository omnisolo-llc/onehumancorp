use axum::{extract::{State, Json}, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;
use crate::services::chat::service::ChatService;
use crate::db::DB;
use serde_json::json;

#[derive(Clone)]
pub struct ChatAppState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
pub struct DummyWebhookPayload {
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub channel: String,
    pub sender_type: String,
    pub content: String,
}

pub async fn handle_dummy_webhook(
    State(state): State<ChatAppState>,
    Json(payload): Json<DummyWebhookPayload>,
) -> Result<StatusCode, StatusCode> {
    let tenant_id = Uuid::parse_str(&payload.tenant_id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let inbox_id = Uuid::parse_str(&payload.inbox_id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let contact_id = Uuid::parse_str(&payload.contact_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let chat_service = ChatService::new(state.db.pool.clone());

    let conversation = chat_service.start_conversation(
        tenant_id,
        inbox_id,
        contact_id,
        None
    ).await.map_err(|e| {
        tracing::error!("Failed to start conversation: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let message = chat_service.send_message(
        tenant_id,
        conversation.id,
        payload.sender_type,
        None,
        payload.content.clone(),
    ).await.map_err(|e| {
        tracing::error!("Failed to send message: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Publish to redis so websocket gets it
    if let Some(mut redis_client) = crate::redis_pool::get_redis_client() {
        let topic = format!("unified:sync:tenant-{}:chat_messages", payload.tenant_id);
        let event_payload = json!({
            "action": "new_message",
            "message_id": message.id.to_string(),
            "conversation_id": conversation.id.to_string(),
            "content": payload.content,
        }).to_string();

        let _: Result<(), _> = redis::cmd("PUBLISH")
            .arg(topic)
            .arg(event_payload)
            .query_async(&mut redis_client).await;
    }

    Ok(StatusCode::OK)
}
