use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    http::StatusCode,
    routing::get,
    Router,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use sqlx::PgPool;
use ::server_common::Claims;
use crate::services::chat::service::ChatService;
use std::sync::Arc;

#[derive(Clone)]
pub struct ChatApiState {
    pub service: Arc<ChatService>,
}

pub fn router<S>(pool: PgPool) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let service = Arc::new(ChatService::new(pool));
    let state = ChatApiState { service };

    Router::new()
        .route("/conversations", get(list_conversations).post(start_conversation))
        .route("/ws", get(ws_handler))
        .route("/conversations/:id/messages", get(list_messages).post(send_message))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct StartConversationReq {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
}

async fn start_conversation(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<StartConversationReq>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        Some(org_id) => org_id,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    match state.service.start_conversation(
        tenant_id,
        payload.inbox_id,
        payload.contact_id,
        payload.assignee_id,
    ).await {
        Ok(conv) => (StatusCode::CREATED, Json(conv)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn list_conversations(
    State(_state): State<ChatApiState>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let _tenant_id = match claims.organization_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        Some(org_id) => org_id,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    // For simplicity, implement list_conversations directly here or add it to ChatService
    (StatusCode::NOT_IMPLEMENTED, "Not Implemented").into_response()
}

#[derive(Deserialize)]
pub struct SendMessageReq {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

async fn send_message(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<Claims>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<SendMessageReq>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        Some(org_id) => org_id,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    match state.service.send_message(
        tenant_id,
        conversation_id,
        payload.sender_type,
        payload.sender_id,
        payload.content,
    ).await {
        Ok(msg) => (StatusCode::CREATED, Json(msg)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn list_messages(
    State(_state): State<ChatApiState>,
    Extension(claims): Extension<Claims>,
    Path(_conversation_id): Path<Uuid>,
) -> impl IntoResponse {
    let _tenant_id = match claims.organization_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        Some(org_id) => org_id,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    // For simplicity, implement list_messages directly here or add it to ChatService
    (StatusCode::NOT_IMPLEMENTED, "Not Implemented").into_response()
}

use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::response::Response;
use futures::stream::StreamExt;

#[derive(Clone)]
struct ChatWsState {
    // In a real implementation, you'd use a better pub/sub like Redis or in-memory broadcast channels.
    // This is just a stub for compiling.
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(_state): State<ChatApiState>,
    Extension(claims): Extension<Claims>,
) -> Response {
    let tenant_id = match claims.organization_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        Some(org_id) => org_id,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };
    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id))
}

async fn handle_socket(mut socket: WebSocket, _tenant_id: Uuid) {
    while let Some(msg) = socket.next().await {
        if let Ok(msg) = msg {
            // Echo back for now
            if socket.send(msg).await.is_err() {
                break;
            }
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod test;
