use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;

use super::service::ChatService;
use super::models::{ChatInbox, ChatContact, ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct ChatAppState {
    pub service: Arc<ChatService>,
}

#[derive(Deserialize)]
pub struct CreateInboxReq {
    pub name: String,
    pub channel_type: String,
    pub channel_config: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CreateContactReq {
    pub name: Option<String>,
    pub identifier: String,
    pub attributes: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CreateConversationReq {
    pub inbox_id: String,
    pub contact_id: String,
}

#[derive(Deserialize)]
pub struct CreateMessageReq {
    pub sender_id: Option<String>,
    pub sender_type: String,
    pub content: Option<String>,
    pub message_type: String,
    pub additional_attributes: Option<serde_json::Value>,
}

pub fn router<S>(pool: PgPool) -> Router<S> where S: Clone + Send + Sync + 'static {
    let state = ChatAppState {
        service: Arc::new(ChatService::new(pool)),
    };

    Router::new()
        .route("/inboxes", post(create_inbox))
        .route("/contacts", post(create_contact))
        .route("/conversations", post(create_conversation))
        .route("/inboxes/:inbox_id/conversations", get(get_inbox_conversations))
        .route("/conversations/:conversation_id/messages", post(create_message).get(get_conversation_messages))
        .with_state(state)
}

async fn create_inbox(
    State(state): State<ChatAppState>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<CreateInboxReq>,
) -> axum::response::Result<Json<ChatInbox>> {
    let tenant_id = claims.organization_id.clone().unwrap_or_default();
    let config = payload.channel_config.unwrap_or_else(|| serde_json::json!({}));
    let res = state.service.create_inbox(&tenant_id, &payload.name, &payload.channel_type, config)
        .await
        .map_err(|e| {
            tracing::error!("Error creating inbox: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(res))
}

async fn create_contact(
    State(state): State<ChatAppState>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<CreateContactReq>,
) -> axum::response::Result<Json<ChatContact>> {
    let tenant_id = claims.organization_id.clone().unwrap_or_default();
    let attrs = payload.attributes.unwrap_or_else(|| serde_json::json!({}));
    let res = state.service.create_contact(&tenant_id, payload.name.as_deref(), &payload.identifier, attrs)
        .await
        .map_err(|e| {
            tracing::error!("Error creating contact: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(res))
}

async fn create_conversation(
    State(state): State<ChatAppState>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<CreateConversationReq>,
) -> axum::response::Result<Json<ChatConversation>> {
    let tenant_id = claims.organization_id.clone().unwrap_or_default();
    let res = state.service.create_conversation(&tenant_id, &payload.inbox_id, &payload.contact_id)
        .await
        .map_err(|e| {
            tracing::error!("Error creating conversation: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(res))
}

async fn create_message(
    State(state): State<ChatAppState>,
    Path(conversation_id): Path<String>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<CreateMessageReq>,
) -> axum::response::Result<Json<ChatMessage>> {
    let tenant_id = claims.organization_id.clone().unwrap_or_default();
    let attrs = payload.additional_attributes.unwrap_or_else(|| serde_json::json!({}));
    let res = state.service.create_message(
        &tenant_id,
        &conversation_id,
        payload.sender_id.as_deref(),
        &payload.sender_type,
        payload.content.as_deref(),
        &payload.message_type,
        attrs
    ).await.map_err(|e| {
        tracing::error!("Error creating message: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(res))
}

async fn get_inbox_conversations(
    State(state): State<ChatAppState>,
    Path(inbox_id): Path<String>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
) -> axum::response::Result<Json<Vec<ChatConversation>>> {
    let tenant_id = claims.organization_id.clone().unwrap_or_default();
    let res = state.service.get_conversations_for_inbox(&tenant_id, &inbox_id)
        .await
        .map_err(|e| {
            tracing::error!("Error getting conversations: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(res))
}

async fn get_conversation_messages(
    State(state): State<ChatAppState>,
    Path(conversation_id): Path<String>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
) -> axum::response::Result<Json<Vec<ChatMessage>>> {
    let tenant_id = claims.organization_id.clone().unwrap_or_default();
    let res = state.service.get_messages_for_conversation(&tenant_id, &conversation_id)
        .await
        .map_err(|e| {
            tracing::error!("Error getting messages: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(res))
}
