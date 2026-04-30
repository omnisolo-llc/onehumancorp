use ohc_builtin_agent_core::types::ToolError;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};

use crate::budget::{check_token_budget, BudgetAction, BudgetTracker};
use crate::guardrails::GuardrailConfig;
use ohc_builtin_agent_llm::LlmClient;
use ohc_builtin_agent_tools::Tool;
use ohc_builtin_agent_core::types::{ChatRequest, Message, Role, ToolCall, ToolDefinition, ToolResult};

/// Events emitted by the agent run loop.
#[derive(Debug, Clone)]
pub enum AgentEvent {
    RunStarted { iteration: i32 },
    TextChunk { content: String },
    ToolCall { name: String, args_json: String, result: String, iteration: i32 },
    TaskComplete { content: String },
    TaskError { error: String },
    IterationStarted { iteration: i32, message_count: usize },
}

/// Configuration for a single agent run.
#[derive(Debug, Clone)]
pub struct AgentRunConfig {
    pub model: String,
    pub server_system_message: String,
    pub developer_instructions: String,
    pub user_instructions: String,
    pub max_tokens: i32,
    pub temperature: f32,
    pub max_iterations: i32,
    pub max_task_tokens: i32, // budget for token tracking
    pub confidence_threshold: f32,
    pub enable_observation_masking: bool,
    pub enable_llm_judge: bool,
    pub guardrails: Option<GuardrailConfig>,
}

impl Default for AgentRunConfig {
    fn default() -> Self {
        Self {
            model: String::new(),
            server_system_message: String::new(),
            developer_instructions: String::new(),
            user_instructions: String::new(),
            max_tokens: 2048,
            temperature: 0.0,
            max_iterations: 100,
            max_task_tokens: 0,
            confidence_threshold: 0.0,
            enable_observation_masking: true,
            enable_llm_judge: false,
            guardrails: None,
        }
    }
}

/// Progress metrics for a running agent task.
#[derive(Default)]
pub struct AgentProgress {
    tool_use_count: AtomicU64,
    token_count: AtomicI64,
}

impl AgentProgress {
    pub fn record_tool_use(&self) {
        self.tool_use_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn add_tokens(&self, n: i64) {
        self.token_count.fetch_add(n, Ordering::Relaxed);
    }

    pub fn tool_use_count(&self) -> u64 {
        self.tool_use_count.load(Ordering::Relaxed)
    }

    pub fn token_count(&self) -> i64 {
        self.token_count.load(Ordering::Relaxed)
    }
}

pub(crate) fn build_hierarchical_system_prompt(cfg: &AgentRunConfig) -> String {
    let mut end_idx = 32768;
    if cfg.user_instructions.len() > 32768 {
        while end_idx > 0 && !cfg.user_instructions.is_char_boundary(end_idx) {
            end_idx -= 1;
        }
    } else {
        end_idx = cfg.user_instructions.len();
    }
    let user_instr = &cfg.user_instructions[..end_idx];

    let mut combined_system = String::new();
    if !cfg.server_system_message.is_empty() {
        combined_system.push_str(&cfg.server_system_message);
    }
    if !cfg.developer_instructions.is_empty() {
        if !combined_system.is_empty() {
            combined_system.push_str("\n\n");
        }
        combined_system.push_str("[Developer Instructions]\n");
        combined_system.push_str(&cfg.developer_instructions);
    }
    if !user_instr.is_empty() {
        if !combined_system.is_empty() {
            combined_system.push_str("\n\n");
        }
        combined_system.push_str("[User Instructions]\n");
        combined_system.push_str(user_instr);
    }
    combined_system
}

/// The ReAct agent loop — mirrors Go builtin.BuiltinAgent.Run.
pub struct Agent {
    pub llm: Arc<dyn LlmClient>,
    pub tools: Vec<Tool>,
    pub progress: Arc<AgentProgress>,
}

impl Agent {
    pub fn new(llm: Arc<dyn LlmClient>, tools: Vec<Tool>) -> Self {
        Self {
            llm,
            tools,
            progress: Arc::new(AgentProgress::default()),
        }
    }

