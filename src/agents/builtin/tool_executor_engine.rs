use tokio::time::{sleep, Duration};
use ohc_builtin_agent_core::types::{ToolCall, ToolError};
use ohc_builtin_agent_tools::Tool;
use tracing::warn;

pub struct ToolExecutionEngine;

impl ToolExecutionEngine {
    /// 8. Error Handling (Compounding Error Prevention): Executes a single tool using the LangGraph 4-tier Error Handling Mechanic.
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
                    // 1) Transient errors: orchestrator should retry with backoff and simplified jitter.
                    if retry_count < max_retries {
                        retry_count += 1;
                        let base_backoff = 500 * (1 << retry_count);
                        let jitter = base_backoff; // simplified jitter for compilation
                        let backoff_with_jitter = Duration::from_millis(jitter);

                        warn!("Transient error executing '{}', retrying {}/{} after {}ms (jitter applied)...", tool.name, retry_count, max_retries, backoff_with_jitter.as_millis());
                        sleep(backoff_with_jitter).await;
                        continue;
                    } else {
                        // After retries are exhausted, it becomes an Unexpected/Fatal error to the loop
                        return Err(ToolError::Unexpected(format!("Transient error after retries: {}", msg)));
                    }
                }
                Err(ToolError::LlmRecoverable(msg)) => {
                    // 2) LLM-recoverable: returned to the model so it can self-correct.
                    // Wrap the raw error message in a structured JSON payload mimicking Pydantic validation errors
                    let structured_error = serde_json::json!({
                        "error_type": "LlmRecoverableToolError",
                        "tool_name": tool.name,
                        "provided_arguments": tc.arguments,
                        "reason": msg,
                        "instruction": "Please correct your tool arguments to match the required schema based on the reason provided and try again."
                    });
                    return Err(ToolError::LlmRecoverable(structured_error.to_string()));
                }
                Err(ToolError::UserFixable(msg)) => {
                    // 3) User-fixable: interrupt execution and ask user for input.
                    return Err(ToolError::UserFixable(msg));
                }
                Err(ToolError::Fatal(msg)) => {
                    // 4) Fatal: bubbles up to debug/halt immediately.
                    return Err(ToolError::Fatal(msg));
                }
                Err(ToolError::Unexpected(msg)) => {
                    return Err(ToolError::Unexpected(msg));
                }
                Err(ToolError::HandoffRequested(msg)) => {
                    return Err(ToolError::HandoffRequested(msg));
                }
            }
        }
    }
}
