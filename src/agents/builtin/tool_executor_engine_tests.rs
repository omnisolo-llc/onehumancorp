



#[cfg(test)]
mod tests {

    use ohc_builtin_agent::tool_executor_engine::ToolExecutionEngine;
use ohc_builtin_agent::agent::AgentRunConfig;
    use ohc_builtin_agent_core::types::{ToolCall, ToolError};
    use ohc_builtin_agent_tools::Tool;
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
                Err(ToolError::Transient(format!("transient error attempt {}", count)))
            } else {
                Ok("success".to_string())
            }
        }
    }

    #[tokio::test(start_paused = true)]
    async fn test_transient_retry_jitter_calc() {
        let call_count = Arc::new(AtomicUsize::new(0));
        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(TransientRetryExecutor {
                call_count: call_count.clone(),
                fail_until: 1,
            }),
        };

        let tc = ToolCall {
            id: "1".to_string(),
            name: "dummy".to_string(),
            arguments: json!({}),
        };

        let handle = tokio::spawn(async move {
            ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &AgentRunConfig::default()).await
        });


        let res = handle.await.unwrap();

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "success");
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_transient_retry_immediate_success() {
        let call_count = Arc::new(AtomicUsize::new(0));
        let tool = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(TransientRetryExecutor {
                call_count: call_count.clone(),
                fail_until: 0, // Succeeds immediately
            }),
        };

        let tc = ToolCall {
            id: "1".to_string(),
            name: "dummy".to_string(),
            arguments: json!({}),
        };

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &AgentRunConfig::default()).await;

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "success");
        // The loop returns immediately, so no backoff occurs and count is exactly 1
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test(start_paused = true)]
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

        let handle = tokio::spawn(async move {
            ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &AgentRunConfig::default()).await
        });


        let res = handle.await.unwrap();

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "success");
        assert_eq!(call_count.load(Ordering::SeqCst), 3); // 2 failures + 1 success = 3 calls
    }

    #[tokio::test(start_paused = true)]
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

        let handle = tokio::spawn(async move {
            ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &AgentRunConfig::default()).await
        });


        let res = handle.await.unwrap();

        assert!(res.is_err());
        match res.expect_err("Expected error in test") {
            ToolError::Unexpected(msg) => assert_eq!(msg, "Transient error after retries: transient error attempt 2"),
            _ => panic!("Expected Unexpected error"),
        }
        assert_eq!(call_count.load(Ordering::SeqCst), 3); // 1 initial + 2 retries = 3 calls
    }

    #[tokio::test]
    async fn test_pydantic_to_engine_integration() {
        use ohc_builtin_agent_tools::pydantic::{PydanticAdapter, PydanticToolExecutor};
        use serde::Deserialize;

        #[derive(Deserialize)]
        struct MyTypedArgs {
            required_string: String,
            required_int: i32,
        }

        struct RealExecutor;

        #[async_trait::async_trait]
        impl PydanticToolExecutor<MyTypedArgs> for RealExecutor {
            async fn execute_typed(&self, args: MyTypedArgs) -> Result<String, ToolError> {
                Ok(format!("{}-{}", args.required_string, args.required_int))
            }
        }

        let pydantic_adapter = PydanticAdapter::new(RealExecutor);

        let tool = Tool {
            name: "real_tool".to_string(),
            description: "test tool".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(pydantic_adapter),
        };

        // Create a ToolCall with invalid arguments (missing `required_int`)
        let tc = ToolCall {
            id: "1".to_string(),
            name: "real_tool".to_string(),
            arguments: json!({ "required_string": "test" }),
        };

        // Execute via the engine
        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &AgentRunConfig::default()).await;

        assert!(res.is_err());
        match res.expect_err("Expected error in test") {
            ToolError::LlmRecoverable(msg) => {
                assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
                assert!(msg.contains("missing field `required_int`"));
            },
            _ => panic!("Expected LlmRecoverable error from Pydantic adapter"),
        }
    }

    #[tokio::test]
    async fn test_llm_recoverable_pydantic_retry() {
        let tool_fail = Tool {
            name: "dummy".to_string(),
            description: "dummy".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(DummyToolExecutor {
                result: Err(ToolError::LlmRecoverable("Validation Error (Pydantic-first tool schema): Failed to parse".to_string())),
            }),
        };

        let tc = ToolCall {
            id: "1".to_string(),
            name: "dummy".to_string(),
            arguments: json!({}),
        };

        // We simulate the pydantic loop which returns the recoverable error directly
        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool_fail, &tc, 2, &AgentRunConfig::default()).await;
        assert!(res.is_err());
        assert!(res.as_ref().expect_err("Expected error in test").to_string().contains("Validation Error (Pydantic-first tool schema)"));
        match res.expect_err("Expected error in test") {
            ToolError::LlmRecoverable(msg) => assert!(msg.contains("Validation Error (Pydantic-first tool schema)")),
            _ => panic!("Expected LlmRecoverable error"),
        }
    }

    #[tokio::test]
    async fn test_llm_recoverable_pydantic_integration_loop() {
        // This test simulates the orchestrator loop receiving an LlmRecoverable error and returning it to the LLM.
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

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool_fail, &tc, 2, &AgentRunConfig::default()).await;

        // Ensure the engine correctly bubbles up the exact recoverable error back to the orchestration loop
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

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &AgentRunConfig::default()).await;
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

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &AgentRunConfig::default()).await;
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

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &AgentRunConfig::default()).await;
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

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &AgentRunConfig::default()).await;
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

        let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &AgentRunConfig::default()).await;
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
            ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 5, &AgentRunConfig::default()).await
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
}

