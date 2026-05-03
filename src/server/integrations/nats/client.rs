use async_nats::Client;
use async_trait::async_trait;
use opentelemetry::{global, KeyValue};
use opentelemetry::metrics::Counter;

#[async_trait]
pub trait NatsClientWrapper: Send + Sync {
    async fn publish(&self, subject: &str, data: Vec<u8>) -> Result<(), String>;
    async fn subscribe(
        &self,
        subject: &str,
        handler: Box<dyn Fn(Vec<u8>) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String>;
    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String>;
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String>;
}

pub struct RealNatsClient {
    client: Option<Client>,
    publish_counter: Counter<u64>,
}

impl RealNatsClient {
    pub async fn new(url: &str) -> Result<Self, async_nats::ConnectError> {
        let client = async_nats::connect(url).await?;
        let meter = global::meter("ohc.nats");
        let publish_counter = meter.u64_counter("ohc.nats.messages_published").build();
        Ok(Self { client: Some(client), publish_counter })
    }

    pub fn dummy() -> Self {
        let meter = global::meter("ohc.nats");
        let publish_counter = meter.u64_counter("ohc.nats.messages_published").build();
        Self { client: None, publish_counter }
    }
}

#[async_trait]
impl NatsClientWrapper for RealNatsClient {
    async fn publish(&self, subject: &str, data: Vec<u8>) -> Result<(), String> {
        if let Some(client) = &self.client {
            client.publish(subject.to_string(), data.into()).await.map_err(|e| e.to_string())?;
            self.publish_counter.add(1, &[KeyValue::new("subject", subject.to_string())]);
        }
        Ok(())
    }

    async fn subscribe(
        &self,
        subject: &str,
        handler: Box<dyn Fn(Vec<u8>) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        if let Some(client) = &self.client {
            let mut subscriber = client.subscribe(subject.to_string()).await.map_err(|e| e.to_string())?;
            let subject_string = subject.to_string();

            let worker = tokio::spawn(async move {
                use futures::StreamExt;
                let meter = global::meter("ohc.nats");
                let counter: Counter<u64> = meter.u64_counter("ohc.nats.messages_received").build();
                let labels = [KeyValue::new("subject", subject_string)];
                while let Some(msg) = subscriber.next().await {
                    counter.add(1, &labels);
                    handler(msg.payload.to_vec());
                }
            });

            let cancel = Box::new(move || {
                worker.abort();
            });

            Ok(cancel)
        } else {
            Ok(Box::new(|| {}))
        }
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, _ttl_seconds: u64) -> Result<bool, String> {
        if let Some(client) = &self.client {
            let js = async_nats::jetstream::new(client.clone());
            let kv = match js.get_key_value("ohc_locks").await {
                Ok(store) => store,
                Err(_) => return Err("JetStream KV 'ohc_locks' not configured, locking failed".to_string()),
            };

            match kv.create(resource.to_string(), owner.to_string().into()).await {
                Ok(_) => {
                    Ok(true)
                }
                Err(e) => {
                    if e.to_string().contains("WrongLastSequence") || e.to_string().contains("already exists") || e.to_string().contains("wrong last sequence") {
                        Ok(false)
                    } else {
                        Err(e.to_string())
                    }
                }
            }
        } else {
            Err("NatsClient is not connected, cannot acquire lock".to_string())
        }
    }

    async fn release_lock(&self, resource: &str, _owner: &str) -> Result<(), String> {
        if let Some(client) = &self.client {
            let js = async_nats::jetstream::new(client.clone());
            if let Ok(kv) = js.get_key_value("ohc_locks").await {
                let _ = kv.delete(resource.to_string()).await;
            }
        }
        Ok(())
    }
}
