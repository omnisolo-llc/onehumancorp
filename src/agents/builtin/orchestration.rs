use std::sync::Arc;
use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use crate::budget::{check_token_budget, BudgetAction, BudgetTracker};
use ohc_builtin_agent_core::types::{ChatRequest, Message, Role, ToolCall, ToolResult, ToolError};

/// 1. The Orchestration Loop
/// Mechanic: "Termination conditions are layered: model returns text with no tool calls, max turn limit exceeded, token budget exhausted, guardrail tripwire fires, or safety refusal."
pub struct OrchestrationLoop<'a> {
    pub agent: &'a Agent,
    pub cfg: &'a AgentRunConfig,
}

impl<'a> OrchestrationLoop<'a> {
    pub fn new(agent: &'a Agent, cfg: &'a AgentRunConfig) -> Self {
        Self { agent, cfg }
    }

    /// Evaluates all 5 termination conditions.
    pub fn check_termination_conditions<F>(
        &self,
        turn_count: i32,
        stop_reason: &str,
        tool_calls: &[ToolCall],
        budget_tracker: &mut BudgetTracker,
        global_turn_tokens: i32,
        on_event: &mut F,
        last_assistant_content: &str,
    ) -> Result<Option<String>, String>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {
        // 1. Max turn limit exceeded
        if turn_count >= self.cfg.max_iterations {
            let err = format!("Terminal condition reached: max turn limit exceeded ({} iterations).", self.cfg.max_iterations);
            on_event(AgentEvent::TaskError { error: err.clone() });
            return Err(err);
        }

        // 2. Safety refusal
        if stop_reason == "content_filter" || stop_reason == "safety" {
            let err = "Terminal condition reached: Safety refusal. The model halted execution due to content safety policy.".to_string();
            on_event(AgentEvent::TaskError { error: err.clone() });
            return Err(err);
        }
        if stop_reason == "content_filter" || stop_reason == "safety" {
            let err = "Terminal condition reached: Safety refusal. The model halted execution due to content safety policy.".to_string();
            on_event(AgentEvent::TaskError { error: err.clone() });
            return Err(err);
        }

        // 3. Token budget exhausted
        if stop_reason == "max_tokens" || stop_reason == "length" {
            let decision = check_token_budget(
                budget_tracker,
                self.cfg.max_task_tokens,
                global_turn_tokens,
            );

            if decision.action == BudgetAction::Stop {
                let msg = "I've reached my token budget for this task. Please upgrade your plan to unlock longer interactions!".to_string();
                on_event(AgentEvent::TextChunk { content: msg.clone() });
                on_event(AgentEvent::TaskComplete { content: msg.clone() });
                return Ok(Some(msg));
            }
        }

        // 4. Model returns text with no tool calls
        if tool_calls.is_empty() {
            // Guardrail checks happen inside orchestrator before completion,
            // but we signal it's ready to complete.
            return Ok(Some(last_assistant_content.to_string()));
        }

        // 5. Guardrail tripwire fires
        // Handled during execution phase.

        Ok(None)
    }
}
