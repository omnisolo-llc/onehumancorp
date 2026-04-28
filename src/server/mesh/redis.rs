use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;
use redis::AsyncCommands;
use crate::mesh::MeshTransport;
use tokio_stream::StreamExt;

pub struct RedisMeshTransport {
    client: redis::Client,
}

impl RedisMeshTransport {
    pub fn new(client: redis::Client) -> Self {
        RedisMeshTransport {
            client,
        }
    }
}

#[async_trait]
impl MeshTransport for RedisMeshTransport {

    async fn publish(&self, channel: String, payload: Vec<u8>) -> Result<(), String> {
        let mut con = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        let _ : () = con.publish(&channel, &payload).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(&self, channel: String, handler: Box<dyn Fn(Vec<u8>) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let client_clone = self.client.clone();

        let worker = tokio::spawn(async move {
            loop {
                if let Ok(mut con) = client_clone.get_async_connection().await {
                    let mut pubsub = con.into_pubsub();
                    if pubsub.subscribe(&channel).await.is_ok() {
                        let mut stream = pubsub.on_message();
                        while let Some(msg) = stream.next().await {
                            if let Ok(payload) = msg.get_payload::<Vec<u8>>() {
                                handler(payload);
                            }
                        }
                    }
                }
                // Sleep before retry
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        });

        let cancel = Box::new(move || {
            worker.abort();
        });

        Ok(cancel)
    }

    async fn broadcast_presence(&self, agent_id: String, status: String) -> Result<(), String> {
        self.publish("mesh:presence".to_string(), format!("{}:{}", agent_id, status).into_bytes()).await
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_redis_mesh_instantiation() {
        if let Ok(client) = redis::Client::open("redis://127.0.0.1:0") {
            let transport = RedisMeshTransport::new(client);
            let res = transport.publish("test".to_string(), vec![1]).await;
            assert!(res.is_err());
        } else {
            assert!(true);
        }
    }
}
