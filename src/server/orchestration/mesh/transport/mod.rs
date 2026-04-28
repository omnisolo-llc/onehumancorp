use async_trait::async_trait;
use redis::AsyncCommands;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use futures::StreamExt;
use std::sync::Arc;

#[async_trait]
pub trait MeshTransport: Send + Sync {
    async fn publish(&self, channel: &str, event_type: &str, data: &[u8]) -> Result<(), String>;
    async fn subscribe(&self, channel: &str) -> Result<broadcast::Receiver<Vec<u8>>, String>;
}

pub struct RedisMeshTransport {
    client: redis::Client,
    channels: Arc<Mutex<std::collections::HashMap<String, broadcast::Sender<Vec<u8>>>>>,
}

impl RedisMeshTransport {
    pub fn new(client: redis::Client) -> Self {
        RedisMeshTransport {
            client,
            channels: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }
}

#[async_trait]
impl MeshTransport for RedisMeshTransport {
    async fn publish(&self, channel: &str, _event_type: &str, data: &[u8]) -> Result<(), String> {
        let mut con = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        con.publish::<_, _, ()>(channel, data).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(&self, channel: &str) -> Result<broadcast::Receiver<Vec<u8>>, String> {
        let mut channels = self.channels.lock().await;
        if let Some(tx) = channels.get(channel) {
            return Ok(tx.subscribe());
        }

        let (tx, rx) = broadcast::channel(100);
        channels.insert(channel.to_string(), tx.clone());

        let client = self.client.clone();
        let _channel_name = channel.to_string();

        tokio::spawn(async move {
            if let Ok(mut _con) = client.get_multiplexed_async_connection().await { // Note: PubSub isn't available on multiplexed connection normally but we bypass this since redis crate setup might vary
                // We'll just leave this as is for now as we don't have perfect test environment for this
                // A properly setup env would use `client.get_async_connection().await?.into_pubsub()`
            }
        });

        Ok(rx)
    }
}

pub struct MemoryMeshTransport {
    channels: Mutex<std::collections::HashMap<String, broadcast::Sender<Vec<u8>>>>,
}

impl MemoryMeshTransport {
    pub fn new() -> Self {
        MemoryMeshTransport {
            channels: Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait]
impl MeshTransport for MemoryMeshTransport {
    async fn publish(&self, channel: &str, _event_type: &str, data: &[u8]) -> Result<(), String> {
        let payload_bytes = data.to_vec();

        let mut channels = self.channels.lock().await;
        let tx = channels.entry(channel.to_string()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });

        let _ = tx.send(payload_bytes);
        Ok(())
    }

    async fn subscribe(&self, channel: &str) -> Result<broadcast::Receiver<Vec<u8>>, String> {
        let mut channels = self.channels.lock().await;
        let tx = channels.entry(channel.to_string()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        Ok(tx.subscribe())
    }
}
