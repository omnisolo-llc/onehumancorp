use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::api::inbox_api::AppState;
use crate::services::inbox::chat_models::{Conversation, Message};
use crate::services::inbox::chat_repository::ChatRepository;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/inbox/:tenant_id/conversations", get(get_conversations))
        .route("/api/v1/inbox/:tenant_id/conversations/:conversation_id/messages", get(get_messages))
        .route("/api/v1/inbox/:tenant_id/conversations/:conversation_id/messages", post(insert_message))
}

async fn get_conversations(
    Path(tenant_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Conversation>>, (StatusCode, String)> {
    let pool = state.inbox_service.get_pool();
    let convos = ChatRepository::get_conversations(pool, &tenant_id)
        .await
        .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(convos))
}

async fn get_messages(
    Path((tenant_id, conversation_id)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    let pool = state.inbox_service.get_pool();
    let msgs = ChatRepository::get_messages(pool, &tenant_id, &conversation_id)
        .await
        .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(msgs))
}

#[derive(serde::Deserialize)]
pub struct CreateMessageRequest {
    pub sender_type: String,
    pub sender_id: Option<String>,
    pub content: Option<String>,
    pub message_type: String,
}

async fn insert_message(
    Path((tenant_id, conversation_id)): Path<(String, String)>,
    State(state): State<AppState>,
    Json(payload): Json<CreateMessageRequest>,
) -> Result<Json<Message>, (StatusCode, String)> {
    let pool = state.inbox_service.get_pool();

    let msg = Message {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        conversation_id,
        sender_type: payload.sender_type,
        sender_id: payload.sender_id,
        content: payload.content,
        message_type: payload.message_type,
        created_at: None,
        updated_at: None,
    };

    ChatRepository::insert_message(pool, &tenant_id, msg.clone())
        .await
        .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(msg))
}
