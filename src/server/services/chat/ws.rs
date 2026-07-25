use axum::{
    extract::{
        ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
        State, Path,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::db::DB;

#[derive(Clone)]
pub struct ChatWsState {
    pub db: Arc<DB>,
}

pub fn router(state: ChatWsState) -> Router {
    Router::new()
        .route("/ws/chat/:tenant_id", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(tenant_id): Path<Uuid>,
    State(state): State<ChatWsState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id, state))
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
enum ClientEvent {
    Join { conversation_id: Uuid },
    SendMessage { conversation_id: Uuid, text: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
enum ServerEvent {
    MessageSent { message_id: Uuid, text: String },
    Error { reason: String },
}

async fn handle_socket(socket: WebSocket, tenant_id: Uuid, state: ChatWsState) {
    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(msg)) = receiver.next().await {
        if let WsMessage::Text(text) = msg {
            if let Ok(event) = serde_json::from_str::<ClientEvent>(&text) {
                match event {
                    ClientEvent::Join { conversation_id } => {
                        // Subscribe to redis/valkey pubsub channel logic here in future
                    },
                    ClientEvent::SendMessage { conversation_id, text } => {
                        // Mock saving and broadcasting
                        let message_id = Uuid::new_v4();
                        let response = ServerEvent::MessageSent {
                            message_id,
                            text,
                        };

                        if let Ok(json) = serde_json::to_string(&response) {
                            let _ = sender.send(WsMessage::Text(json)).await;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn test_ws_events() {
        let ev = ClientEvent::SendMessage {
            conversation_id: Uuid::new_v4(),
            text: "Hello".to_string(),
        };
        let s = serde_json::to_string(&ev).unwrap();
        assert!(s.contains("SendMessage"));
    }
}
