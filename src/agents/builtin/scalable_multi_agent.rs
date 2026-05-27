use crate::agent::{Agent, AgentRunConfig};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// SOTA Harness Patterns (2025-2026): 4. Scalable multi-agent -> single-user CLI to 1000+ agent cloud deployments
/// Provides an orchestration harness to deploy and manage a massive number of agents concurrently.
pub struct ScalableMultiAgentManager {
    base_agent: Arc<Agent>,
    base_config: AgentRunConfig,
}

impl ScalableMultiAgentManager {
    pub fn new(base_agent: Arc<Agent>, base_config: AgentRunConfig) -> Self {
        Self {
            base_agent,
            base_config,
        }
    }

    /// Spawns `num_agents` concurrently, distributing the workload.
    pub async fn execute_mass_deployment(
        &self,
        task_prefix: &str,
        num_agents: usize,
    ) -> Result<Vec<String>, String> {
        let (tx, mut rx) = mpsc::channel(num_agents);
        let mut handles = Vec::new();

        for i in 0..num_agents {
            let agent = self.base_agent.clone();
            let mut config = self.base_config.clone();
            config.thread_id = Some(format!("scalable-agent-{}", i));

            let task = format!("{} (Agent {})", task_prefix, i);
            let tx_clone = tx.clone();

            let handle: JoinHandle<()> = tokio::spawn(async move {
                let mut on_event = |_| {};
                let result = agent.run(&config, &task, &mut on_event).await;
                let _ = tx_clone.send((i, result)).await;
            });
            handles.push(handle);
        }

        drop(tx); // Close the original sender

        let mut results = vec![String::new(); num_agents];
        let mut errors = Vec::new();

        while let Some((index, res)) = rx.recv().await {
            match res {
                Ok(output) => {
                    results[index] = output;
                }
                Err(e) => {
                    errors.push(format!("Agent {} failed: {}", index, e));
                }
            }
        }

        for handle in handles {
            let _ = handle.await;
        }

        if !errors.is_empty() {
            return Err(format!("Mass deployment had errors: {}", errors.join("; ")));
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage};

    struct MockScalableLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for MockScalableLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("Scalable agent processed task"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_scalable_multi_agent_deployment() {
        let client = Arc::new(MockScalableLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let config = AgentRunConfig::default();

        let manager = ScalableMultiAgentManager::new(agent, config);
        let results = manager.execute_mass_deployment("Process chunk", 10).await.unwrap();

        assert_eq!(results.len(), 10);
        for res in results {
            assert_eq!(res, "Scalable agent processed task");
        }
    }
}
