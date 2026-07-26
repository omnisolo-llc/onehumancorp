use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct ChatAppState {
    pub chat_service: std::sync::Arc<ChatService>,
}

#[derive(Deserialize)]
pub struct CreateInboxReq {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateChannelReq {
    pub channel_type: String,
    pub config: serde_json::Value,
}

#[derive(Deserialize)]
pub struct CreateContactReq {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct StartConversationReq {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct SendMessageReq {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

pub fn router(pool: sqlx::PgPool) -> Router {
    let state = ChatAppState {
        chat_service: std::sync::Arc::new(ChatService::new(pool)),
    };

    Router::new()
        .route("/api/v1/chat/:tenant_id/inboxes", post(create_inbox))
        .route("/api/v1/chat/:tenant_id/inboxes/:inbox_id/channels", post(create_channel))
        .route("/api/v1/chat/:tenant_id/contacts", post(create_contact))
        .route("/api/v1/chat/:tenant_id/conversations", post(start_conversation))
        .route("/api/v1/chat/:tenant_id/conversations/:conversation_id/messages", post(send_message))
        .with_state(state)
}

async fn create_inbox(
    State(state): State<ChatAppState>,
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

async fn create_channel(
    State(state): State<ChatAppState>,
    Path((tenant_id, inbox_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CreateChannelReq>,
) -> Result<Json<ChatChannel>, axum::http::StatusCode> {
    match state.chat_service.create_channel(tenant_id, inbox_id, payload.channel_type, payload.config).await {
        Ok(channel) => Ok(Json(channel)),
        Err(e) => {
            tracing::error!("Failed to create channel: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_contact(
    State(state): State<ChatAppState>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<CreateContactReq>,
) -> Result<Json<ChatContact>, axum::http::StatusCode> {
    match state.chat_service.create_contact(tenant_id, payload.name, payload.email, payload.phone).await {
        Ok(contact) => Ok(Json(contact)),
        Err(e) => {
            tracing::error!("Failed to create contact: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn start_conversation(
    State(state): State<ChatAppState>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<StartConversationReq>,
) -> Result<Json<ChatConversation>, axum::http::StatusCode> {
    match state.chat_service.start_conversation(tenant_id, payload.inbox_id, payload.contact_id, payload.assignee_id).await {
        Ok(conversation) => Ok(Json(conversation)),
        Err(e) => {
            tracing::error!("Failed to start conversation: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn send_message(
    State(state): State<ChatAppState>,
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<SendMessageReq>,
) -> Result<Json<ChatMessage>, axum::http::StatusCode> {
    match state.chat_service.send_message(tenant_id, conversation_id, payload.sender_type, payload.sender_id, payload.content).await {
        Ok(message) => Ok(Json(message)),
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
