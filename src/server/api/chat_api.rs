use axum::{
    extract::{Path, State, Extension},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct ChatAppState {
    pub chat_service: Arc<ChatService>,
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateChannelRequest {
    pub channel_type: String,
    pub config: serde_json::Value,
}

#[derive(Deserialize)]
pub struct CreateContactRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct StartConversationRequest {
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

pub fn chat_router(pool: PgPool) -> Router {
    let state = ChatAppState {
        chat_service: Arc::new(ChatService::new(pool)),
    };

    Router::new()
        .route("/inboxes", post(create_inbox))
        .route("/inboxes/:inbox_id/channels", post(create_channel))
        .route("/contacts", post(create_contact))
        .route("/inboxes/:inbox_id/conversations", post(start_conversation))
        .route("/conversations/:conversation_id/messages", post(send_message))
        .with_state(state)
}

async fn create_inbox(
    State(state): State<ChatAppState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateInboxRequest>,
) -> Result<Json<ChatInbox>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
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
    Path(inbox_id): Path<Uuid>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateChannelRequest>,
) -> Result<Json<ChatChannel>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
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
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateContactRequest>,
) -> Result<Json<ChatContact>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
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
    Path(inbox_id): Path<Uuid>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<StartConversationRequest>,
) -> Result<Json<ChatConversation>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    match state.chat_service.start_conversation(tenant_id, inbox_id, payload.contact_id, payload.assignee_id).await {
        Ok(conv) => Ok(Json(conv)),
        Err(e) => {
            tracing::error!("Failed to start conversation: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn send_message(
    State(state): State<ChatAppState>,
    Path(conversation_id): Path<Uuid>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<ChatMessage>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    match state.chat_service.send_message(tenant_id, conversation_id, payload.sender_type, payload.sender_id, payload.content).await {
        Ok(msg) => Ok(Json(msg)),
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
