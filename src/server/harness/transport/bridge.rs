use super::{Transport, InProcessTransport, RedisPubSubTransport};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone)]
pub enum TransportMode {
    Standalone,
    Cloud(String), // Redis URL
}

pub struct UniversalTransportBridge {
    transport: Arc<dyn Transport>,
}

impl UniversalTransportBridge {
    pub async fn new(mode: TransportMode) -> Result<Self, String> {
        let transport: Arc<dyn Transport> = match mode {
            TransportMode::Standalone => {
                Arc::new(InProcessTransport::new())
            }
            TransportMode::Cloud(redis_url) => {
                Arc::new(RedisPubSubTransport::new(&redis_url).await?)
            }
        };

        Ok(Self { transport })
    }
}

#[async_trait]
impl Transport for UniversalTransportBridge {
    async fn send(&self, topic: &str, message: &str) -> Result<(), String> {
        self.transport.send(topic, message).await
    }

    async fn subscribe(&self, topic: &str) -> Result<broadcast::Receiver<String>, String> {
        self.transport.subscribe(topic).await
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        self.transport.acquire_lock(resource, owner, ttl_seconds).await
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        self.transport.release_lock(resource, owner).await
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        self.transport.register_presence(agent_id, status, ttl_seconds).await
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        self.transport.get_active_agents().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_process_transport() {
        let bridge = UniversalTransportBridge::new(TransportMode::Standalone).await.unwrap();

        let bridge_clone = Arc::new(bridge);
        let bridge_clone_2 = bridge_clone.clone();

        let mut rx = bridge_clone.subscribe("test_topic").await.unwrap();

        tokio::spawn(async move {
            bridge_clone_2.send("test_topic", "hello world").await.unwrap();
        });

        let msg = rx.recv().await.unwrap();
        assert_eq!(msg, "hello world");
    }
}
