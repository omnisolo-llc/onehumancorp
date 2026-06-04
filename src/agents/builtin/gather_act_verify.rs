use std::sync::Arc;
use tokio::sync::mpsc;
use ohc_builtin_agent_core::types::{ChatRequest, Message, Role};
use crate::llm::LlmClient;
use crate::tools::Tool;
use crate::agent::AgentEvent;

/// Configuration for the Gather-Act-Verify Harness
#[derive(Clone)]
pub struct GatherActVerifyConfig {
    pub max_iterations: usize,
    pub system_message: String,
    pub model: String,
}

impl Default for GatherActVerifyConfig {
    fn default() -> Self {
        Self {
            max_iterations: 15,
            system_message: "You are an agent executing the Gather-Act-Verify cycle.".to_string(),
            model: "claude-3-5-sonnet".to_string(),
        }
    }
}

/// Anthropic Claude Agent SDK & Claude Code Archetype:
/// Implements the harness via a single `query()` function that returns an async iterator streaming messages.
/// Uses a "dumb loop" Gather-Act-Verify cycle:
/// 1. gather context (search files, read code)
/// 2. take action (edit files, run commands)
/// 3. verify results (run tests, check output).
pub struct GatherActVerifyHarness {
    pub llm: Arc<dyn LlmClient>,
    pub gather_tools: Vec<Tool>,
    pub act_tools: Vec<Tool>,
    pub verify_tools: Vec<Tool>,
}

impl GatherActVerifyHarness {
    pub fn new(
        llm: Arc<dyn LlmClient>,
        gather_tools: Vec<Tool>,
        act_tools: Vec<Tool>,
        verify_tools: Vec<Tool>,
    ) -> Self {
        Self {
            llm,
            gather_tools,
            act_tools,
            verify_tools,
        }
    }

    /// The single query function returning an async channel of AgentEvents.
    pub fn query(
        &self,
        config: GatherActVerifyConfig,
        task: String,
    ) -> mpsc::UnboundedReceiver<AgentEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        let llm = self.llm.clone();

        // We clone tools since we need to move them into the async task.
        let gather_tools = self.gather_tools.clone();
        let act_tools = self.act_tools.clone();
        let verify_tools = self.verify_tools.clone();

