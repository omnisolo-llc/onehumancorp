use async_trait::async_trait;
use ohc_builtin_agent::mesh::transport::{MeshTransport, Message};
use crate::harness::transport::{UniversalTransportBridge, TransportMode, Transport};
use std::sync::Arc;

pub struct BridgedMeshTransport {
    pub bridge: Arc<UniversalTransportBridge>,
}

impl BridgedMeshTransport {
    pub async fn new(bridge: Arc<UniversalTransportBridge>) -> Result<Self, String> {
        Ok(Self { bridge })
    }
}

#[async_trait]
impl MeshTransport for BridgedMeshTransport {
    async fn publish(&self, topic: &str, message: ::server_ohc::orchestration::TeammateMeshEvent) -> Result<(), String> {
        let serialized = serde_json::to_string(&message).map_err(|e| format!("Failed to serialize message: {}", e))?;
        self.bridge.send(topic, &serialized).await
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let topic_clone = topic.to_string();
        let bridge_clone = self.bridge.clone();

        let handler_arc = Arc::new(handler);

        let mut rx = match bridge_clone.subscribe(&topic_clone).await {
            Ok(rx) => rx,
            Err(e) => return Err(e),
        };

        let (cancel_tx, mut cancel_rx) = tokio::sync::mpsc::channel(1);

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancel_rx.recv() => {
                        break;
                    }
                    res = rx.recv() => {
                        match res {
                            Ok(serialized) => {
                                // Dummy conversion for integration
                                if let Ok(parsed_event) = serde_json::from_str::<::server_ohc::orchestration::TeammateMeshEvent>(&serialized) {
                                    let msg = Message {
                                        agent_id: parsed_event.agent_id,
                                        action: parsed_event.action,
                                        status: parsed_event.status,
                                        payload: parsed_event.payload,
                                        msg_id: parsed_event.msg_id,
                                    };
                                    handler_arc(msg);
                                }
                            },
                            Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                                break;
                            },
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                                continue;
                            }
                        }
                    }
                }
            }
        });

        // The bridge implementation doesn't currently support unsubscribing via token.
        // As a simplification for integration, we return a no-op function.
        let unsub = move || {
            let _ = cancel_tx.try_send(());
        };

        Ok(Box::new(unsub))
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        self.bridge.acquire_lock(resource, owner, ttl_seconds).await
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        self.bridge.release_lock(resource, owner).await
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        self.bridge.register_presence(agent_id, status, ttl_seconds).await
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        self.bridge.get_active_agents().await
    }
}

pub async fn create_transport(redis_url: Option<&str>, is_cloud: bool) -> Result<Arc<dyn MeshTransport>, String> {
    let mode = if is_cloud {
        TransportMode::Cloud(redis_url.unwrap_or("redis://127.0.0.1:6379").to_string())
    } else {
        TransportMode::Standalone
    };

    let bridge = Arc::new(UniversalTransportBridge::new(mode).await?);
    let transport = BridgedMeshTransport::new(bridge).await?;

    Ok(Arc::new(transport))
}
