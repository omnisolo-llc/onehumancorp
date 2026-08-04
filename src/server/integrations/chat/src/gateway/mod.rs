use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::stream::StreamExt;

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.next().await {
        let _msg = if let Ok(msg) = msg {
            msg
        } else {
            return;
        };

        // TODO: Handle incoming message via ChatService
        // if socket.send(_msg).await.is_err() {
        //     return;
        // }
    }
}
