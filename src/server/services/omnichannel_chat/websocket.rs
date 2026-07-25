use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info};

pub struct AppState {
    pub redis_client: redis::Client,
    pub tx: broadcast::Sender<String>,
}

pub fn router(redis_client: redis::Client) -> Router {
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState { redis_client, tx });

    let state_clone = app_state.clone();
    tokio::spawn(async move {
        if let Ok(mut pubsub) = state_clone.redis_client.get_async_pubsub().await {

            if pubsub.subscribe("omnichannel_chat_messages").await.is_ok() {
                let mut stream = pubsub.on_message();
                while let Some(msg) = stream.next().await {
                    if let Ok(payload) = msg.get_payload::<String>() {
                        let _ = state_clone.tx.send(payload);
                    }
                }
            }
        }
    });

    Router::new()
        .route("/ws/omnichannel", get(ws_handler))
        .with_state(app_state)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            info!("Received WS message: {}", text);
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_omnichannel_websocket() {
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        if let Ok(client) = redis::Client::open(redis_url) {
            // Just verifying that router can be instantiated without panicking
            let _router = router(client);
        }
    }
}
