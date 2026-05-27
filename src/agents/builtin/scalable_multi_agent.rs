use std::sync::Arc;
use tokio::sync::mpsc;
use crate::agent::{Agent, AgentRunConfig};

/// SOTA Harness Patterns (2025-2026): 4. Scalable multi-agent: single-user CLI to 1000+ agent cloud deployments
/// This module provides a transparent scaling abstraction. An agent can be deployed
/// as a local thread for a CLI, or as a remote worker in a cloud deployment using the
/// same interface.
pub trait AgentDeployment: Send + Sync {
    fn spawn(&self, task: String) -> tokio::task::JoinHandle<Result<String, String>>;
}

/// Local deployment: runs the agent in a local tokio task. Ideal for single-user CLI.
pub struct LocalDeployment {
    pub agent: Arc<Agent>,
    pub config: AgentRunConfig,
}

impl AgentDeployment for LocalDeployment {
    fn spawn(&self, task: String) -> tokio::task::JoinHandle<Result<String, String>> {
        let agent = self.agent.clone();
        let config = self.config.clone();
        tokio::spawn(async move {
            let mut on_event = |_| {};
            agent.run(&config, &task, &mut on_event).await.map_err(|e| e.to_string())
        })
    }
}

/// Cloud deployment: sends the task to a remote worker queue via an RPC/Messaging interface.
/// Scales to 1000+ agent cloud deployments.
pub struct CloudDeployment {
    // In a real system, this would be a gRPC client, NATS publisher, or Redis queue.
    // For this demonstration, we use a simple channel.
    pub queue_tx: mpsc::Sender<String>,
}

impl AgentDeployment for CloudDeployment {
    fn spawn(&self, task: String) -> tokio::task::JoinHandle<Result<String, String>> {
        let tx = self.queue_tx.clone();
        tokio::spawn(async move {
            tx.send(task).await.map_err(|_| "Failed to enqueue task to cloud deployment".to_string())?;
            // Await result from a response queue (simulated here as immediate success)
            Ok("Task enqueued to cloud successfully".to_string())
        })
    }
}

pub struct ScalableMultiAgentManager {
    deployments: Vec<Arc<dyn AgentDeployment>>,
}

impl ScalableMultiAgentManager {
    pub fn new() -> Self {
        Self { deployments: Vec::new() }
    }

    pub fn add_deployment(&mut self, deployment: Arc<dyn AgentDeployment>) {
        self.deployments.push(deployment);
    }

    pub async fn dispatch_all(&self, task: &str) -> Vec<Result<String, String>> {
        let mut handles = Vec::new();
        for dep in &self.deployments {
            handles.push(dep.spawn(task.to_string()));
        }

        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(res) => results.push(res),
                Err(e) => results.push(Err(e.to_string())),
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage};
    use crate::llm::LlmClient;

    struct MockLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("Local execution complete"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id1".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_scalable_multi_agent_deployments() {
        let mut manager = ScalableMultiAgentManager::new();

        // 1. Local CLI Deployment
        let agent = Arc::new(Agent::new(Arc::new(MockLlmClient), vec![]));
        let local_dep = Arc::new(LocalDeployment {
            agent,
            config: AgentRunConfig::default(),
        });
        manager.add_deployment(local_dep);

        // 2. Cloud Deployment (1000+ scaling)
        let (tx, mut rx) = mpsc::channel(100);
        let cloud_dep = Arc::new(CloudDeployment { queue_tx: tx });
        manager.add_deployment(cloud_dep);

        let task = "Perform distributed computation";
        let results = manager.dispatch_all(task).await;

        assert_eq!(results.len(), 2);

        let mut has_local = false;
        let mut has_cloud = false;

        for res in results {
            if let Ok(val) = res {
                if val == "Local execution complete" {
                    has_local = true;
                } else if val == "Task enqueued to cloud successfully" {
                    has_cloud = true;
                }
            }
        }

        assert!(has_local, "Local execution should succeed");
        assert!(has_cloud, "Cloud enqueue should succeed");

        // Verify the cloud task was actually queued
        let queued_task = rx.recv().await.unwrap();
        assert_eq!(queued_task, task);
    }
}
