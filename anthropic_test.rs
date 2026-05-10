
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
