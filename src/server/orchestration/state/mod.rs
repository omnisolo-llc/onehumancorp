pub mod universal;
#[cfg(test)]
mod test;
#[cfg(test)]
mod parity_test;

use crate::tasks::SharedTask;
use async_trait::async_trait;
use std::sync::Arc;
use crate::orchestration::mesh::TeammateMesh;

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

pub struct MeshLockGuard {
    mesh: Arc<dyn TeammateMesh>,
    resource: String,
    owner: String,
}

impl MeshLockGuard {
    pub async fn acquire(mesh: Arc<dyn TeammateMesh>, resource: String, owner: String, ttl_seconds: u64) -> Result<Self, String> {
        let acquired = mesh.acquire_lock(&resource, &owner, ttl_seconds).await?;
        if acquired {
            Ok(Self { mesh, resource, owner })
        } else {
            Err(format!("Resource {} is currently locked", resource))
        }
    }
}

impl Drop for MeshLockGuard {
    fn drop(&mut self) {
        let mesh = self.mesh.clone();
        let resource = self.resource.clone();
        let owner = self.owner.clone();
        tokio::spawn(async move {
            let _ = mesh.release_lock(&resource, &owner).await;
        });
    }
}
