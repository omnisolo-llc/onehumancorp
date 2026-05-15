use super::MeshTransport;
use crate::proto::hub::TeammateMeshEvent as Message;
use async_trait::async_trait;
use dashmap::DashMap;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct MemoryTransport {
    subs: DashMap<String, broadcast::Sender<Message>>,
    presence: DashMap<String, (String, std::time::Instant)>, // agent_id -> (status, expires_at)
}

impl MemoryTransport {
    pub fn new() -> Self {
        MemoryTransport {
            subs: DashMap::new(),
            presence: DashMap::new(),
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

    async fn subscribe(
        &self,
        topic: &str,
        handler: Box<dyn Fn(Message) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let tx = self
            .subs
            .entry(topic.to_string())
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(100);
                tx
            })
            .clone();

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

    async fn acquire_lock(
        &self,
        resource: &str,
        owner: &str,
        ttl_seconds: u64,
    ) -> Result<bool, String> {
        let lock_path = std::env::temp_dir().join(format!("ohc_mesh_lock_{}", resource));
        let expires_at = chrono::Utc::now().timestamp_millis() + (ttl_seconds * 1000) as i64;
        let payload = format!("{}:{}", owner, expires_at);

        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)
        {
            Ok(mut f) => {
                use std::io::Write;
                let _ = f.write_all(payload.as_bytes());
                Ok(true)
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                if let Ok(owner_bytes) = std::fs::read(&lock_path) {
                    let current_data = String::from_utf8_lossy(&owner_bytes).into_owned();
                    if let Some((stored_owner, stored_exp)) = current_data.split_once(':') {
                        if let Ok(exp) = stored_exp.parse::<i64>() {
                            if stored_owner == owner || exp <= chrono::Utc::now().timestamp_millis()
                            {
                                let _ = std::fs::remove_file(&lock_path);
                                if let Ok(mut f) = std::fs::OpenOptions::new()
                                    .write(true)
                                    .create_new(true)
                                    .open(&lock_path)
                                {
                                    use std::io::Write;
                                    let _ = f.write_all(payload.as_bytes());
                                    return Ok(true);
                                }
                            }
                        }
                    } else {
                        // Malformed, overwrite
                        let _ = std::fs::remove_file(&lock_path);
                        if let Ok(mut f) = std::fs::OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(&lock_path)
                        {
                            use std::io::Write;
                            let _ = f.write_all(payload.as_bytes());
                            return Ok(true);
                        }
                    }
                }
                Ok(false)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        let lock_path = std::env::temp_dir().join(format!("ohc_mesh_lock_{}", resource));
        if let Ok(owner_bytes) = std::fs::read(&lock_path) {
            let current_data = String::from_utf8_lossy(&owner_bytes).into_owned();
            if let Some((stored_owner, _)) = current_data.split_once(':') {
                if stored_owner == owner {
                    let _ = std::fs::remove_file(lock_path);
                }
            }
        }
        Ok(())
    }
    async fn register_presence(
        &self,
        agent_id: &str,
        status: &str,
        ttl_seconds: u64,
    ) -> Result<(), String> {
        let expires_at = std::time::Instant::now() + std::time::Duration::from_secs(ttl_seconds);
        self.presence
            .insert(agent_id.to_string(), (status.to_string(), expires_at));
        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        let now = std::time::Instant::now();

        // Remove expired
        let expired_keys: Vec<String> = self
            .presence
            .iter()
            .filter(|entry| entry.value().1 <= now)
            .map(|entry| entry.key().clone())
            .collect();

        for key in expired_keys {
            self.presence.remove(&key);
        }

        let agents = self
            .presence
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().0.clone()))
            .collect();

        Ok(agents)
    }
}
