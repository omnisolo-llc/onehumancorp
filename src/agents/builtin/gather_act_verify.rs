use std::sync::Arc;
use tokio::sync::mpsc;
use ohc_builtin_agent_core::types::{ChatRequest, Message, Role};
use crate::llm::LlmClient;
use crate::tools::Tool;
use crate::agent::AgentEvent;
use crate::agent::AgentRunConfig;

/// Master Catalog A: Framework Implementation Archetypes: Anthropic Claude Agent SDK & Claude Code.
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
    pub checkpointer: Option<Arc<dyn crate::checkpointer::CheckpointSaver>>,
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
            checkpointer: None,
        }
    }

    pub fn with_checkpointer(mut self, checkpointer: Arc<dyn crate::checkpointer::CheckpointSaver>) -> Self {
        self.checkpointer = Some(checkpointer);
        self
    }

    /// The single query function returning an async channel of AgentEvents.
    pub fn query(
        &self,
        config: AgentRunConfig,
        task: String,
    ) -> mpsc::UnboundedReceiver<AgentEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        let llm = self.llm.clone();

        // We clone tools since we need to move them into the async task.
        let gather_tools = self.gather_tools.clone();
        let act_tools = self.act_tools.clone();
        let verify_tools = self.verify_tools.clone();
        let checkpointer = self.checkpointer.clone();

        tokio::spawn(async move {
            let mut last_checkpoint_id: Option<String> = None;
            let scratchpad_path = config
                .state_scratchpad_path
                .clone()
                .unwrap_or_else(|| format!(".agent_progress_{}.json", config.thread_id.clone().unwrap_or_else(|| "default".to_string())));

            let mut error_counts: std::collections::HashMap<String, u64> = std::collections::HashMap::new();

            if let Some(guardrails) = &config.guardrails {
                if let Err(e) = guardrails.check_input(&task) {
                    let _ = tx.send(AgentEvent::GuardrailTripped { reason: e.clone() });
                    let _ = tx.send(AgentEvent::TaskError { error: format!("Termination: Input Guardrail tripwire fires: {}", e) });
                    return;
                }
            }

            let mut messages = vec![
                Message::system(config.server_system_message.clone()),
                Message::user(format!("Task: {}", task)),
            ];

            let mut total_tokens = 0;
            let mut budget_tracker = crate::budget::BudgetTracker::default();


            let phases = ["Gather", "Act", "Verify"];
            let tools_by_phase = vec![gather_tools, act_tools, verify_tools];

            for iteration in 0..config.max_iterations {
                if config.enable_observation_masking {
                    crate::observation_masking::apply_observation_masking(
                        &mut messages,
                        config.observation_masking_threshold,
                        config.observation_masking_size_limit,
                        config.observation_masking_element_limit,
                    );
                }

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
                    system: config.server_system_message.clone(),
                    messages: current_messages.clone(),
                    tools: tool_defs,
                    max_tokens: 2048,
                    temperature: 0.0,
                };

                match llm.chat(req).await {

                    Ok(resp) => {
                        let msg = resp.message;

                        // Termination Condition: Safety refusal
                        if resp.stop_reason == "safety" || resp.stop_reason == "refusal" {
                            let _ = tx.send(AgentEvent::TaskError { error: "Termination: Safety refusal".to_string() });
                            return;
                        }

                        let usage = resp.usage;
                        total_tokens += usage.input_tokens + usage.output_tokens;

                        // Termination Condition: Token budget exhausted
                        if config.max_task_tokens > 0 && total_tokens > config.max_task_tokens as i64 {
                            let _ = tx.send(AgentEvent::TaskError { error: "Termination: Token budget exhausted".to_string() });
                            return;
                        }
                        if resp.stop_reason == "max_tokens" || resp.stop_reason == "length" {
                            let decision = crate::budget::check_token_budget(&mut budget_tracker, config.max_task_tokens as i64, total_tokens as i64);
                            if decision.action == crate::budget::BudgetAction::Stop {
                                let _ = tx.send(AgentEvent::TaskError { error: "Termination: Token budget exhausted".to_string() });
                                return;
                            }
                            if decision.action == crate::budget::BudgetAction::Continue {
                                if !msg.content.is_empty() {
                                    messages.push(msg.clone());
                                }
                                messages.push(crate::types::Message::user(&decision.nudge_message));
                                continue;
                            }
                        }

                        messages.push(msg.clone());

                        if msg.tool_calls.is_empty() {
                            // Output Guardrail
                            if let Some(guardrails) = &config.guardrails {
                                if let Err(e) = guardrails.check_output(&msg.content) {
                                    let _ = tx.send(AgentEvent::GuardrailTripped { reason: e.clone() });
                                    let _ = tx.send(AgentEvent::TaskError { error: format!("Termination: Output Guardrail tripwire fires: {}", e) });
                                    return;
                                }
                            }

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
                            if let Some(guardrails) = &config.guardrails {
                                if let Err(e) = guardrails.check_tool(&tc) {
                                    let _ = tx.send(AgentEvent::GuardrailTripped { reason: e.clone() });
                                    let _ = tx.send(AgentEvent::TaskError { error: format!("Termination: Tool Guardrail tripwire fires: {}", e) });
                                    return;
                                }
                            }

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
                            let tc_name = tc.name.clone();
                            read_only_futures.push(async move {
                                let _ = tx.send(AgentEvent::TextChunk {
                                    content: format!("Starting read-only tool call: {}", tc.name),
                                });

                                let tool_opt = current_tools.iter().find(|t| t.name == tc.name);
                                let tr = if let Some(tool) = tool_opt {
                                    match crate::tool_executor_engine::ToolExecutionEngine::execute_tool_with_langgraph_mechanics(tool, &tc, 2, &config).await {
                                        Ok(res) => ohc_builtin_agent_core::types::ToolResult {
                                            tool_call_id: tc.id.clone(),
                                            content: res.clone(),
                                            error: String::new(),
                                        },
                                        Err(ohc_builtin_agent_core::types::ToolError::LlmRecoverable(msg)) => ohc_builtin_agent_core::types::ToolResult::new_llm_recoverable(tc.id.clone(), &tc.name, &msg),
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
                                (tc_name, tr)
                            });
                        }

                        let ro_results = futures::future::join_all(read_only_futures).await;
                        let mut tool_results = Vec::new();
                        let cfg_max_retries = std::cmp::min(config.max_retries, 2) as u64;

                        for (name, mut tr) in ro_results {
                            if tr.error.contains("LLM-Recoverable Error") || tr.error.contains("Recoverable error") {
                                let count = *error_counts.entry(name.clone()).or_insert(0) + 1;
                                error_counts.insert(name.clone(), count);
                                if count > cfg_max_retries {
                                    let msg = format!("Fatal tool error: Tool '{}' failed consecutively beyond max_retries limit with recoverable errors. Escalating to Fatal to prevent compounding error loops. Last error: {}", name, tr.error);
                                    let _ = tx.send(AgentEvent::TaskError { error: msg });
                                    return;
                                }
                            } else if tr.error.is_empty() {
                                error_counts.insert(name.clone(), 0);
                            }
                            tool_results.push(tr);
                        }

                        for tc in mutating_calls {
                            let _ = tx.send(AgentEvent::TextChunk {
                                content: format!("Starting mutating tool call: {}", tc.name),
                            });

                            let tool_opt = current_tools.iter().find(|t| t.name == tc.name);
                            let tr = if let Some(tool) = tool_opt {
                                match crate::tool_executor_engine::ToolExecutionEngine::execute_tool_with_langgraph_mechanics(tool, &tc, 2, &config).await {
                                    Ok(res) => ohc_builtin_agent_core::types::ToolResult {
                                        tool_call_id: tc.id.clone(),
                                        content: res.clone(),
                                        error: String::new(),
                                    },
                                    Err(ohc_builtin_agent_core::types::ToolError::LlmRecoverable(msg)) => ohc_builtin_agent_core::types::ToolResult::new_llm_recoverable(tc.id.clone(), &tc.name, &msg),
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

                            if tr.error.contains("LLM-Recoverable Error") || tr.error.contains("Recoverable error") {
                                let count = *error_counts.entry(tc.name.clone()).or_insert(0) + 1;
                                error_counts.insert(tc.name.clone(), count);
                                if count > cfg_max_retries {
                                    let msg = format!("Fatal tool error: Tool '{}' failed consecutively beyond max_retries limit with recoverable errors. Escalating to Fatal to prevent compounding error loops. Last error: {}", tc.name, tr.error);
                                    let _ = tx.send(AgentEvent::TaskError { error: msg });
                                    return;
                                }
                            } else if tr.error.is_empty() {
                                error_counts.insert(tc.name.clone(), 0);
                            }

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

                // Master Catalog B.7. State Management Checkpointing Mechanic
                // 1. Configured Checkpointer (Database or Git)
                if config.enable_state_checkpointing {
                    if let (Some(cp_saver), Some(thread_id)) = (&checkpointer, &config.thread_id) {
                        let checkpoint_id = uuid::Uuid::new_v4().to_string();
                        let cp = crate::checkpointer::Checkpoint {
                            thread_id: thread_id.clone(),
                            checkpoint_id: checkpoint_id.clone(),
                            parent_id: last_checkpoint_id.clone(),
                            data: serde_json::to_value(&messages).unwrap_or(serde_json::Value::Null),
                            metadata: serde_json::json!({
                                "iteration": iteration,
                            }),
                            created_at: chrono::Utc::now(),
                        };
                        if let Err(e) = cp_saver.put_checkpoint(cp).await {
                            tracing::warn!("Failed to save checkpoint: {}", e);
                        } else {
                            last_checkpoint_id = Some(checkpoint_id.clone());
                            let _ = tx.send(AgentEvent::CheckpointSaved {
                                iteration: iteration as i32,
                                path: format!("{}:{}", cp_saver.storage_prefix(), checkpoint_id),
                            });
                        }
                    }

                    // 2. Local File Scratchpad (Claude Code Mechanic)
                    let mut pf = crate::checkpointer::ProgressFile::default();
                    pf.status = format!("GatherActVerify Iteration {}", iteration);
                    pf.notes.push(format!("Completed Iteration {}", iteration));
                    if let Ok(json_state) = serde_json::to_string_pretty(&pf) {
                        if tokio::fs::write(&scratchpad_path, json_state).await.is_ok() {
                            let _ = tx.send(AgentEvent::CheckpointSaved {
                                iteration: iteration as i32,
                                path: scratchpad_path.clone(),
                            });
                        }
                    }
                }
            }

            let _ = tx.send(AgentEvent::TaskError {
                error: "Termination: Max turn limit exceeded".to_string(),
            });
        });

        rx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage};

    #[tokio::test]
    async fn test_gather_act_verify_compounding_error_prevention() {
        use crate::tools::ToolExecutor;
        use ohc_builtin_agent_core::types::ToolError;

        struct FailingToolExecutor;
        #[async_trait::async_trait]
        impl ToolExecutor for FailingToolExecutor {
            async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
                Err(ToolError::LlmRecoverable("Validation Error (Pydantic-first tool schema): Bad args".to_string()))
            }
        }

        let fail_tool = Tool {
            name: "fail_tool".to_string(),
            description: "always fails".to_string(),
            parameters: serde_json::json!({}),
            is_read_only: false,
            execute: Arc::new(FailingToolExecutor),
        };

        struct FailingMockLlm {
            call_count: tokio::sync::Mutex<usize>,
        }

        #[async_trait::async_trait]
        impl LlmClient for FailingMockLlm {
            async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let mut count = self.call_count.lock().await;
                *count += 1;

                let tool_call = ohc_builtin_agent_core::types::ToolCall {
                    id: format!("call_{}", count),
                    name: "fail_tool".to_string(),
                    arguments: serde_json::json!({}),
                };

                Ok(ChatResponse {
                    message: Message {
                        role: ohc_builtin_agent_core::types::Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![tool_call],
                        tool_results: vec![],
                        response_id: None,
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some(format!("resp_{}", count)),
                })
            }
        }

        let llm = Arc::new(FailingMockLlm {
            call_count: tokio::sync::Mutex::new(0),
        });

        let mut harness = GatherActVerifyHarness::new(
            llm.clone() as Arc<dyn LlmClient>,
            vec![fail_tool.clone()],
            vec![fail_tool.clone()],
            vec![fail_tool],
        );
        let mut config = AgentRunConfig::default();
        config.max_retries = 2; // Clamp limits to 2
        let mut rx = harness.query(config, "Do the thing".to_string());

        let mut error_msg = String::new();
        while let Some(evt) = rx.recv().await {
            if let AgentEvent::TaskError { error } = evt {
                error_msg = error;
                break;
            }
        }

        assert!(error_msg.contains("Fatal tool error: Tool 'fail_tool' failed consecutively beyond max_retries limit"));
    }

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
        let mut config = AgentRunConfig::default(); config.max_iterations = 15; config.server_system_message = "You are an agent executing the Gather-Act-Verify cycle.".to_string(); config.model = "claude-3-5-sonnet".to_string();
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

    #[tokio::test]
    async fn test_gather_act_verify_observation_masking_integration() {
        let llm = Arc::new(MockLlm {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: String::new(),
                        tool_calls: vec![ohc_builtin_agent_core::types::ToolCall {
                            id: "call_1".to_string(),
                            name: "test_gather".to_string(),
                            arguments: serde_json::json!({}),
                        }],
                        tool_results: vec![],
                        response_id: Some("mock-1".to_string()),
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("mock-1".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("TASK COMPLETE"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-2".to_string()),
                }
            ]),
        });

        let gather_tools = vec![Tool {
            name: "test_gather".to_string(),
            description: "gather tool".to_string(),
            parameters: serde_json::json!({"type": "object", "properties": {}}),
            execute: Arc::new(MockToolExecutor {
                result: "A very long tool output that should be masked because it is long and old".repeat(50).to_string(),
            }),
            is_read_only: true,
        }];

        let harness = GatherActVerifyHarness::new(llm, gather_tools, vec![], vec![]);

        let mut config = AgentRunConfig::default();
        config.enable_observation_masking = true;
        config.observation_masking_threshold = 0;
        config.observation_masking_size_limit = 10;
        config.observation_masking_element_limit = 10;

        let mut rx = harness.query(config, "Task".to_string());

        let mut events = vec![];
        while let Some(evt) = rx.recv().await {
            events.push(evt);
        }

        let contains_complete = events.iter().any(|e| match e {
            AgentEvent::TaskComplete { .. } => true,
            _ => false,
        });
        assert!(contains_complete);
    }

    #[tokio::test]
    async fn test_gather_act_verify_termination_max_turn_limit() {
        let llm = Arc::new(MockLlm {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ohc_builtin_agent_core::types::ToolCall { id: "1".to_string(), name: "dummy".to_string(), arguments: serde_json::json!({}) }],
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
                        tool_calls: vec![ohc_builtin_agent_core::types::ToolCall { id: "2".to_string(), name: "dummy".to_string(), arguments: serde_json::json!({}) }],
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

        let harness = GatherActVerifyHarness::new(llm, vec![], vec![], vec![]);
        let mut config = AgentRunConfig::default();
        config.max_iterations = 1; // Force termination
        config.server_system_message = "You are an agent executing the Gather-Act-Verify cycle.".to_string();
        config.model = "claude-3-5-sonnet".to_string();

        let mut rx = harness.query(config, "Hello".to_string());

        let mut has_max_turn_err = false;
        while let Some(event) = rx.recv().await {
            if let AgentEvent::TaskError { error } = event {
                if error.contains("Termination: Max turn limit exceeded") {
                    has_max_turn_err = true;
                }
            }
        }

        assert!(has_max_turn_err);
    }

    #[tokio::test]
    async fn test_gather_act_verify_termination_token_budget_exhausted() {
        let llm = Arc::new(MockLlm {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("Too many tokens"),
                    usage: Usage { input_tokens: 500, output_tokens: 600, cache_creation_input_tokens: 0, cache_read_input_tokens: 0 },
                    stop_reason: "stop".to_string(),
                    response_id: None,
                }
            ]),
        });

        let harness = GatherActVerifyHarness::new(llm, vec![], vec![], vec![]);
        let mut config = AgentRunConfig::default();
        config.max_iterations = 5;
        config.max_task_tokens = 1000; // 500 + 600 = 1100 > 1000
        config.server_system_message = "You are an agent executing the Gather-Act-Verify cycle.".to_string();
        config.model = "claude-3-5-sonnet".to_string();

        let mut rx = harness.query(config, "Hello".to_string());

        let mut has_token_err = false;
        while let Some(event) = rx.recv().await {
            if let AgentEvent::TaskError { error } = event {
                if error.contains("Termination: Token budget exhausted") {
                    has_token_err = true;
                }
            }
        }

        assert!(has_token_err);
    }

    #[tokio::test]
    async fn test_gather_act_verify_guardrail_tripwire() {
        let llm = Arc::new(MockLlm {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("I will now destroy everything."),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: None,
                }
            ]),
        });

        let harness = GatherActVerifyHarness::new(llm, vec![], vec![], vec![]);
        let mut config = AgentRunConfig::default();
        config.max_iterations = 5;
        config.server_system_message = "You are an agent executing the Gather-Act-Verify cycle.".to_string();
        config.model = "claude-3-5-sonnet".to_string();
        config.guardrails = Some(ohc_builtin_agent_core::types::GuardrailsConfig {
            input_guardrail: None,
            output_guardrail: Some(Box::new(|output| {
                if output.contains("destroy") {
                    return Err("Output contains forbidden word".to_string());
                }
                Ok(())
            })),
            tool_guardrail: None,
        });

        let mut rx = harness.query(config, "Hello".to_string());

        let mut has_guardrail_err = false;
        while let Some(event) = rx.recv().await {
            if let AgentEvent::TaskError { error } = event {
                if error.contains("Termination: Output Guardrail tripwire fires") {
                    has_guardrail_err = true;
                }
            }
        }

        assert!(has_guardrail_err);
}
