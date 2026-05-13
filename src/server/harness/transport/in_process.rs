use super::Transport;
use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct InProcessTransport {
    channels: Arc<DashMap<String, broadcast::Sender<String>>>,
    locks: Arc<DashMap<String, (String, tokio::time::Instant)>>,
    presence: Arc<DashMap<String, (String, tokio::time::Instant)>>,
}

impl InProcessTransport {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(DashMap::new()),
            locks: Arc::new(DashMap::new()),
            presence: Arc::new(DashMap::new()),
        }
    }

    fn ensure_topic(&self, topic: &str) -> broadcast::Sender<String> {
        self.channels.entry(topic.to_string()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        }).clone()
    }
}

#[async_trait]
impl Transport for InProcessTransport {
    async fn send(&self, topic: &str, message: &str) -> Result<(), String> {
        let tx = self.ensure_topic(topic);
        let _ = tx.send(message.to_string()); // ignore error if no receivers
        Ok(())
    }

    async fn subscribe(&self, topic: &str) -> Result<broadcast::Receiver<String>, String> {
        let tx = self.ensure_topic(topic);
        Ok(tx.subscribe())
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        use dashmap::mapref::entry::Entry;

        let now = tokio::time::Instant::now();
        let expires_at = now + std::time::Duration::from_secs(ttl_seconds);

        match self.locks.entry(resource.to_string()) {
            Entry::Vacant(v) => {
                v.insert((owner.to_string(), expires_at));
                Ok(true)
            },
            Entry::Occupied(mut o) => {
                let current = o.get();
                if current.1 < now {
                    // Lock expired, we can take it
                    o.insert((owner.to_string(), expires_at));
                    Ok(true)
                } else {
                    Ok(false)
                }
            },
        }
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        self.locks.remove_if(resource, |_, current_owner_data| current_owner_data.0 == owner);
        Ok(())
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        let now = tokio::time::Instant::now();
        let expires_at = now + std::time::Duration::from_secs(ttl_seconds);
        self.presence.insert(agent_id.to_string(), (status.to_string(), expires_at));
        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        let now = tokio::time::Instant::now();
        let mut agents = Vec::new();
        for kv in self.presence.iter() {
            if kv.value().1 >= now {
                agents.push((kv.key().clone(), kv.value().0.clone()));
            }
        }
        Ok(agents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_process_pub_sub() {
        let transport = InProcessTransport::new();

        let mut rx1 = transport.subscribe("test-topic").await.unwrap();
        let mut rx2 = transport.subscribe("test-topic").await.unwrap();

        transport.send("test-topic", "hello").await.unwrap();

        assert_eq!(rx1.recv().await.unwrap(), "hello");
        assert_eq!(rx2.recv().await.unwrap(), "hello");
    }

    #[tokio::test]
    async fn test_in_process_locks() {
        let transport = InProcessTransport::new();

        let locked = transport.acquire_lock("my-res", "owner1", 10).await.unwrap();
        assert!(locked);

        let locked2 = transport.acquire_lock("my-res", "owner2", 10).await.unwrap();
        assert!(!locked2);

        transport.release_lock("my-res", "owner1").await.unwrap();

        let locked3 = transport.acquire_lock("my-res", "owner2", 10).await.unwrap();
        assert!(locked3);
    }

    #[tokio::test]
    async fn test_in_process_locks_expiration() {
        // We simulate a fast timeout so we can actually acquire it afterwards
        let transport = InProcessTransport::new();

        // TTL=0 means it's expired the moment it's acquired or shortly after checking
        let locked = transport.acquire_lock("exp-res", "owner1", 0).await.unwrap();
        assert!(locked);

        // small sleep to guarantee expiration
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;

        // now owner2 should be able to acquire it
        let locked2 = transport.acquire_lock("exp-res", "owner2", 10).await.unwrap();
        assert!(locked2);
    }

    #[tokio::test]
    async fn test_in_process_presence() {
        let transport = InProcessTransport::new();

        transport.register_presence("agent1", "online", 10).await.unwrap();

        let agents = transport.get_active_agents().await.unwrap();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0], ("agent1".to_string(), "online".to_string()));
    }
}
