use axum::{
    extract::{Path, State, ws::{WebSocketUpgrade, WebSocket, Message as WsMessage}},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;
use super::service::InboxService;
use serde::{Deserialize, Serialize};

pub struct AppState {
    pub inbox_service: Arc<InboxService>,
    pub tx: broadcast::Sender<String>,
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub sender_type: String,
    pub content: String,
}

pub async fn create_message(
    State(state): State<Arc<AppState>>,
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CreateMessageRequest>,
) -> impl IntoResponse {
    match state.inbox_service.create_message(tenant_id, conversation_id, &payload.sender_type, &payload.content).await {
        Ok(msg) => {
            let _ = state.tx.send(json!({"event": "message.created", "data": msg}).to_string());
            Json(json!({"success": true, "message": msg}))
        }
        Err(e) => Json(json!({"success": false, "error": e.to_string()})),
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx = state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        if socket.send(WsMessage::Text(msg)).await.is_err() {
            break;
        }
    }
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/tenant/:tenant_id/conversations/:conversation_id/messages", post(create_message))
        .route("/ws", get(ws_handler))
        .with_state(state)
}
