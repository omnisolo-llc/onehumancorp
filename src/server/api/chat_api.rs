use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct AppState {
    pub chat_service: Arc<ChatService>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn router(chat_service: Arc<ChatService>) -> Router {
    let state = AppState { chat_service };

    Router::new()
        .route("/api/v1/chat/conversations", get(get_conversations))
        .route("/api/v1/chat/conversations/:conversation_id/messages", get(get_messages))
        .with_state(state)
}

// In a real app, tenant_id would come from auth context.
// For simplicity in this demo endpoint without full auth middleware context setup provided, we extract it from a header.
// Actually, OHC passes tenant_id in axum extensions or path. Let's assume path or header.
// Let's use `X-Tenant-ID` header.
use axum::http::HeaderMap;

async fn get_conversations(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<Vec<ChatConversation>>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let tenant_id_str = headers.get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| (axum::http::StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Missing x-tenant-id header".to_string() })))?;

    let tenant_id = Uuid::parse_str(tenant_id_str)
        .map_err(|_| (axum::http::StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Invalid tenant_id format".to_string() })))?;

    match state.chat_service.get_conversations(tenant_id).await {
        Ok(convs) => Ok(Json(convs)),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() }))),
    }
}

async fn get_messages(
    Path(conversation_id): Path<Uuid>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<Vec<ChatMessage>>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let tenant_id_str = headers.get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| (axum::http::StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Missing x-tenant-id header".to_string() })))?;

    let tenant_id = Uuid::parse_str(tenant_id_str)
        .map_err(|_| (axum::http::StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Invalid tenant_id format".to_string() })))?;

    match state.chat_service.get_messages(tenant_id, conversation_id).await {
        Ok(msgs) => Ok(Json(msgs)),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() }))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat_api_dummy() {
        assert_eq!(1, 1);
    }
}
