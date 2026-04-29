use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub topic: String,
    pub payload: Vec<u8>,
}

#[async_trait]
pub trait MeshTransport: Send + Sync {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String>;
    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;
}

pub struct MemoryTransport {
    subs: Mutex<HashMap<String, broadcast::Sender<Message>>>,
}

impl MemoryTransport {
    pub fn new() -> Self {
        MemoryTransport {
            subs: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl MeshTransport for MemoryTransport {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String> {
        let subs = self.subs.lock().await;
        if let Some(tx) = subs.get(topic) {
            let _ = tx.send(message);
        }
        Ok(())
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let mut subs = self.subs.lock().await;
        let tx = subs.entry(topic.to_string()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });

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
}

pub struct RedisTransport {
    client: redis::Client,
    publish_conn: Mutex<redis::aio::MultiplexedConnection>,
}

impl RedisTransport {
    pub async fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        let publish_conn = client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;

        Ok(RedisTransport {
            client,
            publish_conn: Mutex::new(publish_conn),
        })
    }
}

#[async_trait]
impl MeshTransport for RedisTransport {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String> {
        use redis::AsyncCommands;
        let payload = serde_json::to_string(&message).map_err(|e| e.to_string())?;
        let mut conn = self.publish_conn.lock().await;
        let _: () = conn.publish(topic, payload).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        use tokio_stream::StreamExt;

        let conn = self.client.get_async_connection().await.map_err(|e| e.to_string())?;
        let mut pubsub = conn.into_pubsub();
        pubsub.subscribe(topic).await.map_err(|e| e.to_string())?;

        let mut stream = pubsub.into_on_message();

        let worker = tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                if let Ok(payload_str) = msg.get_payload::<String>() {
                    if let Ok(message) = serde_json::from_str::<Message>(&payload_str) {
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
}

pub async fn create_transport(redis_url: Option<&str>, standalone: bool) -> Arc<dyn MeshTransport> {
    if standalone {
        return Arc::new(MemoryTransport::new());
    }

    if let Some(url) = redis_url {
        match RedisTransport::new(url).await {
            Ok(transport) => Arc::new(transport),
            Err(e) => {
                eprintln!("Failed to connect to Redis for MeshTransport: {}. Falling back to MemoryTransport.", e);
                Arc::new(MemoryTransport::new())
            }
        }
    } else {
        Arc::new(MemoryTransport::new())
    }
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
        let transport = create_transport(None, true).await;
        let msg = Message {
            topic: "test_factory".to_string(),
            payload: b"test".to_vec(),
        };
        assert!(transport.publish("test_factory", msg).await.is_ok());
    }

    #[tokio::test]
    async fn test_create_transport_redis_fallback() {
        let transport = create_transport(Some("redis://localhost:9999"), false).await;
        let msg = Message {
            topic: "test_fallback".to_string(),
            payload: b"test".to_vec(),
        };
        assert!(transport.publish("test_fallback", msg).await.is_ok());
    }
}

    #[tokio::test]
    async fn test_redis_transport() {
        use std::sync::atomic::{AtomicBool, Ordering};
        // Only run this test if a local redis is available
        let redis_url = "redis://127.0.0.1:6379";
        if redis::Client::open(redis_url).is_err() {
            return;
        }

        let client = redis::Client::open(redis_url).unwrap();
        if client.get_async_connection().await.is_err() {
            return;
        }

        let transport = RedisTransport::new(redis_url).await.unwrap();
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "test_redis" && msg.payload == b"hello" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = transport.subscribe("test_redis", handler).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let msg = Message {
            topic: "test_redis".to_string(),
            payload: b"hello".to_vec(),
        };

        transport.publish("test_redis", msg).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        assert!(received.load(Ordering::SeqCst));
        cancel();
    }
