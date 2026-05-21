#[cfg(test)]
mod tests {
    use ohc_builtin_agent::agent::{Agent, AgentRunConfig, AgentEvent};
    use ohc_builtin_agent::types::{ChatRequest, ChatResponse, Message, Role, ToolCall, ToolResult, Usage, ToolError};
    use ohc_builtin_agent::llm::LlmClient;
    use ohc_builtin_agent::tools::{Tool, ToolExecutor};
    use std::sync::Arc;

    struct CompactionMockLlmClient {
        responses: tokio::sync::Mutex<Vec<ChatResponse>>,
        pub received_requests: tokio::sync::Mutex<Vec<ChatRequest>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for CompactionMockLlmClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            self.received_requests.lock().await.push(req);
            let mut resps = self.responses.lock().await;
            if !resps.is_empty() {
                Ok(resps.remove(0))
            } else {
                Ok(ChatResponse {
                    message: Message::assistant("Final answer"),
                    usage: Usage {
                        input_tokens: 50,
                        output_tokens: 10,
                        cache_creation_input_tokens: 0,
                        cache_read_input_tokens: 0,
                    },
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            }
        }
    }

    struct MockToolExecutor;
    #[async_trait::async_trait]
    impl ToolExecutor for MockToolExecutor {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
            Ok("Tool executed successfully".to_string())
        }
    }

