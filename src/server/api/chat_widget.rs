use axum::{
    extract::{Path, Query, State, Extension},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatChannel, ChatContact, ChatConversation, ChatMessage};
use serde_json::Value;

pub fn router(chat_service: Arc<ChatService>) -> Router {
    Router::new()
        .route("/config", get(get_widget_config))
        .route("/contact", post(init_contact))
        .route("/conversations/:conversation_id/messages", get(get_messages))
        .route("/conversations/:conversation_id/messages", post(send_message))
        .with_state(chat_service)
}

#[derive(Deserialize)]
pub struct ConfigQuery {
    pub inbox_id: Uuid,
}

pub async fn get_widget_config(
    State(chat_service): State<Arc<ChatService>>,
    Query(query): Query<ConfigQuery>,
) -> impl IntoResponse {
    match chat_service.get_channel_by_inbox_id(query.inbox_id).await {
        Ok(channel) => (StatusCode::OK, Json(channel)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Config not found"}))).into_response(),
    }
}

#[derive(Deserialize)]
pub struct InitContactReq {
    pub inbox_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize)]
pub struct InitContactRes {
    pub contact: ChatContact,
    pub conversation: ChatConversation,
}

pub async fn init_contact(
    State(chat_service): State<Arc<ChatService>>,
    Json(req): Json<InitContactReq>,
) -> impl IntoResponse {
    // Determine tenant_id from channel
    let channel = match chat_service.get_channel_by_inbox_id(req.inbox_id).await {
        Ok(c) => c,
        Err(_) => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Inbox not found"}))).into_response(),
    };

    let tenant_id = channel.tenant_id;

    let contact = match chat_service.create_contact(tenant_id, req.name, req.email, req.phone).await {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };

    let conversation = match chat_service.start_conversation(tenant_id, req.inbox_id, contact.id, None).await {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };

    (StatusCode::OK, Json(InitContactRes { contact, conversation })).into_response()
}

pub async fn get_messages(
    State(chat_service): State<Arc<ChatService>>,
    Path(conversation_id): Path<Uuid>,
) -> impl IntoResponse {
    match chat_service.get_messages_by_conversation_id(conversation_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to fetch messages"}))).into_response(),
    }
}

#[derive(Deserialize)]
pub struct SendMessageReq {
    pub content: String,
    pub sender_id: Option<Uuid>,
}

pub async fn send_message(
    State(chat_service): State<Arc<ChatService>>,
    Path(conversation_id): Path<Uuid>,
    Json(req): Json<SendMessageReq>,
) -> impl IntoResponse {
    // Need tenant_id. We fetch conversation first.
    let conversation = match chat_service.get_conversation(conversation_id).await {
        Ok(c) => c,
        Err(_) => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Conversation not found"}))).into_response(),
    };

    match chat_service.send_message(
        conversation.tenant_id,
        conversation_id,
        "contact".to_string(), // In context of widget, sender is contact
        req.sender_id,
        req.content,
    ).await {
        Ok(msg) => (StatusCode::OK, Json(msg)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Unit tests would go here, requiring a mocked or real DB pool for ChatService.
    // For scope, we will add E2E tests later.
}

#[cfg(test)]
mod actual_tests {
    use super::*;
    // Mock testing framework setup goes here if we had access to proper mocking for sqlx
    // In typical axum router tests, we'd spawn the router and use hyper to issue requests
}
