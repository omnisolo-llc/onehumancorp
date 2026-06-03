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

        loop {
            match tool.execute.execute(tc.arguments.clone()).await {
                Ok(res) => return Ok(res),
                Err(ToolError::Transient(msg)) => {
                    // 1) Transient errors: orchestrator should retry with backoff.
                    if retry_count < max_retries {
                        retry_count += 1;
                        let base_backoff = 500 * (1 << retry_count);
                        let jitter = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().subsec_millis() as u64 % 100;
                        let backoff = Duration::from_millis((base_backoff as u64) + jitter);
                        warn!("Transient error executing '{}', retrying {}/{} after {}ms...", tool.name, retry_count, max_retries, backoff.as_millis());
                        sleep(backoff).await;
                        continue;
                    } else {
                        // After retries are exhausted, it becomes an Unexpected/Fatal error to the loop
                        return Err(ToolError::Unexpected(format!("Transient error after retries: {}", msg)));
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

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_tools::{Tool, ToolExecute};
    use async_trait::async_trait;
    use serde_json::Value;

    struct DummyExecutor {
        result: Result<String, ToolError>,
    }

    #[async_trait]
    impl ToolExecute for DummyExecutor {
        async fn execute(&self, _args: Value) -> Result<String, ToolError> {
            self.result.clone()
        }
    }

    #[tokio::test]
    async fn test_fatal_error() {
        let tool = Tool {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            parameters: serde_json::json!({}),
            execute: Box::new(DummyExecutor {
                result: Err(ToolError::Fatal("fatal error".to_string())),
            }),
        };

        let tc = ToolCall {
            id: "call_1".to_string(),
            name: "test_tool".to_string(),
            arguments: serde_json::json!({}),
        };

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2).await;
        assert!(matches!(res, Err(ToolError::Fatal(_))));
    }

    #[tokio::test]
    async fn test_transient_error_retry_success() {
        use std::sync::Arc;
        use tokio::sync::Mutex;

        struct RetryExecutor {
            call_count: Arc<Mutex<usize>>,
        }

        #[async_trait]
        impl ToolExecute for RetryExecutor {
            async fn execute(&self, _args: Value) -> Result<String, ToolError> {
                let mut count = self.call_count.lock().await;
                *count += 1;
                if *count <= 1 {
                    Err(ToolError::Transient("network issue".to_string()))
                } else {
                    Ok("success".to_string())
                }
            }
        }

        let call_count = Arc::new(Mutex::new(0));

        let tool = Tool {
            name: "retry_tool".to_string(),
            description: "A test tool".to_string(),
            parameters: serde_json::json!({}),
            execute: Box::new(RetryExecutor {
                call_count: call_count.clone(),
            }),
        };

        let tc = ToolCall {
            id: "call_2".to_string(),
            name: "retry_tool".to_string(),
            arguments: serde_json::json!({}),
        };

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2).await;
        assert_eq!(res.unwrap(), "success");

        let count = call_count.lock().await;
        assert_eq!(*count, 2);
    }

    #[tokio::test]
    async fn test_llm_recoverable_error() {
        let tool = Tool {
            name: "test_tool_recoverable".to_string(),
            description: "A test tool".to_string(),
            parameters: serde_json::json!({}),
            execute: Box::new(DummyExecutor {
                result: Err(ToolError::LlmRecoverable("missing parameter 'x'".to_string())),
            }),
        };

        let tc = ToolCall {
            id: "call_3".to_string(),
            name: "test_tool_recoverable".to_string(),
            arguments: serde_json::json!({}),
        };

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2).await;
        assert!(matches!(res, Err(ToolError::LlmRecoverable(_))));
    }
}
