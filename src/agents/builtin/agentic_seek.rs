use crate::agent::{Agent, AgentRunConfig};
use std::sync::Arc;
use tokio::sync::RwLock;

/// agenticSeek: Fully local agent, no API costs.
/// This module implements the mechanic of a strictly local execution mode,
/// preventing any cloud API LLM usage or external web requests.

#[derive(Debug, Clone, PartialEq)]
pub enum ProviderType {
    Local,
    Cloud,
}

pub struct NoApiCostGuard {
    pub provider_type: ProviderType,
}

impl NoApiCostGuard {
    pub fn new(provider_type: ProviderType) -> Self {
        Self { provider_type }
    }

    pub fn enforce_local_only(&self) -> Result<(), String> {
        if self.provider_type == ProviderType::Cloud {
            return Err("AgenticSeek Constraint Violation: Cloud LLM provider detected. Fully local execution required.".to_string());
        }
        Ok(())
    }
}

pub struct FullyLocalAgent {
    pub agent: Arc<Agent>,
    pub guard: NoApiCostGuard,
}

impl FullyLocalAgent {
    pub fn new(agent: Arc<Agent>, provider_type: ProviderType) -> Self {
        Self {
            agent,
            guard: NoApiCostGuard::new(provider_type),
        }
    }

    pub async fn execute_local(&self, message: &str, config: &AgentRunConfig) -> Result<String, String> {
        self.guard.enforce_local_only()?;

        // Enforce all execution through local agent
        let mut on_event = |_e| {};
        self.agent.run(config, message, &mut on_event).await.map_err(|e| e.to_string())
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
                message: Message::assistant("success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_agentic_seek_local_provider() {
        let agent = Arc::new(Agent::new(Arc::new(MockLlmClient), vec![]));
        let local_agent = FullyLocalAgent::new(agent, ProviderType::Local);
        let config = AgentRunConfig::default();

        let result = local_agent.execute_local("test", &config).await;
        assert!(result.is_ok(), "Local provider should be allowed");
    }

    #[tokio::test]
    async fn test_agentic_seek_cloud_provider_blocked() {
        let agent = Arc::new(Agent::new(Arc::new(MockLlmClient), vec![]));
        let local_agent = FullyLocalAgent::new(agent, ProviderType::Cloud);
        let config = AgentRunConfig::default();

        let result = local_agent.execute_local("test", &config).await;
        assert!(result.is_err(), "Cloud provider should be blocked");
        assert!(result.unwrap_err().contains("AgenticSeek Constraint Violation: Cloud LLM provider detected"));
    }
}
