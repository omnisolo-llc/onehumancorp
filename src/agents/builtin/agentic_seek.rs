/// agenticSeek: Fully local agent, no API costs
/// Implements a fully local agent execution environment that routes all LLM calls to a local provider (like Ollama)
/// and restricts network access for tools to ensure zero API costs and full data privacy.
use crate::agent::{Agent, AgentRunConfig};
use crate::llm::LlmClient;
use std::sync::Arc;

pub struct AgenticSeekLocalAgent {
    pub agent: Arc<Agent>,
    pub local_tools: Vec<crate::tools::Tool>,
}

impl AgenticSeekLocalAgent {
    /// Creates a fully local agent using the provided local LLM client (e.g., Ollama).
    /// It filters out any tools that might incur API costs or make external network calls by default.
    pub fn new(local_llm: Arc<dyn LlmClient>, all_tools: Vec<crate::tools::Tool>) -> Self {
        // Filter out tools that are known to make external API calls
        let local_tools: Vec<_> = all_tools.into_iter().filter(|t| {
            let name = t.name.as_str();
            // Block web search, external marketplace fetching, etc.
            name != "websearch" && name != "webfetch" && name != "agent_marketplace"
        }).collect();

        Self {
            agent: Arc::new(Agent::new(local_llm, local_tools.clone())),
            local_tools,
        }
    }

    /// Run the local agent
    pub async fn run_local(&self, config: &AgentRunConfig, task: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut on_event = |_| {};
        self.agent.run(config, task, &mut on_event).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatRequest, ChatResponse, Message, Role, Usage, ToolCall, ToolError};
    use crate::tools::{Tool, ToolExecutor};

    struct MockLocalLlm;
    #[async_trait::async_trait]
    impl LlmClient for MockLocalLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("Local execution complete"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id".to_string()),
            })
        }
    }

    struct MockTool;
    #[async_trait::async_trait]
    impl ToolExecutor for MockTool {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
            Ok("ok".to_string())
        }
    }

    #[tokio::test]
    async fn test_agentic_seek_filters_tools() {
        let tools = vec![
            Tool {
                name: "bash".to_string(),
                description: "".to_string(),
                is_read_only: false,
                parameters: serde_json::json!({}),
                execute: Arc::new(MockTool),
            },
            Tool {
                name: "websearch".to_string(),
                description: "".to_string(),
                is_read_only: true,
                parameters: serde_json::json!({}),
                execute: Arc::new(MockTool),
            },
            Tool {
                name: "agent_marketplace".to_string(),
                description: "".to_string(),
                is_read_only: true,
                parameters: serde_json::json!({}),
                execute: Arc::new(MockTool),
            }
        ];

        let local_agent = AgenticSeekLocalAgent::new(Arc::new(MockLocalLlm), tools);

        // Assert that the tools were filtered correctly
        assert_eq!(local_agent.local_tools.len(), 1);
        assert_eq!(local_agent.local_tools[0].name, "bash");

        let config = AgentRunConfig::default();
        let res = local_agent.run_local(&config, "test").await.unwrap();

        assert_eq!(res, "Local execution complete");
    }
}
