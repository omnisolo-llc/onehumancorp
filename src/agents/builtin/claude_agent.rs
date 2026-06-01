use crate::agent::{AgentRunConfig};
use crate::types::{ChatRequest, Message, Role, ToolResult};
use crate::tools::Tool;
use crate::llm::LlmClient;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Anthropic Claude Agent SDK & Claude Code Implementation.
/// Implements the harness via a single `query()` function that returns an async iterator streaming messages.
/// Uses a "dumb loop" Gather-Act-Verify cycle:
/// gather context (search files, read code) -> take action (edit files, run commands) -> verify results (run tests, check output).

pub struct ClaudeAgent {
    pub llm: Arc<dyn LlmClient>,
    pub tools: Vec<Tool>,
    pub config: AgentRunConfig,
}

#[derive(Debug)]
pub enum ClaudeEvent {
    Thought(String),
    ToolCall { name: String, args: String },
    ToolResult { name: String, result: String },
    FinalResponse(String),
    Error(String),
}

impl ClaudeAgent {
    pub fn new(llm: Arc<dyn LlmClient>, tools: Vec<Tool>, config: AgentRunConfig) -> Self {
        Self { llm, tools, config }
    }

    /// The `query()` function returns a receiver for an async stream of events
    pub fn query(&self, prompt: &str) -> mpsc::Receiver<ClaudeEvent> {
        let (tx, rx) = mpsc::channel(100);
        let mut messages = vec![Message::user(prompt)];
        let llm = self.llm.clone();
        let tools = self.tools.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut iteration = 0;
            let max_iterations = config.max_iterations;

            loop {
                if iteration >= max_iterations {
                    if tx.send(ClaudeEvent::Error(format!("Max iterations ({}) reached", max_iterations))).await.is_err() { break; }
                    break;
                }
                iteration += 1;

                let tools_def: Vec<_> = tools.iter().map(|t| crate::types::ToolDefinition {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    parameters: t.parameters.clone(),
                }).collect();

                let req = ChatRequest {
                    model: config.model.clone(),
                    system: config.server_system_message.clone() + "\nOperate in a dumb Gather-Act-Verify loop. Gather context, take action, then verify.",
                    messages: messages.clone(),
                    tools: tools_def,
                    max_tokens: config.max_tokens,
                    temperature: config.temperature,
                };

                let resp = match llm.chat(req).await {
                    Ok(r) => r,
                    Err(e) => {
                        if tx.send(ClaudeEvent::Error(format!("LLM Error: {}", e))).await.is_err() { break; }
                        break;
                    }
                };

                messages.push(resp.message.clone());

                if !resp.message.content.is_empty() {
                    if tx.send(ClaudeEvent::Thought(resp.message.content.clone())).await.is_err() { break; }
                }

                if resp.message.tool_calls.is_empty() {
                    if tx.send(ClaudeEvent::FinalResponse(resp.message.content)).await.is_err() { break; }
                    break;
                }

                // Gather-Act-Verify loop mechanics: Execute tools
                let mut tool_results = vec![];
                for tc in resp.message.tool_calls.iter() {
                    if tx.send(ClaudeEvent::ToolCall {
                        name: tc.name.clone(),
                        args: tc.arguments.to_string(),
                    }).await.is_err() { break; }

                    if let Some(tool) = tools.iter().find(|t| t.name == tc.name) {
                        match crate::tool_executor_engine::ToolExecutionEngine::execute_tool_with_langgraph_mechanics(
                            tool,
                            tc,
                            config.max_retries,
                        ).await {
                            Ok(res) => {
                                if tx.send(ClaudeEvent::ToolResult {
                                    name: tc.name.clone(),
                                    result: res.clone(),
                                }).await.is_err() { break; }
                                tool_results.push(ToolResult {
                                    tool_call_id: tc.id.clone(),
                                    content: res,
                                    error: String::new(),
                                });
                            }
                            Err(e) => {
                                if tx.send(ClaudeEvent::ToolResult {
                                    name: tc.name.clone(),
                                    result: format!("Error: {}", e),
                                }).await.is_err() { break; }
                                tool_results.push(ToolResult {
                                    tool_call_id: tc.id.clone(),
                                    content: String::new(),
                                    error: format!("{}", e),
                                });
                            }
                        }
                    } else {
                        let err = format!("Tool {} not found", tc.name);
                        if tx.send(ClaudeEvent::ToolResult {
                            name: tc.name.clone(),
                            result: err.clone(),
                        }).await.is_err() { break; }
                        tool_results.push(ToolResult {
                            tool_call_id: tc.id.clone(),
                            content: String::new(),
                            error: err,
                        });
                    }
                }

                messages.push(Message {
                    role: Role::Tool,
                    content: String::new(),
                    tool_calls: vec![],
                    tool_results,
                    response_id: None,
                    previous_response_id: resp.message.response_id.clone(),
                });
            }
        });

        rx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatResponse, Usage};
    use crate::tools::ToolExecutor;
    use tokio::sync::Mutex;
    use crate::types::ToolCall;
    use serde_json::json;

    struct MockClaudeLlmClient {
        responses: Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockClaudeLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            if !resps.is_empty() {
                Ok(resps.remove(0))
            } else {
                Ok(ChatResponse {
                    message: Message::assistant("default final"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            }
        }
    }

    struct DummyGatherTool;
    #[async_trait::async_trait]
    impl ToolExecutor for DummyGatherTool {
        async fn execute(&self, args: serde_json::Value) -> Result<String, crate::types::ToolError> {
            Ok(format!("Gathered: {}", args["target"].as_str().unwrap_or("nothing")))
        }
    }

    struct DummyActTool;
    #[async_trait::async_trait]
    impl ToolExecutor for DummyActTool {
        async fn execute(&self, args: serde_json::Value) -> Result<String, crate::types::ToolError> {
            Ok(format!("Acted on: {}", args["action"].as_str().unwrap_or("nothing")))
        }
    }

    #[tokio::test]
    async fn test_claude_gather_act_verify_cycle() {
        let client = Arc::new(MockClaudeLlmClient {
            responses: Mutex::new(vec![
                // Iteration 1: Gather
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "Let me gather information".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "gather".to_string(),
                            arguments: json!({"target": "logs"}),
                        }],
                        tool_results: vec![],
                        response_id: None,
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("id1".to_string()),
                },
                // Iteration 2: Act based on gathering
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "Found logs. Now I will act.".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_2".to_string(),
                            name: "act".to_string(),
                            arguments: json!({"action": "restart"}),
                        }],
                        tool_results: vec![],
                        response_id: None,
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("id2".to_string()),
                },
                // Iteration 3: Final verification
                ChatResponse {
                    message: Message::assistant("Process restarted and verified."),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id3".to_string()),
                }
            ]),
        });

        let tools = vec![
            Tool {
                name: "gather".to_string(),
                description: "".to_string(),
                is_read_only: true,
                parameters: json!({}),
                execute: Arc::new(DummyGatherTool),
            },
            Tool {
                name: "act".to_string(),
                description: "".to_string(),
                is_read_only: false,
                parameters: json!({}),
                execute: Arc::new(DummyActTool),
            }
        ];

        let config = AgentRunConfig::default();
        let agent = ClaudeAgent::new(client, tools, config);

        let mut rx = agent.query("Fix the server");
        let mut events = vec![];

        while let Some(evt) = rx.recv().await {
            events.push(evt);
        }

        println!("Events: {:#?}", events);
        // Validate the gathered events map to a Dumb loop: Gather -> Act -> Verify
        // Expected sequence: Thought, ToolCall(gather), ToolResult, Thought, ToolCall(act), ToolResult, FinalResponse



        // 1. Thought
        if let ClaudeEvent::Thought(ref msg) = events[0] {
            assert_eq!(msg, "Let me gather information");
        } else { panic!("Expected Thought"); }

        // 2. ToolCall
        if let ClaudeEvent::ToolCall { ref name, .. } = events[1] {
            assert_eq!(name, "gather");
        } else { panic!("Expected ToolCall"); }

        // 3. ToolResult
        if let ClaudeEvent::ToolResult { ref result, .. } = events[2] {
            assert_eq!(result, "Gathered: logs");
        } else { panic!("Expected ToolResult"); }

        // 4. Thought
        if let ClaudeEvent::Thought(ref msg) = events[3] {
            assert_eq!(msg, "Found logs. Now I will act.");
        } else { panic!("Expected Thought"); }

        // 5. ToolCall
        if let ClaudeEvent::ToolCall { ref name, .. } = events[4] {
            assert_eq!(name, "act");
        } else { panic!("Expected ToolCall"); }

        // 6. ToolResult
        if let ClaudeEvent::ToolResult { ref result, .. } = events[5] {
            assert_eq!(result, "Acted on: restart");
        } else { panic!("Expected ToolResult"); }

        // 7. FinalResponse
        if let ClaudeEvent::Thought(ref msg) = events[6] {
            assert_eq!(msg, "Process restarted and verified.");
        } else { panic!("Expected Thought"); }
        if let ClaudeEvent::FinalResponse(ref msg) = events[7] {
            assert_eq!(msg, "Process restarted and verified.");
        } else { panic!("Expected FinalResponse"); }
    }
}
