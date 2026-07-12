use ohc_builtin_agent_core::types::{ToolCall, ToolError};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use serde_json::json;

use crate::tool_executor_engine::ToolExecutionEngine;
use crate::agent::AgentRunConfig;
use ohc_builtin_agent_tools::{Tool, ToolExecutor};

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
async fn test_llm_recoverable_pydantic_error_routing() {
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

    let mut cfg = AgentRunConfig::default();
    cfg.project_trusted = true;

    let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool_fail, &tc, 2, &cfg).await;

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

    let mut cfg = AgentRunConfig::default();
    cfg.project_trusted = true;

    let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await;
    assert!(res.is_err());
    match res.expect_err("Expected error in test") {
        ToolError::LlmRecoverable(msg) => assert!(msg.contains("parse error")),
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

    let mut cfg = AgentRunConfig::default();
    cfg.project_trusted = true;

    let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await;
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

    let mut cfg = AgentRunConfig::default();
    cfg.project_trusted = true;

    let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await;
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

    let mut cfg = AgentRunConfig::default();
    cfg.project_trusted = true;

    let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await;
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

    let mut cfg = AgentRunConfig::default();
    cfg.project_trusted = true;

    let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await;
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
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;
        ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 5, &cfg).await
    });

    let res = handle.await.unwrap();

    assert!(res.is_err());
    match res.expect_err("Expected error in test") {
        ToolError::Unexpected(msg) => assert_eq!(msg, "Transient error after retries: transient error attempt 2"),
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
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;
        ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await
    });

    let res = handle.await.unwrap();

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "success");
    // Loop should run twice: first is error, second is success.
    assert_eq!(call_count.load(Ordering::SeqCst), 2);
}

#[tokio::test(start_paused = true)]
async fn test_transient_retry_fails_first_then_succeeds_custom() {
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
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;
        ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &cfg).await
    });
    let res = handle.await.unwrap();
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "success");
    assert_eq!(call_count.load(Ordering::SeqCst), 2);
}
