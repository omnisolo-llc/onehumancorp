use axum::{
    extract::{Path, State},
    routing::{post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatInbox, ChatMessage, ChatContact, ChatChannel, ChatConversation};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
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
pub struct ReceiveMessageRequest {
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub is_off_hours: bool,
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct StartConversationRequest {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub bot_assignee_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct InboxResponse {
    pub inbox: ChatInbox,
}

pub fn router(pool: PgPool) -> Router {
    let state = AppState {
        chat_service: Arc::new(ChatService::new(pool)),
    };

    Router::new()
        .route("/api/v1/chat_omni/:tenant_id/inbox", post(create_inbox))
        .route("/api/v1/chat_omni/:tenant_id/inbox/:inbox_id/config", put(update_inbox_config))
        .route("/api/v1/chat_omni/:tenant_id/inbox/:inbox_id/receive", post(receive_message))
        .route("/api/v1/chat_omni/:tenant_id/conversation", post(start_conversation))
        .with_state(state)
}

async fn create_inbox(
    State(state): State<AppState>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<CreateInboxRequest>,
) -> Result<Json<InboxResponse>, axum::http::StatusCode> {
    match state.chat_service.create_inbox(tenant_id, payload.name).await {
        Ok(inbox) => Ok(Json(InboxResponse { inbox })),
        Err(e) => {
            tracing::error!("Failed to create inbox: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn update_inbox_config(
    State(state): State<AppState>,
    Path((tenant_id, inbox_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateInboxConfigRequest>,
) -> Result<Json<ChatInbox>, axum::http::StatusCode> {
    match state.chat_service.update_inbox_config(
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

async fn receive_message(
    State(state): State<AppState>,
    Path((tenant_id, inbox_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<ReceiveMessageRequest>,
) -> Result<Json<ChatMessage>, axum::http::StatusCode> {
    match state.chat_service.receive_message(
        tenant_id,
        inbox_id,
        payload.conversation_id,
        payload.sender_type,
        payload.sender_id,
        payload.content,
        payload.is_off_hours,
    ).await {
        Ok(msg) => Ok(Json(msg)),
        Err(e) => {
            tracing::error!("Failed to receive message: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn start_conversation(
    State(state): State<AppState>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<StartConversationRequest>,
) -> Result<Json<ChatConversation>, axum::http::StatusCode> {
    match state.chat_service.start_conversation(
        tenant_id,
        payload.inbox_id,
        payload.contact_id,
        payload.assignee_id,
        payload.bot_assignee_id,
    ).await {
        Ok(conv) => Ok(Json(conv)),
        Err(e) => {
            tracing::error!("Failed to start conversation: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
