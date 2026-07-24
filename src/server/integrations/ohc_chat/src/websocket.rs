use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Path},
    response::IntoResponse,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use axum::extract::ws::Utf8Bytes;

pub struct WsState {
    // Tenant ID -> Broadcast Channel
    pub channels: Mutex<HashMap<String, broadcast::Sender<String>>>,
}

impl WsState {
    pub fn new() -> Self {
        Self {
            channels: Mutex::new(HashMap::new()),
        }
    }

    pub async fn get_or_create_channel(&self, tenant_id: &str) -> broadcast::Sender<String> {
        let mut channels = self.channels.lock().await;
        if let Some(tx) = channels.get(tenant_id) {
            tx.clone()
        } else {
            let (tx, _) = broadcast::channel(100);
            channels.insert(tenant_id.to_string(), tx.clone());
            tx
        }
    }

    pub async fn broadcast<T: serde::Serialize>(&self, tenant_id: &str, msg: &T) -> Result<(), Box<dyn std::error::Error>> {
        let tx = self.get_or_create_channel(tenant_id).await;
        let s = serde_json::to_string(msg)?;
        let _ = tx.send(s);
        Ok(())
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(tenant_id): Path<String>,
    State(state): State<Arc<WsState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id, state))
}

async fn handle_socket(mut socket: WebSocket, tenant_id: String, state: Arc<WsState>) {
    let tx = state.get_or_create_channel(&tenant_id).await;
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if socket.send(Message::Text(Utf8Bytes::from(msg))).await.is_err() {
                break;
            }
        }
    });
}
