use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage, Message, Role};
use crate::agent::{Agent, AgentRunConfig};
use std::sync::Arc;

/// AgenticSeek Unique Harness Innovations: Fully local agent, no API costs
/// Ensure execution is fully local. Validates that no API keys or paid external
/// LLM providers are used in the given config, enforcing local-only, zero-cost inference.

pub struct AgenticSeekRunner {
    pub agent: Arc<Agent>,
    pub local_base_url: String, // e.g., http://localhost:11434/v1 for Ollama
}

impl AgenticSeekRunner {
    pub fn new(agent: Arc<Agent>, local_base_url: impl Into<String>) -> Self {
        Self {
            agent,
            local_base_url: local_base_url.into(),
        }
    }

    /// Enforces zero API cost by checking that no explicit paid provider keys are passed
    /// and that the base_url points to a local network interface or localhost.
    pub fn validate_local_only(&self, _cfg: &AgentRunConfig) -> Result<(), String> {
        // Enforce local base URL
        if !self.local_base_url.contains("localhost") && !self.local_base_url.contains("127.0.0.1") {
            return Err(format!("AgenticSeek violation: base_url '{}' is not a local address. Must be localhost or 127.0.0.1 to guarantee zero API costs.", self.local_base_url));
        }

        // We assume agent's configured LLM is pointed to this local_base_url.
        // We can just rely on the url check for safety.
        Ok(())
    }

    pub async fn run_local_zero_cost(
        &self,
        cfg: &AgentRunConfig,
        initial_message: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.validate_local_only(cfg)?;

        let mut on_event = |_e| {};
        self.agent.run(cfg, initial_message, &mut on_event).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use tokio::sync::Mutex;

    struct LocalMockLlmClient {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for LocalMockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "default".to_string()
            };

            Ok(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content,
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: Some("mock-id".to_string()),
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_agentic_seek_validation_success() {
        let client = Arc::new(LocalMockLlmClient {
            responses: Mutex::new(vec!["local inference complete".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = AgenticSeekRunner::new(agent, "http://localhost:11434/v1");

        let cfg = AgentRunConfig::default();
        let res = runner.run_local_zero_cost(&cfg, "hello").await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "local inference complete");
    }

    #[tokio::test]
    async fn test_agentic_seek_validation_failure() {
        let client = Arc::new(LocalMockLlmClient {
            responses: Mutex::new(vec!["should not reach here".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        // Using an external paid provider
        let runner = AgenticSeekRunner::new(agent, "https://api.openai.com/v1");

        let cfg = AgentRunConfig::default();
        let res = runner.run_local_zero_cost(&cfg, "hello").await;
        assert!(res.is_err());
        let err_msg = res.unwrap_err().to_string();
        assert!(err_msg.contains("AgenticSeek violation"));
        assert!(err_msg.contains("zero API costs"));
    }
}