        tokio::spawn(async move {
            let mut messages = vec![
                Message::system(config.system_message.clone()),
                Message::user(format!("Task: {}", task)),
            ];

            let phases = ["Gather", "Act", "Verify"];
            let tools_by_phase = vec![gather_tools, act_tools, verify_tools];

            for iteration in 0..config.max_iterations {
                let phase_idx = iteration % 3;
                let current_phase = phases[phase_idx];
                let current_tools = &tools_by_phase[phase_idx];

                let _ = tx.send(AgentEvent::TextChunk {
                    content: format!("Starting Phase: {}", current_phase),
                });

                let phase_instruction = match current_phase {
                    "Gather" => "Phase: GATHER. Use the available tools to gather context. Do NOT make changes yet. If you have enough context, say 'DONE GATHERING'.",
                    "Act" => "Phase: ACT. Use the available tools to perform the task based on the context. If no action is needed, say 'DONE ACTING'.",
                    "Verify" => "Phase: VERIFY. Use the available tools to verify your actions. If verification is successful, say 'TASK COMPLETE' with your final answer. Otherwise, output 'VERIFICATION FAILED'.",
                    _ => unreachable!(),
                };

                let mut current_messages = messages.clone();
                current_messages.push(Message::user(phase_instruction));

                let tool_defs: Vec<_> = current_tools
                    .iter()
                    .map(|t| ohc_builtin_agent_core::types::ToolDefinition {
                        name: t.name.clone(),
                        description: t.description.clone(),
                        parameters: t.parameters.clone(),
                    })
                    .collect();

                let req = ChatRequest {
                    model: config.model.clone(),
                    system: config.system_message.clone(),
                    messages: current_messages.clone(),
                    tools: tool_defs,
                    max_tokens: 2048,
                    temperature: 0.0,
                };

                match llm.chat(req).await {
                    Ok(resp) => {
                        let msg = resp.message;
                        messages.push(msg.clone());

                        if msg.tool_calls.is_empty() {
                            let content = &msg.content;
                            let _ = tx.send(AgentEvent::TextChunk {
                                content: content.clone(),
                            });

                            if current_phase == "Verify" && content.contains("TASK COMPLETE") {
                                let _ = tx.send(AgentEvent::TaskComplete {
                                    content: content.clone(),
                                });
                                return; // Finished
                            } else {
                                // If the model outputs text without tool calls, we consider it the end of this phase
                                // and transition to the next phase in the loop.
                                messages.push(Message::user(format!("Phase {} completed. Proceed to next phase.", current_phase)));
                                continue;
                            }
                        }

                        let mut read_only_calls = Vec::new();
                        let mut mutating_calls = Vec::new();

                        for tc in msg.tool_calls {
                            let is_read_only = current_tools.iter().find(|t| t.name == tc.name).map(|t| t.is_read_only).unwrap_or(false);
                            if is_read_only {
                                read_only_calls.push(tc);
                            } else {
                                mutating_calls.push(tc);
                            }
                        }

                        let mut read_only_futures = Vec::new();
                        for tc in read_only_calls {
                            let current_tools = current_tools.clone();
                            let tx = tx.clone();
                            read_only_futures.push(async move {
                                let _ = tx.send(AgentEvent::TextChunk {
                                    content: format!("Starting read-only tool call: {}", tc.name),
                                });

                                let tool_opt = current_tools.iter().find(|t| t.name == tc.name);
                                let tr = if let Some(tool) = tool_opt {
                                    match crate::tool_executor_engine::ToolExecutionEngine::execute_tool_with_langgraph_mechanics(tool, &tc, 2).await {
                                        Ok(res) => ohc_builtin_agent_core::types::ToolResult {
                                            tool_call_id: tc.id.clone(),
                                            content: res.clone(),
                                            error: String::new(),
                                        },
                                        Err(e) => ohc_builtin_agent_core::types::ToolResult {
                                            tool_call_id: tc.id.clone(),
                                            content: String::new(),
                                            error: e.to_string(),
                                        },
                                    }
                                } else {
                                    ohc_builtin_agent_core::types::ToolResult {
                                        tool_call_id: tc.id.clone(),
                                        content: String::new(),
                                        error: format!("Tool {} not found in this phase", tc.name),
                                    }
                                };

                                let _ = tx.send(AgentEvent::ToolCall {
                                    name: tc.name.clone(),
                                    args_json: tc.arguments.to_string(),
                                    result: tr.content.clone() + &tr.error,
                                    iteration: iteration as i32,
                                });
                                tr
                            });
                        }

                        let ro_results = futures::future::join_all(read_only_futures).await;
                        let mut tool_results = ro_results;

                        for tc in mutating_calls {
                            let _ = tx.send(AgentEvent::TextChunk {
                                content: format!("Starting mutating tool call: {}", tc.name),
                            });

                            let tool_opt = current_tools.iter().find(|t| t.name == tc.name);
                            let tr = if let Some(tool) = tool_opt {
                                match crate::tool_executor_engine::ToolExecutionEngine::execute_tool_with_langgraph_mechanics(tool, &tc, 2).await {
                                    Ok(res) => ohc_builtin_agent_core::types::ToolResult {
                                        tool_call_id: tc.id.clone(),
                                        content: res.clone(),
                                        error: String::new(),
                                    },
                                    Err(e) => ohc_builtin_agent_core::types::ToolResult {
                                        tool_call_id: tc.id.clone(),
                                        content: String::new(),
                                        error: e.to_string(),
                                    },
                                }
                            } else {
                                ohc_builtin_agent_core::types::ToolResult {
                                    tool_call_id: tc.id.clone(),
                                    content: String::new(),
                                    error: format!("Tool {} not found in this phase", tc.name),
                                }
                            };

                            let _ = tx.send(AgentEvent::ToolCall {
                                name: tc.name.clone(),
                                args_json: "{}".to_string(),
                                result: if tr.error.is_empty() { tr.content.clone() } else { tr.error.clone() },
                                iteration: iteration as i32,
                            });

                            tool_results.push(tr);
                        }

                        messages.push(Message {
                            role: Role::Tool,
                            content: String::new(),
                            tool_calls: vec![],
                            tool_results,
                            response_id: None,
                            previous_response_id: None,
                        });
                    }
                    Err(e) => {
                        let _ = tx.send(AgentEvent::TaskError {
                            error: format!("LLM Error: {}", e),
                        });
                        return;
                    }
                }
            }

            let _ = tx.send(AgentEvent::TaskError {
                error: "Max iterations reached".to_string(),
            });
        });

        rx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage};

    struct MockLlm {
        responses: tokio::sync::Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
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
    async fn test_gather_act_verify_progression() {
        let responses = vec![
            ChatResponse {
                message: Message::assistant("DONE GATHERING"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("1".to_string()),
            },
            ChatResponse {
                message: Message::assistant("DONE ACTING"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("2".to_string()),
            },
            ChatResponse {
                message: Message::assistant("TASK COMPLETE"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("3".to_string()),
            },
        ];

        let llm = Arc::new(MockLlm {
            responses: tokio::sync::Mutex::new(responses),
        });

        let harness = GatherActVerifyHarness::new(llm, vec![], vec![], vec![]);
        let config = GatherActVerifyConfig::default();
        let mut rx = harness.query(config, "Test task".to_string());

        let mut events = vec![];
        while let Some(event) = rx.recv().await {
            events.push(event);
        }

        // Verify the progression of phases
        let mut gather_started = false;
        let mut act_started = false;
        let mut verify_started = false;
        let mut task_complete = false;

        for event in events {
            match event {
                AgentEvent::TextChunk { content } => {
                    if content == "Starting Phase: Gather" { gather_started = true; }
                    if content == "Starting Phase: Act" { act_started = true; }
                    if content == "Starting Phase: Verify" { verify_started = true; }
                }
                AgentEvent::TaskComplete { .. } => {
                    task_complete = true;
                }
                _ => {}
            }
        }

        assert!(gather_started, "Gather phase should have started");
        assert!(act_started, "Act phase should have started");
        assert!(verify_started, "Verify phase should have started");
        assert!(task_complete, "Task should have completed");
    }
}
