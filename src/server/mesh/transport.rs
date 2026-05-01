use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast;
use dashmap::DashMap;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize, prost::Message)]
pub struct Message {
    #[prost(string, tag = "1")]
    pub topic: String,
    #[prost(bytes = "vec", tag = "2")]
    pub payload: Vec<u8>,
}

#[async_trait]
pub trait MeshTransport: Send + Sync {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String>;
    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String>;
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String>;

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String>;
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String>;
}

pub struct MemoryTransport {
    subs: DashMap<String, broadcast::Sender<Message>>,
    presence: DashMap<String, (String, std::time::Instant)>, // agent_id -> (status, expires_at)
    locks: DashMap<String, (String, std::time::Instant)>, // resource -> (owner, expires_at)
}

impl MemoryTransport {
    pub fn new() -> Self {
        MemoryTransport {
            subs: DashMap::new(),
            presence: DashMap::new(),
            locks: DashMap::new(),
        }
    }
}

#[async_trait]
impl MeshTransport for MemoryTransport {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String> {
        if let Some(tx) = self.subs.get(topic) {
            let _ = tx.send(message);
        }
        Ok(())
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let tx = self.subs.entry(topic.to_string()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        }).clone();

        let mut rx = tx.subscribe();

        let worker = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                handler(msg);
            }
        });

        let cancel = Box::new(move || {
            worker.abort();
        });

        Ok(cancel)
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        let now = std::time::Instant::now();

        // Remove expired locks
        self.locks.retain(|_, (_, expires_at)| *expires_at > now);

        let expires_at = now + std::time::Duration::from_secs(ttl_seconds);
        use dashmap::mapref::entry::Entry;
        match self.locks.entry(resource.to_string()) {
            Entry::Vacant(e) => {
                e.insert((owner.to_string(), expires_at));
                Ok(true)
            }
            Entry::Occupied(_) => {
                Ok(false)
            }
        }
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        self.locks.remove_if(resource, |_, (lock_owner, _)| lock_owner == owner);
        Ok(())
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        let expires_at = std::time::Instant::now() + std::time::Duration::from_secs(ttl_seconds);
        self.presence.insert(agent_id.to_string(), (status.to_string(), expires_at));
        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        let now = std::time::Instant::now();

        // Remove expired
        self.presence.retain(|_, (_, expires_at)| *expires_at > now);

        let agents = self.presence.iter()
            .map(|entry| (entry.key().clone(), entry.value().0.clone()))
            .collect();

        Ok(agents)
    }
}

pub struct RedisTransport {
    client: redis::Client,
    publish_conn: tokio::sync::Mutex<redis::aio::MultiplexedConnection>,
}

impl RedisTransport {
    pub async fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        let publish_conn = client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;

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
        use base64::{Engine as _, engine::general_purpose::STANDARD};

        let mut conn = self.publish_conn.lock().await;

        let mut buf = Vec::new();
        message.encode(&mut buf).unwrap();
        let payload_b64 = STANDARD.encode(&buf);

        let _: () = conn.publish(topic, payload_b64).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        use prost::Message as ProstMessage;
        use futures_util::StreamExt;
        use base64::{Engine as _, engine::general_purpose::STANDARD};

        // We use into_pubsub to get a pubsub connection
        // The deprecation warning indicates this uses a different underlying connection, which is what we want for subscribe anyway
        #[allow(deprecated)]
        let mut pubsub = self.client.get_async_connection().await.map_err(|e| e.to_string())?.into_pubsub();

        pubsub.subscribe(topic).await.map_err(|e| e.to_string())?;
        let mut stream = pubsub.into_on_message();

        let worker = tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                if let Ok(payload_b64) = msg.get_payload::<String>() {
                    if let Ok(buf) = STANDARD.decode(&payload_b64) {
                        if let Ok(message) = Message::decode(&buf[..]) {
                            handler(message);
                        }
                    }
                }
            }
        });

        let cancel = Box::new(move || {
            worker.abort();
        });

        Ok(cancel)
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        let mut conn = self.publish_conn.lock().await;

        let key = format!("lock:{}", resource);
        let result: bool = redis::cmd("SET")
            .arg(&key)
            .arg(owner)
            .arg("NX")
            .arg("EX")
            .arg(ttl_seconds)
            .query_async(&mut *conn)
            .await
            .unwrap_or(false);

        Ok(result)
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        let mut conn = self.publish_conn.lock().await;

        let key = format!("lock:{}", resource);

        // Use a Lua script to ensure we only delete the lock if we own it
        let script = redis::Script::new(
            "if redis.call('get', KEYS[1]) == ARGV[1] then return redis.call('del', KEYS[1]) else return 0 end"
        );

        let _: () = script
            .key(&key)
            .arg(owner)
            .invoke_async(&mut *conn)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        let mut conn = self.publish_conn.lock().await;

        let key = "mesh:presence";

        let mut pipe = redis::pipe();
        pipe.atomic()
            .cmd("HSET").arg(key).arg(agent_id).arg(status)
            .cmd("EXPIRE").arg(key).arg(ttl_seconds);

        let _: () = pipe.query_async(&mut *conn).await.map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        let mut conn = self.publish_conn.lock().await;

        let key = "mesh:presence";
        let hash: std::collections::HashMap<String, String> = conn.hgetall(key).await.unwrap_or_default();

        let agents = hash.into_iter().collect();
        Ok(agents)
    }
}

