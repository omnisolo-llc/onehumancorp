use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    pub tenant_id: String,
    pub user_id: String,
    pub content: String,
    pub platform: String,
}

pub async fn chat_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(text) = msg {
            if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&text) {
                // Here we would integrate with the actual broadcasting logic
                let reply = format!("Received: {}", chat_msg.content);
                if socket.send(Message::Text(reply.into())).await.is_err() {
                    break;
                }
            }
        }
    }
}
