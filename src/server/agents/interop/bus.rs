use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use async_trait::async_trait;
use redis::AsyncCommands;
use futures::StreamExt;
use prost::Message as ProstMessage;

// We will use the raw bytes so we can serialize/deserialize protobuf.
// Alternatively we can use ohc::agent::SubagentLifecycleEvent if imported, but
// we will stick to raw payload and let the callers handle the protobuf translation,
// or define a typed bus if we want. The prompt instructed: "All wire formats are protobuf. No ad-hoc JSON on the wire between services."
// By exposing `payload: Vec<u8>`, callers must use `.encode()` and `.decode()` on their prost types.

#[derive(Debug, Clone)]
pub struct Message {
    pub topic: String,
    pub payload: Vec<u8>,
}

#[async_trait]
pub trait BusProvider: Send + Sync {
    async fn publish(&self, msg: Message) -> Result<(), String>;
    async fn subscribe(&self, topic: String, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;
    async fn close(&self) -> Result<(), String>;
}

pub struct StandaloneBusProvider {
    subs: Mutex<std::collections::HashMap<String, broadcast::Sender<Message>>>,
}

impl StandaloneBusProvider {
    pub fn new() -> Self {
        StandaloneBusProvider {
            subs: Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait]
impl BusProvider for StandaloneBusProvider {
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

impl Default for StandaloneBusProvider {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CloudBusProvider {
    client: redis::Client,
    workers: Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>>,
}

impl CloudBusProvider {
    pub fn new(client: redis::Client) -> Self {
        CloudBusProvider {
            client,
            workers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl BusProvider for CloudBusProvider {
    async fn publish(&self, msg: Message) -> Result<(), String> {
        let mut con = self.client.get_async_connection().await.map_err(|e| e.to_string())?;
        con.publish::<_, _, ()>(&msg.topic, &msg.payload).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(&self, topic: String, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let client = self.client.clone();

        let worker = tokio::spawn(async move {
            if let Ok(mut pubsub) = client.get_async_pubsub().await {
                if pubsub.subscribe(&topic).await.is_ok() {
                    let mut stream = pubsub.on_message();
                    while let Some(msg) = stream.next().await {
                        if let Ok(payload) = msg.get_payload::<Vec<u8>>() {
                            let m = Message {
                                topic: msg.get_channel_name().to_string(),
                                payload,
                            };
                            handler(m);
                        }
                    }
                }
            }
        });

        let worker_abort_handle = worker.abort_handle();

        let mut workers = self.workers.lock().await;
        workers.push(worker);

        let cancel = Box::new(move || {
            worker_abort_handle.abort();
        });

        Ok(cancel)
    }

    async fn close(&self) -> Result<(), String> {
        let mut workers = self.workers.lock().await;
        for w in workers.drain(..) {
            w.abort();
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[tokio::test]
    async fn test_standalone_bus_pub_sub() {
        let bus = StandaloneBusProvider::new();
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            println!("Received message: {:?}", msg);
            received_clone.store(true, Ordering::SeqCst);
        });

        let cancel = bus.subscribe("test_topic".to_string(), handler).await.unwrap();

        let msg = Message {
            topic: "test_topic".to_string(),
            // Since we mandate protobuf format, we pass some raw bytes as a mock
            payload: b"\x0a\x03\x68\x65\x78".to_vec(),
        };

        bus.publish(msg).await.unwrap();

        // Wait for worker to process message!
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        assert!(received.load(Ordering::SeqCst));

        cancel(); // Clean up worker!
    }
}
