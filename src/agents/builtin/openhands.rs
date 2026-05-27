use std::sync::Arc;
use crate::agent::{Agent, AgentRunConfig, AgentEvent};

/// SOTA Harness Pattern: OpenHands CLI / SDK Mechanic
/// OpenHands/OpenDevin: Python SDK + CLI, MIT licensed
///
/// This provides a robust CLI/SDK facade to orchestrate agent operations
/// modeled exactly after OpenHands (OpenDevin). It handles the agent initialization,
/// encapsulates the config, and exposes simple synchronous or streaming CLI interfaces.
pub struct OpenHandsSdk {
    pub agent: Arc<Agent>,
}

impl OpenHandsSdk {
    pub fn new(agent: Arc<Agent>) -> Self {
        Self { agent }
    }

    /// Exposes a CLI-like execution method mirroring the OpenHands implementation
    pub async fn execute_cli_task(&self, task: &str) -> Result<String, String> {
        let cfg = AgentRunConfig::default();
        let mut on_event = |_| {};

        match self.agent.run(&cfg, task, &mut on_event).await {
            Ok(output) => Ok(output),
            Err(e) => Err(format!("OpenHands Execution Error: {}", e)),
        }
    }

    /// Exposes an interactive stream mirroring the OpenHands SDK
    pub fn execute_stream(&self, task: &str) -> tokio::sync::mpsc::UnboundedReceiver<AgentEvent> {
        let cfg = AgentRunConfig::default();
        self.agent.clone().query(cfg, task.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage};

    struct MockLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("OpenHands task completed successfully"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_openhands_sdk_execution() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let sdk = OpenHandsSdk::new(agent);

        let result = sdk.execute_cli_task("Build me a calculator").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "OpenHands task completed successfully");
    }

    #[tokio::test]
    async fn test_openhands_sdk_stream() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let sdk = OpenHandsSdk::new(agent);

        let mut rx = sdk.execute_stream("Build me a calculator");

        // Ensure the stream is alive and yields events
        let mut events = vec![];
        while let Some(event) = rx.recv().await {
            events.push(event);
        }

        assert!(!events.is_empty());
        let has_complete = events.iter().any(|e| matches!(e, AgentEvent::TaskComplete { .. }));
        assert!(has_complete);
    }
}
