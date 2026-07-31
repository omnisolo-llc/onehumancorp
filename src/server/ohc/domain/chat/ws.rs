use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub tenant_id: Uuid,
    pub payload: serde_json::Value,
}

pub struct ChatHub {
    // Map of tenant_id to a broadcast channel
    tenant_channels: Mutex<HashMap<Uuid, broadcast::Sender<WsMessage>>>,
}

impl ChatHub {
    pub fn new() -> Self {
        Self {
            tenant_channels: Mutex::new(HashMap::new()),
        }
    }

    pub async fn broadcast(&self, tenant_id: Uuid, message: WsMessage) {
        let channels = self.tenant_channels.lock().await;
        if let Some(tx) = channels.get(&tenant_id) {
            let _ = tx.send(message); // Ignore send errors (e.g., no receivers)
        }
    }

    pub async fn subscribe(&self, tenant_id: Uuid) -> broadcast::Receiver<WsMessage> {
        let mut channels = self.tenant_channels.lock().await;
        let tx = channels.entry(tenant_id).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        tx.subscribe()
    }
}

pub async fn handle_socket(socket: WebSocket, tenant_id: Uuid, hub: Arc<ChatHub>) {
    let (mut sender, mut receiver) = socket.split();

    let mut rx = hub.subscribe(tenant_id).await;

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(text) = serde_json::to_string(&msg.payload) {
                if sender.send(Message::Text(text.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // For now, simply echo back or handle incoming commands if necessary.
            // In a real app, parse `text` and invoke the API layer.
            println!("Received WS message from tenant {}: {}", tenant_id, text);
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::time::Duration;

    #[tokio::test]
    async fn test_chathub_broadcast_and_subscribe() {
        let hub = Arc::new(ChatHub::new());
        let tenant_id_1 = Uuid::new_v4();
        let tenant_id_2 = Uuid::new_v4();

        // Subscribe to tenant_id_1
        let mut rx1 = hub.subscribe(tenant_id_1).await;
        // Subscribe to tenant_id_2
        let mut rx2 = hub.subscribe(tenant_id_2).await;

        let msg_payload = json!({"text": "Hello, tenant 1!"});
        let msg = WsMessage {
            tenant_id: tenant_id_1,
            payload: msg_payload.clone(),
        };

        // Broadcast to tenant_id_1
        hub.broadcast(tenant_id_1, msg).await;

        // rx1 should receive the message
        let received = tokio::time::timeout(Duration::from_millis(100), rx1.recv())
            .await
            .expect("timeout")
            .expect("recv error");
        assert_eq!(received.tenant_id, tenant_id_1);
        assert_eq!(received.payload, msg_payload);

        // rx2 should NOT receive the message
        let result = tokio::time::timeout(Duration::from_millis(100), rx2.recv()).await;
        assert!(result.is_err(), "tenant 2 should not receive tenant 1's message");
    }
}
