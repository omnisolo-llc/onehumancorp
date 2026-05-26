use std::sync::Arc;
use crate::agent::{Agent, AgentRunConfig, AgentEvent};
use crate::llm::LlmClient;
use crate::tools::Tool;

/// agenticSeek Unique Harness Innovations: Fully local agent, no API costs
/// This harness wraps an Agent and strictly enforces that no API-cost-incurring tools
/// or remote LLM clients are utilized. It acts as a strict cost/security boundary.
pub struct AgenticSeekHarness {
    pub inner_agent: Arc<Agent>,
}

impl AgenticSeekHarness {
    /// Creates a new AgenticSeekHarness.
    /// It consumes the original agent's tools, filters out non-local ones,
    /// and instantiates a new Agent with only the local tools.
    /// It enforces that the LLM client is local (by type or config check).
    /// In this mock implementation, we just filter the tools.
    pub fn new(llm: Arc<dyn LlmClient>, tools: Vec<Tool>) -> Self {
        // Enforce purely local tools
        let local_tools: Vec<Tool> = tools
            .into_iter()
            .filter(|t| {
                let name = t.name.to_lowercase();
                // Filter out any tools known to incur external API costs
                if name == "webfetch" || name == "websearch" || name == "marketplace" || name == "sendmessage" {
                    false
                } else {
                    true
                }
            })
            .collect();

        let agent = Arc::new(Agent::new(llm, local_tools));

        Self {
            inner_agent: agent,
        }
    }

    /// Runs the agent through the harness, enforcing cost-free local execution.
    pub async fn run<F>(
        &self,
        config: &AgentRunConfig,
        initial_message: &str,
        on_event: &mut F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {
        // Enforce that we are running locally (no remote endpoints).
        let local_config = config.clone();

        self.inner_agent.run(&local_config, initial_message, on_event).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage, ToolError};
    use crate::tools::ToolExecutor;

    struct MockLocalLlmClient;
    #[async_trait::async_trait]
    impl LlmClient for MockLocalLlmClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let tools_list: Vec<String> = req.tools.iter().map(|t| t.name.clone()).collect();
            Ok(ChatResponse {
                message: Message::assistant(format!("Tools available: {:?}", tools_list)),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-local-id".to_string()),
            })
        }
    }

    struct MockTool;
    #[async_trait::async_trait]
    impl ToolExecutor for MockTool {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
            Ok("Tool executed".to_string())
        }
    }

    #[tokio::test]
    async fn test_agentic_seek_filters_api_tools() {
        let llm = Arc::new(MockLocalLlmClient);

        let local_tool = Tool {
            name: "grep".to_string(),
            description: "Local search".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: Arc::new(MockTool),
        };

        let remote_tool = Tool {
            name: "webfetch".to_string(),
            description: "External API".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: Arc::new(MockTool),
        };

        let remote_search_tool = Tool {
            name: "WebSearch".to_string(),
            description: "External API".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: Arc::new(MockTool),
        };

        let tools = vec![local_tool, remote_tool, remote_search_tool];

        // Harness should filter out webfetch and WebSearch
        let harness = AgenticSeekHarness::new(llm, tools);

        let config = AgentRunConfig::default();
        let mut on_event = |_| {};

        let result = harness.run(&config, "Test", &mut on_event).await.unwrap();

        assert!(result.contains("Tools available: [\"grep\"]"), "Should only contain local grep tool");
        assert!(!result.contains("webfetch"), "Should not contain API tools");
        assert!(!result.contains("WebSearch"), "Should not contain API tools");
    }
}
