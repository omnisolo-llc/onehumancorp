use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use async_trait::async_trait;
use crate::mesh::MeshTransport;
use std::collections::HashMap;

pub struct MemoryMeshTransport {
    subs: Mutex<HashMap<String, broadcast::Sender<Vec<u8>>>>,
}

impl MemoryMeshTransport {
    pub fn new() -> Self {
        MemoryMeshTransport {
            subs: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl MeshTransport for MemoryMeshTransport {
    async fn publish(&self, channel: String, payload: Vec<u8>) -> Result<(), String> {
        let subs = self.subs.lock().await;
        if let Some(tx) = subs.get(&channel) {
            let _ = tx.send(payload);
        }
        Ok(())
    }

    async fn subscribe(&self, channel: String, handler: Box<dyn Fn(Vec<u8>) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let mut subs = self.subs.lock().await;
        let tx = subs.entry(channel.clone()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });

        let mut rx = tx.subscribe();

        let worker = tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(msg) => handler(msg),
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
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

impl Default for MemoryMeshTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[tokio::test]
    async fn test_memory_mesh_pubsub() {
        let bus = MemoryMeshTransport::new();
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Vec<u8>| {
            if msg == b"hello" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = bus.subscribe("test_topic".to_string(), handler).await.unwrap();

        bus.publish("test_topic".to_string(), b"hello".to_vec()).await.unwrap();

        // Wait for worker to process message
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        assert!(received.load(Ordering::SeqCst));

        cancel(); // Clean up worker
    }
}
