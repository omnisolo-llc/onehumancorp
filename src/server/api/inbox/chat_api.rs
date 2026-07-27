use axum::{
    extract::{Path, State},
    routing::{put, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatInbox, ChatConversation};

#[derive(Clone)]
pub struct ChatAppState {
    pub chat_service: Arc<ChatService>,
}

#[derive(Deserialize)]
pub struct UpdateInboxConfigRequest {
    pub working_hours_enabled: Option<bool>,
    pub out_of_office_message: Option<String>,
    pub greeting_enabled: Option<bool>,
    pub greeting_message: Option<String>,
}

#[derive(Deserialize)]
pub struct AssignConversationRequest {
    pub assignee_id: Option<Uuid>,
    pub bot_assignee_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct UpdateConversationStatusRequest {
    pub status: String,
}

pub fn router(pool: PgPool) -> Router {
    let state = ChatAppState {
        chat_service: Arc::new(ChatService::new(pool)),
    };

    Router::new()
        .route("/api/v1/chat/:tenant_id/inbox/:inbox_id", put(update_inbox_config))
        .route("/api/v1/chat/:tenant_id/conversation/:conversation_id/assign", post(assign_conversation))
        .route("/api/v1/chat/:tenant_id/conversation/:conversation_id/status", put(update_conversation_status))
        .with_state(state)
}

async fn update_inbox_config(
    State(state): State<ChatAppState>,
    Path((tenant_id_str, inbox_id_str)): Path<(String, String)>,
    Json(payload): Json<UpdateInboxConfigRequest>,
) -> Result<Json<ChatInbox>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    let inbox_id = Uuid::parse_str(&inbox_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    match state.chat_service.update_inbox(
        tenant_id,
        inbox_id,
        payload.working_hours_enabled,
        payload.out_of_office_message,
        payload.greeting_enabled,
        payload.greeting_message,
    ).await {
        Ok(inbox) => Ok(Json(inbox)),
        Err(e) => {
            tracing::error!("Failed to update inbox config: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn assign_conversation(
    State(state): State<ChatAppState>,
    Path((tenant_id_str, conversation_id_str)): Path<(String, String)>,
    Json(payload): Json<AssignConversationRequest>,
) -> Result<Json<ChatConversation>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    let conversation_id = Uuid::parse_str(&conversation_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    match state.chat_service.assign_conversation(
        tenant_id,
        conversation_id,
        payload.assignee_id,
        payload.bot_assignee_id,
    ).await {
        Ok(conversation) => Ok(Json(conversation)),
        Err(e) => {
            tracing::error!("Failed to assign conversation: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn update_conversation_status(
    State(state): State<ChatAppState>,
    Path((tenant_id_str, conversation_id_str)): Path<(String, String)>,
    Json(payload): Json<UpdateConversationStatusRequest>,
) -> Result<Json<ChatConversation>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    let conversation_id = Uuid::parse_str(&conversation_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    match state.chat_service.update_conversation_status(
        tenant_id,
        conversation_id,
        payload.status,
    ).await {
        Ok(conversation) => Ok(Json(conversation)),
        Err(e) => {
            tracing::error!("Failed to update conversation status: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
