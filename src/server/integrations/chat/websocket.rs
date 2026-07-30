use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};

pub fn create_ws_router() -> Router {
    Router::new().route("/ws/chat", get(ws_handler))
}

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    println!("Client sent str: {:?}", t);
                    // Handle incoming real-time web widget messages
                    // Echo back for now
                    if socket.send(Message::Text(format!("Echo: {}", t).into())).await.is_err() {
                        return;
                    }
                }
                Message::Binary(b) => {
                    println!("Client sent binary data: {:?}", b);
                }
                Message::Ping(p) => {
                    println!("Ping received");
                    let _ = socket.send(Message::Pong(p)).await;
                }
                Message::Pong(_) => {
                    println!("Pong received");
                }
                Message::Close(_) => {
                    println!("Client disconnected");
                    return;
                }
            }
        } else {
            return;
        }
    }
}
