use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};

use super::handlers::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct WsMessage {
    pub tenant_id: String,
    pub payload: serde_json::Value,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, _state: AppState) {
    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(text) = msg {
            if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                // Here we would typically publish to NATS or Redis PubSub
                println!("Received message for tenant: {}", ws_msg.tenant_id);
                let _ = socket
                    .send(Message::Text(serde_json::to_string(&ws_msg).unwrap()))
                    .await;
            }
        }
    }
}
