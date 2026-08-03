// Native Omnichannel API Routes
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json
};
use uuid::Uuid;
use crate::db::DB;
use std::sync::Arc;

pub async fn list_conversations(
    State(db): State<Arc<DB>>,
) -> impl IntoResponse {
    let pool = db.pool.clone();
    let repo = server_ohc::domain::omnichannel::models::OmnichannelRepo::new(pool);
    let tenant_id = "00000000-0000-0000-0000-000000000000";
    match repo.list_conversations(tenant_id).await {
        Ok(conversations) => axum::Json(serde_json::json!({ "conversations": conversations })).into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to list conversations").into_response(),
    }
}

pub async fn list_messages(
    State(db): State<Arc<DB>>,
    Path(conversation_id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = db.pool.clone();
    let repo = server_ohc::domain::omnichannel::models::OmnichannelRepo::new(pool);
    let tenant_id = "00000000-0000-0000-0000-000000000000";
    match repo.list_messages(tenant_id, conversation_id).await {
        Ok(messages) => axum::Json(serde_json::json!({ "messages": messages })).into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to list messages").into_response(),
    }
}

#[derive(serde::Deserialize)]
pub struct CreateMessageRequest {
    pub content: String,
    pub sender_type: String,
}

pub async fn add_message(
    State(db): State<Arc<DB>>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<CreateMessageRequest>,
) -> impl IntoResponse {
    let pool = db.pool.clone();
    let repo = server_ohc::domain::omnichannel::models::OmnichannelRepo::new(pool);
    let tenant_id = "00000000-0000-0000-0000-000000000000";

    match repo.add_message(tenant_id, conversation_id, &payload.sender_type, &payload.content, "Outgoing").await {
        Ok(message) => axum::Json(serde_json::json!({ "message": message })).into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to add message").into_response(),
    }
}

#[derive(serde::Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
    pub channel_type: String,
}

pub async fn create_inbox(
    State(db): State<Arc<DB>>,
    Json(payload): Json<CreateInboxRequest>,
) -> impl IntoResponse {
    let pool = db.pool.clone();
    let repo = server_ohc::domain::omnichannel::models::OmnichannelRepo::new(pool);
    let tenant_id = "00000000-0000-0000-0000-000000000000";

    match repo.create_inbox(tenant_id, &payload.name, &payload.channel_type).await {
        Ok(inbox) => axum::Json(serde_json::json!({ "inbox": inbox })).into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to create inbox").into_response(),
    }
}

#[derive(serde::Deserialize)]
pub struct CreateConversationRequest {
    pub inbox_id: Uuid,
    pub contact_name: String,
    pub message: String,
}

pub async fn create_conversation(
    State(db): State<Arc<DB>>,
    Json(payload): Json<CreateConversationRequest>,
) -> impl IntoResponse {
    let pool = db.pool.clone();
    let repo = server_ohc::domain::omnichannel::models::OmnichannelRepo::new(pool);
    let tenant_id = "00000000-0000-0000-0000-000000000000";

    let contact = repo.create_contact(tenant_id, &payload.contact_name, None, None).await.unwrap();
    let conv = repo.create_conversation(tenant_id, payload.inbox_id, contact.id, "Open").await.unwrap();
    repo.add_message(tenant_id, conv.id, "Contact", &payload.message, "Incoming").await.unwrap();

    axum::Json(serde_json::json!({ "conversation": conv })).into_response()
}
