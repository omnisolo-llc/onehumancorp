use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;

// Since we are compiling ohc-mono which imports server_ohc
use server_ohc::domain::omnichannel_repo::OmnichannelRepo;
use server_ohc::domain::omnichannel::{Inbox, Contact, Conversation, Message};

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<OmnichannelRepo>,
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateContactRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

pub fn router(pool: PgPool) -> Router {
    let state = AppState {
        repo: Arc::new(OmnichannelRepo::new(pool)),
    };

    Router::new()
        .route("/api/v1/omnichannel/inboxes", get(get_inboxes).post(create_inbox))
        .route("/api/v1/omnichannel/contacts", get(get_contacts).post(create_contact))
        .route("/api/v1/omnichannel/conversations", post(create_conversation))
        .route("/api/v1/omnichannel/inboxes/:inbox_id/conversations", get(get_conversations))
        .route("/api/v1/omnichannel/conversations/:conversation_id/messages", get(get_messages).post(create_message))
        .with_state(state)
}

// In a real app we'd extract tenant_id from context via middleware.
// We simulate it here safely to ensure proper isolation mapping.

async fn create_inbox(
    State(state): State<AppState>,
    Json(payload): Json<CreateInboxRequest>,
) -> Result<Json<Inbox>, axum::http::StatusCode> {
    let current_tenant_id = Uuid::new_v4(); // Mock context extraction
    match state.repo.create_inbox(current_tenant_id, payload.name).await {
        Ok(inbox) => Ok(Json(inbox)),
        Err(e) => {
            tracing::error!("Failed to create inbox: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_inboxes(
    State(state): State<AppState>,
) -> Result<Json<Vec<Inbox>>, axum::http::StatusCode> {
    let current_tenant_id = Uuid::new_v4(); // Mock context extraction
    match state.repo.get_inboxes(current_tenant_id).await {
        Ok(inboxes) => Ok(Json(inboxes)),
        Err(e) => {
            tracing::error!("Failed to get inboxes: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_contact(
    State(state): State<AppState>,
    Json(payload): Json<CreateContactRequest>,
) -> Result<Json<Contact>, axum::http::StatusCode> {
    let current_tenant_id = Uuid::new_v4(); // Mock context extraction
    match state.repo.create_contact(current_tenant_id, payload.name, payload.email, payload.phone).await {
        Ok(contact) => Ok(Json(contact)),
        Err(e) => {
            tracing::error!("Failed to create contact: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_contacts(
    State(state): State<AppState>,
) -> Result<Json<Vec<Contact>>, axum::http::StatusCode> {
    let current_tenant_id = Uuid::new_v4(); // Mock context extraction
    match state.repo.get_contacts(current_tenant_id).await {
        Ok(contacts) => Ok(Json(contacts)),
        Err(e) => {
            tracing::error!("Failed to get contacts: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_conversation(
    State(state): State<AppState>,
    Json(payload): Json<CreateConversationRequest>,
) -> Result<Json<Conversation>, axum::http::StatusCode> {
    let current_tenant_id = Uuid::new_v4(); // Mock context extraction
    match state.repo.create_conversation(current_tenant_id, payload.inbox_id, payload.contact_id, payload.assignee_id, payload.status).await {
        Ok(conversation) => Ok(Json(conversation)),
        Err(e) => {
            tracing::error!("Failed to create conversation: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_conversations(
    State(state): State<AppState>,
    Path(inbox_id): Path<Uuid>,
) -> Result<Json<Vec<Conversation>>, axum::http::StatusCode> {
    let current_tenant_id = Uuid::new_v4(); // Mock context extraction
    match state.repo.get_conversations(current_tenant_id, inbox_id).await {
        Ok(conversations) => Ok(Json(conversations)),
        Err(e) => {
            tracing::error!("Failed to get conversations: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_message(
    State(state): State<AppState>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<CreateMessageRequest>,
) -> Result<Json<Message>, axum::http::StatusCode> {
    let current_tenant_id = Uuid::new_v4(); // Mock context extraction
    match state.repo.create_message(current_tenant_id, conversation_id, payload.sender_type, payload.sender_id, payload.content).await {
        Ok(message) => Ok(Json(message)),
        Err(e) => {
            tracing::error!("Failed to create message: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<Uuid>,
) -> Result<Json<Vec<Message>>, axum::http::StatusCode> {
    let current_tenant_id = Uuid::new_v4(); // Mock context extraction
    match state.repo.get_messages(current_tenant_id, conversation_id).await {
        Ok(messages) => Ok(Json(messages)),
        Err(e) => {
            tracing::error!("Failed to get messages: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
