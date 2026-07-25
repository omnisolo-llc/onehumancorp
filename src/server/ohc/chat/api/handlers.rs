use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::chat::domain::{
    models::{Conversation, Message},
    repository::ChatRepository,
};
use sqlx::PgPool;

pub struct AppState {
    pub repository: ChatRepository,
}

pub async fn list_conversations(
    State(state): State<Arc<AppState>>,
    Path((tenant_id, inbox_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<Conversation>>, (StatusCode, String)> {
    match state.repository.list_conversations(tenant_id, inbox_id).await {
        Ok(conversations) => Ok(Json(conversations)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )),
    }
}

pub async fn list_messages(
    State(state): State<Arc<AppState>>,
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    match state.repository.list_messages(tenant_id, conversation_id).await {
        Ok(messages) => Ok(Json(messages)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )),
    }
}

pub async fn send_message(
    State(state): State<Arc<AppState>>,
    Json(message): Json<Message>,
) -> Result<(StatusCode, Json<Message>), (StatusCode, String)> {
    match state.repository.send_message(&message).await {
        Ok(created_message) => Ok((StatusCode::CREATED, Json(created_message))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )),
    }
}
