pub mod cloud;
pub mod standalone;
#[cfg(test)]
mod test;

use crate::tasks::SharedTask;
use async_trait::async_trait;
use std::sync::Arc;

pub struct MeshLockGuard {
    transport: Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>,
    key: String,
    owner: String,
}

impl MeshLockGuard {
    pub async fn acquire(
        transport: Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>,
        key: String,
        owner: String,
        ttl_seconds: u64,
    ) -> Result<Self, String> {
        let acquired = transport.acquire_lock(&key, &owner, ttl_seconds).await?;
        if acquired {
            Ok(Self {
                transport,
                key,
                owner,
            })
        } else {
            Err(format!("Task {} is currently locked via MeshTransport", key))
        }
    }
}

impl Drop for MeshLockGuard {
    fn drop(&mut self) {
        let transport = self.transport.clone();
        let key = self.key.clone();
        let owner = self.owner.clone();
        tokio::spawn(async move {
            let _ = transport.release_lock(&key, &owner).await;
        });
    }
}

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
