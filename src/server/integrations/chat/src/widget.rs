use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct WidgetState {
    pub tx: broadcast::Sender<String>,
}

pub fn widget_router(state: Arc<WidgetState>) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WidgetState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<WidgetState>) {
    let mut rx = state.tx.subscribe();

    // Simple echo/broadcast for now
    loop {
        tokio::select! {
            msg = socket.recv() => {
                if let Some(Ok(Message::Text(text))) = msg {
                    let _ = state.tx.send(text.to_string());
                } else {
                    break;
                }
            }
            msg = rx.recv() => {
                if let Ok(text) = msg {
                    if socket.send(Message::Text(text.into())).await.is_err() {
                        break;
                    }
                }
            }
        }
    }
}
