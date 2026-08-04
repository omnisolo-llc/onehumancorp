use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::db::DB;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct ChatState {
    pub service: Arc<ChatService>,
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let pool = match &db.store {
        crate::db::DbStore::Postgres => db.pool.clone(),
        crate::db::DbStore::Sqlite(_) => panic!("Sqlite not supported for ChatService yet"),
    };

    let service = Arc::new(ChatService::new(pool));
    let state = ChatState { service };

    Router::new()
        .route("/api/v1/chat/inboxes", post(create_inbox))
        .route("/api/v1/chat/conversations/{conversation_id}/messages", post(send_message))
        // FULL CRUD PLACEHOLDERS
        .route("/api/v1/chat/channels", get(get_channels))
        .route("/api/v1/chat/contacts", get(get_contacts))
        .route("/api/v1/chat/conversations", get(get_conversations))
        .with_state(state)
}

async fn get_channels() -> impl IntoResponse { (StatusCode::OK, "[]") }
async fn get_contacts() -> impl IntoResponse { (StatusCode::OK, "[]") }
async fn get_conversations() -> impl IntoResponse { (StatusCode::OK, "[]") }

async fn create_inbox(
    State(state): State<ChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateInboxRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.and_then(|id| Uuid::parse_str(&id).ok()) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    match state.service.create_inbox(tenant_id, payload.name).await {
        Ok(inbox) => (StatusCode::CREATED, Json(inbox)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error").into_response()
    }
}

async fn send_message(
    State(state): State<ChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<SendMessageRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.and_then(|id| Uuid::parse_str(&id).ok()) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    match state.service.send_message(tenant_id, conversation_id, payload.sender_type, payload.sender_id, payload.content).await {
        Ok(message) => (StatusCode::CREATED, Json(message)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error").into_response()
    }
}
