use ohc_builtin_agent_core::types::{ChatRequest, Message, ToolCall, ToolResult, Role, ToolError};
use crate::llm::LlmClient;
use std::sync::Arc;
use crate::tools::Tool;

/// SOTA Harness Patterns (2025-2026): 1. The Orchestration Loop
/// Mechanically, it is a `while` loop executing the TAO (Thought-Action-Observation) cycle:
/// Assemble prompt -> Call LLM API -> Parse output -> Execute tool calls -> Format results back -> Repeat.
/// Termination conditions are layered: model returns text with no tool calls, max turn limit exceeded,
/// token budget exhausted, guardrail tripwire fires, or safety refusal.
pub struct OrchestrationLoop {
    llm: Arc<dyn LlmClient>,
    max_turns: usize,
}

impl OrchestrationLoop {
    pub fn new(llm: Arc<dyn LlmClient>, max_turns: usize) -> Self {
        Self { llm, max_turns }
    }

    /// Executes the TAO cycle until a termination condition is met.
    pub async fn run(
        &self,
        mut messages: Vec<Message>,
        tools: Vec<Tool>,
    ) -> Result<String, String> {
        let mut turn_count = 0;

        while turn_count < self.max_turns {
            turn_count += 1;

            // Step 1: Assemble prompt (in this basic loop, it is just `messages`)
            let req = ChatRequest {
                model: "default-model".to_string(),
                system: "You are an AI assistant.".to_string(),
                messages: messages.clone(),
                tools: tools.iter().map(|t| ohc_builtin_agent_core::types::ToolDefinition {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    parameters: t.parameters.clone(),
                }).collect(),
                max_tokens: 1000,
                temperature: 0.0,
            };

            // Step 2: Call LLM API
            let resp = self.llm.chat(req).await.map_err(|e| e.to_string())?;

            // Layered Termination: Safety Refusal
            if resp.stop_reason == "content_filter" || resp.stop_reason == "safety" {
                return Err("Terminal condition reached: Safety refusal.".to_string());
            }

            messages.push(resp.message.clone());

            // Layered Termination: Model returns text with no tool calls
            if resp.message.tool_calls.is_empty() {
                return Ok(resp.message.content.clone());
            }

            // Step 3 & 4: Parse output & Execute tool calls
            let mut tool_results = Vec::new();
            for tc in &resp.message.tool_calls {
                let result = self.execute_single_tool(tc, &tools).await;
                tool_results.push(result);
            }

            // Step 5: Format results back
            messages.push(Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results,
                response_id: None,
                previous_response_id: resp.response_id.clone(),
            });
        }

        // Layered Termination: Max turn limit exceeded
        Err("Terminal condition reached: Max turn limit exceeded.".to_string())
    }

    async fn execute_single_tool(&self, tc: &ToolCall, tools: &[Tool]) -> ToolResult {
        if let Some(tool) = tools.iter().find(|t| t.name == tc.name) {
            match tool.execute.execute(tc.arguments.clone()).await {
                Ok(res) => ToolResult {
                    tool_call_id: tc.id.clone(),
                    content: res,
                    error: String::new(),
                },
                Err(e) => ToolResult {
                    tool_call_id: tc.id.clone(),
                    content: String::new(),
                    error: e.to_string(),
                },
            }
        } else {
            ToolResult {
                tool_call_id: tc.id.clone(),
                content: String::new(),
                error: format!("Unknown tool: {}", tc.name),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage};
    use serde_json::json;
    use tokio::sync::Mutex;

    struct MockLlm {
        responses: Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            if !resps.is_empty() {
                Ok(resps.remove(0))
            } else {
                Ok(ChatResponse {
                    message: Message::assistant("default"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id".to_string()),
                })
            }
        }
    }

    #[tokio::test]
    async fn test_orchestration_loop_termination_no_tools() {
        let llm = Arc::new(MockLlm {
            responses: Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("Final answer"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: None,
                }
            ]),
        });

        let orchestrator = OrchestrationLoop::new(llm, 5);
        let result = orchestrator.run(vec![], vec![]).await;
        assert_eq!(result.unwrap(), "Final answer");
    }

    #[tokio::test]
    async fn test_orchestration_loop_max_turns() {
        let llm = Arc::new(MockLlm {
            responses: Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ToolCall { id: "1".to_string(), name: "test".to_string(), arguments: json!({}) }],
                        tool_results: vec![],
                        response_id: None,
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: None,
                },
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ToolCall { id: "2".to_string(), name: "test".to_string(), arguments: json!({}) }],
                        tool_results: vec![],
                        response_id: None,
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: None,
                }
            ]),
        });

        let orchestrator = OrchestrationLoop::new(llm, 1);
        let result = orchestrator.run(vec![], vec![]).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Terminal condition reached: Max turn limit exceeded.");
    }
}
