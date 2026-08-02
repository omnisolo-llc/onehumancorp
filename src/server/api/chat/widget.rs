use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::db::DB;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatMessage, ChatChannel};

#[derive(Deserialize)]
pub struct WidgetConfigQuery {
    pub inbox_id: Uuid,
    pub tenant_id: Uuid,
}

#[derive(Serialize)]
pub struct WidgetConfigResponse {
    pub success: bool,
    pub config: Option<serde_json::Value>,
}

pub async fn get_widget_config_handler(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<WidgetConfigQuery>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(&query.tenant_id.to_string()) {
        return (StatusCode::UNAUTHORIZED, Json(WidgetConfigResponse {
            success: false,
            config: None,
        })).into_response();
    }

    let service = ChatService::new(db.pool.clone());

    match service.get_channel_by_inbox(query.tenant_id, query.inbox_id, "WebWidget").await {
        Ok(Some(channel)) => {
            (StatusCode::OK, Json(WidgetConfigResponse {
                success: true,
                config: Some(channel.config),
            })).into_response()
        }
        Ok(None) => {
            (StatusCode::NOT_FOUND, Json(WidgetConfigResponse {
                success: false,
                config: None,
            })).into_response()
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(WidgetConfigResponse {
                success: false,
                config: None,
            })).into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct SessionRequest {
    pub inbox_id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize)]
pub struct SessionResponse {
    pub success: bool,
    pub contact_id: Option<Uuid>,
    pub conversation_id: Option<Uuid>,
}

pub async fn create_session_handler(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<SessionRequest>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(&payload.tenant_id.to_string()) {
        return (StatusCode::UNAUTHORIZED, Json(SessionResponse { success: false, contact_id: None, conversation_id: None })).into_response();
    }

    let service = ChatService::new(db.pool.clone());

    let contact = match service.upsert_contact(
        payload.tenant_id,
        payload.email,
        payload.name,
        payload.phone
    ).await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(SessionResponse { success: false, contact_id: None, conversation_id: None })).into_response(),
    };

    let conversation = match service.start_conversation(
        payload.tenant_id,
        payload.inbox_id,
        contact.id,
        None
    ).await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(SessionResponse { success: false, contact_id: None, conversation_id: None })).into_response(),
    };

    (StatusCode::OK, Json(SessionResponse {
        success: true,
        contact_id: Some(contact.id),
        conversation_id: Some(conversation.id),
    })).into_response()
}

#[derive(Deserialize)]
pub struct MessagesQuery {
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
}

#[derive(Serialize)]
pub struct MessagesResponse {
    pub success: bool,
    pub messages: Vec<ChatMessage>,
}

use server_common::Claims;
use axum::extract::Extension;

pub async fn get_messages_handler(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<MessagesQuery>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(&query.tenant_id.to_string()) {
        return (StatusCode::UNAUTHORIZED, Json(MessagesResponse {
            success: false,
            messages: vec![],
        })).into_response();
    }

    let service = ChatService::new(db.pool.clone());

    match service.get_messages(query.tenant_id, query.conversation_id).await {
        Ok(messages) => {
            (StatusCode::OK, Json(MessagesResponse {
                success: true,
                messages,
            })).into_response()
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(MessagesResponse {
                success: false,
                messages: vec![],
            })).into_response()
        }
    }
}
