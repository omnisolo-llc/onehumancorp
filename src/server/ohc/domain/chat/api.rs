use axum::{
    routing::{post},
    Router, Json, Extension,
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use crate::domain::chat::models::{Inbox, Contact, Conversation, Message};
use crate::domain::chat::repository::ChatRepository;
use serde::{Deserialize, Serialize};

pub struct ChatState {
    pub repository: ChatRepository,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInboxRequest {
    pub tenant_id: Uuid,
    pub name: String,
}

pub async fn create_inbox(
    Extension(state): Extension<Arc<ChatState>>,
    Json(payload): Json<CreateInboxRequest>,
) -> Result<Json<Inbox>, StatusCode> {
    let inbox = Inbox {
        id: Uuid::new_v4(),
        tenant_id: payload.tenant_id,
        name: payload.name,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    state.repository.create_inbox(&inbox).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(inbox))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub message_content: String,
}

pub async fn webhook_ingest(
    Extension(state): Extension<Arc<ChatState>>,
    Json(payload): Json<WebhookPayload>,
) -> Result<Json<Message>, StatusCode> {
    // 1. Create or find contact
    let contact = Contact {
        id: Uuid::new_v4(),
        tenant_id: payload.tenant_id,
        name: payload.contact_name,
        email: payload.contact_email,
        phone: payload.contact_phone,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    state.repository.create_contact(&contact).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 2. Create conversation
    let conversation = Conversation {
        id: Uuid::new_v4(),
        tenant_id: payload.tenant_id,
        inbox_id: payload.inbox_id,
        contact_id: contact.id,
        status: "open".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    state.repository.create_conversation(&conversation).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 3. Create message
    let message = Message {
        id: Uuid::new_v4(),
        tenant_id: payload.tenant_id,
        conversation_id: conversation.id,
        sender_id: None, // From customer
        content: payload.message_content,
        is_ai_draft: false,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    state.repository.create_message(&message).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // TODO: Trigger Work Triage Agent event

    Ok(Json(message))
}

pub fn chat_routes(pool: PgPool) -> Router {
    let state = Arc::new(ChatState {
        repository: ChatRepository::new(pool),
    });

    Router::new()
        .route("/inboxes", post(create_inbox))
        .route("/webhook", post(webhook_ingest))
        .layer(Extension(state))
}
