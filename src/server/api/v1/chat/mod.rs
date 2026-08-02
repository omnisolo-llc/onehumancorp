use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::Pool;

#[derive(Clone)]
pub struct ChatState {
    pub db: Pool,
}

pub fn router(state: ChatState) -> Router {
    Router::new()
        .route("/inboxes", get(list_inboxes).post(create_inbox))
        .route("/inboxes/:id", get(get_inbox).put(update_inbox).delete(delete_inbox))
        .route("/conversations", get(list_conversations).post(create_conversation))
        .route("/conversations/:id", get(get_conversation).put(update_conversation))
        .route("/conversations/:id/messages", get(list_messages).post(create_message))
        .with_state(state)
}

#[derive(Serialize)]
pub struct Inbox {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateInboxPayload {
    pub name: String,
}

async fn list_inboxes(State(state): State<ChatState>) -> Json<Vec<Inbox>> {
    // Stub
    Json(vec![])
}

async fn create_inbox(State(state): State<ChatState>, Json(payload): Json<CreateInboxPayload>) -> Json<Inbox> {
    // Stub
    Json(Inbox { id: Uuid::new_v4(), name: payload.name })
}

async fn get_inbox(State(state): State<ChatState>, Path(id): Path<Uuid>) -> Json<Option<Inbox>> {
    Json(None)
}

async fn update_inbox(State(state): State<ChatState>, Path(id): Path<Uuid>, Json(payload): Json<CreateInboxPayload>) -> Json<Option<Inbox>> {
    Json(None)
}

async fn delete_inbox(State(state): State<ChatState>, Path(id): Path<Uuid>) -> axum::http::StatusCode {
    axum::http::StatusCode::NO_CONTENT
}

#[derive(Serialize)]
pub struct Conversation {
    pub id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateConversationPayload {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateConversationPayload {
    pub status: String,
}

async fn list_conversations(State(state): State<ChatState>) -> Json<Vec<Conversation>> {
    Json(vec![])
}

async fn create_conversation(State(state): State<ChatState>, Json(payload): Json<CreateConversationPayload>) -> Json<Conversation> {
    Json(Conversation { id: Uuid::new_v4(), inbox_id: payload.inbox_id, contact_id: payload.contact_id, status: "open".to_string() })
}

async fn get_conversation(State(state): State<ChatState>, Path(id): Path<Uuid>) -> Json<Option<Conversation>> {
    Json(None)
}

async fn update_conversation(State(state): State<ChatState>, Path(id): Path<Uuid>, Json(payload): Json<UpdateConversationPayload>) -> Json<Option<Conversation>> {
    Json(None)
}

#[derive(Serialize)]
pub struct Message {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct CreateMessagePayload {
    pub sender_type: String,
    pub content: String,
}

async fn list_messages(State(state): State<ChatState>, Path(conversation_id): Path<Uuid>) -> Json<Vec<Message>> {
    Json(vec![])
}

async fn create_message(State(state): State<ChatState>, Path(conversation_id): Path<Uuid>, Json(payload): Json<CreateMessagePayload>) -> Json<Message> {
    Json(Message { id: Uuid::new_v4(), conversation_id, sender_type: payload.sender_type, content: payload.content })
}
