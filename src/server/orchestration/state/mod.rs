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
