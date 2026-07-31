use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use super::service::ChatService;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct ChatAppState {
    pub pool: PgPool,
    pub chat_service: std::sync::Arc<ChatService>,
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> Router<S> {
    let chat_service = std::sync::Arc::new(ChatService::new(pool.clone()));
    let state = ChatAppState { pool, chat_service };

    Router::new()
        .route("/inboxes", post(create_inbox))
        .route("/channels", post(create_channel))
        .route("/contacts", post(create_contact))
        .route("/conversations", post(start_conversation))
        .route("/messages", post(send_message))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct CreateInboxReq {
    tenant_id: Uuid,
    name: String,
}
async fn create_inbox(State(state): State<ChatAppState>, Json(req): Json<CreateInboxReq>) -> Result<Json<ChatInbox>, axum::http::StatusCode> {
    match state.chat_service.create_inbox(req.tenant_id, req.name).await {
        Ok(inbox) => Ok(Json(inbox)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct CreateChannelReq {
    tenant_id: Uuid,
    inbox_id: Uuid,
    channel_type: String,
    config: serde_json::Value,
}
async fn create_channel(State(state): State<ChatAppState>, Json(req): Json<CreateChannelReq>) -> Result<Json<ChatChannel>, axum::http::StatusCode> {
    match state.chat_service.create_channel(req.tenant_id, req.inbox_id, req.channel_type, req.config).await {
        Ok(channel) => Ok(Json(channel)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct CreateContactReq {
    tenant_id: Uuid,
    name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
}
async fn create_contact(State(state): State<ChatAppState>, Json(req): Json<CreateContactReq>) -> Result<Json<ChatContact>, axum::http::StatusCode> {
    match state.chat_service.create_contact(req.tenant_id, req.name, req.email, req.phone).await {
        Ok(contact) => Ok(Json(contact)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct StartConversationReq {
    tenant_id: Uuid,
    inbox_id: Uuid,
    contact_id: Uuid,
    assignee_id: Option<Uuid>,
}
async fn start_conversation(State(state): State<ChatAppState>, Json(req): Json<StartConversationReq>) -> Result<Json<ChatConversation>, axum::http::StatusCode> {
    match state.chat_service.start_conversation(req.tenant_id, req.inbox_id, req.contact_id, req.assignee_id).await {
        Ok(conversation) => Ok(Json(conversation)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct SendMessageReq {
    tenant_id: Uuid,
    conversation_id: Uuid,
    sender_type: String,
    sender_id: Option<Uuid>,
    content: String,
}
async fn send_message(State(state): State<ChatAppState>, Json(req): Json<SendMessageReq>) -> Result<Json<ChatMessage>, axum::http::StatusCode> {
    match state.chat_service.send_message(req.tenant_id, req.conversation_id, req.sender_type, req.sender_id, req.content).await {
        Ok(message) => Ok(Json(message)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