pub async fn create_transport(redis_url: Option<&str>, is_cloud: bool) -> Result<Arc<dyn MeshTransport>, String> {
    if is_cloud {
        if let Some(url) = redis_url {
            match RedisTransport::new(url).await {
                Ok(t) => {
                    println!("Initialized RedisTransport");
                    return Ok(Arc::new(t));
                },
                Err(e) => {
                    return Err(format!("Failed to initialize RedisTransport in cloud mode: {}", e));
                }
            }
        } else {
            return Err("Redis URL is required in cloud mode".to_string());
        }
    }

    // Standalone fallback
    if let Some(url) = redis_url {
        match RedisTransport::new(url).await {
            Ok(t) => {
                println!("Initialized RedisTransport (Standalone)");
                return Ok(Arc::new(t));
            },
            Err(e) => {
                println!("Failed to initialize RedisTransport (Standalone): {}. Falling back to MemoryTransport.", e);
            }
        }
    }

    println!("Initialized MemoryTransport");
    Ok(Arc::new(MemoryTransport::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[tokio::test]
    async fn test_memory_transport() {
        let transport = MemoryTransport::new();
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "test_topic" && msg.payload == b"hello" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = transport.subscribe("test_topic", handler).await.unwrap();

        let msg = Message {
            topic: "test_topic".to_string(),
            payload: b"hello".to_vec(),
        };

        transport.publish("test_topic", msg).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        assert!(received.load(Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_create_transport_standalone() {
        let transport = create_transport(None, false).await.unwrap();
        // Since MemoryTransport isn't easily castable back without Any, we just ensure it didn't err
        assert!(true);
    }

    #[tokio::test]
    async fn test_create_transport_redis_fails() {
        // Provide invalid url
        let transport = create_transport(Some("redis://localhost:9999"), false).await;
        // In standalone, it should fallback to Memory, so it's Ok
        assert!(transport.is_ok());

        // In cloud, it should err
        let transport = create_transport(Some("redis://localhost:9999"), true).await;
        assert!(transport.is_err());
    }

    #[tokio::test]
    async fn test_memory_transport_locking() {
        let transport = MemoryTransport::new();

        // Test lock acquisition
        let acquired = transport.acquire_lock("my_resource", "agent_1", 10).await.unwrap();
        assert!(acquired);

        // Test mutual exclusion
        let acquired_again = transport.acquire_lock("my_resource", "agent_2", 10).await.unwrap();
        assert!(!acquired_again);

        // Test lock release
        transport.release_lock("my_resource", "agent_1").await.unwrap();

        // Test lock acquisition after release
        let acquired_after_release = transport.acquire_lock("my_resource", "agent_2", 10).await.unwrap();
        assert!(acquired_after_release);
    }

    #[tokio::test]
    async fn test_memory_transport_lock_expiration() {
        let transport = MemoryTransport::new();

        // Acquire lock with short TTL (1 second)
        let acquired = transport.acquire_lock("expiring_resource", "agent_1", 1).await.unwrap();
        assert!(acquired);

        // Sleep for 2 seconds to let lock expire
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Second agent should be able to acquire lock now
        let acquired_after_expiration = transport.acquire_lock("expiring_resource", "agent_2", 10).await.unwrap();
        assert!(acquired_after_expiration);
    }

    #[tokio::test]
    async fn test_memory_transport_presence() {
        let transport = MemoryTransport::new();

        // Register presence
        transport.register_presence("agent_1", "online", 10).await.unwrap();
        transport.register_presence("agent_2", "busy", 1).await.unwrap();

        // Get active agents
        let mut active_agents = transport.get_active_agents().await.unwrap();
        active_agents.sort();

        assert_eq!(active_agents.len(), 2);
        assert_eq!(active_agents[0], ("agent_1".to_string(), "online".to_string()));
        assert_eq!(active_agents[1], ("agent_2".to_string(), "busy".to_string()));

        // Wait for agent_2 presence to expire
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Get active agents again
        let active_agents_after_expiration = transport.get_active_agents().await.unwrap();
        assert_eq!(active_agents_after_expiration.len(), 1);
        assert_eq!(active_agents_after_expiration[0], ("agent_1".to_string(), "online".to_string()));
    }

    #[tokio::test]
    async fn test_redis_transport() {
        // Needs running Redis instance
        let transport = RedisTransport::new("redis://localhost:6379").await;
        if transport.is_err() {
            println!("Skipping redis transport test due to missing redis connection");
            return;
        }
        let transport = transport.unwrap();

        // Setup channel for verification
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        let tx_arc = Arc::new(tokio::sync::Mutex::new(tx));
        let handler = Box::new(move |msg: Message| {
            let tx_clone = tx_arc.clone();
            tokio::spawn(async move {
                let mut tx = tx_clone.lock().await;
                let _ = tx.send(msg).await;
            });
        });

        // Wait for connection to settle
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let cancel = transport.subscribe("test_topic_redis", handler).await.unwrap();

        // Wait for subscription to propagate
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let msg = Message {
            topic: "test_topic_redis".to_string(),
            payload: b"hello redis".to_vec(),
        };

        transport.publish("test_topic_redis", msg.clone()).await.unwrap();

        // Use timeout to prevent hanging test
        let result = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv()).await;

        assert!(result.is_ok());
        if let Ok(Some(received_msg)) = result {
             assert_eq!(received_msg.topic, "test_topic_redis");
             assert_eq!(received_msg.payload, b"hello redis");
        } else {
             panic!("Did not receive message");
        }

        cancel();
    }
}
