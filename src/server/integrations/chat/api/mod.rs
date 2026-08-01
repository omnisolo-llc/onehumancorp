use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

use crate::domain::{ChatInbox, ChatConversation, ChatMessage};
use super::services::ChatService;

#[derive(Clone)]
pub struct ChatApiState {
    pub service: Arc<ChatService>,
}

pub fn router(state: ChatApiState) -> Router {
    Router::new()
        .route("/inboxes", get(list_inboxes).post(create_inbox))
        .route("/conversations", post(create_conversation))
        .route("/conversations/:id/messages", post(send_message))
        .with_state(state)
}

// Handlers
async fn list_inboxes(
    State(_state): State<ChatApiState>,
) -> Result<Json<Vec<ChatInbox>>, StatusCode> {
    Ok(Json(vec![]))
}

#[derive(Deserialize)]
pub struct CreateInboxReq {
    pub tenant_id: Uuid,
    pub name: String,
}

async fn create_inbox(
    State(state): State<ChatApiState>,
    Json(payload): Json<CreateInboxReq>,
) -> Result<Json<ChatInbox>, StatusCode> {
    match state.service.create_inbox(payload.tenant_id, payload.name).await {
        Ok(inbox) => Ok(Json(inbox)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct CreateConversationReq {
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
}

async fn create_conversation(
    State(state): State<ChatApiState>,
    Json(payload): Json<CreateConversationReq>,
) -> Result<Json<ChatConversation>, StatusCode> {
    match state.service.start_conversation(
        payload.tenant_id,
        payload.inbox_id,
        payload.contact_id,
        payload.assignee_id,
    ).await {
        Ok(conv) => Ok(Json(conv)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct SendMessageReq {
    pub tenant_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

async fn send_message(
    State(state): State<ChatApiState>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<SendMessageReq>,
) -> Result<Json<ChatMessage>, StatusCode> {
    match state.service.send_message(
        payload.tenant_id,
        conversation_id,
        payload.sender_type,
        payload.sender_id,
        payload.content,
    ).await {
        Ok(msg) => Ok(Json(msg)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
