use axum::{
    extract::{State, WebSocketUpgrade, ws::{WebSocket, Message as WsMessage}},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct ChatWsState {
    pub db: Arc<crate::db::DB>,
    pub redis_client: Option<redis::Client>,
}

#[derive(Deserialize)]
pub struct ChatWsConnectPayload {
    pub token: String,
}

pub fn router(state: ChatWsState) -> Router {
    Router::new()
        .route("/api/v1/omnichannel/chat/ws", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<ChatWsState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: ChatWsState) {
    while let Some(Ok(msg)) = socket.next().await {
        if let WsMessage::Text(text) = msg {
            let _ = socket.send(WsMessage::Text(format!("Echo: {}", text))).await;
        }
    }
}
