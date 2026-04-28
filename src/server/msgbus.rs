use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use async_trait::async_trait;
use redis::AsyncCommands;
use futures::StreamExt;

#[derive(Debug, Clone)]
pub struct Message {
    pub topic: String,
    pub payload: Vec<u8>,
}

#[async_trait]
pub trait Bus: Send + Sync {
    async fn publish(&self, msg: Message) -> Result<(), String>;
    async fn subscribe(&self, topic: String, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;
    async fn close(&self) -> Result<(), String>;
}

pub struct MemoryBus {
    subs: Mutex<std::collections::HashMap<String, broadcast::Sender<Message>>>,
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            subs: Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait]
impl Bus for MemoryBus {
    async fn publish(&self, msg: Message) -> Result<(), String> {
        let subs = self.subs.lock().await;
        if let Some(tx) = subs.get(&msg.topic) {
            let _ = tx.send(msg);
        }
        Ok(())
    }

    async fn subscribe(&self, topic: String, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let mut subs = self.subs.lock().await;
        let tx = subs.entry(topic.clone()).or_insert_with(|| {
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

    async fn close(&self) -> Result<(), String> {
        Ok(())
    }
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self::new()
    }
}


pub struct RedisBus {
    client: redis::Client,
    pubsub_tasks: Mutex<Vec<tokio::task::JoinHandle<()>>>,
}

impl RedisBus {
    pub fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(RedisBus {
            client,
            pubsub_tasks: Mutex::new(Vec::new()),
        })
    }
}

#[async_trait]
impl Bus for RedisBus {
    async fn publish(&self, msg: Message) -> Result<(), String> {
        let mut con = self.client.get_async_connection().await.map_err(|e| e.to_string())?;
        let _ : () = con.publish(msg.topic, msg.payload).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(&self, topic: String, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let client = self.client.clone();

        let worker = tokio::spawn(async move {
            let con_res = client.get_async_connection().await;
            if let Ok(con) = con_res {
                let mut pubsub = con.into_pubsub();
                if pubsub.subscribe(&topic).await.is_ok() {
                    let mut stream = pubsub.on_message();
                    while let Some(msg) = stream.next().await {
                        let payload: Vec<u8> = msg.get_payload().unwrap_or_default();
                        handler(Message {
                            topic: topic.clone(),
                            payload,
                        });
                    }
                }
            }
        });

        self.pubsub_tasks.lock().await.push(worker.clone());

        let cancel = Box::new(move || {
            worker.abort();
        });

        Ok(cancel)
    }

    async fn close(&self) -> Result<(), String> {
        let mut tasks = self.pubsub_tasks.lock().await;
        for task in tasks.drain(..) {
            task.abort();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    
    #[tokio::test]
    async fn test_memory_bus_pub_sub() {
        let bus = MemoryBus::new();
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();
        
        let handler = Box::new(move |msg: Message| {
            println!("Received message: {:?}", msg);
            received_clone.store(true, Ordering::SeqCst);
        });
        
        let cancel = bus.subscribe("test_topic".to_string(), handler).await.unwrap();
        
        let msg = Message {
            topic: "test_topic".to_string(),
            payload: b"hello".to_vec(),
        };
        
        bus.publish(msg).await.unwrap();
        
        // Wait for worker to process message!
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        assert!(received.load(Ordering::SeqCst));
        
        cancel(); // Clean up worker!
    }
}
