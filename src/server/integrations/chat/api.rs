use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use super::models::{ChatConversation, ChatInbox, ChatMessage};

#[derive(Clone)]
pub struct ChatApiState {
    pub pool: sqlx::PgPool,
}

pub async fn list_inboxes(
    State(state): State<ChatApiState>,
    Path(tenant_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, ChatInbox>("SELECT * FROM chat_inboxes WHERE tenant_id = $1")
        .bind(tenant_id)
        .fetch_all(&state.pool)
        .await;

    match result {
        Ok(inboxes) => (StatusCode::OK, Json(inboxes)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn get_conversations(
    State(state): State<ChatApiState>,
    Path((tenant_id, inbox_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, ChatConversation>(
        "SELECT * FROM chat_conversations WHERE tenant_id = $1 AND inbox_id = $2"
    )
        .bind(tenant_id)
        .bind(inbox_id)
        .fetch_all(&state.pool)
        .await;

    match result {
        Ok(convos) => (StatusCode::OK, Json(convos)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

#[derive(Deserialize)]
pub struct SendMessagePayload {
    pub content: String,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
}

pub async fn send_message(
    State(state): State<ChatApiState>,
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<SendMessagePayload>,
) -> impl IntoResponse {
    let msg_id = Uuid::new_v4();
    let result = sqlx::query_as::<_, ChatMessage>(
        r#"
        INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    )
        .bind(msg_id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(&payload.sender_type)
        .bind(payload.sender_id)
        .bind(&payload.content)
        .fetch_one(&state.pool)
        .await;

    match result {
        Ok(msg) => (StatusCode::CREATED, Json(msg)).into_response(),
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}
