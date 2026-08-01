use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State, Path
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::api::ChatApiState;

pub async fn chat_ws_handler(
    ws: WebSocketUpgrade,
    Path(tenant_id): Path<Uuid>,
    State(state): State<ChatApiState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id, state))
}

async fn handle_socket(mut socket: WebSocket, _tenant_id: Uuid, _state: ChatApiState) {
    while let Some(msg) = socket.next().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    // Echo back for now or process incoming live typing events
                    if socket.send(Message::Text(format!("Echo: {}", t).into())).await.is_err() {
                        break;
                    }
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        } else {
            break;
        }
    }
}