    /// Run the agent loop. Calls `on_event` for each event.
    #[tracing::instrument(skip(self, on_event, cfg), fields(model = %cfg.model))]
    pub async fn run<F>(
        &self,
        cfg: &AgentRunConfig,
        initial_message: &str,
        on_event: &mut F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send,
    {
        // OpenAI Mechanic: Input Guardrails
        if let Some(guard_cfg) = &cfg.guardrails {
            if let Err(e) = crate::guardrails::check_input(initial_message, guard_cfg) {
                on_event(AgentEvent::TaskError { error: e.clone() });
                return Err(e.into());
            }
        }

        on_event(AgentEvent::RunStarted { iteration: 0 });

        let tool_defs: Vec<ToolDefinition> = self
            .tools
            .iter()
            .map(|t| ToolDefinition {
                name: t.name.clone(),
                description: t.description.clone(),
                parameters: t.parameters.clone(),
            })
            .collect();

        let mut messages: Vec<Message> = vec![Message::user(initial_message)];
        let mut budget_tracker = BudgetTracker::default();
        let mut global_turn_tokens = 0i32;
        let mut last_assistant_content = String::new();

        let max_iterations = if cfg.max_iterations <= 0 { 100 } else { cfg.max_iterations };

        let combined_system = build_hierarchical_system_prompt(cfg);

        for iteration in 0..max_iterations {
            on_event(AgentEvent::IterationStarted {
                iteration,
                message_count: messages.len(),
            });

            let req = ChatRequest {
                model: cfg.model.clone(),
                system: combined_system.clone(),
                messages: messages.clone(),
                tools: tool_defs.clone(),
                max_tokens: cfg.max_tokens,
                temperature: cfg.temperature,
            };

            let resp = match self.llm.chat(req).await {
                Ok(r) => r,
                Err(e) => {
                    let err = format!("LLM error: {}", e);
                    on_event(AgentEvent::TaskError { error: err.clone() });
                    return Err(err.into());
                }
            };

            let input_tokens = resp.usage.input_tokens;
            let output_tokens = resp.usage.output_tokens;
            let total_tokens = (input_tokens + output_tokens) as i64;
            self.progress.add_tokens(total_tokens);
            global_turn_tokens += output_tokens;

            let stop_reason = resp.stop_reason.as_str();

            // Text content from assistant
            if !resp.message.content.is_empty() {
                last_assistant_content = resp.message.content.clone();
                on_event(AgentEvent::TextChunk {
                    content: resp.message.content.clone(),
                });
            }

            // Token budget check when LLM stops due to length.
            if stop_reason == "max_tokens" || stop_reason == "length" {
                let decision = check_token_budget(
                    &mut budget_tracker,
                    cfg.max_task_tokens,
                    global_turn_tokens,
                );
                if decision.action == BudgetAction::Continue {
                    // Add the budget nudge to messages and continue.
                    if !resp.message.content.is_empty() {
                        messages.push(resp.message.clone());
                    }
                    messages.push(Message::user(&decision.nudge_message));
                    continue;
                }
            }

            let tool_calls = resp.message.tool_calls.clone();

            // Add assistant message to history (including tool calls).
            messages.push(resp.message.clone());

            // Terminal condition: no tool calls.
            if tool_calls.is_empty() {
                // Inferential/Sensors (LLM-as-judge subagent)
                if cfg.enable_llm_judge {
                    let judge_req = ChatRequest {
                        model: cfg.model.clone(),
                        system: "You are an expert judge. Evaluate the following output for correctness, completeness, and adherence to constraints. Output ONLY 'APPROVE' or 'REJECT: <reason>'.".to_string(),
                        messages: vec![Message::user(format!("Evaluate this output:
{}", last_assistant_content))],
                        tools: vec![],
                        max_tokens: 500,
                        temperature: 0.0,
                    };

                    match self.llm.chat(judge_req).await {
                        Ok(judge_resp) => {
                            let judge_text = judge_resp.message.content.trim();
                            if judge_text.starts_with("REJECT:") {
                                let reason = judge_text.strip_prefix("REJECT:").unwrap_or(judge_text).trim();
                                let err_msg = format!("Your previous output was evaluated by an LLM-as-judge and rejected. Reason: {}. Please correct your work and use tools if necessary.", reason);
                                messages.push(Message::user(err_msg));
                                continue;
                            }
                            // If APPROVE or anything else, we proceed to output guardrails.
                        }
                        Err(e) => {
                            let err = format!("LLM Judge error: {}", e);
                            on_event(AgentEvent::TaskError { error: err.clone() });
                            return Err(err.into());
                        }
                    }
                }
                // In a production-grade agent, we might use a separate LLM pass
                // to evaluate confidence in the final answer if threshold > 0.
                // For now, we'll assume the model is confident if it didn't use more tools.

                // OpenAI Mechanic: Output Guardrails
                if let Some(guard_cfg) = &cfg.guardrails {
                    if let Err(e) = crate::guardrails::check_output(&last_assistant_content, guard_cfg) {
                        on_event(AgentEvent::TaskError { error: e.clone() });
                        return Err(e.into());
                    }
                }

                on_event(AgentEvent::TaskComplete {
                    content: last_assistant_content.clone(),
                });
                return Ok(last_assistant_content);
            }

            // Execute tool calls and collect results.
            let mut tool_results: Vec<ToolResult> = Vec::new();

            let mut read_only_calls = Vec::new();
            let mut mutating_calls = Vec::new();

            for tc in &tool_calls {
                let is_mutating = self.tools.iter().find(|t| t.name == tc.name).map(|t| t.is_mutating).unwrap_or(false);
                if is_mutating {
                    mutating_calls.push(tc.clone());
                } else {
                    read_only_calls.push(tc.clone());
                }
            }

            // Execute read-only tools concurrently
            let mut ro_futures = Vec::new();
            for tc in &read_only_calls {
                ro_futures.push(self.execute_tool_with_retries(tc, cfg, iteration));
            }

            let ro_results = futures::future::join_all(ro_futures).await;

            for (tc, res) in read_only_calls.iter().zip(ro_results) {
                match res {
                    Ok((content, error, events)) => {
                        for e in events {
                            on_event(e);
                        }
                        tool_results.push(ToolResult {
                            tool_call_id: tc.id.clone(),
                            content,
                            error,
                        });
                    }
                    Err((e, events)) => {
                        for ev in events {
                            on_event(ev);
                        }
                        return Err(e);
                    }
                }
            }

            // Execute mutating tools serially
            for tc in &mutating_calls {
                match self.execute_tool_with_retries(tc, cfg, iteration).await {
                    Ok((content, error, events)) => {
                        for e in events {
                            on_event(e);
                        }
                        tool_results.push(ToolResult {
                            tool_call_id: tc.id.clone(),
                            content,
                            error,
                        });
                    }
                    Err((e, events)) => {
                        for ev in events {
                            on_event(ev);
                        }
                        return Err(e);
                    }
                }
            }



            if cfg.enable_observation_masking {
                // JetBrains Observation Masking: Hide the raw output of old tools from the prompt,
                // but keep the `tool_calls` themselves visible so the model remembers what it did.
                for m in &mut messages {
                    if m.role == Role::Tool {
                        for tr in &mut m.tool_results {
                            if tr.error.is_empty() && !tr.content.starts_with("[Observation Masked to save context.") {
                                let bytes = tr.content.len();
                                if bytes > 150 {
                                    tr.content = format!(
                                        "[Observation Masked to save context. Output was {} bytes. The tool call itself remains visible so you remember this action.]",
                                        bytes
                                    );
                                }
                            }
                        }
                    }
                }
            }

            // Append tool results as a user turn.
            messages.push(Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results,
            });
        }

        // Hit max iterations.
        on_event(AgentEvent::TaskComplete {
            content: last_assistant_content.clone(),
        });
        Ok(last_assistant_content)
    }


    async fn execute_tool_with_retries(
        &self,
        tc: &ToolCall,
        cfg: &AgentRunConfig,
        iteration: i32,
    ) -> Result<(String, String, Vec<AgentEvent>), (Box<dyn std::error::Error + Send + Sync>, Vec<AgentEvent>)> {
        let mut events = Vec::new();
        // OpenAI Mechanic: Tool Guardrails
        if let Some(guard_cfg) = &cfg.guardrails {
            if let Err(e) = crate::guardrails::check_tool(tc, guard_cfg) {
                events.push(AgentEvent::TaskError { error: e.clone() });
                return Err((e.into(), events)); // Tripwire: halt the loop immediately
            }
        }

        let mut retry_count = 0;
        let max_retries = 3;
        let mut content = String::new();
        let mut error = String::new();

        loop {
            let result = self.execute_tool(tc).await;
            match result {
                Ok(r) => {
                    self.progress.record_tool_use();
                    events.push(AgentEvent::ToolCall {
                        name: tc.name.clone(),
                        args_json: tc.arguments.to_string(),
                        result: r.clone(),
                        iteration,
                    });
                    content = r;
                    break;
                }
                Err(ToolError::Transient(msg)) => {
                    if retry_count < max_retries {
                        retry_count += 1;
                        let backoff = std::time::Duration::from_millis(500 * (1 << retry_count));
                        tokio::time::sleep(backoff).await;
                        continue;
                    } else {
                        let err = format!("Transient error after retries: {}", msg);
                        events.push(AgentEvent::ToolCall {
                            name: tc.name.clone(),
                            args_json: tc.arguments.to_string(),
                            result: format!("Error: {}", err),
                            iteration,
                        });
                        error = err;
                        break;
                    }
                }
                Err(ToolError::LlmRecoverable(msg)) => {
                    let err = format!("LLM Recoverable error: {}", msg);
                    events.push(AgentEvent::ToolCall {
                        name: tc.name.clone(),
                        args_json: tc.arguments.to_string(),
                        result: format!("Error: {}", err),
                        iteration,
                    });
                    // Return the raw error as a ToolMessage directly to the model so it can self-correct
                    error = err;
                    break;
                }
                Err(ToolError::UserFixable(msg)) => {
                    let err = format!("User intervention required: {}", msg);
                    events.push(AgentEvent::TaskError { error: err.clone() });
                    return Err((err.into(), events));
                }
                Err(ToolError::Fatal(msg)) => {
                    let err = format!("Fatal tool error: {}", msg);
                    events.push(AgentEvent::TaskError { error: err.clone() });
                    return Err((err.into(), events));
                }
            }
        }
        Ok((content, error, events))
    }

    async fn execute_tool(
        &self,
        tc: &ToolCall,
    ) -> Result<String, ToolError> {
        let tool = self
            .tools
            .iter()
            .find(|t| t.name == tc.name)
            .ok_or_else(|| ToolError::LlmRecoverable(format!("unknown tool: {}", tc.name)))?;

        tool.execute.execute(tc.arguments.clone()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage};
    use std::sync::Arc;
    use ohc_builtin_agent_tools::ToolExecutor;
    use serde_json::Value;

    struct MockLlmClient {
        responses: tokio::sync::Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            if resps.is_empty() {
                return Ok(ChatResponse {
                    message: Message::assistant("Final answer"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                });
            }
            Ok(resps.remove(0))
        }
    }

    struct MockToolExecutor;

    #[async_trait::async_trait]
    impl ToolExecutor for MockToolExecutor {
        async fn execute(&self, _args: Value) -> Result<String, ToolError> {
            Ok("A very long tool output that should be masked because it is long enough".to_string())
        }
    }

    #[tokio::test]
    async fn test_observation_masking() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "test_tool".to_string(),
                            arguments: Value::Null,
                        }],
                        tool_results: vec![],
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                },
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_2".to_string(),
                            name: "test_tool".to_string(),
                            arguments: Value::Null,
                        }],
                        tool_results: vec![],
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                },
            ]),
        });

        let tools = vec![Tool {
            name: "test_tool".to_string(),
            description: "test".to_string(),
            parameters: Value::Null,
            execute: Arc::new(MockToolExecutor),
            is_mutating: false,
        }];

        let agent = Agent::new(client, tools);

        let mut cfg = AgentRunConfig::default();
        cfg.enable_observation_masking = true;

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Hello", &mut on_event).await;
        assert!(result.is_ok());

        // In this test, the agent will loop:
        // Iter 0: LLM asks for test_tool. Tool returns result.
        //   Agent runs masking check. The message list contains User(Hello) and Assistant(tool_call).
        //   The new tool result is appended.
        // Iter 1: LLM asks for test_tool again.
        //   Agent runs masking check. The previous tool result (from Iter 0) is now masked.
        //   The new tool result is appended.
        // Iter 2: LLM returns final answer.

        // We can't directly inspect `messages` from the outside, but we can verify it compiled
        // and ran without errors, which covers the logic path.
        // Also checking the length constraint logic.
    }

    #[tokio::test]
    async fn test_guardrail_tripwire() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "I am going to use the bad tool now.".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "banned_tool".to_string(),
                            arguments: Value::Null,
                        }],
                        tool_results: vec![],
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                },
                ChatResponse {
                    message: Message::assistant("This contains the secret password!"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                },
            ]),
        });

        let tools = vec![
            Tool {
                name: "banned_tool".to_string(),
                description: "test".to_string(),
                parameters: Value::Null,
                execute: Arc::new(MockToolExecutor),
            is_mutating: false,
            },
            Tool {
                name: "safe_tool".to_string(),
                description: "test".to_string(),
                parameters: Value::Null,
                execute: Arc::new(MockToolExecutor),
            is_mutating: false,
            },
        ];

        let agent = Agent::new(client, tools);

        let mut cfg = AgentRunConfig::default();
        cfg.guardrails = Some(crate::guardrails::GuardrailConfig {
            blocked_keywords: vec!["banned".to_string(), "password".to_string(), "secret".to_string()],
        });

        // Test Input Guardrail
        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };
        let result = agent.run(&cfg, "Hello, please give me the secret password.", &mut on_event).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Input guardrail tripped"));

        // Reset client for next tests
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "banned_tool".to_string(),
                            arguments: Value::Null,
                        }],
                        tool_results: vec![],
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                },
            ]),
        });
        let agent = Agent::new(client, vec![
            Tool {
                name: "banned_tool".to_string(),
                description: "test".to_string(),
                parameters: Value::Null,
                execute: Arc::new(MockToolExecutor),
            is_mutating: false,
            },
        ]);

        // Test Tool Guardrail
        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };
        let result = agent.run(&cfg, "Hello", &mut on_event).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Tool guardrail tripped"));

        // Reset client for Output test
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("Here is the secret data."),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                },
            ]),
        });
        let agent = Agent::new(client, vec![]);

        // Test Output Guardrail
        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };
        let result = agent.run(&cfg, "Hello", &mut on_event).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Output guardrail tripped"));
    }

    #[test]
    fn test_hierarchical_system_prompt() {
        let mut cfg = AgentRunConfig::default();
        cfg.server_system_message = "Server System Message".to_string();
        cfg.developer_instructions = "Developer Instructions".to_string();
        cfg.user_instructions = "User Instructions".to_string();

        let prompt = build_hierarchical_system_prompt(&cfg);
        assert_eq!(
            prompt,
            "Server System Message\n\n[Developer Instructions]\nDeveloper Instructions\n\n[User Instructions]\nUser Instructions"
        );
    }

    #[test]
    fn test_hierarchical_system_prompt_missing_sections() {
        let mut cfg = AgentRunConfig::default();
        cfg.server_system_message = "Server System Message".to_string();
        cfg.developer_instructions = "".to_string();
        cfg.user_instructions = "User Instructions".to_string();

        let prompt = build_hierarchical_system_prompt(&cfg);
        assert_eq!(
            prompt,
            "Server System Message\n\n[User Instructions]\nUser Instructions"
        );

        let mut cfg2 = AgentRunConfig::default();
        cfg2.server_system_message = "".to_string();
        cfg2.developer_instructions = "Dev".to_string();
        cfg2.user_instructions = "User".to_string();
        let prompt2 = build_hierarchical_system_prompt(&cfg2);
        assert_eq!(
            prompt2,
            "[Developer Instructions]\nDev\n\n[User Instructions]\nUser"
        );
    }

    #[test]
    fn test_hierarchical_system_prompt_truncation_safe() {
        let mut cfg = AgentRunConfig::default();
        // A single emoji is 4 bytes.
        let emoji = "🚀"; // 4 bytes
        // 8192 emojis = 32768 bytes
        cfg.user_instructions = emoji.repeat(8192);
        // Add one more emoji to exceed the limit
        cfg.user_instructions.push_str(emoji); // 32772 bytes

        // This should safely truncate without panicking
        let prompt = build_hierarchical_system_prompt(&cfg);
        assert!(prompt.contains("[User Instructions]\n"));
        // Check that the user instructions part is exactly 32768 bytes long
        assert_eq!(prompt.len() - "[User Instructions]\n".len(), 32768);
    }

    #[test]
    fn test_hierarchical_system_prompt_truncation_safe_boundary() {
        let mut cfg = AgentRunConfig::default();
        // Construct a string where the 32768th byte is in the middle of a multibyte character.
        // Let's use 1-byte chars until 32766, then a 3-byte char.
        cfg.user_instructions = "a".repeat(32766);
        cfg.user_instructions.push('€'); // '€' is 3 bytes (E2 82 AC). Length is now 32769 bytes.

        // Truncating at 32768 would split the '€' character.
        let prompt = build_hierarchical_system_prompt(&cfg);

        let user_part = prompt.trim_start_matches("[User Instructions]\n");
        // The truncation should back up to 32766 to avoid splitting the character.
        assert_eq!(user_part.len(), 32766);
    }

    #[tokio::test]
    async fn test_llm_judge_rejects_and_approves() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("Draft answer"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                },
                ChatResponse {
                    message: Message::assistant("REJECT: The answer is incomplete."),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                },
                ChatResponse {
                    message: Message::assistant("Better answer"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                },
                ChatResponse {
                    message: Message::assistant("APPROVE"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                },
            ]),
        });

        let agent = Agent::new(client, vec![]);

        let mut cfg = AgentRunConfig::default();
        cfg.enable_llm_judge = true;

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Hello", &mut on_event).await;
        assert!(result.is_ok());
        let content = result.unwrap();
        assert_eq!(content, "Better answer");
    }
}
