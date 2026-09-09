use axum::{
    extract::{
        ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
        State, Path,
    },
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;
use dashmap::DashMap;

use super::models::Message as ChatMessage;
use super::controllers::AppState;

pub struct WsState {
    // Map tenant_id to a broadcast channel
    pub tenant_channels: DashMap<Uuid, broadcast::Sender<ChatMessage>>,
}

impl WsState {
    pub fn new() -> Self {
        Self {
            tenant_channels: DashMap::new(),
        }
    }

    pub fn get_or_create_channel(&self, tenant_id: Uuid) -> broadcast::Sender<ChatMessage> {
        self.tenant_channels
            .entry(tenant_id)
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(100);
                tx
            })
            .value()
            .clone()
    }

    pub fn broadcast_message(&self, tenant_id: Uuid, message: ChatMessage) {
        if let Some(tx) = self.tenant_channels.get(&tenant_id) {
            let _ = tx.send(message);
        }
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(tenant_id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id, state.ws_state))
}

async fn handle_socket(mut socket: WebSocket, tenant_id: Uuid, state: Arc<WsState>) {
    let tx = state.get_or_create_channel(tenant_id);
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if socket.send(WsMessage::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    });
}
