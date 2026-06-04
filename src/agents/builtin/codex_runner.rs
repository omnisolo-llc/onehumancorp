use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use ohc_builtin_agent_core::types::Message;
use std::sync::Arc;
use tokio::sync::mpsc;


/// OpenAI Codex & Agents SDK Archetype:
/// Uses a `Runner` class with async, sync, and streamed modes.
pub struct Runner {
    pub agent: Arc<Agent>,
}

impl Runner {
    pub fn new(agent: Arc<Agent>) -> Self {
        Self { agent }
    }

    /// Asynchronous execution mode
    pub async fn run_async(&self, cfg: &AgentRunConfig, initial_message: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut on_event = |_e| {};
        self.agent.run(cfg, initial_message, &mut on_event).await
    }

    /// Synchronous execution mode (blocks the current thread)
    pub fn run_sync_blocking(&self, cfg: &AgentRunConfig, initial_message: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let cfg = cfg.clone();
        let initial_message = initial_message.to_string();
        let agent = self.agent.clone();

        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(async move {
            let mut on_event = |_e| {};
            agent.run(&cfg, &initial_message, &mut on_event).await
        })
    }

    /// Streamed execution mode (returns a receiver for AgentEvents)
    pub fn run_streamed(&self, cfg: &AgentRunConfig, initial_message: &str) -> mpsc::UnboundedReceiver<AgentEvent> {
        self.agent.clone().query(cfg.clone(), initial_message.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Usage};
    use std::sync::Arc;

    struct MockLlmClient {
        responses: tokio::sync::Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            if !resps.is_empty() {
                Ok(resps.remove(0))
            } else {
                Ok(ChatResponse {
                    message: Message::assistant("default output"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            }
        }
    }

    #[tokio::test]
    async fn test_runner_async() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message::assistant("async success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            }]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Runner::new(agent);
        let cfg = AgentRunConfig::default();
        let result = runner.run_async(&cfg, "test").await.unwrap();
        assert_eq!(result, "async success");
    }

    #[test]
    fn test_runner_sync() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message::assistant("sync success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            }]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Runner::new(agent);
        let cfg = AgentRunConfig::default();
        let result = runner.run_sync_blocking(&cfg, "test").unwrap();
        assert_eq!(result, "sync success");
    }

    #[tokio::test]
    async fn test_runner_streamed() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message::assistant("stream success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            }]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Runner::new(agent);
        let cfg = AgentRunConfig::default();
        let mut rx = runner.run_streamed(&cfg, "test");

        let mut events = vec![];
        while let Some(event) = rx.recv().await {
            events.push(event);
        }

        let has_complete = events.iter().any(|e| matches!(e, AgentEvent::TaskComplete { .. }));
        assert!(has_complete);
    }
}
