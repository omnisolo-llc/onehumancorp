use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Path},
    response::IntoResponse,
    routing::{get, post},
    Router, Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
}

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub tenant_id: Uuid,
    pub channel_id: Uuid,
    pub contact_id: Uuid,
    pub content: String,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/ws/:tenant_id", get(ws_handler))
        .route("/webhook", post(webhook_handler))
        .with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(tenant_id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id, state))
}

async fn handle_socket(mut socket: WebSocket, tenant_id: Uuid, state: AppState) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if let Message::Text(text) = msg {
                println!("Received message in tenant {}: {}", tenant_id, text);
            }
        } else {
            break;
        }
    }
}

async fn webhook_handler(
    State(state): State<AppState>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    // Process incoming webhook message and save to DB
    // 1. Find or create conversation
    // 2. Insert message
    // 3. Trigger RoutingEngine

    axum::http::StatusCode::OK
}
