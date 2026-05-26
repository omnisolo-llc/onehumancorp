use std::sync::Arc;
use tokio::sync::mpsc;
use crate::agent::{Agent, AgentRunConfig, AgentEvent};

/// SOTA Harness Patterns (2025-2026): 4. Scalable multi-agent -> single-user CLI to 1000+ agent cloud deployments
/// This module provides a CloudDeployment harness that can fan out a single CLI prompt to
/// thousands of local or cloud agent instances concurrently.
pub struct CloudDeployment {
    pub agents: Vec<Arc<Agent>>,
}

impl CloudDeployment {
    pub fn new(agents: Vec<Arc<Agent>>) -> Self {
        Self { agents }
    }

    /// Deploys a task concurrently to all registered agents.
    pub async fn deploy(&self, task: &str, base_config: &AgentRunConfig) -> Vec<Result<String, String>> {
        let (tx, mut rx) = mpsc::channel(self.agents.len().max(1));

        for (i, agent) in self.agents.iter().enumerate() {
            let tx = tx.clone();
            let agent = agent.clone();
            let task = task.to_string();
            let mut cfg = base_config.clone();
            cfg.agent_id = format!("cloud-agent-{}", i);

            tokio::spawn(async move {
                let mut on_event = |_e: AgentEvent| {
                    // We could collect events here to monitor cluster progress
                };
                let res = agent.run(&cfg, &task, &mut on_event).await;
                let _ = tx.send(res.map_err(|e| e.to_string())).await;
            });
        }

        drop(tx);

        let mut results = Vec::new();
        while let Some(res) = rx.recv().await {
            results.push(res);
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage};
    use crate::llm::LlmClient;

    struct CloudMockLlmClient {
        id: usize,
    }

    #[async_trait::async_trait]
    impl LlmClient for CloudMockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(format!("Agent {} reporting for duty", self.id)),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some(format!("mock-id-{}", self.id)),
            })
        }
    }

    #[tokio::test]
    async fn test_cloud_deployment_scalable_fanout() {
        let mut agents = Vec::new();
        // Test with 50 agents to simulate scalable fanout
        for i in 0..50 {
            let client = Arc::new(CloudMockLlmClient { id: i });
            let agent = Arc::new(Agent::new(client, vec![]));
            agents.push(agent);
        }

        let deployment = CloudDeployment::new(agents);
        let config = AgentRunConfig::default();

        let results = deployment.deploy("System update protocol", &config).await;

        assert_eq!(results.len(), 50);
        let successes = results.into_iter().filter(|r| r.is_ok()).count();
        assert_eq!(successes, 50, "All agents should complete successfully");
    }
}
