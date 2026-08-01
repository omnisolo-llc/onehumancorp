use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use futures::{SinkExt, StreamExt};

pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(text) = msg {
            // Echo back or process real-time message
            if socket.send(Message::Text(format!("Echo: {}", text).into())).await.is_err() {
                break;
            }
        }
    }
}
