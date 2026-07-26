use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatInbox, ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct AppState {
    pub chat_service: Arc<ChatService>,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: sqlx::PgPool) -> Router<S> {
    let state = AppState {
        chat_service: Arc::new(ChatService::new(pool)),
    };

    Router::new()
        .route("/api/v1/chat/inboxes", get(list_inboxes))
        .route("/api/v1/chat/conversations/:inbox_id", get(list_conversations))
        .route("/api/v1/chat/messages/:conversation_id", get(list_messages))
        .route("/api/v1/chat/messages/:conversation_id", post(send_message))
        .with_state(state)
}

async fn list_inboxes(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
) -> Result<Json<Vec<ChatInbox>>, axum::http::StatusCode> {
    match state.chat_service.list_inboxes(Uuid::parse_str(user.organization_id.as_deref().unwrap_or_default()).unwrap_or_default()).await {
        Ok(inboxes) => Ok(Json(inboxes)),
        Err(e) => {
            tracing::error!("Failed to list chat inboxes: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn list_conversations(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
    Path(inbox_id): Path<Uuid>,
) -> Result<Json<Vec<ChatConversation>>, axum::http::StatusCode> {
    match state.chat_service.list_conversations(Uuid::parse_str(user.organization_id.as_deref().unwrap_or_default()).unwrap_or_default(), inbox_id).await {
        Ok(convs) => Ok(Json(convs)),
        Err(e) => {
            tracing::error!("Failed to list chat conversations: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn list_messages(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
    Path(conversation_id): Path<Uuid>,
) -> Result<Json<Vec<ChatMessage>>, axum::http::StatusCode> {
    match state.chat_service.list_messages(Uuid::parse_str(user.organization_id.as_deref().unwrap_or_default()).unwrap_or_default(), conversation_id).await {
        Ok(msgs) => Ok(Json(msgs)),
        Err(e) => {
            tracing::error!("Failed to list chat messages: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn send_message(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<ChatMessage>, axum::http::StatusCode> {
    match state.chat_service.send_message(
        Uuid::parse_str(user.organization_id.as_deref().unwrap_or_default()).unwrap_or_default(),
        conversation_id,
        payload.sender_type,
        payload.sender_id,
        payload.content,
    ).await {
        Ok(msg) => Ok(Json(msg)),
        Err(e) => {
            tracing::error!("Failed to send chat message: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
