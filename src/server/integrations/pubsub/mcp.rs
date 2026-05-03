use std::sync::Arc;
use ohc_builtin_agent::mesh::transport::{MeshTransport};
use crate::ohc::orchestration::TeammateMeshEvent;

pub struct PubSubManager {
    transport: Arc<dyn MeshTransport>,
    is_cloud: bool,
}

impl PubSubManager {
    pub fn new(transport: Arc<dyn MeshTransport>, is_cloud: bool) -> Self {
        PubSubManager {
            transport,
            is_cloud,
        }
    }

    pub fn from_env(transport: Arc<dyn MeshTransport>) -> Self {
        let is_cloud = std::env::var("OHC_MULTITENANT").unwrap_or_default() == "true";
        Self::new(transport, is_cloud)
    }

    fn format_topic(&self, tenant_id: &str, topic: &str) -> String {
        if self.is_cloud {
            format!("{}:{}", tenant_id, topic)
        } else {
            topic.to_string()
        }
    }

    pub async fn publish(&self, tenant_id: &str, topic: &str, payload: Vec<u8>) -> Result<(), String> {
        let formatted_topic = self.format_topic(tenant_id, topic);

        use prost::Message as ProstMessage;
        let event = crate::ohc::orchestration::TeammateMeshEvent {
            agent_id: "mcp".to_string(),
            action: "publish".to_string(),
            status: "ok".to_string(),
            payload: payload.clone(),
        };
        let mut buf = Vec::new();
        let _ = event.encode(&mut buf);

        let message = crate::ohc::orchestration::TeammateMeshEvent {
            agent_id: "mcp".to_string(),
            action: formatted_topic.clone(),
            status: "ok".to_string(),
            payload: buf,
        };
        self.transport.publish(&formatted_topic, message).await
    }

    pub async fn subscribe(
        &self,
        tenant_id: &str,
        topic: &str,
        handler: Box<dyn Fn(TeammateMeshEvent) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let formatted_topic = self.format_topic(tenant_id, topic);

        let wrapped_handler = Box::new(move |msg: TeammateMeshEvent| {
            use prost::Message as ProstMessage;
            if let Ok(event) = crate::ohc::orchestration::TeammateMeshEvent::decode(&msg.payload[..]) {
                let mut new_msg = msg.clone();
                new_msg.payload = event.payload;
                handler(new_msg);
            } else {
                handler(msg);
            }
        });

        self.transport.subscribe(&formatted_topic, wrapped_handler).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent::mesh::transport::MemoryTransport;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_pubsub_manager_standalone() {
        let transport = Arc::new(MemoryTransport::new());
        let manager = PubSubManager::new(transport, false);
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: TeammateMeshEvent| {
            // In standalone, topic is NOT prefixed
            if msg.action == "test_topic" && msg.payload == b"hello" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = manager
            .subscribe("tenant_123", "test_topic", handler)
            .await
            .unwrap();

        manager
            .publish("tenant_123", "test_topic", b"hello".to_vec())
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        assert!(received.load(Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_pubsub_manager_cloud() {
        let transport = Arc::new(MemoryTransport::new());
        let manager = PubSubManager::new(transport, true);
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: TeammateMeshEvent| {
            // In cloud, topic IS prefixed with tenant_id
            if msg.action == "tenant_123:test_topic" && msg.payload == b"hello" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = manager
            .subscribe("tenant_123", "test_topic", handler)
            .await
            .unwrap();

        manager
            .publish("tenant_123", "test_topic", b"hello".to_vec())
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        assert!(received.load(Ordering::SeqCst));
        cancel();
    }
}
