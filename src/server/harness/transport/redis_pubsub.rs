use super::Transport;
use async_trait::async_trait;
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::StreamExt;
use dashmap::DashMap;

pub struct RedisPubSubTransport {
    client: redis::Client,
    connection: Arc<Mutex<redis::aio::MultiplexedConnection>>,
    topic_receivers: Arc<DashMap<String, tokio::sync::broadcast::Sender<String>>>,
}

impl RedisPubSubTransport {
    pub async fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| format!("Failed to create Redis client: {}", e))?;

        let con = client.get_multiplexed_async_connection().await
            .map_err(|e| format!("Failed to connect to Redis: {}", e))?;

        Ok(Self {
            client,
            connection: Arc::new(Mutex::new(con)),
            topic_receivers: Arc::new(DashMap::new()),
        })
    }

    async fn ensure_subscription(&self, topic: &str) -> Result<tokio::sync::broadcast::Receiver<String>, String> {
        use dashmap::mapref::entry::Entry;

        // Use a loop to retry if we hit the brief race condition where
        // the background task hasn't attached yet.
        loop {
            let tx = match self.topic_receivers.entry(topic.to_string()) {
                Entry::Occupied(o) => o.get().clone(),
                Entry::Vacant(v) => {
                    let (tx, _rx) = tokio::sync::broadcast::channel(100);
                    v.insert(tx.clone());

                    let client = self.client.clone();
                    let topic_string = topic.to_string();
                    let tx_clone = tx.clone();
                    let receivers_map = self.topic_receivers.clone();

                    tokio::spawn(async move {
                        let mut delay = std::time::Duration::from_secs(1);
                        // Subscribe to the channel immediately to ensure receiver count is > 0
                        let mut keepalive_rx = tx_clone.subscribe();
                        loop {
                            match client.get_async_pubsub().await {
                                Ok(mut pubsub) => {
                                    if let Err(e) = pubsub.subscribe(&topic_string).await {
                                        tracing::error!("Failed to subscribe to topic {}: {}", topic_string, e);
                                        tokio::time::sleep(delay).await;
                                        continue;
                                    }

                                    let mut stream = pubsub.on_message();
                                    loop {
                                        tokio::select! {
                                            msg_opt = stream.next() => {
                                                if let Some(msg) = msg_opt {
                                                    if let Ok(payload) = msg.get_payload::<String>() {
                                                        // if the send fails, there are no other active receivers
                                                        // (keepalive_rx is dropped or not enough)
                                                        // We check receiver count - 1 to account for keepalive_rx
                                                        if tx_clone.receiver_count() <= 1 {
                                                            receivers_map.remove(&topic_string);
                                                            return;
                                                        }
                                                        let _ = tx_clone.send(payload);
                                                    }
                                                } else {
                                                    break; // stream ended
                                                }
                                            }
                                            _ = keepalive_rx.recv() => {
                                                // just empty the queue
                                            }
                                        }

                                        if tx_clone.receiver_count() <= 1 {
                                            receivers_map.remove(&topic_string);
                                            return;
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Failed to get async pubsub for {}: {}", topic_string, e);
                                }
                            }
                            tokio::time::sleep(delay).await;
                            delay = std::cmp::min(delay * 2, std::time::Duration::from_secs(30));
                        }
                    });

                    tx
                }
            };

            return Ok(tx.subscribe());
        }
    }
}

#[async_trait]
impl Transport for RedisPubSubTransport {
    async fn send(&self, topic: &str, message: &str) -> Result<(), String> {
        let mut con = self.connection.lock().await;
        let _: () = con.publish(topic, message)
            .await
            .map_err(|e| format!("Failed to publish to Redis: {}", e))?;
        Ok(())
    }

    async fn subscribe(&self, topic: &str) -> Result<tokio::sync::broadcast::Receiver<String>, String> {
        self.ensure_subscription(topic).await
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        let mut con = self.connection.lock().await;
        let result: Option<String> = redis::cmd("SET")
            .arg(format!("lock:{}", resource))
            .arg(owner)
            .arg("NX")
            .arg("EX")
            .arg(ttl_seconds)
            .query_async(&mut *con)
            .await
            .map_err(|e| format!("Redis error acquiring lock: {}", e))?;

        Ok(result.is_some())
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        let script = redis::Script::new(r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#);

        let mut con = self.connection.lock().await;
        let _: () = script
            .key(format!("lock:{}", resource))
            .arg(owner)
            .invoke_async(&mut *con)
            .await
            .map_err(|e| format!("Redis error releasing lock: {}", e))?;

        Ok(())
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        let mut con = self.connection.lock().await;
        let _: () = redis::cmd("SETEX")
            .arg(format!("presence:{}", agent_id))
            .arg(ttl_seconds)
            .arg(status)
            .query_async(&mut *con)
            .await
            .map_err(|e| format!("Redis error registering presence: {}", e))?;
        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        let mut con = self.client.get_multiplexed_async_connection().await
            .map_err(|e| format!("Redis error connecting for active agents: {}", e))?;

        let keys: Vec<String> = redis::cmd("KEYS")
            .arg("presence:*")
            .query_async(&mut con)
            .await
            .map_err(|e| format!("Redis error getting presence keys: {}", e))?;

        let mut agents = Vec::new();
        for key in keys {
            let status: String = redis::cmd("GET")
                .arg(&key)
                .query_async(&mut con)
                .await
                .map_err(|e| format!("Redis error getting presence value: {}", e))?;

            let agent_id = key.strip_prefix("presence:").unwrap_or(&key).to_string();
            agents.push((agent_id, status));
        }

        Ok(agents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Due to lack of real Redis in local unit tests, we only define the signature
    // for coverage check, actual integration logic is tested via e2e.
    #[tokio::test]
    async fn test_redis_transport_init_error() {
        let res = RedisPubSubTransport::new("redis://invalid-url:12345").await;
        assert!(res.is_err());
    }
}
