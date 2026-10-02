use crate::db::DB;
use crate::domain::repository::omnichannel_repo::OmniChannelRepo;
use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

pub struct WidgetChatState {
    pub repo: OmniChannelRepo,
}

impl WidgetChatState {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            repo: OmniChannelRepo::new(db),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub tenant_id: Uuid,
    pub channel: Option<String>,
}

#[derive(Serialize)]
pub struct CreateConversationResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub channel: String,
    pub status: String,
}

pub async fn create_conversation(
    State(state): State<Arc<WidgetChatState>>,
    Json(req): Json<CreateConversationRequest>,
) -> Result<Json<CreateConversationResponse>, axum::http::StatusCode> {
    let channel = req.channel.unwrap_or_else(|| "widget".to_string());

    match state
        .repo
        .create_conversation(req.tenant_id, None, None, channel, "OPEN".to_string())
        .await
    {
        Ok(conv) => Ok(Json(CreateConversationResponse {
            id: conv.id,
            tenant_id: conv.tenant_id,
            channel: conv.channel,
            status: conv.status,
        })),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub direction: Option<String>,
    pub content: String,
}

#[derive(Serialize)]
pub struct CreateMessageResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub direction: String,
    pub content: String,
}

pub async fn create_message(
    State(state): State<Arc<WidgetChatState>>,
    Json(req): Json<CreateMessageRequest>,
) -> Result<Json<CreateMessageResponse>, axum::http::StatusCode> {
    let direction = req.direction.unwrap_or_else(|| "INBOUND".to_string());

    match state
        .repo
        .create_message(req.tenant_id, req.conversation_id, direction, req.content)
        .await
    {
        Ok(msg) => Ok(Json(CreateMessageResponse {
            id: msg.id,
            tenant_id: msg.tenant_id,
            conversation_id: msg.conversation_id,
            direction: msg.direction,
            content: msg.content,
        })),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_messages(
    State(state): State<Arc<WidgetChatState>>,
    Path((_tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<CreateMessageResponse>>, axum::http::StatusCode> {
    match state
        .repo
        .get_messages_by_conversation_id(conversation_id)
        .await
    {
        Ok(messages) => {
            let res = messages
                .into_iter()
                .map(|msg| CreateMessageResponse {
                    id: msg.id,
                    tenant_id: msg.tenant_id,
                    conversation_id: msg.conversation_id,
                    direction: msg.direction,
                    content: msg.content,
                })
                .collect();
            Ok(Json(res))
        }
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
