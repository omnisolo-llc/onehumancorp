use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use std::sync::Arc;

/// agenticSeek Archetype: Fully local agent, no API costs
/// This module implements an execution loop tailored for local models (e.g., Ollama, Llama.cpp)
/// that emphasizes aggressive prompt truncation, prompt caching, and cost-free execution.

pub struct AgenticSeekRunner {
    pub agent: Arc<Agent>,
}

impl AgenticSeekRunner {
    pub fn new(agent: Arc<Agent>) -> Self {
        Self { agent }
    }

    pub async fn run_local<F>(
        &self,
        cfg: &AgentRunConfig,
        initial_message: &str,
        on_event: &mut F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {
        // 1. Force context optimization for local models (strict limits)
        let mut local_cfg = cfg.clone();
        local_cfg.enable_acon_context_strategy = true;
        local_cfg.enable_observation_masking = true;
        // Aggressively mask tools for local models
        local_cfg.observation_masking_threshold = 1;
        local_cfg.observation_masking_size_limit = 50;

        on_event(AgentEvent::RunStarted { iteration: 0 });

        tracing::info!("Starting agenticSeek Fully Local loop...");
        self.agent.run(&local_cfg, initial_message, on_event).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage};

    struct MockLocalLlm;
    #[async_trait::async_trait]
    impl LlmClient for MockLocalLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("agenticSeek local response"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("local-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_agentic_seek_runner() {
        let agent = Arc::new(Agent::new(Arc::new(MockLocalLlm), vec![]));
        let runner = AgenticSeekRunner::new(agent);
        let cfg = AgentRunConfig::default();

        let mut events = vec![];
        let mut on_event = |e| events.push(e);

        let result = runner.run_local(&cfg, "hello local", &mut on_event).await.unwrap();
        assert_eq!(result, "agenticSeek local response");
        assert!(!events.is_empty());
    }
}
