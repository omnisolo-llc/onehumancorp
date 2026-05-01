#![allow(dead_code, unused_mut, unused_variables, unused_imports, deprecated)]
use async_trait::async_trait;
use crate::queue::{Job, TaskJobHandler, TaskQueue};
use std::sync::Arc;

pub struct SubAgentWorker {
    queue: Arc<dyn TaskQueue>,
}

impl SubAgentWorker {
    pub fn new(queue: Arc<dyn TaskQueue>) -> Self {
        SubAgentWorker { queue }
    }
}

#[async_trait]
impl TaskJobHandler for SubAgentWorker {
    async fn handle(&self, job: Job) -> Result<(), String> {
        println!(
            "Worker executing job {} for task {} with role {}",
            job.id, job.parent_task_id, job.agent_role
        );

        // Simulate execution context initialization
        // e.g. initialize LLM context, tool access, reporting progress back to Shared Task List

        // In Rust, the Worker loop handles marking as complete, but if the subagent specific logic requires it:
        // We just return Ok(()) and the Worker wrapper in queue.rs calls self.queue.complete(&job.id).await;
        // So we don't need to manually call ProcessSubAgentJob here, but we could do any custom queue logic if we want.

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::queue::MemoryTaskQueue;
    use chrono::Utc;

    #[tokio::test]
    async fn test_sub_agent_worker_handle() {
        let queue = Arc::new(MemoryTaskQueue::new());
        let worker = SubAgentWorker::new(queue.clone());

        let job = Job {
            id: "job-1".to_string(),
            parent_task_id: "task-1".to_string(),
            agent_role: "test-role".to_string(),
            payload: "{}".to_string(),
            status: "PENDING".to_string(),
            attempts: 0,
            max_attempts: 3,
            run_after: Utc::now(),
            locked_until: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let result = worker.handle(job).await;
        assert!(result.is_ok());
    }
}
