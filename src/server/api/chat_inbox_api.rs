use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::chat::service::ChatService;
use crate::AppState;

#[derive(Deserialize)]
pub struct SendMessageReq {
    pub content: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/chat/conversations", get(list_conversations))
        .route("/api/v1/chat/conversations/:id/messages", get(list_messages))
        .route("/api/v1/chat/conversations/:id/messages", post(send_message))
}

async fn list_conversations(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    // Hardcode tenant_id for simplicity in this task
    let tenant_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let service = ChatService::new(state.db_pool.clone());

    match service.get_conversations(tenant_id).await {
        Ok(conversations) => Ok(Json(serde_json::json!({ "conversations": conversations }))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let service = ChatService::new(state.db_pool.clone());

    match service.get_messages(tenant_id, conversation_id).await {
        Ok(messages) => Ok(Json(serde_json::json!({ "messages": messages }))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn send_message(
    State(state): State<AppState>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<SendMessageReq>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let service = ChatService::new(state.db_pool.clone());

    match service.create_message(tenant_id, conversation_id, "agent".to_string(), payload.content).await {
        Ok(msg) => Ok(Json(serde_json::json!({ "message": msg }))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
