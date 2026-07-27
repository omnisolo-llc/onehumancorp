use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use serde_json::json;

#[derive(Clone)]
pub struct ChatState {
    pub db: Arc<PgPool>,
}

pub fn router<S>(db_pool: Arc<PgPool>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = ChatState { db: db_pool };
    Router::new()
        .route("/ws", get(ws_handler))
        .route("/webhooks/incoming", post(webhook_handler))
        .with_state(state)
}

async fn ws_handler(ws: WebSocketUpgrade, State(_state): State<ChatState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                WsMessage::Text(t) => {
                    // Simple echo for now, would typically parse and route to Redis PubSub
                    println!("Received ws msg: {}", t);
                    let _ = socket.send(WsMessage::Text(format!("Echo: {}", t).into())).await;
                }
                WsMessage::Close(_) => {
                    break;
                }
                _ => {}
            }
        } else {
            break;
        }
    }
}

async fn webhook_handler(State(_state): State<ChatState>, Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    // Basic webhook ingestion: simulate creating a message and triggering AI job queue via Redis PubSub

    // 1. In a real scenario, map payload via generic channel adapter to Message
    // 2. Insert into DB (with RLS constraint enforced by current_tenant setting)
    // 3. Emit event to Redis
    println!("Webhook payload received: {:?}", payload);

    // Simulate DB insertion and event emission
    Json(json!({"status": "received", "event": "emitted to AI Job Queue"}))
}
