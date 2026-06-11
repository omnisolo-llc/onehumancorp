use crate::mesh::protocol::TeammateMessage;
use crate::mesh::transport::MeshTransport;
use async_trait::async_trait;
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RedisMeshTransport {
    client: redis::Client,
}

impl RedisMeshTransport {
    pub fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(RedisMeshTransport { client })
    }

    fn get_topic(tenant_id: &str) -> String {
        format!("mesh:tenant:{}", tenant_id)
    }
}

#[async_trait]
impl MeshTransport for RedisMeshTransport {
    async fn publish(&self, message: TeammateMessage) -> Result<(), String> {
        if message.tenant_id.is_empty() {
            return Err("tenant_id is required".to_string());
        }
        let topic = Self::get_topic(&message.tenant_id);
        let payload = serde_json::to_string(&message).map_err(|e| e.to_string())?;

        let mut conn = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        conn.publish::<&str, String, ()>(&topic, payload).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(&self, tenant_id: &str, handler: Box<dyn Fn(TeammateMessage) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        if tenant_id.is_empty() {
            return Err("tenant_id is required".to_string());
        }
        let topic = Self::get_topic(tenant_id);

        let mut pubsub = self.client.get_async_pubsub().await.map_err(|e| e.to_string())?;
        pubsub.subscribe(&topic).await.map_err(|e| e.to_string())?;

        let mut stream = pubsub.into_on_message();

        let worker = tokio::spawn(async move {
            use futures_util::StreamExt;
            while let Some(msg) = stream.next().await {
                if let Ok(payload) = msg.get_payload::<String>() {
                    if let Ok(m) = serde_json::from_str::<TeammateMessage>(&payload) {
                        handler(m);
                    }
                }
            }
        });

        let cancel = Box::new(move || {
            worker.abort();
        });

        Ok(cancel)
    }
}
