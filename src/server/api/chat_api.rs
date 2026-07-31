use axum::{
    extract::{Path, State, Extension},
    routing::{get, post},
    Json, Router,
    response::IntoResponse,
    http::StatusCode,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatInbox, ChatConversation, ChatMessage};
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;
use dashmap::DashMap;

#[derive(Clone)]
pub struct AppState {
    pub chat_service: Arc<ChatService>,
    // Mapping tenant_id to a broadcast channel
    pub active_connections: Arc<DashMap<Uuid, broadcast::Sender<ChatMessage>>>,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
}

pub fn router(pool: PgPool) -> Router {
    let state = AppState {
        chat_service: Arc::new(ChatService::new(pool)),
        active_connections: Arc::new(DashMap::new()),
    };

    Router::new()
        .route("/api/v1/chat/inboxes", get(get_inboxes))
        .route("/api/v1/chat/inboxes/:inbox_id/conversations", get(get_conversations))
        .route("/api/v1/chat/conversations/:conversation_id/messages", get(get_messages).post(send_message))
        .route("/api/v1/chat/ws", get(chat_ws_handler))
        .with_state(state)
}

async fn get_inboxes(
    State(state): State<AppState>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id_str = match claims.organization_id {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(vec![])).into_response(),
    };
    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(vec![])).into_response(),
    };

    // Assuming we would normally fetch inboxes, returning empty or a mock for now
    // In a full implementation, ChatService would have get_inboxes
    (StatusCode::OK, Json(Vec::<ChatInbox>::new())).into_response()
}

async fn get_conversations(
    State(state): State<AppState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(inbox_id_str): Path<String>,
) -> impl IntoResponse {
    let tenant_id_str = match claims.organization_id {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(vec![])).into_response(),
    };
    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(vec![])).into_response(),
    };
    let inbox_id = match Uuid::parse_str(&inbox_id_str) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(vec![])).into_response(),
    };

    match state.chat_service.get_conversations(tenant_id, inbox_id).await {
        Ok(convs) => (StatusCode::OK, Json(convs)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<ChatConversation>::new())).into_response(),
    }
}

async fn get_messages(
    State(state): State<AppState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(conversation_id_str): Path<String>,
) -> impl IntoResponse {
    let tenant_id_str = match claims.organization_id {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(vec![])).into_response(),
    };
    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(vec![])).into_response(),
    };
    let conversation_id = match Uuid::parse_str(&conversation_id_str) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(vec![])).into_response(),
    };

    match state.chat_service.get_messages(tenant_id, conversation_id).await {
        Ok(msgs) => (StatusCode::OK, Json(msgs)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<ChatMessage>::new())).into_response(),
    }
}

async fn send_message(
    State(state): State<AppState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(conversation_id_str): Path<String>,
    Json(payload): Json<SendMessageRequest>,
) -> impl IntoResponse {
    let tenant_id_str = match claims.organization_id {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };
    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid tenant"}))).into_response(),
    };
    let conversation_id = match Uuid::parse_str(&conversation_id_str) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid conv"}))).into_response(),
    };

    let user_id = claims.sub.clone();
    let sender_id = Uuid::parse_str(&user_id).ok();

    match state.chat_service.send_message(tenant_id, conversation_id, "agent".to_string(), sender_id, payload.content).await {
        Ok(msg) => {
            // Broadcast the new message
            if let Some(tx) = state.active_connections.get(&tenant_id) {
                let _ = tx.send(msg.clone());
            }
            (StatusCode::OK, Json(msg)).into_response()
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "failed"}))).into_response(),
    }
}

use axum::extract::Query;

#[derive(Deserialize)]
pub struct WsAuthQuery {
    pub token: String,
}

async fn chat_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(query): Query<WsAuthQuery>,
) -> impl IntoResponse {
    let token = query.token;

    // Validate token to get tenant ID securely
    let http_auth_store = ::server_auth::Store::new(crate::db::get_pool());
    let claims = match http_auth_store.validate_token(&token).await {
        Ok(c) => c,
        Err(_) => return (StatusCode::UNAUTHORIZED, "unauthorized").into_response(),
    };

    let tenant_id_str = match claims.organization_id {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, "unauthorized").into_response(),
    };
    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, "bad request").into_response(),
    };

    ws.on_upgrade(move |socket| handle_ws_connection(socket, state, tenant_id))
}

async fn handle_ws_connection(mut socket: WebSocket, state: AppState, tenant_id: Uuid) {
    let mut rx = {
        let entry = state.active_connections.entry(tenant_id).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        entry.subscribe()
    };

    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    Ok(msg) => {
                        let json = serde_json::to_string(&msg).unwrap_or_default();
                        if socket.send(Message::Text(json)).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        // missed messages, try to catch up
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
            client_msg = socket.recv() => {
                if let Some(Ok(Message::Close(_))) = client_msg {
                    break;
                }
            }
        }
    }
}
