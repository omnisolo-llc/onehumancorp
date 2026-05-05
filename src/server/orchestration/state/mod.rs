pub mod cloud;
pub mod standalone;
#[cfg(test)]
mod test;

use crate::tasks::SharedTask;
use async_trait::async_trait;
use std::sync::Arc;
use ohc_builtin_agent::mesh::transport::MeshTransport;

pub(crate) struct MeshLockGuard {
    transport: Arc<dyn MeshTransport>,
    resource: String,
    owner: String,
}

impl MeshLockGuard {
    pub(crate) async fn acquire(transport: Arc<dyn MeshTransport>, resource: String, owner: String, ttl_seconds: u64) -> Result<Self, String> {
        let acquired = transport.acquire_lock(&resource, &owner, ttl_seconds).await?;
        if acquired {
            Ok(Self { transport, resource, owner })
        } else {
            Err(format!("Resource {} is currently locked", resource))
        }
    }
}

impl Drop for MeshLockGuard {
    fn drop(&mut self) {
        let transport = self.transport.clone();
        let resource = self.resource.clone();
        let owner = self.owner.clone();
        tokio::spawn(async move {
            let _ = transport.release_lock(&resource, &owner).await;
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
