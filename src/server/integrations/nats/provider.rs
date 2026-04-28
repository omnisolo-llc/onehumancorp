use std::sync::Arc;
use async_trait::async_trait;
use async_nats::{Client, ConnectOptions};
use async_nats::jetstream::{self, Context, stream::Config as StreamConfig};
use bytes::Bytes;
use opentelemetry::{global, metrics::Counter};
use tokio::sync::mpsc;
use futures_util::StreamExt;

#[async_trait]
pub trait Integration: Send + Sync {
    async fn publish(&self, subject: &str, data: Vec<u8>) -> Result<(), String>;
    async fn subscribe(&self, subject: &str) -> Result<mpsc::Receiver<Vec<u8>>, String>;
}

pub struct NatsIntegration {
    client: Client,
    jetstream: Context,
    messages_published: Counter<u64>,
    messages_received: Counter<u64>,
}

impl NatsIntegration {
    pub async fn new(url: &str) -> Result<Self, String> {
        let client = async_nats::connect(url)
            .await
            .map_err(|e| format!("Failed to connect to NATS: {}", e))?;

        let jetstream = jetstream::new(client.clone());

        let meter = global::meter("ohc.nats");
        let messages_published = meter.u64_counter("ohc.nats.messages_published").init();
        let messages_received = meter.u64_counter("ohc.nats.messages_received").init();

        Ok(Self {
            client,
            jetstream,
            messages_published,
            messages_received,
        })
    }

    pub async fn get_or_create_stream(&self, stream_name: &str, subjects: Vec<String>) -> Result<(), String> {
        let config = StreamConfig {
            name: stream_name.to_string(),
            subjects,
            ..Default::default()
        };

        self.jetstream.get_or_create_stream(config)
            .await
            .map_err(|e| format!("Failed to get or create stream: {}", e))?;

        Ok(())
    }
}

#[async_trait]
impl Integration for NatsIntegration {
    async fn publish(&self, subject: &str, data: Vec<u8>) -> Result<(), String> {
        self.client.publish(subject.to_string(), Bytes::from(data))
            .await
            .map_err(|e| format!("Failed to publish message: {}", e))?;

        self.messages_published.add(1, &[]);

        Ok(())
    }

    async fn subscribe(&self, subject: &str) -> Result<mpsc::Receiver<Vec<u8>>, String> {
        let mut subscriber = self.client.subscribe(subject.to_string())
            .await
            .map_err(|e| format!("Failed to subscribe to subject: {}", e))?;

        let (tx, rx) = mpsc::channel(100);
        let counter = self.messages_received.clone();

        tokio::spawn(async move {
            while let Some(msg) = subscriber.next().await {
                counter.add(1, &[]);
                if tx.send(msg.payload.to_vec()).await.is_err() {
                    break;
                }
            }
        });

        Ok(rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    struct MockIntegration {
        publish_tx: mpsc::Sender<(String, Vec<u8>)>,
        subscribe_rx: Arc<tokio::sync::Mutex<mpsc::Receiver<(String, Vec<u8>)>>>,
    }

    impl MockIntegration {
        fn new() -> (Self, mpsc::Receiver<(String, Vec<u8>)>, mpsc::Sender<(String, Vec<u8>)>) {
            let (pub_tx, pub_rx) = mpsc::channel(10);
            let (sub_tx, sub_rx) = mpsc::channel(10);
            (
                Self {
                    publish_tx: pub_tx,
                    subscribe_rx: Arc::new(tokio::sync::Mutex::new(sub_rx)),
                },
                pub_rx,
                sub_tx,
            )
        }
    }

    #[async_trait]
    impl Integration for MockIntegration {
        async fn publish(&self, subject: &str, data: Vec<u8>) -> Result<(), String> {
            self.publish_tx.send((subject.to_string(), data)).await.map_err(|e| e.to_string())
        }

        async fn subscribe(&self, subject: &str) -> Result<mpsc::Receiver<Vec<u8>>, String> {
            let (tx, rx) = mpsc::channel(10);
            let sub_rx = self.subscribe_rx.clone();
            let subject = subject.to_string();

            tokio::spawn(async move {
                let mut sub_rx = sub_rx.lock().await;
                while let Some((subj, data)) = sub_rx.recv().await {
                    if subj == subject {
                        if tx.send(data).await.is_err() {
                            break;
                        }
                    }
                }
            });
            Ok(rx)
        }
    }

    #[tokio::test]
    async fn test_integration_trait_abstraction() {
        let (integration, mut pub_rx, sub_tx) = MockIntegration::new();

        // Test publish
        integration.publish("test.subject", b"hello".to_vec()).await.unwrap();
        let (subj, data) = pub_rx.recv().await.unwrap();
        assert_eq!(subj, "test.subject");
        assert_eq!(data, b"hello");

        // Test subscribe
        let mut rx = integration.subscribe("test.subject").await.unwrap();
        sub_tx.send(("test.subject".to_string(), b"world".to_vec())).await.unwrap();

        let received = timeout(Duration::from_secs(1), rx.recv()).await.unwrap().unwrap();
        assert_eq!(received, b"world");
    }
}
