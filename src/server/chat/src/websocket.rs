use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};

pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    println!("Client sent string: {:?}", t);
                    // Echo back for now
                    if socket.send(Message::Text(t.into())).await.is_err() {
                        println!("Client disconnected");
                        return;
                    }
                }
                Message::Close(_) => {
                    println!("Client disconnected");
                    return;
                }
                _ => {}
            }
        } else {
            println!("Client disconnected");
            return;
        }
    }
}
