use super::client::{NatsClientWrapper, RealNatsClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;
use ohc_builtin_agent::mesh::transport::{MeshTransport, Message};
use crate::ohc::orchestration::TeammateMeshEvent;
use async_trait::async_trait;
use prost::Message as ProstMessage;

pub struct NatsProvider {
    client: Arc<dyn NatsClientWrapper>,
    metadata: ProviderMetadata,
}

impl NatsProvider {
    pub async fn new(url: &str) -> Result<Self, String> {
        let client = RealNatsClient::new(url).await.map_err(|e| e.to_string())?;

        Ok(Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "nats".to_string(),
                name: "NATS Event Mesh".to_string(),
                category: "event_mesh".to_string(),
                base_url: url.to_string(),
            },
        })
    }

    pub fn with_client(client: Arc<dyn NatsClientWrapper>, url: &str) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "nats".to_string(),
                name: "NATS Event Mesh".to_string(),
                category: "event_mesh".to_string(),
                base_url: url.to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn publish(&self, subject: &str, data: Vec<u8>) -> Result<(), String> {
        self.client.publish(subject, data).await
    }

    pub async fn subscribe(
        &self,
        subject: &str,
        handler: Box<dyn Fn(Vec<u8>) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        self.client.subscribe(subject, handler).await
    }
}

#[async_trait]
impl MeshTransport for NatsProvider {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String> {
        let mut buf = Vec::new();
        message.encode(&mut buf).map_err(|e| e.to_string())?;
        self.client.publish(topic, buf).await
    }

    async fn subscribe(
        &self,
        topic: &str,
        handler: Box<dyn Fn(Message) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let wrapped_handler = Box::new(move |data: Vec<u8>| {
            if let Ok(msg) = Message::decode(&data[..]) {
                handler(msg);
            }
        });
        self.client.subscribe(topic, wrapped_handler).await
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        self.client.acquire_lock(resource, owner, ttl_seconds).await
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        self.client.release_lock(resource, owner).await
    }

    async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> {
        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockNatsClient {
        published_messages: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl NatsClientWrapper for MockNatsClient {
        async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> {
            Ok(true)
        }
        async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> {
            Ok(())
        }
        async fn publish(&self, _subject: &str, _data: Vec<u8>) -> Result<(), String> {
            self.published_messages.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        async fn subscribe(
            &self,
            _subject: &str,
            handler: Box<dyn Fn(Vec<u8>) + Send + Sync>,
        ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
            handler(b"hello nats".to_vec());
            Ok(Box::new(|| {}))
        }
    }

    #[tokio::test]
    async fn test_nats_provider_integration() {
        let published = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockNatsClient { published_messages: published.clone() });
        let provider = NatsProvider::with_client(mock, "mock_url");

        let received = Arc::new(AtomicUsize::new(0));
        let received_clone = received.clone();
        let _ = provider.subscribe("test_topic", Box::new(move |_| { received_clone.fetch_add(1, Ordering::SeqCst); })).await.unwrap();

        provider.publish("test_topic", vec![]).await.unwrap();
        assert_eq!(published.load(Ordering::SeqCst), 1);
        assert_eq!(received.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_mesh_transport_impl() {
        let published = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockNatsClient { published_messages: published.clone() });
        let provider = NatsProvider::with_client(mock, "mock_url");

        let msg = Message {
            agent_id: "agent_1".to_string(),
            action: "test_action".to_string(),
            status: "ok".to_string(),
            payload: vec![],
        };

        // Note: the mock returns b"hello nats", which is not a valid protobuf message,
        // so the handler will likely drop it. This just tests it compiles and runs.
        let _ = MeshTransport::publish(&provider, "mesh:test", msg).await;
        assert_eq!(published.load(Ordering::SeqCst), 1);
    }
}
