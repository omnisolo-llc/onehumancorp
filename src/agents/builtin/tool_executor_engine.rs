use crate::agent::AgentRunConfig;
use crate::tools_gating::ToolGater;
use ohc_builtin_agent_core::types::{ToolCall, ToolError};
use crate::retry::{RetryStrategy, ExponentialBackoffWithJitter};
use ohc_builtin_agent_tools::Tool;
/// Master Catalog B.8. Error Handling (Compounding Error Prevention): Stripe limits retries to exactly 2. LangGraph Mechanic (4-types): 1) Transient (retry with backoff), 2) LLM-recoverable (return the raw error as a ToolMessage directly to the model so it can self-correct), 3) User-fixable (interrupt execution and ask user for input), 4) Unexpected (bubble up to debug).
use tracing::{error, info, warn};

pub struct ToolExecutionEngine;

impl ToolExecutionEngine {
    /// Executes a single tool using the LangGraph 4-tier Error Handling Mechanic (Compounding Error Prevention).
    #[tracing::instrument(skip(tool, tc), fields(tool_name = %tc.name, tool_call_id = %tc.id))]
    pub async fn execute_tool_with_langgraph_mechanics(
        tool: &Tool,
        tc: &ToolCall,
        max_retries: usize,
        cfg: &AgentRunConfig,
    ) -> Result<String, ToolError> {
        // Enforce Anthropic 3-stage tool gating before execution
        ToolGater::check_gating(tc, tool.is_read_only, cfg)?;
        // SOTA Harness Patterns (2025-2026): Error Handling
        let max_retries = std::cmp::min(max_retries, 2); // Stripe limits retries to exactly 2
        let mut retry_count = 0;
        let retry_strategy = ExponentialBackoffWithJitter::default();

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
                        let backoff = retry_strategy.next_backoff(retry_count);
                        retry_count += 1;
                        warn!(
                            "Transient error executing '{}', retrying {}/{} after {}ms... Error details: {}",
                            tool.name,
                            retry_count,
                            max_retries,
                            backoff.as_millis(),
                            msg
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
                    let formatted_msg = msg; // Do not format twice. Let agent.rs call `new_llm_recoverable`.
                    return Err(ToolError::LlmRecoverable(formatted_msg));
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
    use ohc_builtin_agent_core::types::{ToolCall, ToolError};
    use ohc_builtin_agent_tools::{Tool, ToolExecutor};
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use serde_json::json;

    struct MockToolExecutor {
        results: Mutex<Vec<Result<String, ToolError>>>,
    }

    impl MockToolExecutor {
        fn new(results: Vec<Result<String, ToolError>>) -> Self {
            Self {
                results: Mutex::new(results),
            }
        }
    }

    #[async_trait::async_trait]
    impl ToolExecutor for MockToolExecutor {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
            let mut results = self.results.lock().await;
            if !results.is_empty() {
                results.remove(0)
            } else {
                Err(ToolError::Unexpected("No more mocked results".to_string()))
            }
        }
    }

    fn create_tool(executor: MockToolExecutor) -> Tool {
        Tool {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            parameters: json!({}),
            is_read_only: true,
            execute: Arc::new(executor),
        }
    }

    fn create_tool_call() -> ToolCall {
        ToolCall {
            id: "call_123".to_string(),
            name: "test_tool".to_string(),
            arguments: json!({}),
        }
    }

    #[tokio::test]
    async fn test_execute_tool_success() {
        let executor = MockToolExecutor::new(vec![Ok("success".to_string())]);
        let tool = create_tool(executor);
        let tc = create_tool_call();
        let cfg = AgentRunConfig::default();

        let result = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[tokio::test]
    async fn test_execute_tool_transient_retry_success() {
        let executor = MockToolExecutor::new(vec![
            Err(ToolError::Transient("timeout".to_string())),
            Ok("success after retry".to_string()),
        ]);
        let tool = create_tool(executor);
        let tc = create_tool_call();
        let cfg = AgentRunConfig::default();

        let result = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success after retry");
    }

    #[tokio::test]
    async fn test_execute_tool_transient_exhausted() {
        let executor = MockToolExecutor::new(vec![
            Err(ToolError::Transient("timeout 1".to_string())),
            Err(ToolError::Transient("timeout 2".to_string())),
            Err(ToolError::Transient("timeout 3".to_string())),
        ]);
        let tool = create_tool(executor);
        let tc = create_tool_call();
        let cfg = AgentRunConfig::default();

        let result = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::Unexpected(msg) => assert!(msg.contains("Transient error after retries")),
            _ => panic!("Expected Unexpected error after retries exhausted"),
        }
    }

    #[tokio::test]
    async fn test_execute_tool_llm_recoverable() {
        let executor = MockToolExecutor::new(vec![
            Err(ToolError::LlmRecoverable("bad schema".to_string())),
        ]);
        let tool = create_tool(executor);
        let tc = create_tool_call();
        let cfg = AgentRunConfig::default();

        let result = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::LlmRecoverable(msg) => {
                // We no longer format inside execute_tool_with_langgraph_mechanics
                assert!(msg.contains("bad schema"));
            }
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_execute_tool_user_fixable() {
        let executor = MockToolExecutor::new(vec![
            Err(ToolError::UserFixable("needs human".to_string())),
        ]);
        let tool = create_tool(executor);
        let tc = create_tool_call();
        let cfg = AgentRunConfig::default();

        let result = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::UserFixable(msg) => assert_eq!(msg, "needs human"),
            _ => panic!("Expected UserFixable error"),
        }
    }

    #[tokio::test]
    async fn test_execute_tool_fatal() {
        let executor = MockToolExecutor::new(vec![
            Err(ToolError::Fatal("system crash".to_string())),
        ]);
        let tool = create_tool(executor);
        let tc = create_tool_call();
        let cfg = AgentRunConfig::default();

        let result = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::Fatal(msg) => assert_eq!(msg, "system crash"),
            _ => panic!("Expected Fatal error"),
        }
    }

    #[tokio::test]
    async fn test_execute_tool_unexpected() {
        let executor = MockToolExecutor::new(vec![
            Err(ToolError::Unexpected("unknown issue".to_string())),
        ]);
        let tool = create_tool(executor);
        let tc = create_tool_call();
        let cfg = AgentRunConfig::default();

        let result = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::Unexpected(msg) => assert_eq!(msg, "unknown issue"),
            _ => panic!("Expected Unexpected error"),
        }
    }

    #[tokio::test]
    async fn test_execute_tool_handoff() {
        let executor = MockToolExecutor::new(vec![
            Err(ToolError::HandoffRequested("other_agent".to_string())),
        ]);
        let tool = create_tool(executor);
        let tc = create_tool_call();
        let cfg = AgentRunConfig::default();

        let result = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::HandoffRequested(msg) => assert_eq!(msg, "other_agent"),
            _ => panic!("Expected HandoffRequested error"),
        }
    }
}
