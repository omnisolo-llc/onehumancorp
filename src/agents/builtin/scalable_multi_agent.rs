use futures::future::join_all;
use std::sync::Arc;

/// SOTA Harness Patterns (2025-2026): 4. Scalable multi-agent -> single-user CLI to 1000+ agent cloud deployments

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeploymentMode {
    LocalCli,
    CloudDistributed,
}

#[derive(Debug, Clone)]
pub struct TaskChunk {
    pub id: String,
    pub payload: String,
}

#[derive(Debug, Clone)]
pub struct TaskResult {
    pub chunk_id: String,
    pub output: String,
}

#[async_trait::async_trait]
pub trait AgentNode: Send + Sync {
    async fn execute(&self, chunk: TaskChunk) -> Result<TaskResult, String>;
}

pub struct CloudOrchestrator {
    max_retries: usize,
    concurrency_limit: usize,
    timeout_secs: u64,
    mode: DeploymentMode,
    nodes: Vec<Arc<dyn AgentNode>>,
}

impl CloudOrchestrator {
    pub fn new(
        mode: DeploymentMode,
        nodes: Vec<Arc<dyn AgentNode>>,
        max_retries: usize,
        concurrency_limit: usize,
        timeout_secs: u64,
    ) -> Self {
        Self {
            mode,
            nodes,
            max_retries,
            concurrency_limit,
            timeout_secs,
        }
    }

