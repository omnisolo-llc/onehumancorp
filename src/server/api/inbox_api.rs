use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::services::inbox::service::{InboxService, UnifiedTriageAction};
use crate::domain::repository::omnichannel_repo::{OmniChannelRepo, Conversation, Message};
use crate::db::DB;

#[derive(Clone)]
pub struct AppState {
    pub inbox_service: Arc<InboxService>,
    pub omni_repo: Arc<OmniChannelRepo>,
}

#[derive(Deserialize)]
pub struct ResolveActionRequest {
    pub tenant_id: String,
    pub resolution: String, // "approved", "rejected", "edited"
}

#[derive(Serialize)]
pub struct OmnichannelInboxResponse {
    pub conversations: Vec<Conversation>,
}

#[derive(Serialize)]
pub struct ConversationMessagesResponse {
    pub messages: Vec<Message>,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
}

pub fn router(pool: PgPool) -> Router {
    let db = Arc::new(DB { pool: pool.clone() });
    let state = AppState {
        inbox_service: Arc::new(InboxService::new(pool)),
        omni_repo: Arc::new(OmniChannelRepo::new(db)),
    };

    Router::new()
        .route("/api/v1/inbox/{tenant_id}/actions", get(get_pending_actions))
        .route("/api/v1/inbox/{tenant_id}/actions/{action_id}/resolve", post(resolve_action))
        // New omnichannel chat endpoints
        .route("/api/v1/omnichannel/{tenant_id}/conversations", get(get_conversations))
        .route("/api/v1/omnichannel/{tenant_id}/conversations/{conversation_id}/messages", get(get_messages))
        .route("/api/v1/omnichannel/{tenant_id}/messages", post(send_message))
        .with_state(state)
}

async fn get_pending_actions(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> Result<Json<Vec<UnifiedTriageAction>>, axum::http::StatusCode> {
    match state.inbox_service.get_pending_actions(&tenant_id).await {
        Ok(actions) => Ok(Json(actions)),
        Err(e) => {
            tracing::error!("Failed to fetch pending triage actions: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn resolve_action(
    State(state): State<AppState>,
    Path((tenant_id, action_id)): Path<(String, String)>,
    Json(payload): Json<ResolveActionRequest>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    if tenant_id != payload.tenant_id {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
    match state.inbox_service.resolve_action(&tenant_id, &action_id, &payload.resolution).await {
        Ok(_) => Ok(axum::http::StatusCode::OK),
        Err(e) => {
            tracing::error!("Failed to resolve triage action: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_conversations(
    State(state): State<AppState>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<OmnichannelInboxResponse>, axum::http::StatusCode> {
    match state.omni_repo.get_conversations_by_tenant_id(tenant_id).await {
        Ok(conversations) => Ok(Json(OmnichannelInboxResponse { conversations })),
        Err(e) => {
            tracing::error!("Failed to get conversations: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_messages(
    State(state): State<AppState>,
    Path((_tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ConversationMessagesResponse>, axum::http::StatusCode> {
    match state.omni_repo.get_messages_by_conversation_id(conversation_id).await {
        Ok(messages) => Ok(Json(ConversationMessagesResponse { messages })),
        Err(e) => {
            tracing::error!("Failed to get messages: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn send_message(
    State(state): State<AppState>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    if tenant_id != payload.tenant_id {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
    match state.omni_repo.create_message(tenant_id, payload.conversation_id, "OUTBOUND".to_string(), payload.content).await {
        Ok(_) => {
            Ok(axum::http::StatusCode::OK)
        },
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
