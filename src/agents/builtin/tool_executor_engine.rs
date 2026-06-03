
use tokio::time::{sleep, Duration};
use ohc_builtin_agent_core::types::{ToolCall, ToolError};
use ohc_builtin_agent_tools::Tool;
use tracing::warn;

pub struct ToolExecutionEngine;

impl ToolExecutionEngine {
    async fn prompt_user(msg: &str) -> String {
        // Read the mock input from env to allow automated tests to provide responses.
        if let Ok(mock_input) = std::env::var("OHC_MOCK_USER_INPUT") {
            return mock_input;
        }

        #[cfg(test)]
        {
            // Always abort in tests if no specific mock is provided to prevent blocking tests.
            // Also prefix unused variables to silence warnings.
            let _msg = msg;
            return "abort".to_string();
        }

        #[cfg(not(test))]
        {
            println!("\n[Agent Harness] USER INTERVENTION REQUIRED:");
            println!("{}", msg);
            print!("Please provide input to resolve this (or type 'abort' to cancel): ");
            tokio::task::spawn_blocking(|| {
                use std::io::{self, Write};
                let mut input = String::new();
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut input);
                input.trim().to_string()
            }).await.unwrap_or_else(|_| "abort".to_string())
        }
    }

    /// Executes a single tool using the LangGraph 4-tier Error Handling Mechanic (Compounding Error Prevention).
    pub async fn execute_tool_with_langgraph_mechanics(
        tool: &Tool,
        tc: &ToolCall,
        max_retries: usize,
    ) -> Result<String, ToolError> {
        let max_retries = std::cmp::min(max_retries, 2); // Stripe limits retries to exactly 2
        let mut retry_count = 0;
        let mut retry_history: Vec<String> = Vec::new();

        loop {
            match tool.execute.execute(tc.arguments.clone()).await {
                Ok(res) => {
                    if retry_count > 0 {
                        let history_str = retry_history.join(" | ");
                        return Ok(format!("[Note: Tool succeeded after {} transient retries. History: {}]\n{}", retry_count, history_str, res));
                    }
                    return Ok(res);
                }
                Err(ToolError::Transient(msg)) => {
                    // 1) Transient errors: orchestrator should retry with backoff.
                    retry_history.push(msg.clone());

                    if retry_count < max_retries {
                        retry_count += 1;
                        let base_backoff = 500 * (1 << retry_count);
                        let jitter = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().subsec_millis() as u64 % 100;
                        let backoff = Duration::from_millis((base_backoff as u64) + jitter);
                        warn!("Transient error executing '{}', retrying {}/{} after {}ms...", tool.name, retry_count, max_retries, backoff.as_millis());
                        sleep(backoff).await;
                        continue;
                    } else {
                        // After retries are exhausted, feed the context back to the LLM as LlmRecoverable
                        let history_str = retry_history.join(" | ");
                        return Err(ToolError::LlmRecoverable(format!(
                            "Transient error exhausted after {} retries. The system may be unstable. History: {}",
                            max_retries, history_str
                        )));
                    }
                }
                Err(ToolError::LlmRecoverable(msg)) => {
                    // 2) LLM-recoverable: returned to the model so it can self-correct.
                    return Err(ToolError::LlmRecoverable(msg));
                }
                Err(ToolError::UserFixable(msg)) => {
                    // 3) User-fixable: interrupt execution and ask user for input.
                    let input = Self::prompt_user(&msg).await;
                    if input.is_empty() || input.to_lowercase() == "abort" {
                        return Err(ToolError::UserFixable(format!("User aborted. Original error: {}", msg)));
                    } else {
                        return Ok(format!("User provided input to resolve the issue: {}", input));
                    }
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
