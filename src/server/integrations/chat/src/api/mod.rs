use crate::models::{ChannelType, Conversation, Inbox, Message, MessageType};
use crate::services::ChatService;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

pub struct AppState {
    pub chat_service: Arc<ChatService>,
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
    pub channel_type: ChannelType,
}

pub async fn create_inbox(
    State(state): State<Arc<AppState>>,
    // TODO: Extract tenant_id from auth context
    Json(payload): Json<CreateInboxRequest>,
) -> (StatusCode, Json<Inbox>) {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID
    let inbox = state.chat_service.create_inbox(tenant_id, payload.name, payload.channel_type).await;
    (StatusCode::CREATED, Json(inbox))
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub contact_id: Uuid,
}

pub async fn create_conversation(
    State(state): State<Arc<AppState>>,
    Path(inbox_id): Path<Uuid>,
    Json(payload): Json<CreateConversationRequest>,
) -> (StatusCode, Json<Conversation>) {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID
    let conversation = state.chat_service.create_conversation(tenant_id, inbox_id, payload.contact_id).await;
    (StatusCode::CREATED, Json(conversation))
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub content: String,
    pub message_type: MessageType,
    pub sender_id: Option<Uuid>,
}

pub async fn create_message(
    State(state): State<Arc<AppState>>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<CreateMessageRequest>,
) -> (StatusCode, Json<Message>) {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID
    let message = state.chat_service.create_message(
        tenant_id,
        conversation_id,
        payload.sender_id,
        payload.content,
        payload.message_type,
    ).await;
    (StatusCode::CREATED, Json(message))
}

pub async fn get_messages(
    State(state): State<Arc<AppState>>,
    Path(conversation_id): Path<Uuid>,
) -> (StatusCode, Json<Vec<Message>>) {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID
    let messages = state.chat_service.get_messages_for_conversation(tenant_id, conversation_id).await;
    (StatusCode::OK, Json(messages))
}

pub fn router(chat_service: Arc<ChatService>) -> Router {
    let state = Arc::new(AppState { chat_service });

    Router::new()
        .route("/inboxes", post(create_inbox))
        .route("/inboxes/:inbox_id/conversations", post(create_conversation))
        .route("/conversations/:conversation_id/messages", post(create_message).get(get_messages))
        .with_state(state)
}
