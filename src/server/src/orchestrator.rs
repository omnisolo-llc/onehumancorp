use async_trait::async_trait;
use std::sync::Arc;
use crate::hub::Hub;
use crate::tasks::SharedTask;

#[async_trait]
pub trait TaskOrchestrator: Send + Sync {
    async fn receive_high_level_request(&self, org_id: &str, title: &str) -> Result<String, String>;
    async fn enqueue_task(&self, task: SharedTask, depends_on: Vec<String>) -> Result<SharedTask, String>;
    async fn acquire_ready_task(&self, agent_id: &str, capabilities: Vec<String>) -> Result<Option<SharedTask>, String>;
    async fn complete_task(&self, task_id: &str, agent_id: &str, result: &str) -> Result<(), String>;
}

pub struct DefaultTaskOrchestrator {
    hub: Arc<Hub>,
}

impl DefaultTaskOrchestrator {
    pub fn new(hub: Arc<Hub>) -> Self {
        DefaultTaskOrchestrator { hub }
    }
}

#[async_trait]
impl TaskOrchestrator for DefaultTaskOrchestrator {
    async fn receive_high_level_request(&self, org_id: &str, title: &str) -> Result<String, String> {
        let task = self.hub.task_manager().create_task(
            org_id.to_string(),
            String::new(),
            title.to_string(),
            "High level request".to_string(),
            "P1".to_string(),
        )?;
        Ok(task.id)
    }

    async fn enqueue_task(&self, task: SharedTask, depends_on: Vec<String>) -> Result<SharedTask, String> {
        let mut task = task;
        task.status = if depends_on.is_empty() { "READY".to_string() } else { "PENDING".to_string() };
        task.dependencies = depends_on;
        
        self.hub.task_manager().insert_task(task.clone());
        Ok(task)
    }

    async fn acquire_ready_task(&self, agent_id: &str, _capabilities: Vec<String>) -> Result<Option<SharedTask>, String> {
        let tasks = self.hub.task_manager().poll_tasks(agent_id, 1);
        Ok(tasks.into_iter().next())
    }

    async fn complete_task(&self, task_id: &str, agent_id: &str, result: &str) -> Result<(), String> {
        self.hub.task_manager().complete_task(task_id, agent_id, result.to_string())?;
        Ok(())
    }
}