#[cfg(test)]
mod additional_transient_tests {
    use ohc_builtin_agent::tool_executor_engine::ToolExecutionEngine;
use ohc_builtin_agent::agent::AgentRunConfig;
    use ohc_builtin_agent_core::types::{ToolCall, ToolError};
    use ohc_builtin_agent_tools::Tool;
    use ohc_builtin_agent_tools::ToolExecutor;
    use serde_json::json;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

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
            ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &AgentRunConfig::default()).await
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
            ToolExecutionEngine::execute_tool_with_langgraph_mechanics(&tool, &tc, 2, &AgentRunConfig::default()).await
        });
        let res = handle.await.unwrap();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "success");
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_execute_tool_calls_with_concurrency_mechanics_ordering() {
        use std::sync::Mutex;

        struct TrackerExecutor {
            id: usize,
            is_read_only: bool,
            execution_log: Arc<Mutex<Vec<String>>>,
        }

        #[async_trait::async_trait]
        impl ToolExecutor for TrackerExecutor {
            async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
                if self.is_read_only {
                    {
                        let mut log = self.execution_log.lock().unwrap();
                        log.push(format!("RO_START_{}", self.id));
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                    {
                        let mut log = self.execution_log.lock().unwrap();
                        log.push(format!("RO_END_{}", self.id));
                    }
                } else {
                    {
                        let mut log = self.execution_log.lock().unwrap();
                        log.push(format!("MUT_START_{}", self.id));
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
                    {
                        let mut log = self.execution_log.lock().unwrap();
                        log.push(format!("MUT_END_{}", self.id));
                    }
                }
                Ok(format!("success_{}", self.id))
            }
        }

        let execution_log = Arc::new(Mutex::new(Vec::new()));

        let mut tools = Vec::new();
        let mut calls = Vec::new();

        // 0: Mutating
        tools.push(Tool {
            name: "tool_mut_0".to_string(),
            description: "".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(TrackerExecutor { id: 0, is_read_only: false, execution_log: execution_log.clone() }),
        });
        calls.push(ToolCall { id: "call_0".to_string(), name: "tool_mut_0".to_string(), arguments: json!({}) });

        // 1: Read-Only
        tools.push(Tool {
            name: "tool_ro_1".to_string(),
            description: "".to_string(),
            parameters: json!({}),
            is_read_only: true,
            execute: Arc::new(TrackerExecutor { id: 1, is_read_only: true, execution_log: execution_log.clone() }),
        });
        calls.push(ToolCall { id: "call_1".to_string(), name: "tool_ro_1".to_string(), arguments: json!({}) });

        // 2: Read-Only
        tools.push(Tool {
            name: "tool_ro_2".to_string(),
            description: "".to_string(),
            parameters: json!({}),
            is_read_only: true,
            execute: Arc::new(TrackerExecutor { id: 2, is_read_only: true, execution_log: execution_log.clone() }),
        });
        calls.push(ToolCall { id: "call_2".to_string(), name: "tool_ro_2".to_string(), arguments: json!({}) });

        // 3: Mutating
        tools.push(Tool {
            name: "tool_mut_3".to_string(),
            description: "".to_string(),
            parameters: json!({}),
            is_read_only: false,
            execute: Arc::new(TrackerExecutor { id: 3, is_read_only: false, execution_log: execution_log.clone() }),
        });
        calls.push(ToolCall { id: "call_3".to_string(), name: "tool_mut_3".to_string(), arguments: json!({}) });

        let cfg = AgentRunConfig::default();
        let results = ToolExecutionEngine::execute_tool_calls_with_concurrency_mechanics(&calls, &tools, 2, &cfg).await;

        assert_eq!(results.len(), 4);
        assert_eq!(results[0].as_ref().unwrap().content, "success_0");
        assert_eq!(results[1].as_ref().unwrap().content, "success_1");
        assert_eq!(results[2].as_ref().unwrap().content, "success_2");
        assert_eq!(results[3].as_ref().unwrap().content, "success_3");

        let log = execution_log.lock().unwrap().clone();

        let ro1_start = log.iter().position(|r| r == "RO_START_1").unwrap();
        let ro2_start = log.iter().position(|r| r == "RO_START_2").unwrap();
        let ro1_end = log.iter().position(|r| r == "RO_END_1").unwrap();
        let ro2_end = log.iter().position(|r| r == "RO_END_2").unwrap();

        assert!(ro1_start < ro1_end);
        assert!(ro2_start < ro2_end);
        assert!(ro2_start < ro1_end, "RO2 should start before RO1 ends");
        assert!(ro1_start < ro2_end, "RO1 should start before RO2 ends");

        let mut_0_start = log.iter().position(|r| r == "MUT_START_0").unwrap();
        let mut_0_end = log.iter().position(|r| r == "MUT_END_0").unwrap();
        let mut_3_start = log.iter().position(|r| r == "MUT_START_3").unwrap();
        let mut_3_end = log.iter().position(|r| r == "MUT_END_3").unwrap();

        assert_eq!(mut_0_end, mut_0_start + 1, "Mutating tool 0 should not overlap");
        assert_eq!(mut_3_end, mut_3_start + 1, "Mutating tool 3 should not overlap");
    }

}
