use async_nats::Client;


use async_trait::async_trait;

#[async_trait]
pub trait NatsClientWrapper: Send + Sync {
    async fn publish(&self, subject: &str, data: Vec<u8>) -> Result<(), String>;
    async fn subscribe(
        &self,
        subject: &str,
        handler: Box<dyn Fn(Vec<u8>) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String>;
}

pub struct RealNatsClient {
    client: Option<Client>,
}

impl RealNatsClient {
    pub async fn new(url: &str) -> Result<Self, async_nats::ConnectError> {
        let client = async_nats::connect(url).await?;
        Ok(Self { client: Some(client) })
    }

    pub fn dummy() -> Self {
        Self { client: None }
    }
}

#[async_trait]
impl NatsClientWrapper for RealNatsClient {
    async fn publish(&self, subject: &str, data: Vec<u8>) -> Result<(), String> {
        if let Some(client) = &self.client {
            client.publish(subject.to_string(), data.into()).await.map_err(|e| e.to_string())?;
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

            let worker = tokio::spawn(async move {
                use futures::StreamExt;
                while let Some(msg) = subscriber.next().await {
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
}
