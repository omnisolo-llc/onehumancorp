use ohc_builtin_agent_core::types::{ToolCall, ToolError};
use ohc_builtin_agent_tools::Tool;
use tokio::time::Duration;
use tracing::{error, info, warn};

/// Master Catalog B.8. Error Handling (Compounding Error Prevention): Stripe limits retries to exactly 2.
/// LangGraph Mechanic (4-types): 1) Transient (retry with backoff), 2) LLM-recoverable (return the raw error as a ToolMessage directly to the model so it can self-correct), 3) User-fixable (interrupt execution and ask user for input), 4) Unexpected (bubble up to debug).
pub struct LangGraphErrorHandlingMechanic;

impl LangGraphErrorHandlingMechanic {
    /// Executes a single tool using the LangGraph 4-tier Error Handling Mechanic (Compounding Error Prevention).
    #[tracing::instrument(skip(tool, tc), fields(tool_name = %tc.name, tool_call_id = %tc.id))]
    pub async fn execute(
        tool: &Tool,
        tc: &ToolCall,
        max_retries: usize,
    ) -> Result<String, ToolError> {
        // SOTA Harness Patterns (2025-2026): Error Handling
        let max_retries = std::cmp::min(max_retries, 2); // Stripe limits retries to exactly 2
        let mut retry_count = 0;

        loop {
            // Enhanced telemetry to explicitly log the start of the LangGraph tool execution mechanic
            tracing::info!(
                tool_name = %tool.name,
                tool_id = %tc.id,
                "Executing tool using LangGraph mechanics (Attempt {}/{})",
                retry_count + 1,
                max_retries + 1
            );

            match tool.execute.execute(tc.arguments.clone()).await {
                Ok(res) => {
                    info!("Tool execution successful");
                    return Ok(res);
                }
                Err(ToolError::Transient(msg)) => {
                    // 1) Transient errors: orchestrator should retry with backoff.
                    if retry_count < max_retries {
                        let base_backoff = 500 * (1 << retry_count);
                        retry_count += 1;
                        use rand::Rng;
                        let jitter = rand::thread_rng().gen_range(0..100);
                        let backoff = Duration::from_millis((base_backoff as u64) + jitter);
                        warn!(
                            "Transient error executing '{}', retrying {}/{} after {}ms...",
                            tool.name,
                            retry_count,
                            max_retries,
                            backoff.as_millis()
                        );
                        tokio::time::sleep(backoff).await;
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
                    // E.g., when the schema fails validation (Pydantic-first approach).
                    warn!(
                        "LLM-recoverable error encountered in tool '{}' (Pydantic-first schema failure or similar): {}",
                        tool.name, msg
                    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_tools::ToolExecutor;
    use serde_json::json;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct DummyToolExecutor {
        result: Result<String, ToolError>,
    }

    #[async_trait::async_trait]
    impl ToolExecutor for DummyToolExecutor {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
            self.result.clone()
        }
    }

    struct TransientRetryExecutor {
        call_count: Arc<AtomicUsize>,
        fail_until: usize,
    }

    #[async_trait::async_trait]
    impl ToolExecutor for TransientRetryExecutor {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
            let count = self.call_count.fetch_add(1, Ordering::SeqCst);
            if count < self.fail_until {
                Err(ToolError::Transient(format!("transient error attempt {}", count + 1)))
            } else {
                Ok("success".to_string())
            }
        }
    }

    #[tokio::test]
    async fn test_success() {
        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(DummyToolExecutor {
                result: Ok("success".to_string()),
            }),
        };

        let tc = ToolCall {
            id: "1".to_string(),
            name: "dummy".to_string(),
            arguments: json!({}),
        };

        let res = LangGraphErrorHandlingMechanic::execute(&tool, &tc, 2).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "success");
    }

    #[tokio::test]
    async fn test_llm_recoverable_pydantic_integration_loop() {
        let tool_fail = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(DummyToolExecutor {
                result: Err(ToolError::LlmRecoverable("Validation Error (Pydantic-first tool schema): Failed to parse arguments".to_string())),
            }),
        };

        let tc = ToolCall {
            id: "1".to_string(),
            name: "dummy".to_string(),
            arguments: json!({}),
        };

        let res = LangGraphErrorHandlingMechanic::execute(&tool_fail, &tc, 2).await;

        assert!(res.is_err());
        match res.expect_err("Expected error in test") {
            ToolError::LlmRecoverable(msg) => {
                assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            },
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_llm_recoverable() {
        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(DummyToolExecutor {
                result: Err(ToolError::LlmRecoverable("parse error".to_string())),
            }),
        };

        let tc = ToolCall {
            id: "1".to_string(),
            name: "dummy".to_string(),
            arguments: json!({}),
        };

        let res = LangGraphErrorHandlingMechanic::execute(&tool, &tc, 2).await;
        assert!(res.is_err());
        match res.expect_err("Expected error in test") {
            ToolError::LlmRecoverable(msg) => assert_eq!(msg, "parse error"),
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_user_fixable() {
        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(DummyToolExecutor {
                result: Err(ToolError::UserFixable("ask user".to_string())),
            }),
        };

        let tc = ToolCall {
            id: "1".to_string(),
            name: "dummy".to_string(),
            arguments: json!({}),
        };

        let res = LangGraphErrorHandlingMechanic::execute(&tool, &tc, 2).await;
        assert!(res.is_err());
        match res.expect_err("Expected error in test") {
            ToolError::UserFixable(msg) => assert_eq!(msg, "ask user"),
            _ => panic!("Expected UserFixable error bubbled up"),
        }
    }

    #[tokio::test]
    async fn test_fatal() {
        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(DummyToolExecutor {
                result: Err(ToolError::Fatal("fatal error".to_string())),
            }),
        };

        let tc = ToolCall {
            id: "1".to_string(),
            name: "dummy".to_string(),
            arguments: json!({}),
        };

        let res = LangGraphErrorHandlingMechanic::execute(&tool, &tc, 2).await;
        assert!(res.is_err());
        match res.expect_err("Expected error in test") {
            ToolError::Fatal(msg) => assert_eq!(msg, "fatal error"),
            _ => panic!("Expected Fatal error"),
        }
    }

    #[tokio::test]
    async fn test_unexpected() {
        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(DummyToolExecutor {
                result: Err(ToolError::Unexpected("unexpected error".to_string())),
            }),
        };

        let tc = ToolCall {
            id: "1".to_string(),
            name: "dummy".to_string(),
            arguments: json!({}),
        };

        let res = LangGraphErrorHandlingMechanic::execute(&tool, &tc, 2).await;
        assert!(res.is_err());
        match res.expect_err("Expected error in test") {
            ToolError::Unexpected(msg) => assert_eq!(msg, "unexpected error"),
            _ => panic!("Expected Unexpected error"),
        }
    }

    #[tokio::test]
    async fn test_handoff_requested() {
        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(DummyToolExecutor {
                result: Err(ToolError::HandoffRequested("agent_2".to_string())),
            }),
        };

        let tc = ToolCall {
            id: "1".to_string(),
            name: "dummy".to_string(),
            arguments: json!({}),
        };

        let res = LangGraphErrorHandlingMechanic::execute(&tool, &tc, 2).await;
        assert!(res.is_err());
        match res.expect_err("Expected error in test") {
            ToolError::HandoffRequested(msg) => assert_eq!(msg, "agent_2"),
            _ => panic!("Expected HandoffRequested error"),
        }
    }

