mod tool_executor_engine;
use tool_executor_engine::ToolExecutionEngine;

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ToolCall, ToolError};
    use ohc_builtin_agent_tools::{Tool, ToolExecutor};
    use serde_json::json;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

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
                Err(ToolError::Transient(format!("transient error attempt {}", count)))
            } else {
                Ok("success".to_string())
            }
        }
    }

    #[tokio::test]
    async fn test_transient_retry_success_eventually() {
        let call_count = Arc::new(AtomicUsize::new(0));
        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(TransientRetryExecutor {
                call_count: call_count.clone(),
                fail_until: 2, // Fails on 0 and 1, succeeds on 2
            }),
        };

        let tc = ToolCall {
            id: "1".to_string(),
            name: "dummy".to_string(),
            arguments: json!({}),
        };

        // We use mock time via tokio::time::pause to avoid waiting during the test
        tokio::time::pause();
        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2).await;

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "success");
        assert_eq!(call_count.load(Ordering::SeqCst), 3); // 2 failures + 1 success = 3 calls
        tokio::time::resume();
    }

    #[tokio::test]
    async fn test_transient_retry_exhausted() {
        let call_count = Arc::new(AtomicUsize::new(0));
        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(TransientRetryExecutor {
                call_count: call_count.clone(),
                fail_until: 5, // Keep failing
            }),
        };

        let tc = ToolCall {
            id: "1".to_string(),
            name: "dummy".to_string(),
            arguments: json!({}),
        };

        tokio::time::pause();
        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2).await;

        assert!(res.is_err());
        match res.unwrap_err() {
            ToolError::Transient(msg) => assert_eq!(msg, "transient error attempt 2"),
            _ => panic!("Expected Transient error"),
        }
        assert_eq!(call_count.load(Ordering::SeqCst), 3); // 1 initial + 2 retries = 3 calls
        tokio::time::resume();
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

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2).await;
        assert!(res.is_err());
        match res.unwrap_err() {
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

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2).await;
        assert!(res.is_err());
        match res.unwrap_err() {
            ToolError::UserFixable(msg) => assert_eq!(msg, "ask user"),
            _ => panic!("Expected UserFixable error"),
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

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2).await;
        assert!(res.is_err());
        match res.unwrap_err() {
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

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2).await;
        assert!(res.is_err());
        match res.unwrap_err() {
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

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2).await;
        assert!(res.is_err());
        match res.unwrap_err() {
            ToolError::HandoffRequested(msg) => assert_eq!(msg, "agent_2"),
            _ => panic!("Expected HandoffRequested error"),
        }
    }
}
