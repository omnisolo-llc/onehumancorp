use super::{Bus, DistributedLock, Message};
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use async_trait::async_trait;

#[allow(dead_code)]
pub struct MemoryBus {
    subs: Mutex<std::collections::HashMap<String, broadcast::Sender<Message>>>,
    locks: Mutex<std::collections::HashMap<String, (String, std::time::Instant)>>,
}

#[allow(dead_code)]
impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            subs: Mutex::new(std::collections::HashMap::new()),
            locks: Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait]
impl Bus for MemoryBus {
    async fn publish(&self, msg: Message) -> Result<(), String> {
        let subs = self.subs.lock().await;
        for (topic, tx) in subs.iter() {
            if msg.topic == *topic || (topic.ends_with(':') && msg.topic.starts_with(topic)) {
                let _ = tx.send(msg.clone());
            }
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
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DistributedLock for MemoryBus {
    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        let mut locks = self.locks.lock().await;
        let now = std::time::Instant::now();

        // Remove expired locks
        locks.retain(|_, (_, expires_at)| *expires_at > now);

        let expires_at = now + std::time::Duration::from_secs(ttl_seconds);
        if let Some((current_owner, _)) = locks.get(resource) {
            if current_owner == owner {
                locks.insert(resource.to_string(), (owner.to_string(), expires_at));
                return Ok(true);
            }
            return Ok(false);
        }

        locks.insert(resource.to_string(), (owner.to_string(), expires_at));
        Ok(true)
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        let mut locks = self.locks.lock().await;
        if let Some((current_owner, _)) = locks.get(resource) {
            if current_owner == owner {
                locks.remove(resource);
            }
        }
        Ok(())
    }
}
