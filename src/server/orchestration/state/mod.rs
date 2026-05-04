pub mod cloud;
pub mod standalone;
#[cfg(test)]
mod test;

use crate::tasks::SharedTask;
use async_trait::async_trait;

#[async_trait]
pub trait StateManager: Send + Sync {
    async fn transition_state(
        &self,
        task_id: &str,
        tenant_id: &str,
        from_state: &str,
        to_state: &str,
        agent_id: Option<&str>,
        reason: Option<&str>,
    ) -> Result<(), String>;

    async fn pull_available_tasks(&self, limit: i64) -> Result<Vec<SharedTask>, String>;
}

use ohc_builtin_agent::mesh::transport::MeshTransport;
use std::sync::Arc;
use std::time::Duration;

pub struct TransportLockGuard {
    transport: Arc<dyn MeshTransport>,
    key: String,
    owner: String,
}

impl TransportLockGuard {
    pub async fn acquire(
        transport: Arc<dyn MeshTransport>,
        key: String,
        owner: String,
        wait: bool,
    ) -> Result<Self, String> {
        let mut retries = if wait { 100 } else { 1 };
        let delay = Duration::from_millis(50);

        while retries > 0 {
            match transport.acquire_lock(&key, &owner, 30).await {
                Ok(true) => {
                    return Ok(Self {
                        transport,
                        key,
                        owner,
                    });
                }
                Ok(false) => {
                    if !wait {
                        return Err(format!(
                            "Task {} is currently locked via MeshTransport",
                            key
                        ));
                    }
                    retries -= 1;
                    tokio::time::sleep(delay).await;
                }
                Err(e) => {
                    return Err(format!("Failed to acquire lock: {}", e));
                }
            }
        }

        Err(format!(
            "Task {} is currently locked via MeshTransport (timeout)",
            key
        ))
    }
}

impl Drop for TransportLockGuard {
    fn drop(&mut self) {
        let transport = self.transport.clone();
        let key = self.key.clone();
        let owner = self.owner.clone();
        tokio::spawn(async move {
            let _ = transport.release_lock(&key, &owner).await;
        });
    }
}
