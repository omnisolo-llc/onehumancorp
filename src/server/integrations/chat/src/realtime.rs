use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::Response,
};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatEvent {
    pub event_type: String,
    pub payload: serde_json::Value,
}

pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.next().await {
        if let Ok(msg) = msg {
            if let Message::Text(text) = msg {
                tracing::info!("Received ws text: {}", text);
                // Echo for basic testing
                if let Err(e) = socket.send(Message::Text(text)).await {
                    tracing::error!("Error sending ws message: {:?}", e);
                    break;
                }
            }
        } else {
            break;
        }
    }
}
