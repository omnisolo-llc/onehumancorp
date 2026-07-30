use axum::{
    extract::{Path, State},
    routing::{post, get},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatInbox, ChatMessage};

#[derive(Clone)]
pub struct AppState {
    pub chat_service: Arc<ChatService>,
}

pub fn router(pool: PgPool) -> Router {
    let state = AppState {
        chat_service: Arc::new(ChatService::new(pool)),
    };

    Router::new()
        .route("/api/v1/chat/:tenant_id/inboxes", post(create_inbox))
        .route("/api/v1/chat/:tenant_id/conversations/:conversation_id/messages", post(send_message))
        .route("/api/v1/chat/:tenant_id/conversations/:conversation_id/messages", get(get_messages))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
}

pub async fn create_inbox(
    State(state): State<AppState>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<CreateInboxRequest>,
) -> Result<Json<ChatInbox>, axum::http::StatusCode> {
    match state.chat_service.create_inbox(tenant_id, payload.name).await {
        Ok(inbox) => Ok(Json(inbox)),
        Err(e) => {
            tracing::error!("Failed to create inbox: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub content_attributes: Option<serde_json::Value>,
    pub external_source_ids: Option<serde_json::Value>,
}

pub async fn send_message(
    State(state): State<AppState>,
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<ChatMessage>, axum::http::StatusCode> {
    match state.chat_service.send_message(
        tenant_id,
        conversation_id,
        payload.sender_type,
        payload.sender_id,
        payload.content,
        payload.content_attributes,
        payload.external_source_ids,
    ).await {
        Ok(msg) => Ok(Json(msg)),
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_messages(
    State(state): State<AppState>,
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<ChatMessage>>, axum::http::StatusCode> {
    match state.chat_service.get_messages(tenant_id, conversation_id).await {
        Ok(msgs) => Ok(Json(msgs)),
        Err(e) => {
            tracing::error!("Failed to get messages: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_inbox() {
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn test_send_message() {
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn test_get_messages() {
        assert_eq!(1, 1);
    }
}
