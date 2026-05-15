use super::*;
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage};

    struct StreamMockLlmClient {
        responses: tokio::sync::Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for StreamMockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            if !resps.is_empty() {
                Ok(resps.remove(0))
            } else {
                Ok(ChatResponse {
                    message: Message::assistant("default stream content"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            }
        }
    }

    #[tokio::test]
    async fn test_query_async_stream() {
        let client = Arc::new(StreamMockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("Streamed response chunk 1"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                }
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let mut rx = agent.query(cfg, "Start streaming".to_string());

        let mut events = vec![];
        while let Some(event) = rx.recv().await {
            events.push(event);
        }

        let has_task_complete = events.iter().any(|e| matches!(e, AgentEvent::TaskComplete { .. }));
        assert!(has_task_complete, "Stream should eventually emit TaskComplete event");
    }

    #[tokio::test]
    async fn test_time_travel_rewind_mechanic() {
        use ohc_builtin_agent_tools::ToolExecutor;
        use crate::checkpointer::{CheckpointSaver, Checkpoint};

        struct MockCheckpointerRewind {
            checkpoints: tokio::sync::Mutex<std::collections::HashMap<String, Checkpoint>>,
        }

        #[async_trait::async_trait]
        impl CheckpointSaver for MockCheckpointerRewind {
            async fn get_checkpoint(&self, _tid: &str, cid: &str) -> Result<Option<Checkpoint>, String> {
                Ok(self.checkpoints.lock().await.get(cid).cloned())
            }
            async fn put_checkpoint(&self, cp: Checkpoint) -> Result<(), String> {
                self.checkpoints.lock().await.insert(cp.checkpoint_id.clone(), cp);
                Ok(())
            }
            async fn list_checkpoints(&self, _tid: &str) -> Result<Vec<Checkpoint>, String> { Ok(vec![]) }
            async fn restore_checkpoint(&self, _cid: &str) -> Result<(), String> { Ok(()) }
        }

        struct RewindMockLlm {
            call_count: tokio::sync::Mutex<usize>,
        }

        #[async_trait::async_trait]
        impl LlmClient for RewindMockLlm {
            async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let mut count = self.call_count.lock().await;
                *count += 1;

                if *count == 1 {
                    // Turn 1: Normal tool call. This will create the first checkpoint.
                    Ok(ChatResponse {
                        message: Message {
                            role: Role::Assistant,
                            content: "Initial".to_string(),
                            tool_calls: vec![ToolCall { id: "c1".to_string(), name: "good_tool".to_string(), arguments: serde_json::Value::Null }],
                            tool_results: vec![],
                            response_id: Some("r1".to_string()),
                            previous_response_id: None,
                        },
                        usage: Usage::default(),
                        stop_reason: "tool_calls".to_string(),
                        response_id: Some("r1".to_string()),
                    })
                } else if *count == 2 {
                    // Turn 2: Call the failing tool.
                    Ok(ChatResponse {
                        message: Message {
                            role: Role::Assistant,
                            content: "Failing".to_string(),
                            tool_calls: vec![ToolCall { id: "c2".to_string(), name: "fail_tool".to_string(), arguments: serde_json::Value::Null }],
                            tool_results: vec![],
                            response_id: Some("r2".to_string()),
                            previous_response_id: Some("r1".to_string()),
                        },
                        usage: Usage::default(),
                        stop_reason: "tool_calls".to_string(),
                        response_id: Some("r2".to_string()),
                    })
                } else {
                    // After rewind, it should see the system nudge and hopefully finish.
                    // We check if the system nudge is present in the request.
                    let has_rewind_msg = req.messages.iter().any(|m| m.role == Role::System && m.content.contains("TIME-TRAVEL REWIND"));
                    if has_rewind_msg {
                         Ok(ChatResponse {
                            message: Message::assistant("Success after rewind"),
                            usage: Usage::default(),
                            stop_reason: "stop".to_string(),
                            response_id: Some("r3".to_string()),
                        })
                    } else {
                        // Keep failing until rewind happens
                        Ok(ChatResponse {
                            message: Message {
                                role: Role::Assistant,
                                content: "Failing again".to_string(),
                                tool_calls: vec![ToolCall { id: "c2".to_string(), name: "fail_tool".to_string(), arguments: serde_json::Value::Null }],
                                tool_results: vec![],
                                response_id: Some("r2".to_string()),
                                previous_response_id: Some("r1".to_string()),
                            },
                            usage: Usage::default(),
                            stop_reason: "tool_calls".to_string(),
                            response_id: Some("r2".to_string()),
                        })
                    }
                }
            }
        }

        struct FailTool;
        #[async_trait::async_trait]
        impl ToolExecutor for FailTool {
            async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
                Err(ToolError::LlmRecoverable("I always fail".to_string()))
            }
        }
        struct GoodTool;
        #[async_trait::async_trait]
        impl ToolExecutor for GoodTool {
            async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
                Ok("Success".to_string())
            }
        }

        let tools = vec![
            Tool { name: "fail_tool".to_string(), description: "fails".to_string(), is_read_only: false, parameters: serde_json::Value::Null, execute: Arc::new(FailTool) },
            Tool { name: "good_tool".to_string(), description: "works".to_string(), is_read_only: false, parameters: serde_json::Value::Null, execute: Arc::new(GoodTool) },
        ];

        let llm = Arc::new(RewindMockLlm { call_count: tokio::sync::Mutex::new(0) });
        let checkpointer = Arc::new(MockCheckpointerRewind { checkpoints: tokio::sync::Mutex::new(std::collections::HashMap::new()) });

        let agent = Agent::new(llm, tools).with_checkpointer(checkpointer);

        let mut cfg = AgentRunConfig::default();
        cfg.enable_time_travel_rewind = true;
        cfg.thread_id = Some("rewind-thread".to_string());
        cfg.max_rewind_attempts = 1;

        let mut events = vec![];
        let result = agent.run(&cfg, "Start", &mut |e| events.push(e)).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success after rewind");

        let rewind_emitted = events.iter().any(|e| matches!(e, AgentEvent::RewindOccurred { .. }));
        assert!(rewind_emitted, "RewindOccurred event should have been emitted");
    }

    struct DumbLoopMockClient;
    #[async_trait::async_trait]
    impl crate::llm::LlmClient for DumbLoopMockClient {
        async fn chat(&self, req: crate::types::ChatRequest) -> Result<crate::types::ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            if req.system.contains("Phase: Gather") {
                Ok(crate::types::ChatResponse {
                    message: crate::types::Message {
                        role: crate::types::Role::Assistant,
                        content: String::new(),
                        tool_calls: vec![crate::types::ToolCall {
                            id: "call_gather".to_string(),
                            name: "mock_read".to_string(),
                            arguments: serde_json::json!({}),
                        }],
                        tool_results: vec![],
                        response_id: None,
                        previous_response_id: None,
                    },
                    usage: crate::types::Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("id1".to_string()),
                })
            } else if req.system.contains("Phase: Act") {
                Ok(crate::types::ChatResponse {
                    message: crate::types::Message {
                        role: crate::types::Role::Assistant,
                        content: String::new(),
                        tool_calls: vec![crate::types::ToolCall {
                            id: "call_act".to_string(),
                            name: "mock_read".to_string(),
                            arguments: serde_json::json!({}),
                        }],
                        tool_results: vec![],
                        response_id: None,
                        previous_response_id: None,
                    },
                    usage: crate::types::Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("id2".to_string()),
                })
            } else {
                Ok(crate::types::ChatResponse {
                    message: crate::types::Message::assistant("Final verified result"),
                    usage: crate::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id3".to_string()),
                })
            }
        }
    }

    struct DumbLoopMockExecutor;
    #[async_trait::async_trait]
    impl ohc_builtin_agent_tools::ToolExecutor for DumbLoopMockExecutor {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, crate::types::ToolError> {
            Ok("read".to_string())
        }
    }

    #[tokio::test]
    async fn test_anthropic_dumb_loop() {
        let mock_tool = ohc_builtin_agent_tools::Tool {
            name: "mock_read".to_string(),
            description: "reads".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: std::sync::Arc::new(DumbLoopMockExecutor),
        };

        let client = std::sync::Arc::new(DumbLoopMockClient);
        let agent = crate::agent::Agent::new(client, vec![mock_tool]);
        let cfg = crate::agent::AgentRunConfig::default();

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run_anthropic_dumb_loop(&cfg, "Hello", &agent.tools, &mut on_event).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Final verified result");
    }
