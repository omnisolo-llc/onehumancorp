use axum::{
    extract::{State, Path},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::db::DB;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatInbox, ChatConversation, ChatMessage};
use crate::api::auth::UserSession;

#[derive(Clone)]
pub struct ChatAppState {
    pub db: Arc<DB>,
    pub chat_service: Arc<ChatService>,
}

pub fn router(state: ChatAppState) -> Router {
    Router::new()
        .route("/inboxes", post(create_inbox))
        .route("/conversations", post(start_conversation))
        .route("/conversations/:id/messages", post(send_message))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct CreateInboxReq {
    name: String,
}

async fn create_inbox(
    State(state): State<ChatAppState>,
    session: UserSession,
    Json(payload): Json<CreateInboxReq>,
) -> Result<Json<ChatInbox>, (axum::http::StatusCode, String)> {
    let inbox = state.chat_service
        .create_inbox(session.tenant_id, payload.name)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(inbox))
}

#[derive(Deserialize)]
pub struct StartConversationReq {
    inbox_id: Uuid,
    contact_id: Uuid,
}

async fn start_conversation(
    State(state): State<ChatAppState>,
    session: UserSession,
    Json(payload): Json<StartConversationReq>,
) -> Result<Json<ChatConversation>, (axum::http::StatusCode, String)> {
    let conv = state.chat_service
        .start_conversation(session.tenant_id, payload.inbox_id, payload.contact_id, None)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(conv))
}

#[derive(Deserialize)]
pub struct SendMessageReq {
    content: String,
    sender_type: String,
    sender_id: Option<Uuid>,
}

async fn send_message(
    State(state): State<ChatAppState>,
    Path(conversation_id): Path<Uuid>,
    session: UserSession,
    Json(payload): Json<SendMessageReq>,
) -> Result<Json<ChatMessage>, (axum::http::StatusCode, String)> {
    let msg = state.chat_service
        .send_message(session.tenant_id, conversation_id, payload.sender_type, payload.sender_id, payload.content)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(msg))
}
