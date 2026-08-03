use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct AppState {
    pub chat_service: Arc<ChatService>,
    pub pool: PgPool,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub sender_type: String,
}

pub fn router(pool: PgPool) -> Router {
    let state = AppState {
        chat_service: Arc::new(ChatService::new(pool.clone())),
        pool,
    };

    Router::new()
        .route("/conversations/{tenant_id}", get(get_conversations))
        .route("/messages/{tenant_id}/{conversation_id}", get(get_messages))
        .route("/messages/{tenant_id}/{conversation_id}", post(send_message))
        .with_state(state)
}

async fn get_conversations(
    State(state): State<AppState>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<Vec<ChatConversation>>, axum::http::StatusCode> {
    match sqlx::query_as::<_, ChatConversation>(
        "SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at FROM chat_conversations WHERE tenant_id = $1 ORDER BY updated_at DESC"
    )
    .bind(tenant_id)
    .fetch_all(&state.pool)
    .await {
        Ok(conversations) => Ok(Json(conversations)),
        Err(e) => {
            tracing::error!("Failed to fetch chat conversations: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_messages(
    State(state): State<AppState>,
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<ChatMessage>>, axum::http::StatusCode> {
    match sqlx::query_as::<_, ChatMessage>(
        "SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at FROM chat_messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC"
    )
    .bind(tenant_id)
    .bind(conversation_id)
    .fetch_all(&state.pool)
    .await {
        Ok(messages) => Ok(Json(messages)),
        Err(e) => {
            tracing::error!("Failed to fetch chat messages: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn send_message(
    State(state): State<AppState>,
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<ChatMessage>, axum::http::StatusCode> {
    match state.chat_service.send_message(tenant_id, conversation_id, payload.sender_type, None, payload.content).await {
        Ok(msg) => Ok(Json(msg)),
        Err(e) => {
            tracing::error!("Failed to send chat message: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
