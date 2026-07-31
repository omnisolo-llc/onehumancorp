use axum::{
    extract::ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatWsMessage {
    pub topic: String,
    pub payload: serde_json::Value,
}

pub async fn chat_ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.next().await {
        if let WsMessage::Text(text) = msg {
            if let Ok(chat_msg) = serde_json::from_str::<ChatWsMessage>(&text) {
                tracing::info!("Received chat ws message: {:?}", chat_msg);
                // In a real application we would broadcast here via redis pubsub
                // Since this module shouldn't import the huge OHC dependencies directly if it doesn't need to,
                // we'll keep the handler generic. The main server binds this handler and injects the pubsub trait.
            }
        }
    }
}
