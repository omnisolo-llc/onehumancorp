mod tool_executor_engine;
use tool_executor_engine::ToolExecutionEngine;

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ToolCall, ToolError};
    use ohc_builtin_agent_tools::{Tool, ToolExecutor};
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    struct DummyToolExecutor {
        error_type: String,
        error_message: String,
        call_count: Mutex<usize>,
    }

    #[async_trait::async_trait]
    impl ToolExecutor for DummyToolExecutor {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
            let mut count = self.call_count.lock().unwrap();
            *count += 1;

            match self.error_type.as_str() {
                "transient" => Err(ToolError::Transient(self.error_message.clone())),
                "llm_recoverable" => Err(ToolError::LlmRecoverable(self.error_message.clone())),
                "user_fixable" => Err(ToolError::UserFixable(self.error_message.clone())),
                "fatal" => Err(ToolError::Fatal(self.error_message.clone())),
                _ => Ok("success".to_string()),
            }
        }
    }

    #[tokio::test]
    async fn test_transient_retry() {
        let executor = Arc::new(DummyToolExecutor {
            error_type: "transient".to_string(),
            error_message: "transient error".to_string(),
            call_count: Mutex::new(0),
        });

        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: executor.clone(),
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

        // It should have retried up to max_retries (2 retries + 1 initial call = 3 calls)
        assert_eq!(*executor.call_count.lock().unwrap(), 3);
    }

    #[tokio::test]
    async fn test_llm_recoverable() {
        let executor = Arc::new(DummyToolExecutor {
            error_type: "llm_recoverable".to_string(),
            error_message: "parse error".to_string(),
            call_count: Mutex::new(0),
        });

        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: executor.clone(),
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

        // Should not retry
        assert_eq!(*executor.call_count.lock().unwrap(), 1);
    }

    #[tokio::test]
    async fn test_user_fixable() {
        let executor = Arc::new(DummyToolExecutor {
            error_type: "user_fixable".to_string(),
            error_message: "ask user".to_string(),
            call_count: Mutex::new(0),
        });

        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: executor.clone(),
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

        // Should not retry
        assert_eq!(*executor.call_count.lock().unwrap(), 1);
    }

    #[tokio::test]
    async fn test_fatal() {
        let executor = Arc::new(DummyToolExecutor {
            error_type: "fatal".to_string(),
            error_message: "fatal error".to_string(),
            call_count: Mutex::new(0),
        });

        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: executor.clone(),
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

        // Should not retry
        assert_eq!(*executor.call_count.lock().unwrap(), 1);
    }
}
