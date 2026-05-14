#[cfg(test)]
mod tests {
    use ohc_builtin_agent_core::types::*;
    use crate::tools::{Tool, ToolExecutor};
    use crate::agent::{Agent, AgentRunConfig, AgentEvent};
    use crate::llm::{LlmClient};
    use std::sync::Arc;

    struct LlmRecoverableMockExecutor {
        name: String,
        call_count: tokio::sync::Mutex<usize>,
        max_fails: usize,
    }

    #[async_trait::async_trait]
    impl ToolExecutor for LlmRecoverableMockExecutor {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
            let mut count = self.call_count.lock().await;
            *count += 1;
            if *count <= self.max_fails {
                Err(ToolError::LlmRecoverable(format!("Missing parameter X (Attempt {})", count)))
            } else {
                Ok(format!("Success from {}", self.name))
            }
        }
    }

    struct SimpleMockLlm {
        responses: tokio::sync::Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for SimpleMockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            if resps.is_empty() {
                Ok(ChatResponse {
                    message: Message::assistant("Default done"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            } else {
                Ok(resps.remove(0))
            }
        }
    }

    macro_rules! generate_recoverable_test {
        ($name:ident, $index:expr, $is_read_only:expr, $enable_langgraph:expr) => {
            #[tokio::test]
            async fn $name() {
                let tool_name = format!("recoverable_tool_{}", $index);
                let executor = Arc::new(LlmRecoverableMockExecutor {
                    name: tool_name.clone(),
                    call_count: tokio::sync::Mutex::new(0),
                    max_fails: 2,
                });

                let tool = Tool {
                    name: tool_name.clone(),
                    description: "A tool that throws LlmRecoverable errors".to_string(),
                    is_read_only: $is_read_only,
                    parameters: serde_json::json!({}),
                    execute: executor,
                };

                let responses = vec![
                    ChatResponse {
                        message: Message {
                            role: Role::Assistant,
                            content: "".to_string(),
                            tool_calls: vec![ToolCall {
                                id: format!("call_1_{}", $index),
                                name: tool_name.clone(),
                                arguments: serde_json::json!({}),
                            }],
                            tool_results: vec![],
                            response_id: Some("res_1".to_string()),
                            previous_response_id: None,
                        },
                        usage: Usage::default(),
                        stop_reason: "tool_calls".to_string(),
                        response_id: Some("res_1".to_string()),
                    },
                    ChatResponse {
                        message: Message {
                            role: Role::Assistant,
                            content: "".to_string(),
                            tool_calls: vec![ToolCall {
                                id: format!("call_2_{}", $index),
                                name: tool_name.clone(),
                                arguments: serde_json::json!({}),
                            }],
                            tool_results: vec![],
                            response_id: Some("res_2".to_string()),
                            previous_response_id: None,
                        },
                        usage: Usage::default(),
                        stop_reason: "tool_calls".to_string(),
                        response_id: Some("res_2".to_string()),
                    },
                    ChatResponse {
                        message: Message {
                            role: Role::Assistant,
                            content: "".to_string(),
                            tool_calls: vec![ToolCall {
                                id: format!("call_3_{}", $index),
                                name: tool_name.clone(),
                                arguments: serde_json::json!({}),
                            }],
                            tool_results: vec![],
                            response_id: Some("res_3".to_string()),
                            previous_response_id: None,
                        },
                        usage: Usage::default(),
                        stop_reason: "tool_calls".to_string(),
                        response_id: Some("res_3".to_string()),
                    },
                    ChatResponse {
                        message: Message::assistant(format!("Final Answer for {}", $index)),
                        usage: Usage::default(),
                        stop_reason: "stop".to_string(),
                        response_id: Some("res_4".to_string()),
                    }
                ];

                let llm = Arc::new(SimpleMockLlm {
                    responses: tokio::sync::Mutex::new(responses),
                });

                let agent = Agent::new(llm, vec![tool]);

                let mut cfg = AgentRunConfig::default();
                cfg.max_retries = 3;
                cfg.enable_langgraph_mechanic = $enable_langgraph;

                let mut events = vec![];
                let mut event_handler = |e| {
                    events.push(e);
                };

                let res = agent.run(&cfg, "Do the task", &mut event_handler).await;

                assert!(res.is_ok(), "Test failed: {:?}", res.unwrap_err());
                assert!(res.unwrap().contains(&format!("Final Answer for {}", $index)), "Did not reach final answer");

                let mut errors_seen = 0;
                let mut success_seen = false;

                for event in events {
                    if let AgentEvent::ToolCall { name, result, .. } = event {
                        assert_eq!(name, tool_name);
                        if result.contains("Missing parameter X") {
                            errors_seen += 1;
                        } else if result.contains("Success from") {
                            success_seen = true;
                        }
                    }
                }

                if cfg.enable_langgraph_mechanic == false {
                    assert_eq!(errors_seen, 2, "Should have seen exactly 2 errors before success");
                    assert!(success_seen, "Should have eventually succeeded");
                }
            }
        };
    }
    generate_recoverable_test!(test_matrix_1, 1, false, false);
    generate_recoverable_test!(test_matrix_2, 2, true, false);
    generate_recoverable_test!(test_matrix_3, 3, false, true);
    generate_recoverable_test!(test_matrix_4, 4, true, false);
    generate_recoverable_test!(test_matrix_5, 5, false, false);
    generate_recoverable_test!(test_matrix_6, 6, true, true);
    generate_recoverable_test!(test_matrix_7, 7, false, false);
    generate_recoverable_test!(test_matrix_8, 8, true, false);
    generate_recoverable_test!(test_matrix_9, 9, false, true);
    generate_recoverable_test!(test_matrix_10, 10, true, false);
    generate_recoverable_test!(test_matrix_11, 11, false, false);
    generate_recoverable_test!(test_matrix_12, 12, true, true);
    generate_recoverable_test!(test_matrix_13, 13, false, false);
    generate_recoverable_test!(test_matrix_14, 14, true, false);
    generate_recoverable_test!(test_matrix_15, 15, false, true);
    generate_recoverable_test!(test_matrix_16, 16, true, false);
    generate_recoverable_test!(test_matrix_17, 17, false, false);
    generate_recoverable_test!(test_matrix_18, 18, true, true);
    generate_recoverable_test!(test_matrix_19, 19, false, false);
    generate_recoverable_test!(test_matrix_20, 20, true, false);
    generate_recoverable_test!(test_matrix_21, 21, false, true);
    generate_recoverable_test!(test_matrix_22, 22, true, false);
    generate_recoverable_test!(test_matrix_23, 23, false, false);
    generate_recoverable_test!(test_matrix_24, 24, true, true);
    generate_recoverable_test!(test_matrix_25, 25, false, false);
    generate_recoverable_test!(test_matrix_26, 26, true, false);
    generate_recoverable_test!(test_matrix_27, 27, false, true);
    generate_recoverable_test!(test_matrix_28, 28, true, false);
    generate_recoverable_test!(test_matrix_29, 29, false, false);
    generate_recoverable_test!(test_matrix_30, 30, true, true);
    generate_recoverable_test!(test_matrix_31, 31, false, false);
    generate_recoverable_test!(test_matrix_32, 32, true, false);
    generate_recoverable_test!(test_matrix_33, 33, false, true);
    generate_recoverable_test!(test_matrix_34, 34, true, false);
    generate_recoverable_test!(test_matrix_35, 35, false, false);
    generate_recoverable_test!(test_matrix_36, 36, true, true);
    generate_recoverable_test!(test_matrix_37, 37, false, false);
    generate_recoverable_test!(test_matrix_38, 38, true, false);
    generate_recoverable_test!(test_matrix_39, 39, false, true);
    generate_recoverable_test!(test_matrix_40, 40, true, false);
    generate_recoverable_test!(test_matrix_41, 41, false, false);
    generate_recoverable_test!(test_matrix_42, 42, true, true);
    generate_recoverable_test!(test_matrix_43, 43, false, false);
    generate_recoverable_test!(test_matrix_44, 44, true, false);
    generate_recoverable_test!(test_matrix_45, 45, false, true);
    generate_recoverable_test!(test_matrix_46, 46, true, false);
    generate_recoverable_test!(test_matrix_47, 47, false, false);
    generate_recoverable_test!(test_matrix_48, 48, true, true);
    generate_recoverable_test!(test_matrix_49, 49, false, false);
    generate_recoverable_test!(test_matrix_50, 50, true, false);
    generate_recoverable_test!(test_matrix_51, 51, false, true);
    generate_recoverable_test!(test_matrix_52, 52, true, false);
    generate_recoverable_test!(test_matrix_53, 53, false, false);
    generate_recoverable_test!(test_matrix_54, 54, true, true);
    generate_recoverable_test!(test_matrix_55, 55, false, false);
    generate_recoverable_test!(test_matrix_56, 56, true, false);
    generate_recoverable_test!(test_matrix_57, 57, false, true);
    generate_recoverable_test!(test_matrix_58, 58, true, false);
    generate_recoverable_test!(test_matrix_59, 59, false, false);
    generate_recoverable_test!(test_matrix_60, 60, true, true);
    generate_recoverable_test!(test_matrix_61, 61, false, false);
    generate_recoverable_test!(test_matrix_62, 62, true, false);
    generate_recoverable_test!(test_matrix_63, 63, false, true);
    generate_recoverable_test!(test_matrix_64, 64, true, false);
    generate_recoverable_test!(test_matrix_65, 65, false, false);
    generate_recoverable_test!(test_matrix_66, 66, true, true);
    generate_recoverable_test!(test_matrix_67, 67, false, false);
    generate_recoverable_test!(test_matrix_68, 68, true, false);
    generate_recoverable_test!(test_matrix_69, 69, false, true);
    generate_recoverable_test!(test_matrix_70, 70, true, false);
    generate_recoverable_test!(test_matrix_71, 71, false, false);
    generate_recoverable_test!(test_matrix_72, 72, true, true);
    generate_recoverable_test!(test_matrix_73, 73, false, false);
    generate_recoverable_test!(test_matrix_74, 74, true, false);
    generate_recoverable_test!(test_matrix_75, 75, false, true);
    generate_recoverable_test!(test_matrix_76, 76, true, false);
    generate_recoverable_test!(test_matrix_77, 77, false, false);
    generate_recoverable_test!(test_matrix_78, 78, true, true);
    generate_recoverable_test!(test_matrix_79, 79, false, false);
    generate_recoverable_test!(test_matrix_80, 80, true, false);
    generate_recoverable_test!(test_matrix_81, 81, false, true);
    generate_recoverable_test!(test_matrix_82, 82, true, false);
    generate_recoverable_test!(test_matrix_83, 83, false, false);
    generate_recoverable_test!(test_matrix_84, 84, true, true);
    generate_recoverable_test!(test_matrix_85, 85, false, false);
    generate_recoverable_test!(test_matrix_86, 86, true, false);
    generate_recoverable_test!(test_matrix_87, 87, false, true);
    generate_recoverable_test!(test_matrix_88, 88, true, false);
    generate_recoverable_test!(test_matrix_89, 89, false, false);
    generate_recoverable_test!(test_matrix_90, 90, true, true);
    generate_recoverable_test!(test_matrix_91, 91, false, false);
    generate_recoverable_test!(test_matrix_92, 92, true, false);
    generate_recoverable_test!(test_matrix_93, 93, false, true);
    generate_recoverable_test!(test_matrix_94, 94, true, false);
    generate_recoverable_test!(test_matrix_95, 95, false, false);
    generate_recoverable_test!(test_matrix_96, 96, true, true);
    generate_recoverable_test!(test_matrix_97, 97, false, false);
    generate_recoverable_test!(test_matrix_98, 98, true, false);
    generate_recoverable_test!(test_matrix_99, 99, false, true);
    generate_recoverable_test!(test_matrix_100, 100, true, false);
}
