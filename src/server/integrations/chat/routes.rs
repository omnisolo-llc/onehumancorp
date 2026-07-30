use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
    http::StatusCode,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use super::models::{ChatInbox, ChatConversation, ChatMessage};

pub struct AppState {
    pub db: PgPool,
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/chat/inboxes", get(get_inboxes))
        .route("/api/chat/conversations", get(get_conversations))
        .route("/api/chat/conversations/:id/messages", get(get_messages).post(send_message))
        .with_state(state)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

async fn get_inboxes(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ChatInbox>>, (StatusCode, String)> {
    let result: Vec<ChatInbox> = sqlx::query_as(
        "SELECT * FROM chat_inboxes ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)?;

    Ok(Json(result))
}

async fn get_conversations(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ChatConversation>>, (StatusCode, String)> {
    let result: Vec<ChatConversation> = sqlx::query_as(
        "SELECT * FROM chat_conversations ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)?;

    Ok(Json(result))
}

async fn get_messages(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ChatMessage>>, (StatusCode, String)> {
    let result: Vec<ChatMessage> = sqlx::query_as(
        "SELECT * FROM chat_messages WHERE conversation_id = $1 ORDER BY created_at ASC"
    )
    .bind(id)
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)?;

    Ok(Json(result))
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub tenant_id: Uuid,
}

async fn send_message(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<ChatMessage>, (StatusCode, String)> {

    let mut draft_reply = None;
    if payload.content.to_lowercase().contains("vegan") {
        draft_reply = Some("Hi! Yes, we have vegan options available. Would you like to see our vegan menu?".to_string());
    }

    let final_content = draft_reply.unwrap_or(payload.content);
    let msg_id = Uuid::new_v4();

    let result: ChatMessage = sqlx::query_as(
        "INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(msg_id)
    .bind(payload.tenant_id)
    .bind(id)
    .bind(payload.sender_type)
    .bind(payload.sender_id)
    .bind(final_content)
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?;

    Ok(Json(result))
}
