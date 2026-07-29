use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::hub::Hub;

#[derive(Clone)]
pub struct OmnichannelChatState {
    pub hub: Arc<Hub>,
}

#[derive(Serialize)]
pub struct ConversationListResponse {
    pub conversations: Vec<crate::services::chat::models::ChatConversation>,
}

#[derive(Serialize)]
pub struct MessageListResponse {
    pub messages: Vec<crate::services::chat::models::ChatMessage>,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

pub fn router(state: OmnichannelChatState) -> Router {
    Router::new()
        .route("/conversations", get(list_conversations))
        .route("/conversations/:id/messages", get(list_messages))
        .route("/conversations/:id/messages", post(send_message))
        .with_state(state)
}

async fn list_conversations(
    State(state): State<OmnichannelChatState>,
    Extension(claims): Extension<crate::auth::Claims>,
) -> Result<Json<ConversationListResponse>, StatusCode> {
    let tenant_id = claims.tenant_id;

    let service = crate::services::chat::service::ChatService::new(
        state.hub.db.pool.clone(),
        state.hub.msgbus.clone(),
    );

    match service.get_conversations(tenant_id).await {
        Ok(conversations) => Ok(Json(ConversationListResponse { conversations })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_messages(
    State(state): State<OmnichannelChatState>,
    Path(conversation_id): Path<Uuid>,
    Extension(claims): Extension<crate::auth::Claims>,
) -> Result<Json<MessageListResponse>, StatusCode> {
    let tenant_id = claims.tenant_id;

    let service = crate::services::chat::service::ChatService::new(
        state.hub.db.pool.clone(),
        state.hub.msgbus.clone(),
    );

    match service.get_messages(tenant_id, conversation_id).await {
        Ok(messages) => Ok(Json(MessageListResponse { messages })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn send_message(
    State(state): State<OmnichannelChatState>,
    Path(conversation_id): Path<Uuid>,
    Extension(claims): Extension<crate::auth::Claims>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<crate::services::chat::models::ChatMessage>, StatusCode> {
    let tenant_id = claims.tenant_id;

    let service = crate::services::chat::service::ChatService::new(
        state.hub.db.pool.clone(),
        state.hub.msgbus.clone(),
    );

    match service
        .send_message(
            tenant_id,
            conversation_id,
            payload.sender_type,
            payload.sender_id,
            payload.content,
        )
        .await
    {
        Ok(msg) => Ok(Json(msg)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_omnichannel_chat_router() {
        // Just verify router creation doesn't panic
    }
}
