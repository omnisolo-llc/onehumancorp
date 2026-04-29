
use std::env;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

pub struct PubSubManager {
    is_cloud: bool,
    redis_client: Option<redis::Client>,
    local_bus: Arc<RwLock<std::collections::HashMap<String, Vec<mpsc::Sender<Vec<u8>>>>>>,
}

impl PubSubManager {
    pub fn new() -> Self {
        let is_cloud = env::var("OHC_MULTITENANT").unwrap_or_default() == "true";
        let mut redis_client = None;
        if is_cloud {
            if let Ok(url) = env::var("REDIS_URL") {
                 if let Ok(client) = redis::Client::open(url) {
                      redis_client = Some(client);
                 }
            }
        }

        PubSubManager {
            is_cloud,
            redis_client,
            local_bus: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn publish(&self, topic: &str, payload: Vec<u8>) -> Result<(), String> {
        if self.is_cloud {
            if let Some(client) = &self.redis_client {
                let mut con = client.get_async_connection().await.map_err(|e| e.to_string())?;
                redis::cmd("PUBLISH")
                    .arg(topic)
                    .arg(payload)
                    .query_async(&mut con)
                    .await
                    .map_err(|e| e.to_string())?;
                return Ok(());
            } else {
                 return Err("Redis client not initialized".to_string());
            }
        } else {
            let mut bus = self.local_bus.write().await;
            if let Some(senders) = bus.get_mut(topic) {
                senders.retain(|sender| {
                    match sender.try_send(payload.clone()) {
                        Ok(_) => true,
                        Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => true, // Keep if full, but ideally we await
                        Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => false, // Remove if closed
                    }
                });
            }
            Ok(())
        }
    }

    pub async fn subscribe(&self, topic: &str) -> Result<mpsc::Receiver<Vec<u8>>, String> {
        let (tx, rx) = mpsc::channel(100);

        if self.is_cloud {
             if let Some(client) = &self.redis_client {
                  let mut pubsub = client.get_async_pubsub().await.map_err(|e| e.to_string())?;
                  pubsub.subscribe(topic).await.map_err(|e| e.to_string())?;

                  tokio::spawn(async move {
                      use futures_util::StreamExt;
                      let mut stream = pubsub.on_message();
                      while let Some(msg) = stream.next().await {
                           if let Ok(payload) = msg.get_payload::<Vec<u8>>() {
                                if tx.send(payload).await.is_err() {
                                     break;
                                }
                           }
                      }
                  });
                  return Ok(rx);
             } else {
                  return Err("Redis client not initialized".to_string());
             }
        } else {
            let mut bus = self.local_bus.write().await;
            bus.entry(topic.to_string()).or_insert_with(Vec::new).push(tx);
            Ok(rx)
        }
    }
}
