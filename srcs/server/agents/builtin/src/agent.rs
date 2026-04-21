use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};

use crate::budget::{check_token_budget, BudgetAction, BudgetTracker};
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
    pub system: String,
    pub max_tokens: i32,
    pub temperature: f32,
    pub max_iterations: i32,
    pub max_task_tokens: i32, // budget for token tracking
}

impl Default for AgentRunConfig {
    fn default() -> Self {
        Self {
            model: String::new(),
            system: String::new(),
            max_tokens: 2048,
            temperature: 0.0,
            max_iterations: 100,
            max_task_tokens: 0,
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
    pub async fn run<F>(
        &self,
        cfg: &AgentRunConfig,
        initial_message: &str,
        on_event: &mut F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send,
    {
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

        for iteration in 0..max_iterations {
            on_event(AgentEvent::IterationStarted {
                iteration,
                message_count: messages.len(),
            });

            let req = ChatRequest {
                model: cfg.model.clone(),
                system: cfg.system.clone(),
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
                on_event(AgentEvent::TaskComplete {
                    content: last_assistant_content.clone(),
                });
                return Ok(last_assistant_content);
            }

            // Execute tool calls and collect results.
            let mut tool_results: Vec<ToolResult> = Vec::new();
            for tc in &tool_calls {
                let result = self.execute_tool(&tc).await;
                let (content, error) = match result {
                    Ok(r) => {
                        self.progress.record_tool_use();
                        on_event(AgentEvent::ToolCall {
                            name: tc.name.clone(),
                            args_json: tc.arguments.to_string(),
                            result: r.clone(),
                            iteration,
                        });
                        (r, String::new())
                    }
                    Err(e) => {
                        let err = e.to_string();
                        on_event(AgentEvent::ToolCall {
                            name: tc.name.clone(),
                            args_json: tc.arguments.to_string(),
                            result: format!("Error: {}", err),
                            iteration,
                        });
                        (String::new(), err)
                    }
                };
                tool_results.push(ToolResult {
                    tool_call_id: tc.id.clone(),
                    content,
                    error,
                });
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

    async fn execute_tool(
        &self,
        tc: &ToolCall,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let tool = self
            .tools
            .iter()
            .find(|t| t.name == tc.name)
            .ok_or_else(|| format!("unknown tool: {}", tc.name))?;

        tool.execute.execute(tc.arguments.clone()).await
    }
}
