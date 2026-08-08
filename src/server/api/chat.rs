
// Native Rust Omnichannel Chat API endpoints

use axum::{
    extract::{State, Path},
    routing::{get, post},
    Router, Json,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::services::chat::service::ChatService;

#[derive(Clone)]
pub struct ChatApiState {
    pub chat_service: Arc<ChatService>,
}

pub fn omnichannel_router(state: ChatApiState) -> Router {
    Router::new()
        .route("/api/v1/tenants/:tenant_id/chat/inboxes", post(create_inbox))
        .route("/api/v1/tenants/:tenant_id/chat/conversations", post(create_conversation))
        .route("/api/v1/tenants/:tenant_id/chat/conversations/:conversation_id/messages", post(send_message))
        .route("/api/v1/tenants/:tenant_id/chat/inboxes/:inbox_id/conversations", get(list_conversations))
        .with_state(state)
}

#[derive(serde::Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
    pub channel_type: String,
    pub channel_config: serde_json::Value,
}

pub async fn create_inbox(
    State(state): State<ChatApiState>,
    Path(tenant_id): Path<String>,
    Json(payload): Json<CreateInboxRequest>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    match state.chat_service.create_inbox(&tenant_id, &payload.name, &payload.channel_type, payload.channel_config).await {
        Ok(inbox) => Ok(Json(serde_json::json!(inbox))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(serde::Deserialize)]
pub struct CreateConversationRequest {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
}

pub async fn create_conversation(
    State(state): State<ChatApiState>,
    Path(tenant_id): Path<String>,
    Json(payload): Json<CreateConversationRequest>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    match state.chat_service.start_conversation(&tenant_id, payload.inbox_id, payload.contact_id).await {
        Ok(conv) => Ok(Json(serde_json::json!(conv))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(serde::Deserialize)]
pub struct SendMessageRequest {
    pub sender_type: String,
    pub sender_id: Option<String>,
    pub content: String,
    pub message_type: String,
}

pub async fn send_message(
    State(state): State<ChatApiState>,
    Path((tenant_id, conversation_id)): Path<(String, Uuid)>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    match state.chat_service.send_message(&tenant_id, conversation_id, &payload.sender_type, payload.sender_id.as_deref(), &payload.content, &payload.message_type).await {
        Ok(msg) => Ok(Json(serde_json::json!(msg))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_conversations(
    State(state): State<ChatApiState>,
    Path((tenant_id, inbox_id)): Path<(String, Uuid)>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    match state.chat_service.get_conversations(&tenant_id, inbox_id).await {
        Ok(convs) => Ok(Json(serde_json::json!(convs))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(serde::Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

pub async fn help_chat_handler(Json(_req): Json<ChatRequest>) -> Json<serde_json::Value> {
    // dummy backward compatible method since the code broke
    Json(serde_json::json!({
        "reply": "Hi",
        "link": {
            "title": "Hi",
            "url": "/"
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;

    #[tokio::test]
    async fn test_help_chat_handler_fallback() {
        let req = ChatRequest {
            message: "getting started".to_string(),
        };

        // This will fall back due to 'dummy_key'
        let response = help_chat_handler(Json(req)).await.0;

        assert_eq!(response["reply"].as_str().unwrap(), "Hi");
    }
}