    /// Distributes tasks across the fleet of agents.
    /// Simulates scaling from a single-user CLI context to 1000+ cloud agents.
    pub async fn distribute_and_execute(
        &self,
        tasks: Vec<TaskChunk>,
    ) -> Result<Vec<TaskResult>, String> {
        if tasks.is_empty() {
            return Ok(Vec::new());
        }

        if self.nodes.is_empty() {
            return Err("No agent nodes available for deployment.".to_string());
        }

        match self.mode {
            DeploymentMode::LocalCli => {
                // In local CLI mode, we might just run them sequentially or concurrently but bounded by local resources
                let mut results = Vec::new();
                for task in tasks {
                    // Just pick the first node for local simulation
                    let node = &self.nodes[0];
                    let res = node.execute(task).await?;
                    results.push(res);
                }
                Ok(results)
            }
            DeploymentMode::CloudDistributed => {
                // In cloud distributed mode, we fan-out tasks to our nodes.
                // Using a semaphore to limit concurrency.
                let semaphore = Arc::new(tokio::sync::Semaphore::new(self.concurrency_limit));
                let mut futures = Vec::new();
                let num_nodes = self.nodes.len();
                let max_retries = self.max_retries;
                let timeout_secs = self.timeout_secs;

                for (i, task) in tasks.into_iter().enumerate() {
                    let node = self.nodes[i % num_nodes].clone();
                    let sem_clone = semaphore.clone();

                    let fut = async move {
                        let _permit = sem_clone.acquire().await.unwrap();
                        let mut retries = 0;
                        let mut last_error = String::new();

                        while retries <= max_retries {
                            let execute_fut = node.execute(task.clone());
                            let timeout_duration = std::time::Duration::from_secs(timeout_secs);

                            match tokio::time::timeout(timeout_duration, execute_fut).await {
                                Ok(Ok(res)) => return Ok(res),
                                Ok(Err(e)) => {
                                    last_error = e;
                                }
                                Err(_) => {
                                    last_error = "Task timed out".to_string();
                                }
                            }

                            retries += 1;
                            if retries <= max_retries {
                                // Exponential backoff
                                let backoff = std::time::Duration::from_millis(
                                    100 * 2_u64.pow(retries as u32 - 1),
                                );
                                tokio::time::sleep(backoff).await;
                            }
                        }

                        Err(format!(
                            "Task {} failed after {} retries: {}",
                            task.id, max_retries, last_error
                        ))
                    };
                    futures.push(tokio::spawn(fut));
                }

                let execution_results = join_all(futures).await;
                let mut results = Vec::new();
                for res in execution_results {
                    match res {
                        Ok(Ok(task_res)) => results.push(task_res),
                        Ok(Err(e)) => return Err(e),
                        Err(e) => return Err(format!("Task execution panicked: {}", e)),
                    }
                }
                Ok(results)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    struct FailingAgentNode {
        execution_count: Arc<AtomicUsize>,
        fail_times: usize,
    }

    impl FailingAgentNode {
        fn new(fail_times: usize) -> Self {
            Self {
                execution_count: Arc::new(AtomicUsize::new(0)),
                fail_times,
            }
        }
    }

    #[async_trait::async_trait]
    impl AgentNode for FailingAgentNode {
        async fn execute(&self, chunk: TaskChunk) -> Result<TaskResult, String> {
            let count = self.execution_count.fetch_add(1, Ordering::SeqCst);
            if count < self.fail_times {
                return Err("Simulated failure".to_string());
            }
            Ok(TaskResult {
                chunk_id: chunk.id,
                output: format!("Recovered: {}", chunk.payload),
            })
        }
    }

    #[tokio::test]
    async fn test_scalable_multi_agent_retries() {
        // Fails 2 times, succeeds on the 3rd. Max retries is 3.
        let node = Arc::new(FailingAgentNode::new(2));
        let orchestrator = CloudOrchestrator::new(
            DeploymentMode::CloudDistributed,
            vec![node.clone() as Arc<dyn AgentNode>],
            3,
            10,
            60,
        );

        let tasks = vec![TaskChunk {
            id: "retry1".to_string(),
            payload: "Data".to_string(),
        }];
        let results = orchestrator.distribute_and_execute(tasks).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].output, "Recovered: Data");
        assert_eq!(node.execution_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_scalable_multi_agent_retries_exceeded() {
        // Fails 4 times. Max retries is 2.
        let node = Arc::new(FailingAgentNode::new(4));
        let orchestrator = CloudOrchestrator::new(
            DeploymentMode::CloudDistributed,
            vec![node.clone() as Arc<dyn AgentNode>],
            2,
            10,
            60,
        );

        let tasks = vec![TaskChunk {
            id: "retry2".to_string(),
            payload: "Data".to_string(),
        }];
        let result = orchestrator.distribute_and_execute(tasks).await;

        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("failed after 2 retries"));
        assert_eq!(node.execution_count.load(Ordering::SeqCst), 3); // 1 initial + 2 retries
    }

    struct TimeoutAgentNode;

    #[async_trait::async_trait]
    impl AgentNode for TimeoutAgentNode {
        async fn execute(&self, chunk: TaskChunk) -> Result<TaskResult, String> {
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            Ok(TaskResult {
                chunk_id: chunk.id,
                output: "Done".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_scalable_multi_agent_timeout() {
        let node = Arc::new(TimeoutAgentNode);
        // Timeout set to 1 second, but task takes 3 seconds
        let orchestrator = CloudOrchestrator::new(
            DeploymentMode::CloudDistributed,
            vec![node.clone() as Arc<dyn AgentNode>],
            0,
            10,
            1,
        );

        let tasks = vec![TaskChunk {
            id: "timeout1".to_string(),
            payload: "Data".to_string(),
        }];
        let result = orchestrator.distribute_and_execute(tasks).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Task timed out"));
    }

    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockAgentNode {
        execution_count: Arc<AtomicUsize>,
    }

    impl MockAgentNode {
        fn new() -> Self {
            Self {
                execution_count: Arc::new(AtomicUsize::new(0)),
            }
        }
    }

    #[async_trait::async_trait]
    impl AgentNode for MockAgentNode {
        async fn execute(&self, chunk: TaskChunk) -> Result<TaskResult, String> {
            self.execution_count.fetch_add(1, Ordering::SeqCst);
            Ok(TaskResult {
                chunk_id: chunk.id,
                output: format!("Processed: {}", chunk.payload),
            })
        }
    }

    #[tokio::test]
    async fn test_scalable_multi_agent_local_cli() {
        let node = Arc::new(MockAgentNode::new());
        let orchestrator =
            CloudOrchestrator::new(DeploymentMode::LocalCli, vec![node.clone()], 3, 10, 60);

        let tasks = vec![
            TaskChunk {
                id: "1".to_string(),
                payload: "Data A".to_string(),
            },
            TaskChunk {
                id: "2".to_string(),
                payload: "Data B".to_string(),
            },
        ];

        let results = orchestrator.distribute_and_execute(tasks).await.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(node.execution_count.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_scalable_multi_agent_cloud_distributed_1000_agents() {
        let mut nodes: Vec<Arc<dyn AgentNode>> = Vec::new();
        let mut node_counts = Vec::new();

        // Simulate 1000 agent nodes in the cloud
        for _ in 0..1000 {
            let node = Arc::new(MockAgentNode::new());
            node_counts.push(node.execution_count.clone());
            nodes.push(node as Arc<dyn AgentNode>);
        }

        let orchestrator =
            CloudOrchestrator::new(DeploymentMode::CloudDistributed, nodes, 3, 100, 60);

        let mut tasks = Vec::new();
        // Send 1000 tasks
        for i in 0..1000 {
            tasks.push(TaskChunk {
                id: i.to_string(),
                payload: format!("Payload {}", i),
            });
        }

        let results = orchestrator.distribute_and_execute(tasks).await.unwrap();
        assert_eq!(results.len(), 1000);

        // Verify each node processed exactly 1 task
        for count in node_counts {
            assert_eq!(count.load(Ordering::SeqCst), 1);
        }
    }
}
