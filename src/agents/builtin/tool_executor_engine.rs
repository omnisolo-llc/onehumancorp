use ohc_builtin_agent_core::types::{ToolCall, ToolError};
use ohc_builtin_agent_tools::Tool;
/// Master Catalog B.8. Error Handling (Compounding Error Prevention)
use tokio::time::{Duration, sleep};
use tracing::{error, info, warn};

pub struct ToolExecutionEngine;

impl ToolExecutionEngine {
    /// Executes a single tool using the LangGraph 4-tier Error Handling Mechanic (Compounding Error Prevention).
    #[tracing::instrument(skip(tool, tc), fields(tool_name = %tc.name, tool_call_id = %tc.id))]
    pub async fn execute_tool_with_langgraph_mechanics(
        tool: &Tool,
        tc: &ToolCall,
        max_retries: usize,
    ) -> Result<String, ToolError> {
        let max_retries = std::cmp::min(max_retries, 2); // Stripe limits retries to exactly 2
        let mut retry_count = 0;

        loop {
            match tool.execute.execute(tc.arguments.clone()).await {
                Ok(res) => {
                    info!("Tool execution successful");
                    return Ok(res);
                }
                Err(ToolError::Transient(msg)) => {
                    // 1) Transient errors: orchestrator should retry with backoff.
                    if retry_count < max_retries {
                        retry_count += 1;
                        let base_backoff = 500 * (1 << retry_count);
                        let jitter = 50;
                        let backoff = Duration::from_millis((base_backoff as u64) + jitter);
                        warn!(
                            "Transient error executing '{}', retrying {}/{} after {}ms...",
                            tool.name,
                            retry_count,
                            max_retries,
                            backoff.as_millis()
                        );
                        sleep(backoff).await;
                        continue;
                    } else {
                        error!("Transient error retries exhausted: {}", msg);
                        // After retries are exhausted, it becomes an Unexpected/Fatal error to the loop
                        return Err(ToolError::Unexpected(format!(
                            "Transient error after retries: {}",
                            msg
                        )));
                    }
                }
                Err(ToolError::LlmRecoverable(msg)) => {
                    // 2) LLM-recoverable: returned to the model so it can self-correct.
                    info!("LLM-recoverable error encountered: {}", msg);
                    return Err(ToolError::LlmRecoverable(msg));
                }
                Err(ToolError::UserFixable(msg)) => {
                    // 3) User-fixable: immediately bubble up to the orchestrator to request human-in-loop input.
                    warn!("User-fixable error encountered, bubbling up: {}", msg);
                    return Err(ToolError::UserFixable(msg));
                }
                Err(ToolError::Fatal(msg)) => {
                    // 4) Fatal: bubbles up to debug/halt immediately.
                    error!("Fatal tool error encountered: {}", msg);
                    return Err(ToolError::Fatal(msg));
                }
                Err(ToolError::Unexpected(msg)) => {
                    error!("Unexpected tool error encountered: {}", msg);
                    return Err(ToolError::Unexpected(msg));
                }
                Err(ToolError::HandoffRequested(msg)) => {
                    info!("Tool execution requested handoff to: {}", msg);
                    return Err(ToolError::HandoffRequested(msg));
                }
            }
        }
    }
}