    #[tokio::test(start_paused = true)]
    async fn test_transient_retry_clamped_to_two() {
        let call_count = Arc::new(AtomicUsize::new(0));
        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(TransientRetryExecutor {
                call_count: call_count.clone(),
                fail_until: 10, // Keep failing
            }),
        };

        let tc = ToolCall {
            id: "1".to_string(),
            name: "dummy".to_string(),
            arguments: json!({}),
        };

        // Pass max_retries = 5, but it should be clamped to 2
        let handle = tokio::spawn(async move {
            LangGraphErrorHandlingMechanic::execute(&tool, &tc, 5).await
        });


        let res = handle.await.expect("Expected string in test");

        assert!(res.is_err());
        match res.expect_err("Expected error in test") {
            ToolError::Unexpected(msg) => assert_eq!(msg, "Transient error after retries: transient error attempt 3"),
            _ => panic!("Expected Unexpected error"),
        }
        // 1 initial + 2 clamped retries = 3 calls
        assert_eq!(call_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test(start_paused = true)]
    async fn test_transient_retry_fails_first_then_succeeds() {
        let call_count = Arc::new(AtomicUsize::new(0));
        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(TransientRetryExecutor {
                call_count: call_count.clone(),
                fail_until: 1, // Fails once, then succeeds
            }),
        };

        let tc = ToolCall {
            id: "1".to_string(),
            name: "dummy".to_string(),
            arguments: json!({}),
        };

        let handle = tokio::spawn(async move {
            LangGraphErrorHandlingMechanic::execute(&tool, &tc, 2).await
        });


        let res = handle.await.expect("Expected string in test");

        assert!(res.is_ok());
        assert_eq!(res.expect("Expected string in test"), "success");
        // Loop should run twice: first is error, second is success.
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }
}
