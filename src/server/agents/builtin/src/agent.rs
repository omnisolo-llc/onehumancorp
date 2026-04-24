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
    pub confidence_threshold: f32,
    pub max_retained_observations: usize,
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
            confidence_threshold: 0.0,
            max_retained_observations: 5,
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

    pub fn mask_observations(messages: &[Message], max_retained: usize) -> Vec<Message> {
        let mut masked_messages = messages.to_vec();
        let tool_role_count = masked_messages.iter().filter(|m| m.role == Role::Tool).count();
        let mut tool_seen = 0;

        for m in &mut masked_messages {
            if m.role == Role::Tool {
                if tool_role_count > tool_seen && tool_role_count - tool_seen > max_retained {
                    for tr in &mut m.tool_results {
                        if !tr.content.is_empty() {
                            tr.content = "[Observation Masked to save context window. The tool call was successful.]".to_string();
                        }
                        if !tr.error.is_empty() {
                            tr.error = "[Error Observation Masked]".to_string();
                        }
                    }
                }
                tool_seen += 1;
            }
        }
        masked_messages
    }

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

            let masked_messages = Self::mask_observations(&messages, cfg.max_retained_observations);

            let req = ChatRequest {
                model: cfg.model.clone(),
                system: cfg.system.clone(),
                messages: masked_messages,
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
                // In a production-grade agent, we might use a separate LLM pass
                // to evaluate confidence in the final answer if threshold > 0.
                // For now, we'll assume the model is confident if it didn't use more tools.

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


#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ToolResult;

    #[test]
    fn test_mask_observations() {
        let mut messages = Vec::new();

        // 1. User message
        messages.push(Message::user("do the task"));

        // 2. Assistant uses tool A
        let mut m2 = Message::assistant("");
        m2.tool_calls.push(ToolCall { id: "1".into(), name: "tool_a".into(), arguments: Default::default() });
        messages.push(m2);

        // 3. Tool result A (oldest, should be masked if max_retained=1)
        messages.push(Message {
            role: Role::Tool,
            content: "".into(),
            tool_calls: vec![],
            tool_results: vec![ToolResult { tool_call_id: "1".into(), content: "result A".into(), error: "".into() }],
        });

        // 4. Assistant uses tool B
        let mut m4 = Message::assistant("");
        m4.tool_calls.push(ToolCall { id: "2".into(), name: "tool_b".into(), arguments: Default::default() });
        messages.push(m4);

        // 5. Tool result B (newest, should be retained if max_retained=1)
        messages.push(Message {
            role: Role::Tool,
            content: "".into(),
            tool_calls: vec![],
            tool_results: vec![ToolResult { tool_call_id: "2".into(), content: "result B".into(), error: "".into() }],
        });

        let masked = Agent::mask_observations(&messages, 1);

        assert_eq!(masked.len(), 5);
        assert_eq!(masked[2].tool_results[0].content, "[Observation Masked to save context window. The tool call was successful.]");
        assert_eq!(masked[4].tool_results[0].content, "result B");
    }

    #[test]
    fn test_mask_observations_error() {
        let mut messages = Vec::new();

        messages.push(Message {
            role: Role::Tool,
            content: "".into(),
            tool_calls: vec![],
            tool_results: vec![ToolResult { tool_call_id: "1".into(), content: "".into(), error: "error A".into() }],
        });
        messages.push(Message {
            role: Role::Tool,
            content: "".into(),
            tool_calls: vec![],
            tool_results: vec![ToolResult { tool_call_id: "2".into(), content: "".into(), error: "error B".into() }],
        });

        let masked = Agent::mask_observations(&messages, 1);

        assert_eq!(masked[0].tool_results[0].error, "[Error Observation Masked]");
        assert_eq!(masked[1].tool_results[0].error, "error B");
    }
}
