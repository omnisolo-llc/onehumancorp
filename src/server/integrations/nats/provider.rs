use super::client::{NatsClientWrapper, RealNatsClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

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

    // A shared mock that acts as a real broker between the instances
    struct SharedBrokerMock {
        published_messages: Arc<AtomicUsize>,
        handlers: std::sync::RwLock<Vec<Box<dyn Fn(Vec<u8>) + Send + Sync>>>,
    }

    #[async_trait]
    impl NatsClientWrapper for SharedBrokerMock {
        async fn publish(&self, _subject: &str, data: Vec<u8>) -> Result<(), String> {
            self.published_messages.fetch_add(1, Ordering::SeqCst);
            let handlers = self.handlers.read().unwrap();
            for handler in handlers.iter() {
                handler(data.clone());
            }
            Ok(())
        }

        async fn subscribe(
            &self,
            _subject: &str,
            handler: Box<dyn Fn(Vec<u8>) + Send + Sync>,
        ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
            self.handlers.write().unwrap().push(handler);
            Ok(Box::new(|| {}))
        }
    }

    #[tokio::test]
    async fn test_nats_e2e_event_propagation() {
        let published_count = Arc::new(AtomicUsize::new(0));
        let broker = Arc::new(SharedBrokerMock {
            published_messages: published_count.clone(),
            handlers: std::sync::RwLock::new(Vec::new()),
        });

        // Mocking Cloud Node sharing the same broker
        let cloud_node = NatsProvider::with_client(broker.clone(), "nats://cloud.test");

        // Mocking Standalone Instance sharing the same broker
        let standalone_instance = NatsProvider::with_client(broker.clone(), "nats://localhost:4222");

        let cloud_received = Arc::new(AtomicUsize::new(0));
        let cloud_received_clone = cloud_received.clone();
        let _ = cloud_node.subscribe("mesh_sync", Box::new(move |_| { cloud_received_clone.fetch_add(1, Ordering::SeqCst); })).await.unwrap();

        let standalone_received = Arc::new(AtomicUsize::new(0));
        let standalone_received_clone = standalone_received.clone();
        let _ = standalone_instance.subscribe("mesh_sync", Box::new(move |_| { standalone_received_clone.fetch_add(1, Ordering::SeqCst); })).await.unwrap();

        // Simulate standalone node propagating an event to cloud
        standalone_instance.publish("mesh_sync", b"standalone_event".to_vec()).await.unwrap();

        // Assert it reached the broker and was processed by both subscribers (cloud & standalone)
        assert_eq!(published_count.load(Ordering::SeqCst), 1);
        assert_eq!(cloud_received.load(Ordering::SeqCst), 1);
        assert_eq!(standalone_received.load(Ordering::SeqCst), 1);

        // Simulate cloud node propagating an event to standalone
        cloud_node.publish("mesh_sync", b"cloud_event".to_vec()).await.unwrap();

        assert_eq!(published_count.load(Ordering::SeqCst), 2);
        assert_eq!(cloud_received.load(Ordering::SeqCst), 2);
        assert_eq!(standalone_received.load(Ordering::SeqCst), 2);
    }
}
