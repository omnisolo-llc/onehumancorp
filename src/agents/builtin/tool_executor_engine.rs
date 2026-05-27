use std::sync::Arc;
use tokio::time::{sleep, Duration};
use ohc_builtin_agent_core::types::{ToolCall, ToolError};
use crate::tools::Tool;
use tracing::warn;

pub struct ToolExecutionEngine;

impl ToolExecutionEngine {
    /// Executes a single tool using the LangGraph 4-tier Error Handling Mechanic.
    pub async fn execute_tool_with_langgraph_mechanics(
        tool: &Tool,
        tc: &ToolCall,
        max_retries: usize,
    ) -> Result<String, ToolError> {
        let max_retries = std::cmp::min(max_retries, 2); // Stripe limits retries to exactly 2
        let mut retry_count = 0;

        loop {
            match tool.execute.execute(tc.arguments.clone()).await {
                Ok(res) => return Ok(res),
                Err(ToolError::Transient(msg)) => {
                    // 1) Transient errors: orchestrator should retry with backoff.
                    if retry_count < max_retries {
                        retry_count += 1;
                        let backoff = Duration::from_millis(500 * (1 << retry_count));
                        warn!("Transient error executing '{}', retrying {}/{} after {}ms...", tool.name, retry_count, max_retries, backoff.as_millis());
                        sleep(backoff).await;
                        continue;
                    } else {
                        // After retries are exhausted, it becomes an Unexpected/Fatal error to the loop
                        return Err(ToolError::Transient(msg));
                    }
                }
                Err(ToolError::LlmRecoverable(msg)) => {
                    // 2) LLM-recoverable: returned to the model so it can self-correct.
                    return Err(ToolError::LlmRecoverable(msg));
                }
                Err(ToolError::UserFixable(msg)) => {
                    // 3) User-fixable: interrupt execution and ask user for input.
                    return Err(ToolError::UserFixable(msg));
                }

                Err(ToolError::Unexpected(msg)) => {
                    return Err(ToolError::Unexpected(msg));
                }

            }
        }
    }
}
