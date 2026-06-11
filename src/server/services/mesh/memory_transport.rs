use crate::mesh::protocol::TeammateMessage;
use crate::mesh::transport::MeshTransport;
use async_trait::async_trait;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use std::collections::HashMap;

pub struct MemoryMeshTransport {
    subs: Mutex<HashMap<String, broadcast::Sender<TeammateMessage>>>,
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
    async fn publish(&self, message: TeammateMessage) -> Result<(), String> {
        let tenant_id = message.tenant_id.clone();
        if tenant_id.is_empty() {
            return Err("tenant_id is required".to_string());
        }

        let subs = self.subs.lock().await;
        if let Some(tx) = subs.get(&tenant_id) {
            let _ = tx.send(message);
        }
        Ok(())
    }

    async fn subscribe(&self, tenant_id: &str, handler: Box<dyn Fn(TeammateMessage) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        if tenant_id.is_empty() {
            return Err("tenant_id is required".to_string());
        }

        let mut subs = self.subs.lock().await;
        let tx = subs.entry(tenant_id.to_string()).or_insert_with(|| {
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
