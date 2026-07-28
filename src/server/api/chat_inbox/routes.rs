use axum::{
    extract::{Path, State},
    routing::{post},
    Json, Router,
};
use serde::{Deserialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatInbox, ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct ChatInboxState {
    pub chat_service: Arc<ChatService>,
}

pub fn router(pool: PgPool) -> Router {
    let state = ChatInboxState {
        chat_service: Arc::new(ChatService::new(pool)),
    };

    Router::new()
        .route("/api/v1/chat/:tenant_id/inbox", post(create_inbox))
        .route("/api/v1/chat/:tenant_id/conversation", post(start_conversation))
        .route("/api/v1/chat/:tenant_id/conversation/:conversation_id/message", post(send_message))
        // WS route is handled by the unified WS logic
        .with_state(state)
}

#[derive(Deserialize)]
pub struct CreateInboxReq {
    pub name: String,
}

async fn create_inbox(
    State(state): State<ChatInboxState>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<CreateInboxReq>,
) -> Result<Json<ChatInbox>, axum::http::StatusCode> {
    match state.chat_service.create_inbox(tenant_id, payload.name).await {
        Ok(inbox) => Ok(Json(inbox)),
        Err(e) => {
            tracing::error!("Failed to create inbox: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct StartConversationReq {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
}

async fn start_conversation(
    State(state): State<ChatInboxState>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<StartConversationReq>,
) -> Result<Json<ChatConversation>, axum::http::StatusCode> {
    match state.chat_service.start_conversation(
        tenant_id,
        payload.inbox_id,
        payload.contact_id,
        payload.assignee_id,
    ).await {
        Ok(conv) => Ok(Json(conv)),
        Err(e) => {
            tracing::error!("Failed to start conversation: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct SendMessageReq {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

async fn send_message(
    State(state): State<ChatInboxState>,
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<SendMessageReq>,
) -> Result<Json<ChatMessage>, axum::http::StatusCode> {
    match state.chat_service.send_message(
        tenant_id,
        conversation_id,
        payload.sender_type,
        payload.sender_id,
        payload.content,
    ).await {
        Ok(msg) => Ok(Json(msg)),
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
