/// Dynamic workflows orchestrate many subagents from a script Claude writes and you can rerun.
/// Use them for codebase audits, large migrations, and cross-checked research.
///
/// Limits implemented:
/// - Up to 16 concurrent agents.
/// - Up to 1,000 agents total per run.

use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};

#[derive(Debug)]
pub struct DynamicWorkflow {
    #[allow(dead_code)]
    script: String,

    max_concurrent: usize,
    max_total_agents: usize,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub instructions: String,
}

#[derive(Debug)]
pub struct TaskResult {
    pub task_id: String,
    pub output: String,
}

#[async_trait::async_trait]
pub trait WorkflowAgent: Send + Sync {
    async fn execute(&self, task: Task) -> Result<TaskResult, String>;
}

impl DynamicWorkflow {
    pub fn new(script: &str) -> Self {
        Self {
            script: script.to_string(),
            max_concurrent: 16,
            max_total_agents: 1000,
        }
    }

    pub async fn run_workflow(
        &self,
        tasks: Vec<Task>,
        agent_factory: Arc<dyn WorkflowAgent>,
    ) -> Result<Vec<TaskResult>, String> {
        if tasks.len() > self.max_total_agents {
            return Err(format!(
                "Workflow abort: Requested {} agents, which exceeds the max limit of {}",
                tasks.len(),
                self.max_total_agents
            ));
        }

        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));
        let (tx, mut rx) = mpsc::channel(tasks.len().max(1));

        let mut handles = Vec::new();

        for task in tasks {
            let sem = semaphore.clone();
            let agent = agent_factory.clone();
            let tx_clone = tx.clone();

            handles.push(tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                let res = agent.execute(task).await;
                let _ = tx_clone.send(res).await;
            }));
        }

        // drop original tx so rx will close after all tasks finish
        drop(tx);

        let mut results = Vec::new();
        while let Some(res) = rx.recv().await {
            match res {
                Ok(r) => results.push(r),
                Err(e) => {
                    for handle in &handles { handle.abort(); }
                    return Err(e);
                }
            }
        }

        for handle in handles {
            let _ = handle.await.map_err(|e| format!("Task failed to join: {}", e))?;
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockAgent {
        active_agents: Arc<AtomicUsize>,
        max_active_observed: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl WorkflowAgent for MockAgent {
        async fn execute(&self, task: Task) -> Result<TaskResult, String> {
            let current = self.active_agents.fetch_add(1, Ordering::SeqCst) + 1;

            // update max observed
            let mut max = self.max_active_observed.load(Ordering::SeqCst);
            while current > max {
                match self.max_active_observed.compare_exchange_weak(max, current, Ordering::SeqCst, Ordering::SeqCst) {
                    Ok(_) => break,
                    Err(x) => max = x,
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            self.active_agents.fetch_sub(1, Ordering::SeqCst);

            Ok(TaskResult {
                task_id: task.id,
                output: format!("Processed: {}", task.instructions),
            })
        }
    }

    #[tokio::test]
    async fn test_dynamic_workflow_success() {
        let wf = DynamicWorkflow::new("let x = 1;");
        let agent = Arc::new(MockAgent {
            active_agents: Arc::new(AtomicUsize::new(0)),
            max_active_observed: Arc::new(AtomicUsize::new(0)),
        });

        let tasks = vec![
            Task { id: "1".to_string(), instructions: "task 1".to_string() },
            Task { id: "2".to_string(), instructions: "task 2".to_string() },
        ];

        let results = wf.run_workflow(tasks, agent.clone()).await.unwrap();
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_dynamic_workflow_concurrency_limit() {
        let wf = DynamicWorkflow::new("orchestrate();");
        let agent = Arc::new(MockAgent {
            active_agents: Arc::new(AtomicUsize::new(0)),
            max_active_observed: Arc::new(AtomicUsize::new(0)),
        });

        let mut tasks = Vec::new();
        for i in 0..50 {
            tasks.push(Task { id: i.to_string(), instructions: format!("task {}", i) });
        }

        let results = wf.run_workflow(tasks, agent.clone()).await.unwrap();
        assert_eq!(results.len(), 50);

        let max_observed = agent.max_active_observed.load(Ordering::SeqCst);
        assert!(max_observed <= 16, "Should not exceed max_concurrent of 16, but got {}", max_observed);
    }

    #[tokio::test]
    async fn test_dynamic_workflow_total_limit() {
        let wf = DynamicWorkflow::new("orchestrate();");
        let agent = Arc::new(MockAgent {
            active_agents: Arc::new(AtomicUsize::new(0)),
            max_active_observed: Arc::new(AtomicUsize::new(0)),
        });

        let mut tasks = Vec::new();
        for i in 0..1005 {
            tasks.push(Task { id: i.to_string(), instructions: format!("task {}", i) });
        }

        let res = wf.run_workflow(tasks, agent).await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("exceeds the max limit of 1000"));
    }
}
