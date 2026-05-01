use async_trait::async_trait;
use std::sync::Arc;
use crate::hub::Hub;
use crate::tasks::SharedTask;

#[async_trait]
#[allow(dead_code)]
pub trait SubAgentSpawner: Send + Sync {
    async fn spawn(&self, task: SharedTask) -> Result<(), String>;
}

#[allow(dead_code)]
pub struct DefaultSubAgentSpawner {
    hub: Arc<Hub>,
}

#[allow(dead_code)]
impl DefaultSubAgentSpawner {
    pub fn new(hub: Arc<Hub>) -> Self {
        DefaultSubAgentSpawner { hub }
    }
}

#[async_trait]
impl SubAgentSpawner for DefaultSubAgentSpawner {
    async fn spawn(&self, task: SharedTask) -> Result<(), String> {
        let hub = self.hub.clone();
        tokio::spawn(async move {
            println!("Spawning sub-agent for task: {}", task.id);
            // Simulate work
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            // Complete task in task manager
            let _ = hub.task_manager().complete_task(&task.id, "sub-agent", "Success from spawned agent".to_string());
        });
        
        Ok(())
    }
}
