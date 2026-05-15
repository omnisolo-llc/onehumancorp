use super::MeshTransport;
use crate::proto::hub::TeammateMeshEvent as Message;
use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct RedisTransport {
    client: redis::Client,
    publish_conn: tokio::sync::Mutex<redis::aio::MultiplexedConnection>,
}

impl RedisTransport {
    pub async fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        let publish_conn = client
            .get_multiplexed_tokio_connection()
            .await
            .map_err(|e| e.to_string())?;

        Ok(RedisTransport {
            client,
            publish_conn: tokio::sync::Mutex::new(publish_conn),
        })
    }
}

#[async_trait]
impl MeshTransport for RedisTransport {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String> {
        use prost::Message as ProstMessage;

        let mut conn = self.publish_conn.lock().await;

        let mut buf = Vec::new();
        message.encode(&mut buf).unwrap();

        let _: () = conn.publish(topic, buf).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(
        &self,
        topic: &str,
        handler: Box<dyn Fn(Message) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        use futures_util::StreamExt;
        use prost::Message as ProstMessage;

        let mut pubsub = self
            .client
            .get_async_pubsub()
            .await
            .map_err(|e| e.to_string())?;

        pubsub.subscribe(topic).await.map_err(|e| e.to_string())?;
        let mut stream = pubsub.into_on_message();

        let worker = tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                if let Ok(buf) = msg.get_payload::<Vec<u8>>() {
                    if let Ok(message) = Message::decode(&buf[..]) {
                        handler(message);
                    }
                }
            }
        });

        let cancel = Box::new(move || {
            worker.abort();
        });

        Ok(cancel)
    }

    async fn acquire_lock(
        &self,
        resource: &str,
        owner: &str,
        ttl_seconds: u64,
    ) -> Result<bool, String> {
        let mut conn = self.publish_conn.lock().await;
        let key = format!("lock:{}", resource);

        let script = redis::Script::new(
            r#"
            local current_owner = redis.call("get", KEYS[1])
            if not current_owner or current_owner == ARGV[1] then
                redis.call("set", KEYS[1], ARGV[1], "EX", ARGV[2])
                return 1
            else
                return 0
            end
        "#,
        );

        let res: i32 = script
            .key(&key)
            .arg(owner)
            .arg(ttl_seconds)
            .invoke_async(&mut *conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(res == 1)
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        let mut conn = self.publish_conn.lock().await;
        let key = format!("lock:{}", resource);
        let script = redis::Script::new(
            r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#,
        );

        let _: i32 = script
            .key(&key)
            .arg(owner)
            .invoke_async(&mut *conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn register_presence(
        &self,
        agent_id: &str,
        status: &str,
        ttl_seconds: u64,
    ) -> Result<(), String> {
        let mut conn = self.publish_conn.lock().await;
        let key = format!("presence:{}", agent_id);
        let _: () = redis::cmd("SET")
            .arg(&key)
            .arg(status)
            .arg("EX")
            .arg(ttl_seconds)
            .query_async(&mut *conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        let mut conn = self.publish_conn.lock().await;
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg("presence:*")
            .query_async(&mut *conn)
            .await
            .map_err(|e| e.to_string())?;

        let mut active = Vec::new();
        for key in keys {
            let status: Option<String> = redis::cmd("GET")
                .arg(&key)
                .query_async(&mut *conn)
                .await
                .map_err(|e| e.to_string())?;
            if let Some(s) = status {
                let agent_id = key.strip_prefix("presence:").unwrap_or(&key).to_string();
                active.push((agent_id, s));
            }
        }
        Ok(active)
    }
}
