pub mod cloud;
#[cfg(test)]
mod parity_test;
pub mod standalone;
#[cfg(test)]
mod test;

use crate::orchestration::mesh::TeammateMesh;
use crate::tasks::SharedTask;
use async_trait::async_trait;
use std::sync::Arc;

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
    pub async fn acquire(
        mesh: Arc<dyn TeammateMesh>,
        resource: String,
        owner: String,
        ttl_seconds: u64,
    ) -> Result<Self, String> {
        let acquired = mesh.acquire_lock(&resource, &owner, ttl_seconds).await?;
        if acquired {
            Ok(Self {
                mesh,
                resource,
                owner,
            })
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

pub fn state_manager_timeout() -> std::time::Duration {
    std::env::var("OHC_STATE_MANAGER_TIMEOUT_MS")
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .map(std::time::Duration::from_millis)
        .unwrap_or_else(|| std::time::Duration::from_secs(2))
}
