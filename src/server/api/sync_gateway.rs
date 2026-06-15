use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension, State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use server_common::Claims;
use crate::api::agent_feed::get_redis_client;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum ClientMessage {
    Subscribe { topics: Vec<String> },
    Unsubscribe { topics: Vec<String> },
    Ping,
}

#[derive(Serialize, Debug, Clone)]
pub struct ServerMessage {
    pub topic: String,
    pub payload: serde_json::Value,
}

static GLOBAL_SYNC_HUB: std::sync::OnceLock<Arc<SyncHub>> = std::sync::OnceLock::new();

pub fn get_sync_hub() -> Arc<SyncHub> {
    GLOBAL_SYNC_HUB.get_or_init(|| Arc::new(SyncHub::new())).clone()
}

pub struct SyncHub {
    subscribers: Arc<Mutex<std::collections::HashMap<String, Vec<tokio::sync::mpsc::Sender<ServerMessage>>>>>,
    redis_cmd_tx: tokio::sync::mpsc::Sender<String>,
}

impl SyncHub {
    fn new() -> Self {
        let (redis_cmd_tx, mut redis_cmd_rx) = tokio::sync::mpsc::channel::<String>(100);
        let subscribers = Arc::new(Mutex::new(std::collections::HashMap::new()));

        let hub = Self {
            subscribers: subscribers.clone(),
            redis_cmd_tx,
        };

        // We can just spawn the redis connection task independently
        tokio::spawn(async move {
            let client = get_redis_client();
            if let Ok(mut pubsub_conn) = client.get_async_pubsub().await {
                loop {
                    // Drop stream before getting new commands so we don't hold the borrow
                    let msg_opt = {
                        let mut pubsub_stream = pubsub_conn.on_message();
                        tokio::time::timeout(std::time::Duration::from_millis(50), pubsub_stream.next()).await
                    };

                    if let Ok(Some(msg)) = msg_opt {
                        let topic_name = msg.get_channel_name().to_string();
                        if let Ok(payload_str) = msg.get_payload::<String>() {
                            if let Ok(payload) = serde_json::from_str(&payload_str) {
                                let server_msg = ServerMessage {
                                    topic: topic_name.clone(),
                                    payload,
                                };

                                let subs_clone = subscribers.clone();
                                tokio::spawn(async move {
                                    let subs = subs_clone.lock().await;
                                    if let Some(senders) = subs.get(&server_msg.topic) {
                                        for sender in senders {
                                            let _ = sender.send(server_msg.clone()).await;
                                        }
                                    }
                                });
                            }
                        }
                    }

                    while let Ok(topic) = redis_cmd_rx.try_recv() {
                        let _ = pubsub_conn.subscribe(&topic).await;
                    }
                }
            }
        });

        hub
    }

    async fn subscribe(&self, topic: String, sender: tokio::sync::mpsc::Sender<ServerMessage>) {
        let mut subs = self.subscribers.lock().await;
        let is_new = !subs.contains_key(&topic);

        subs.entry(topic.clone()).or_insert_with(Vec::new).push(sender);

        if is_new {
            let _ = self.redis_cmd_tx.send(topic).await;
        }
    }
}

pub fn router() -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    Router::new().route("/ws", get(ws_sync_handler))
}

pub async fn ws_sync_handler(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<Claims>,
    _db: State<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return axum::http::StatusCode::UNAUTHORIZED.into_response(),
    };

    ws.on_upgrade(move |socket| handle_sync_socket(socket, tenant_id))
}

async fn handle_sync_socket(socket: WebSocket, tenant_id: String) {
    let (mut sender, mut receiver) = socket.split();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<ServerMessage>(100);

    // We only need to listen for WebSocket messages
    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        let text_str = match std::str::from_utf8(text.as_bytes()) {
                            Ok(v) => v,
                            Err(_) => continue,
                        };
                        if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(text_str) {
                            match client_msg {
                                ClientMessage::Subscribe { topics } => {
                                    for topic in topics {
                                        let expected_suffix = format!(":{}", tenant_id);
                                        if topic.ends_with(&expected_suffix) {
                                            get_sync_hub().subscribe(topic, tx.clone()).await;
                                        }
                                    }
                                }
                                ClientMessage::Unsubscribe { topics: _ } => {
                                    // For simplicity in this refactor, we just keep the channel alive until ws drops
                                }
                                ClientMessage::Ping => {
                                    let _ = sender.send(Message::Text(serde_json::json!({"type": "Pong"}).to_string().into())).await;
                                }
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        break;
                    }
                    _ => {}
                }
            }
            server_msg = rx.recv() => {
                if let Some(msg) = server_msg {
                    if let Ok(json_str) = serde_json::to_string(&msg) {
                        if sender.send(Message::Text(json_str.into())).await.is_err() {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_gateway_types() {
        let msg = ServerMessage {
            topic: "inventory:test_tenant".to_string(),
            payload: serde_json::json!({"status": "updated"}),
        };
        assert_eq!(msg.topic, "inventory:test_tenant");
    }
}
