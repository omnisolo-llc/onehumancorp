use std::sync::Arc;
use tokio::sync::mpsc;
use futures::future::join_all;

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
    mode: DeploymentMode,
    nodes: Vec<Arc<dyn AgentNode>>,
}

impl CloudOrchestrator {
    pub fn new(mode: DeploymentMode, nodes: Vec<Arc<dyn AgentNode>>) -> Self {
        Self { mode, nodes }
    }

    /// Distributes tasks across the fleet of agents.
    /// Simulates scaling from a single-user CLI context to 1000+ cloud agents.
    pub async fn distribute_and_execute(&self, tasks: Vec<TaskChunk>) -> Result<Vec<TaskResult>, String> {
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
                // If we have 1000 tasks and 1000 nodes, they run in parallel.
                let mut futures = Vec::new();
                let num_nodes = self.nodes.len();

                for (i, task) in tasks.into_iter().enumerate() {
                    let node = self.nodes[i % num_nodes].clone();
                    let fut = async move {
                        node.execute(task).await
                    };
                    futures.push(fut);
                }

                let execution_results = join_all(futures).await;
                let mut results = Vec::new();
                for res in execution_results {
                    results.push(res?);
                }
                Ok(results)
            }
        }
    }
}

#[cfg(test)]
mod tests {
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
        let orchestrator = CloudOrchestrator::new(DeploymentMode::LocalCli, vec![node.clone()]);

        let tasks = vec![
            TaskChunk { id: "1".to_string(), payload: "Data A".to_string() },
            TaskChunk { id: "2".to_string(), payload: "Data B".to_string() },
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

        let orchestrator = CloudOrchestrator::new(DeploymentMode::CloudDistributed, nodes);

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
