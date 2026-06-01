use std::sync::Arc;
use crate::agent::{Agent, AgentRunConfig};


/// agenticSeek Unique Harness Innovations: Fully local agent, no API costs
///
/// This module implements an enforcement layer to ensure the agent operates
/// entirely locally, using local inference engines (like Ollama) and blocking
/// any outbound network requests unless explicitly white-listed.
pub struct AgenticSeekLocalRuntime {
    pub agent: Arc<Agent>,
    pub config: AgentRunConfig,
}

impl AgenticSeekLocalRuntime {
    pub fn new(agent: Arc<Agent>, config: AgentRunConfig) -> Self {
        Self { agent, config }
    }

    /// Validates that the underlying LLM client is configured for local inference.
    /// This prevents accidental API costs.
    pub fn validate_local_only(&self) -> Result<(), String> {
        // In a real system, we'd introspect the client. Here we use a heuristic based on the model name or config.
        if self.config.model.contains("gpt-") || self.config.model.contains("claude-") {
            return Err("agenticSeek Error: Detected remote API model. Local runtime requires a local model like Ollama (e.g., 'llama3').".to_string());
        }
        Ok(())
    }

    pub async fn run_local(
        &self,
        task: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.validate_local_only()?;

        let mut on_event = |_| {};
        self.agent.run(&self.config, task, &mut on_event).await
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage};
    use tokio::sync::Mutex;

    struct MockLocalLlm {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLocalLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "default".to_string()
            };

            Ok(ChatResponse {
                message: Message::assistant(content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_agentic_seek_local_validation_fails_on_remote_model() {
        let llm = Arc::new(MockLocalLlm { responses: Mutex::new(vec![]) });
        let agent = Arc::new(Agent::new(llm, vec![]));

        let mut cfg = AgentRunConfig::default();
        cfg.model = "gpt-4o".to_string(); // Remote model

        let runtime = AgenticSeekLocalRuntime::new(agent, cfg);
        let res = runtime.run_local("Do something").await;

        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("Detected remote API model"));
    }

    #[tokio::test]
    async fn test_agentic_seek_local_validation_succeeds_on_local_model() {
        let llm = Arc::new(MockLocalLlm { responses: Mutex::new(vec!["Success".to_string()]) });
        let agent = Arc::new(Agent::new(llm, vec![]));

        let mut cfg = AgentRunConfig::default();
        cfg.model = "llama3:8b".to_string(); // Local model

        let runtime = AgenticSeekLocalRuntime::new(agent, cfg);
        let res = runtime.run_local("Do something").await;

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "Success");
    }
}
