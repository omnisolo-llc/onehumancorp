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
        let client = Arc::new(MockNatsClient { published_messages: published.clone() });
        let provider = NatsProvider::with_client(client, "url");

        let received = Arc::new(AtomicUsize::new(0));
        let received_clone = received.clone();
        let _ = provider.subscribe("test_topic", Box::new(move |_| { received_clone.fetch_add(1, Ordering::SeqCst); })).await.unwrap();

        provider.publish("test_topic", vec![]).await.unwrap();
        assert_eq!(published.load(Ordering::SeqCst), 1);
        assert_eq!(received.load(Ordering::SeqCst), 1);
    }
}