    #[tokio::test]
    async fn test_context_compaction_activates_and_formats_correctly() {
        // We simulate a long history that should trigger compaction.
        // We set compaction_threshold_tokens = 10, so any prompt longer than 10 tokens will trigger it.
        // Note: Our turn_input_tokens comes from the mock response's usage, so we'll set it to 100 for the first response.

        let client = Arc::new(CompactionMockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "test_tool".to_string(),
                            arguments: serde_json::json!({}),
                        }],
                        tool_results: vec![],
                        response_id: Some("1".to_string()),
                        previous_response_id: None,
                     refusal: None, },
                    usage: Usage {
                        input_tokens: 100, // This will trigger compaction for the next iteration
                        output_tokens: 10,
                        cache_creation_input_tokens: 0,
                        cache_read_input_tokens: 0,
                    },
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("Compacted summary content"),
                    usage: Usage {
                        input_tokens: 50,
                        output_tokens: 10,
                        cache_creation_input_tokens: 0,
                        cache_read_input_tokens: 0,
                    },
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id2".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("Final answer after compaction"),
                    usage: Usage {
                        input_tokens: 50,
                        output_tokens: 10,
                        cache_creation_input_tokens: 0,
                        cache_read_input_tokens: 0,
                    },
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id3".to_string()),
                }
            ]),
            received_requests: tokio::sync::Mutex::new(Vec::new()),
        });

        let tool = Tool {
            name: "test_tool".to_string(),
            description: "test".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({"type": "object", "properties": {}}),
            execute: Arc::new(MockToolExecutor),
        };

        let agent = Agent::new(client.clone(), vec![tool]);
        let mut cfg = AgentRunConfig::default();
        cfg.enable_context_compaction = true;
        cfg.compaction_threshold_tokens = 10; // low threshold to force compaction

        // We inject a long history to satisfy `messages.len() > 5`
        cfg.injected_context = Some(vec![
            Message::user("Message 0 (System/Start)"),
            Message::assistant("Message 1"),
            Message::user("Message 2"),
            Message::assistant("Message 3"),
            Message::user("Message 4"),
            Message::assistant("Message 5"),
        ]);

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        // Test run
        let result = agent.run(&cfg, "Message 6 (Initial run message)", &mut on_event).await;
        assert!(result.is_ok());

        let requests = client.received_requests.lock().await;

        // We expect at least 3 requests:
        // 1. Initial request (gets tool_call response)
        // 2. Compaction request (triggered by the 100 input tokens from the first response and history > 5)
        // 3. Final request after compaction
        assert!(requests.len() >= 3, "Expected at least 3 requests, got {}", requests.len());

        // Check the compaction request (it should be the second one)
        let compaction_req = &requests[1];
        assert!(compaction_req.system.contains("expert context compactor for an AI agent"));
        assert!(compaction_req.system.contains("Preserve architectural decisions and unresolved bugs"));
        assert!(compaction_req.system.contains("discard redundant/raw tool outputs"));

        // Verify that the compaction text correctly formatted the tool results to discard raw output
        // The compaction prompt includes the middle text. Since our injected context didn't have tool results,
        // we won't see "Success (raw output discarded during compaction)" here, but we will in the next test.

        // Check the third request to see if the compaction message was injected
        let final_req = &requests[2];
        let compacted_message_exists = final_req.messages.iter().any(|m| m.content.contains("[Context Compacted by Harness]:"));
        assert!(compacted_message_exists, "The context should contain the compacted summary");
    }

    #[tokio::test]
    async fn test_context_compaction_discards_raw_outputs() {
        let client = Arc::new(CompactionMockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "test_tool".to_string(),
                            arguments: serde_json::json!({}),
                        }],
                        tool_results: vec![],
                        response_id: Some("1".to_string()),
                        previous_response_id: None,
                     refusal: None, },
                    usage: Usage {
                        input_tokens: 100, // Trigger compaction
                        output_tokens: 10,
                        cache_creation_input_tokens: 0,
                        cache_read_input_tokens: 0,
                    },
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("Summary"),
                    usage: Usage {
                        input_tokens: 50,
                        output_tokens: 10,
                        cache_creation_input_tokens: 0,
                        cache_read_input_tokens: 0,
                    },
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id-summary".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("Done"),
                    usage: Usage {
                        input_tokens: 50,
                        output_tokens: 10,
                        cache_creation_input_tokens: 0,
                        cache_read_input_tokens: 0,
                    },
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id-done".to_string()),
                }
            ]),
            received_requests: tokio::sync::Mutex::new(Vec::new()),
        });

        let tool = Tool {
            name: "test_tool".to_string(),
            description: "test".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({"type": "object", "properties": {}}),
            execute: Arc::new(MockToolExecutor),
        };

        let agent = Agent::new(client.clone(), vec![tool]);
        let mut cfg = AgentRunConfig::default();
        cfg.enable_context_compaction = true;
        cfg.compaction_threshold_tokens = 10;

        let mut msg_with_tool_result = Message::user("Tool return");
        msg_with_tool_result.role = Role::Tool;
        msg_with_tool_result.tool_results = vec![ToolResult {
            tool_call_id: "test_call_id".to_string(),
            content: "THIS IS RAW REDUNDANT OUTPUT THAT SHOULD BE DISCARDED".to_string(),
            error: "".to_string(),
        }];

        cfg.injected_context = Some(vec![
            Message::user("0"),
            Message::assistant("1"),
            Message::user("2"),
            msg_with_tool_result,
            Message::user("4"),
            Message::assistant("5"),
            Message::user("6"),
        ]);

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let _ = agent.run(&cfg, "7", &mut on_event).await;

        let requests = client.received_requests.lock().await;
        // Request 0 is initial prompt, request 1 is compaction prompt
        assert!(requests.len() >= 2);
        let compaction_req = &requests[1];

        let prompt = &compaction_req.messages[0].content;
        assert!(prompt.contains("Success (raw output discarded during compaction)"));
        assert!(!prompt.contains("THIS IS RAW REDUNDANT OUTPUT THAT SHOULD BE DISCARDED"));
    }

    #[tokio::test]
    async fn test_context_compaction_llm_failure_graceful() {
        struct FailingCompactionMockLlmClient {
            pub call_count: tokio::sync::Mutex<usize>,
        }

        #[async_trait::async_trait]
        impl LlmClient for FailingCompactionMockLlmClient {
            async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let mut count = self.call_count.lock().await;
                *count += 1;

                if req.system.contains("expert context compactor") {
                    return Err("LLM Compaction API Failure".into());
                }

                if *count == 1 {
                    Ok(ChatResponse {
                        message: Message {
                            role: Role::Assistant,
                            content: "".to_string(),
                            tool_calls: vec![ToolCall {
                                id: "call_1".to_string(),
                                name: "test_tool".to_string(),
                                arguments: serde_json::json!({}),
                            }],
                            tool_results: vec![],
                            response_id: Some("1".to_string()),
                            previous_response_id: None,
                         refusal: None, },
                        usage: Usage {
                            input_tokens: 100, // Trigger compaction
                            output_tokens: 10,
                            cache_creation_input_tokens: 0,
                            cache_read_input_tokens: 0,
                        },
                        stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                    })
                } else {
                    Ok(ChatResponse {
                        message: Message::assistant("Final answer"),
                        usage: Usage {
                            input_tokens: 50,
                            output_tokens: 10,
                            cache_creation_input_tokens: 0,
                            cache_read_input_tokens: 0,
                        },
                        stop_reason: "stop".to_string(),
                        response_id: Some("mock-id-done".to_string()),
                    })
                }
            }
        }

        let client = Arc::new(FailingCompactionMockLlmClient {
            call_count: tokio::sync::Mutex::new(0),
        });

        let tool = Tool {
            name: "test_tool".to_string(),
            description: "test".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({"type": "object", "properties": {}}),
            execute: Arc::new(MockToolExecutor),
        };

        let agent = Agent::new(client.clone(), vec![tool]);
        let mut cfg = AgentRunConfig::default();
        cfg.enable_context_compaction = true;
        cfg.compaction_threshold_tokens = 10;

        cfg.injected_context = Some(vec![
            Message::user("0"),
            Message::assistant("1"),
            Message::user("2"),
            Message::assistant("3"),
            Message::user("4"),
            Message::assistant("5"),
            Message::user("6"),
        ]);

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "7", &mut on_event).await;

        // It shouldn't crash, it should return the final answer
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Final answer");

        // We should have an event logging the failure
        let has_error_event = events.iter().any(|e| match e {
            AgentEvent::TaskError { error } => error.contains("Context compaction failed: LLM Compaction API Failure"),
            _ => false,
        });
        assert!(has_error_event, "Expected a TaskError event indicating compaction failed");
    }
}
