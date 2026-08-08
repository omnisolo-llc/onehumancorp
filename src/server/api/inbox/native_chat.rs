use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DB;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatInbox, ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct NativeChatState {
    pub db: Arc<DB>,
    pub chat_service: Arc<ChatService>,
}

#[derive(Deserialize)]
pub struct SendMessagePayload {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

#[derive(Serialize)]
pub struct SendMessageResponse {
    pub success: bool,
    pub message: Option<ChatMessage>,
}

pub fn router<S>(state: NativeChatState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/inboxes", get(get_inboxes))
        .route("/conversations", get(get_conversations))
        .route("/conversations/:conversation_id/messages", get(get_messages))
        .route("/conversations/:conversation_id/messages", post(send_message))
        .with_state(state)
}

pub async fn get_inboxes(
    State(state): State<NativeChatState>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref org_id) => org_id,
        None => return (StatusCode::UNAUTHORIZED, Json(Vec::<ChatInbox>::new())).into_response(),
    };

    let tenant_uuid = match Uuid::parse_str(tenant_id) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(Vec::<ChatInbox>::new())).into_response(),
    };

    match state.chat_service.get_inboxes(tenant_uuid).await {
        Ok(inboxes) => (StatusCode::OK, Json(inboxes)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch inboxes: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<ChatInbox>::new())).into_response()
        }
    }
}

pub async fn get_conversations(
    State(state): State<NativeChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref org_id) => org_id,
        None => return (StatusCode::UNAUTHORIZED, Json(Vec::<ChatConversation>::new())).into_response(),
    };

    let tenant_uuid = match Uuid::parse_str(tenant_id) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(Vec::<ChatConversation>::new())).into_response(),
    };

    let inbox_id = params.get("inbox_id").and_then(|id| Uuid::parse_str(id).ok());

    match state.chat_service.get_conversations(tenant_uuid, inbox_id).await {
        Ok(conversations) => (StatusCode::OK, Json(conversations)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch conversations: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<ChatConversation>::new())).into_response()
        }
    }
}

pub async fn get_messages(
    State(state): State<NativeChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(conversation_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref org_id) => org_id,
        None => return (StatusCode::UNAUTHORIZED, Json(Vec::<ChatMessage>::new())).into_response(),
    };

    let tenant_uuid = match Uuid::parse_str(tenant_id) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(Vec::<ChatMessage>::new())).into_response(),
    };

    let conv_uuid = match Uuid::parse_str(&conversation_id) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(Vec::<ChatMessage>::new())).into_response(),
    };

    match state.chat_service.get_messages(tenant_uuid, conv_uuid).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch messages: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<ChatMessage>::new())).into_response()
        }
    }
}

pub async fn send_message(
    State(state): State<NativeChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(conversation_id): Path<String>,
    Json(payload): Json<SendMessagePayload>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref org_id) => org_id,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(SendMessageResponse {
                    success: false,
                    message: None,
                }),
            )
                .into_response()
        }
    };

    let tenant_uuid = match Uuid::parse_str(tenant_id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(SendMessageResponse {
                    success: false,
                    message: None,
                }),
            )
                .into_response()
        }
    };

    let conv_uuid = match Uuid::parse_str(&conversation_id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(SendMessageResponse {
                    success: false,
                    message: None,
                }),
            )
                .into_response()
        }
    };

    match state
        .chat_service
        .send_message(
            tenant_uuid,
            conv_uuid,
            payload.sender_type,
            payload.sender_id,
            payload.content,
        )
        .await
    {
        Ok(message) => (
            StatusCode::OK,
            Json(SendMessageResponse {
                success: true,
                message: Some(message),
            }),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SendMessageResponse {
                    success: false,
                    message: None,
                }),
            )
                .into_response()
        }
    }
}
