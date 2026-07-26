use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use std::sync::Arc;

use crate::domain::inbox::models::{ChatInbox, ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct ChatInboxState {
    pub pool: PgPool,
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

pub fn router(pool: PgPool) -> Router {
    let state = ChatInboxState { pool };

    Router::new()
        .route("/api/v1/chat/inbox/:tenant_id", post(create_inbox))
        .route("/api/v1/chat/inbox/:tenant_id/conversations", get(list_conversations))
        .route("/api/v1/chat/inbox/:tenant_id/conversations/:conversation_id/messages", post(create_message))
        .with_state(state)
}

async fn create_inbox(
    State(state): State<ChatInboxState>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<CreateInboxRequest>,
) -> Result<Json<ChatInbox>, axum::http::StatusCode> {
    let id = Uuid::new_v4();
    let query = "INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING *";

    match sqlx::query_as::<_, ChatInbox>(query)
        .bind(id)
        .bind(tenant_id)
        .bind(&payload.name)
        .fetch_one(&state.pool)
        .await
    {
        Ok(inbox) => Ok(Json(inbox)),
        Err(e) => {
            tracing::error!("Failed to create inbox: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn list_conversations(
    State(state): State<ChatInboxState>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<Vec<ChatConversation>>, axum::http::StatusCode> {
    let query = "SELECT * FROM chat_conversations WHERE tenant_id = $1 ORDER BY updated_at DESC";

    match sqlx::query_as::<_, ChatConversation>(query)
        .bind(tenant_id)
        .fetch_all(&state.pool)
        .await
    {
        Ok(conversations) => Ok(Json(conversations)),
        Err(e) => {
            tracing::error!("Failed to list conversations: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_message(
    State(state): State<ChatInboxState>,
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CreateMessageRequest>,
) -> Result<Json<ChatMessage>, axum::http::StatusCode> {
    let id = Uuid::new_v4();
    let query = "INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *";

    match sqlx::query_as::<_, ChatMessage>(query)
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(&payload.sender_type)
        .bind(payload.sender_id)
        .bind(&payload.content)
        .fetch_one(&state.pool)
        .await
    {
        Ok(message) => Ok(Json(message)),
        Err(e) => {
            tracing::error!("Failed to create message: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
