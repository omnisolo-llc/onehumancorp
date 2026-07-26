use crate::models::*;
use crate::db::*;
use axum::{
    extract::{Path, State, Extension},
    routing::get,
    Json, Router,
};
use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize};

#[derive(Clone)]
pub struct ChatState {
    pub pool: PgPool,
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub contact_id: Uuid,
}

// NOTE: In the real app, we would extract the tenant_id from the authenticated request
// via middleware (e.g. `Extension<TenantAuth>`). For this implementation, we simulate it
// using a placeholder header or path, but assuming we get it injected somehow.
// For simplicity in these handlers, we just take it from a path parameter or extension.

pub async fn list_inboxes(
    State(state): State<ChatState>,
    Extension(tenant_id): Extension<Uuid>,
) -> Result<Json<Vec<ChatInbox>>, String> {
    match get_inboxes(&state.pool, tenant_id).await {
        Ok(inboxes) => Ok(Json(inboxes)),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn list_conversations(
    State(state): State<ChatState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(inbox_id): Path<Uuid>,
) -> Result<Json<Vec<ChatConversation>>, String> {
    match get_conversations(&state.pool, tenant_id, inbox_id).await {
        Ok(conversations) => Ok(Json(conversations)),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn list_messages(
    State(state): State<ChatState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(conversation_id): Path<Uuid>,
) -> Result<Json<Vec<ChatMessage>>, String> {
    match get_messages(&state.pool, tenant_id, conversation_id).await {
        Ok(messages) => Ok(Json(messages)),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn send_message(
    State(state): State<ChatState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<CreateMessageRequest>,
) -> Result<Json<ChatMessage>, String> {
    match create_message(
        &state.pool,
        tenant_id,
        conversation_id,
        payload.sender_type,
        payload.sender_id,
        payload.content,
    )
    .await
    {
        Ok(msg) => {
            // TODO: Broadcast the new message via WebSocket/PubSub
            Ok(Json(msg))
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn start_conversation(
    State(state): State<ChatState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(inbox_id): Path<Uuid>,
    Json(payload): Json<CreateConversationRequest>,
) -> Result<Json<ChatConversation>, String> {
    match create_conversation(&state.pool, tenant_id, inbox_id, payload.contact_id).await {
        Ok(conv) => Ok(Json(conv)),
        Err(e) => Err(e.to_string()),
    }
}

pub fn chat_routes(pool: PgPool) -> Router {
    let state = ChatState { pool };
    Router::new()
        .route("/api/chat/inboxes", get(list_inboxes))
        .route("/api/chat/inboxes/:inbox_id/conversations", get(list_conversations).post(start_conversation))
        .route("/api/chat/conversations/:conversation_id/messages", get(list_messages).post(send_message))
        .with_state(state)
}
