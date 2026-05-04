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
        global_bus: Option<tokio::sync::broadcast::Sender<(String, Vec<u8>)>>,
    }

    #[async_trait]
    impl NatsClientWrapper for MockNatsClient {
        async fn publish(&self, subject: &str, data: Vec<u8>) -> Result<(), String> {
            self.published_messages.fetch_add(1, Ordering::SeqCst);
            if let Some(bus) = &self.global_bus {
                let _ = bus.send((subject.to_string(), data));
            }
            Ok(())
        }

        async fn subscribe(
            &self,
            subject: &str,
            handler: Box<dyn Fn(Vec<u8>) + Send + Sync>,
        ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
            if let Some(bus) = &self.global_bus {
                let mut rx = bus.subscribe();
                let subject_owned = subject.to_string();
                let worker = tokio::spawn(async move {
                    while let Ok((msg_subject, data)) = rx.recv().await {
                        if msg_subject == subject_owned {
                            handler(data);
                        }
                    }
                });
                Ok(Box::new(move || { worker.abort(); }))
            } else {
                handler(b"hello nats".to_vec());
                Ok(Box::new(|| {}))
            }
        }
    }

    #[tokio::test]
    async fn test_nats_provider_integration() {
        let published = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockNatsClient { published_messages: published.clone(), global_bus: None });
        let provider = NatsProvider::with_client(mock, "mock_url");

        let received = Arc::new(AtomicUsize::new(0));
        let received_clone = received.clone();
        let _ = provider.subscribe("test_topic", Box::new(move |_| { received_clone.fetch_add(1, Ordering::SeqCst); })).await.unwrap();

        provider.publish("test_topic", vec![]).await.unwrap();
        assert_eq!(published.load(Ordering::SeqCst), 1);
        assert_eq!(received.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_nats_e2e_mock_cloud_standalone_propagation() {
        let (tx, _) = tokio::sync::broadcast::channel(16);
        // E2E test validating event propagation between a mock Cloud node and a Standalone instance.
        let cloud_published = Arc::new(AtomicUsize::new(0));
        let cloud_mock = Arc::new(MockNatsClient { published_messages: cloud_published.clone(), global_bus: Some(tx.clone()) });
        let cloud_node = NatsProvider::with_client(cloud_mock, "nats://cloud");

        let standalone_published = Arc::new(AtomicUsize::new(0));
        let standalone_mock = Arc::new(MockNatsClient { published_messages: standalone_published.clone(), global_bus: Some(tx.clone()) });
        let standalone_node = NatsProvider::with_client(standalone_mock, "nats://standalone");

        let received_at_cloud = Arc::new(AtomicUsize::new(0));
        let rx_cloud = received_at_cloud.clone();
        let _ = cloud_node.subscribe("events.from_standalone", Box::new(move |_| { rx_cloud.fetch_add(1, Ordering::SeqCst); })).await.unwrap();

        let received_at_standalone = Arc::new(AtomicUsize::new(0));
        let rx_standalone = received_at_standalone.clone();
        let _ = standalone_node.subscribe("events.from_cloud", Box::new(move |_| { rx_standalone.fetch_add(1, Ordering::SeqCst); })).await.unwrap();

        cloud_node.publish("events.from_cloud", b"data".to_vec()).await.unwrap();
        standalone_node.publish("events.from_standalone", b"data".to_vec()).await.unwrap();

        // give worker some time to dispatch event
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        assert_eq!(cloud_published.load(Ordering::SeqCst), 1);
        assert_eq!(standalone_published.load(Ordering::SeqCst), 1);
        assert_eq!(received_at_cloud.load(Ordering::SeqCst), 1);
        assert_eq!(received_at_standalone.load(Ordering::SeqCst), 1);
    }
}
