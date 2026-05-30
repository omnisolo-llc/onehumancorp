mod tool_executor_engine;
use tool_executor_engine::ToolExecutionEngine;

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ToolCall, ToolError};
    use ohc_builtin_agent_tools::{Tool, ToolExecutor};
    use serde_json::json;
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

    #[tokio::test]
    async fn test_transient_retry() {
        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(DummyToolExecutor {
                result: Err(ToolError::Transient("transient error".to_string())),
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
            ToolError::Transient(msg) => assert_eq!(msg, "transient error"),
            _ => panic!("Expected Transient error"),
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
                result: Err(ToolError::HandoffRequested("target_agent".to_string())),
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
            ToolError::HandoffRequested(msg) => assert_eq!(msg, "target_agent"),
            _ => panic!("Expected HandoffRequested error"),
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

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "success");
    }
}
