use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use sqlx::PgPool;
use crate::services::inbox::omni_chat::{OmniChatService, ChatInbox, ChatConversation, ChatMessage, ChatContact};

#[derive(Clone)]
pub struct AppState {
    pub omni_chat_service: Arc<OmniChatService>,
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub tenant_id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateContactRequest {
    pub tenant_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub tenant_id: String,
    pub contact_id: String,
    pub channel: String,
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub tenant_id: String,
    pub sender_type: String,
    pub sender_id: Option<String>,
    pub content: String,
}

pub fn router(pool: PgPool) -> Router {
    let state = AppState {
        omni_chat_service: Arc::new(OmniChatService::new(pool)),
    };

    Router::new()
        .route("/api/v1/omni-chat/inboxes", post(create_inbox).get(get_inboxes))
        .route("/api/v1/omni-chat/contacts", post(create_contact))
        .route("/api/v1/omni-chat/inboxes/:inbox_id/conversations", post(create_conversation).get(get_conversations))
        .route("/api/v1/omni-chat/conversations/:conversation_id/messages", post(create_message).get(get_messages))
        .with_state(state)
}

async fn create_inbox(
    State(state): State<AppState>,
    Json(payload): Json<CreateInboxRequest>,
) -> Result<Json<ChatInbox>, axum::http::StatusCode> {
    match state.omni_chat_service.create_inbox(&payload.tenant_id, &payload.name).await {
        Ok(inbox) => Ok(Json(inbox)),
        Err(e) => {
            tracing::error!("Failed to create inbox: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

use axum::extract::Query;

async fn get_inboxes(
    State(state): State<AppState>,
    Query(tenant_id): Query<String>,
) -> Result<Json<Vec<ChatInbox>>, axum::http::StatusCode> {
    match state.omni_chat_service.get_inboxes(&tenant_id).await {
        Ok(inboxes) => Ok(Json(inboxes)),
        Err(e) => {
            tracing::error!("Failed to get inboxes: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct TenantQuery {
    tenant_id: String,
}

async fn create_contact(
    State(state): State<AppState>,
    Json(payload): Json<CreateContactRequest>,
) -> Result<Json<ChatContact>, axum::http::StatusCode> {
    match state.omni_chat_service.create_contact(&payload.tenant_id, &payload.name, payload.email.as_deref(), payload.phone.as_deref()).await {
        Ok(contact) => Ok(Json(contact)),
        Err(e) => {
            tracing::error!("Failed to create contact: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_conversation(
    State(state): State<AppState>,
    Path(inbox_id): Path<String>,
    Json(payload): Json<CreateConversationRequest>,
) -> Result<Json<ChatConversation>, axum::http::StatusCode> {
    match state.omni_chat_service.create_conversation(&payload.tenant_id, &inbox_id, &payload.contact_id, &payload.channel).await {
        Ok(conversation) => Ok(Json(conversation)),
        Err(e) => {
            tracing::error!("Failed to create conversation: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_conversations(
    State(state): State<AppState>,
    Path(inbox_id): Path<String>,
    Query(query): Query<TenantQuery>,
) -> Result<Json<Vec<ChatConversation>>, axum::http::StatusCode> {
    match state.omni_chat_service.get_conversations(&query.tenant_id, &inbox_id).await {
        Ok(conversations) => Ok(Json(conversations)),
        Err(e) => {
            tracing::error!("Failed to get conversations: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_message(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Json(payload): Json<CreateMessageRequest>,
) -> Result<Json<ChatMessage>, axum::http::StatusCode> {
    match state.omni_chat_service.create_message(&payload.tenant_id, &conversation_id, &payload.sender_type, payload.sender_id.as_deref(), &payload.content).await {
        Ok(message) => Ok(Json(message)),
        Err(e) => {
            tracing::error!("Failed to create message: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Query(query): Query<TenantQuery>,
) -> Result<Json<Vec<ChatMessage>>, axum::http::StatusCode> {
    match state.omni_chat_service.get_messages(&query.tenant_id, &conversation_id).await {
        Ok(messages) => Ok(Json(messages)),
        Err(e) => {
            tracing::error!("Failed to get messages: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
