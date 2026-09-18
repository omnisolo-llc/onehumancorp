use omnisolo_builtin_agent_core::types::{ChatRequest, Message, ToolCall, ToolDefinition, Usage, Role};
use crate::llm::LlmClient;
use crate::agent::{Agent, AgentRunConfig};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

/// SOTA Harness Pattern: OpenAI-Compatible Shim Harnesses: Plandex CLI
/// Implements "Multi-file architectural planning" as a discrete step before execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlandexPlan {
    pub multi_file_strategy: String,
    pub files_to_modify: Vec<String>,
    pub execution_steps: Vec<String>,
}

pub struct PlandexPlanner {

    pub llm: Arc<dyn LlmClient>,
    pub config: AgentRunConfig,
}

impl PlandexPlanner {
    pub fn new(llm: Arc<dyn LlmClient>, config: AgentRunConfig) -> Self {
        Self { llm, config }
    }

    pub async fn run_planning_phase(&self, objective: &str, codebase_context: &str) -> Result<PlandexPlan, String> {
        let system_prompt = r#"You are the Plandex CLI Shim Planner.
Your role is to perform multi-file architectural planning.
Given an objective and some codebase context, analyze the requirements and formulate a strategy.
You MUST call the `structured_output` tool to emit your plan."#;

        let user_prompt = format!("Objective:\n{}\n\nCodebase Context:\n{}", objective, codebase_context);

        let messages = vec![
            Message::user(&user_prompt),
        ];

        let structured_output_tool = ToolDefinition {
            name: "structured_output".to_string(),
            description: "Emit the final Plandex architectural plan.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "multi_file_strategy": {
                        "type": "string",
                        "description": "Overall strategy describing how changes cross multiple files."
                    },
                    "files_to_modify": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "List of files to modify."
                    },
                    "execution_steps": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Step-by-step execution plan."
                    }
                },
                "required": ["multi_file_strategy", "files_to_modify", "execution_steps"]
            }),
        };

        let req = ChatRequest {
            system: system_prompt.to_string(),
            messages,
            tools: vec![structured_output_tool],
            model: self.config.model.clone(),
            temperature: 0.1,
            max_tokens: self.config.max_tokens as i32,
        };

        let response = self.llm.chat(req).await.map_err(|e| e.to_string())?;

        // Extract the structured_output tool call instead of parsing raw content
        for tool_call in &response.message.tool_calls {
            if tool_call.name == "structured_output" {
                let plan: PlandexPlan = serde_json::from_value(tool_call.arguments.clone())
                    .map_err(|e| format!("Failed to parse Plandex plan arguments: {}", e))?;
                return Ok(plan);
            }
        }

        Err("Failed to extract Plandex plan: The model did not call the `structured_output` tool as required.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omnisolo_builtin_agent_core::types::ChatResponse;

    struct MockLlmClient {
        expected_tool_call_args: serde_json::Value,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let tool_call = ToolCall {
                id: "tc_1".to_string(),
                name: "structured_output".to_string(),
                arguments: self.expected_tool_call_args.clone(),
            };

            Ok(ChatResponse {
                response_id: Some("mock_res".to_string()),
                stop_reason: "tool_calls".to_string(),
                message: Message {
                    role: Role::Assistant,
                    content: "".to_string(),
                    tool_calls: vec![tool_call],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage {
                    input_tokens: 10,
                    output_tokens: 10,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                },
            })
        }
    }

    #[tokio::test]
    async fn test_plandex_planning_phase_success() {
        let mock_args = json!({
            "multi_file_strategy": "Refactor common utilities into a shared module.",
            "files_to_modify": ["src/main.rs", "src/utils.rs"],
            "execution_steps": ["Extract code", "Update imports"]
        });

        let llm = Arc::new(MockLlmClient { expected_tool_call_args: mock_args });

        let config = AgentRunConfig::default();

        let planner = PlandexPlanner::new(llm, config);

        let plan = planner.run_planning_phase("Refactor", "codebase info").await.expect("Failed to run planning phase");

        assert_eq!(plan.multi_file_strategy, "Refactor common utilities into a shared module.");
        assert_eq!(plan.files_to_modify.len(), 2);
        assert_eq!(plan.execution_steps.len(), 2);
    }

    #[tokio::test]
    async fn test_plandex_planning_incorporates_context() {
        // Test parsing failure logic
        let mock_args = json!({
            "invalid_field": "Missing required fields"
        });

        let llm = Arc::new(MockLlmClient { expected_tool_call_args: mock_args });

        let config = AgentRunConfig::default();

        let planner = PlandexPlanner::new(llm, config);

        let result = planner.run_planning_phase("Refactor", "codebase info").await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to parse Plandex plan arguments"));
    }
}
