use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;
use crate::services::chat::service::ChatService;
use crate::db::DB;
use ::server_common::Claims;
use crate::strict_ui_claim_tenant;

#[derive(Clone)]
pub struct ChatInboxState {
    pub chat_service: Arc<ChatService>,
}

#[derive(Deserialize)]
pub struct SendMessagePayload {
    pub sender_type: String,
    pub sender_id: Option<String>,
    pub content: String,
}

pub fn router(db: Arc<DB>) -> Router {
    let state = ChatInboxState {
        chat_service: Arc::new(ChatService::new(db.pool.clone())),
    };

    Router::new()
        .route("/api/v1/ui/chat/conversations", get(get_conversations))
        .route("/api/v1/ui/chat/conversations/:id/messages", get(get_messages))
        .route("/api/v1/ui/chat/conversations/:id/messages", post(send_message))
        .with_state(state)
}

async fn get_conversations(
    State(state): State<ChatInboxState>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id_str: String = match strict_ui_claim_tenant(&claims).map(|s: String| s) {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid tenant id"}))).into_response(),
    };

    match state.chat_service.get_conversations(tenant_id).await {
        Ok(conversations) => (StatusCode::OK, Json(conversations)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch conversations: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "failed to fetch conversations"}))).into_response()
        }
    }
}

async fn get_messages(
    State(state): State<ChatInboxState>,
    Extension(claims): Extension<Claims>,
    Path(conversation_id_str): Path<String>,
) -> impl IntoResponse {
    let tenant_id_str: String = match strict_ui_claim_tenant(&claims).map(|s: String| s) {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid tenant id"}))).into_response(),
    };

    let conversation_id = match Uuid::parse_str(&conversation_id_str) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid conversation id"}))).into_response(),
    };

    match state.chat_service.get_messages(tenant_id, conversation_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch messages: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "failed to fetch messages"}))).into_response()
        }
    }
}

async fn send_message(
    State(state): State<ChatInboxState>,
    Extension(claims): Extension<Claims>,
    Path(conversation_id_str): Path<String>,
    Json(payload): Json<SendMessagePayload>,
) -> impl IntoResponse {
    let tenant_id_str: String = match strict_ui_claim_tenant(&claims).map(|s: String| s) {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid tenant id"}))).into_response(),
    };

    let conversation_id = match Uuid::parse_str(&conversation_id_str) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid conversation id"}))).into_response(),
    };

    let sender_id = payload.sender_id.and_then(|id| Uuid::parse_str(&id).ok());

    match state.chat_service.send_message(tenant_id, conversation_id, payload.sender_type, sender_id, payload.content).await {
        Ok(message) => (StatusCode::OK, Json(message)).into_response(),
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "failed to send message"}))).into_response()
        }
    }
}
