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
use crate::domain::repository::omnichannel_repo::{OmniChannelRepo, Inbox, Channel, Contact, Conversation, Message};
use crate::db::DB;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct AppState {
    pub inbox_service: Arc<InboxService>,
    pub omnichannel_repo: Arc<OmniChannelRepo>,
}

#[derive(Deserialize)]
pub struct ResolveActionRequest {
    pub tenant_id: String,
    pub resolution: String, // "approved", "rejected", "edited"
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateChannelRequest {
    pub channel_type: String,
    pub config: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CreateContactRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub channel: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub direction: String,
    pub content: String,
}

pub fn router(pool: PgPool) -> Router {
    let db = Arc::new(DB { pool: pool.clone() });
    let state = AppState {
        inbox_service: Arc::new(InboxService::new(pool)),
        omnichannel_repo: Arc::new(OmniChannelRepo::new(db)),
    };

    Router::new()
        .route("/api/v1/inbox/:tenant_id/actions", get(get_pending_actions))
        .route("/api/v1/inbox/:tenant_id/actions/:action_id/resolve", post(resolve_action))
        .route("/api/v1/inboxes/:tenant_id", post(create_inbox))
        .route("/api/v1/inboxes/:tenant_id/:inbox_id/channels", post(create_channel))
        .route("/api/v1/contacts/:tenant_id", post(create_contact))
        .route("/api/v1/conversations/:tenant_id", post(create_conversation))
        .route("/api/v1/conversations/:tenant_id/:conversation_id/messages", get(get_messages))
        .route("/api/v1/conversations/:tenant_id/:conversation_id/messages", post(create_message))
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

async fn create_inbox(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
    Json(payload): Json<CreateInboxRequest>,
) -> Result<Json<Inbox>, axum::http::StatusCode> {
    let tenant_uuid = match Uuid::parse_str(&tenant_id) {
        Ok(u) => u,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    match state.omnichannel_repo.create_inbox(tenant_uuid, payload.name).await {
        Ok(inbox) => Ok(Json(inbox)),
        Err(e) => {
            tracing::error!("Failed to create inbox: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_channel(
    State(state): State<AppState>,
    Path((tenant_id, inbox_id)): Path<(String, String)>,
    Json(payload): Json<CreateChannelRequest>,
) -> Result<Json<Channel>, axum::http::StatusCode> {
    let tenant_uuid = match Uuid::parse_str(&tenant_id) {
        Ok(u) => u,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };
    let inbox_uuid = match Uuid::parse_str(&inbox_id) {
        Ok(u) => u,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    match state.omnichannel_repo.create_channel(tenant_uuid, inbox_uuid, payload.channel_type, payload.config).await {
        Ok(channel) => Ok(Json(channel)),
        Err(e) => {
            tracing::error!("Failed to create channel: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_contact(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
    Json(payload): Json<CreateContactRequest>,
) -> Result<Json<Contact>, axum::http::StatusCode> {
    let tenant_uuid = match Uuid::parse_str(&tenant_id) {
        Ok(u) => u,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    match state.omnichannel_repo.create_contact(tenant_uuid, payload.name, payload.email, payload.phone).await {
        Ok(contact) => Ok(Json(contact)),
        Err(e) => {
            tracing::error!("Failed to create contact: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_conversation(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
    Json(payload): Json<CreateConversationRequest>,
) -> Result<Json<Conversation>, axum::http::StatusCode> {
    let tenant_uuid = match Uuid::parse_str(&tenant_id) {
        Ok(u) => u,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    match state.omnichannel_repo.create_conversation(tenant_uuid, payload.channel, payload.status).await {
        Ok(conversation) => Ok(Json(conversation)),
        Err(e) => {
            tracing::error!("Failed to create conversation: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_messages(
    State(state): State<AppState>,
    Path((tenant_id, conversation_id)): Path<(String, String)>,
) -> Result<Json<Vec<Message>>, axum::http::StatusCode> {
    let _tenant_uuid = match Uuid::parse_str(&tenant_id) {
        Ok(u) => u,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };
    let conv_uuid = match Uuid::parse_str(&conversation_id) {
        Ok(u) => u,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    match state.omnichannel_repo.get_messages_by_conversation_id(conv_uuid).await {
        Ok(messages) => Ok(Json(messages)),
        Err(e) => {
            tracing::error!("Failed to fetch messages: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_message(
    State(state): State<AppState>,
    Path((tenant_id, conversation_id)): Path<(String, String)>,
    Json(payload): Json<CreateMessageRequest>,
) -> Result<Json<Message>, axum::http::StatusCode> {
    let tenant_uuid = match Uuid::parse_str(&tenant_id) {
        Ok(u) => u,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };
    let conv_uuid = match Uuid::parse_str(&conversation_id) {
        Ok(u) => u,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    match state.omnichannel_repo.create_message(tenant_uuid, conv_uuid, payload.direction.clone(), payload.content.clone()).await {
        Ok(msg) => {
            // Redis pub/sub logic for websockets
            let event_payload = serde_json::json!({
                "type": "message_created",
                "message": {
                    "id": msg.id,
                    "conversation_id": msg.conversation_id,
                    "direction": msg.direction,
                    "content": msg.content,
                }
            });
            let topic = format!("conversations:{}", tenant_id);
            if let Some(mut redis_client) = crate::redis_pool::get_redis_client() {
                let _: Result<(), _> = redis_client.publish(topic, event_payload.to_string()).await;
            }

            // Also create AI draft if inbound
            if payload.direction == "inbound" {
                let _ = state.omnichannel_repo.create_ai_draft(tenant_uuid, msg.id, String::new(), "DRAFT".to_string()).await;
            }
            Ok(Json(msg))
        },
        Err(e) => {
            tracing::error!("Failed to create message: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
