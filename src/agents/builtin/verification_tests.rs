use crate::agent::{Agent, AgentRunConfig, AgentEvent};
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage, ToolCall};
use std::sync::Arc;

struct MockVerificationClient {
    responses: tokio::sync::Mutex<Vec<String>>,
}

#[async_trait::async_trait]
impl crate::llm::LlmClient for MockVerificationClient {
    async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut resps = self.responses.lock().await;
        let content = if !resps.is_empty() {
            resps.remove(0)
        } else {
            "default".to_string()
        };

        Ok(ChatResponse {
            message: Message::assistant(content),
            usage: Usage::default(),
            stop_reason: "stop".to_string(),
            response_id: Some("mock-id".to_string()),
        })
    }
}

struct DummyToolClient {
    responses: tokio::sync::Mutex<Vec<ChatResponse>>,
}

#[async_trait::async_trait]
impl crate::llm::LlmClient for DummyToolClient {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let has_reject_msg = req.messages.iter().any(|m| m.role == Role::User && (m.content.contains("REJECT") || m.content.contains("Visual verification rejected") || m.content.contains("Visual verification failed")));
        if has_reject_msg {
             return Ok(ChatResponse {
                message: Message::assistant("Fixed"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id2".into()),
            });
        }

        let mut resps = self.responses.lock().await;
        if !resps.is_empty() {
            Ok(resps.remove(0))
        } else {
            Ok(ChatResponse {
                message: Message::assistant("Fixed"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }
}

struct DummyToolExecutor;
#[async_trait::async_trait]
impl ohc_builtin_agent_tools::ToolExecutor for DummyToolExecutor {
    async fn execute(&self, _args: serde_json::Value) -> Result<String, ohc_builtin_agent_core::types::ToolError> {
        Ok("success".to_string())
    }
}


#[tokio::test]
async fn test_visual_verification_reject() {
    let mock_client = Arc::new(DummyToolClient {
        responses: tokio::sync::Mutex::new(vec![
            ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: "Generated output".to_string(),
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: Some("id1".into()),
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id1".into()),
            },
        ]),
    });

    let agent = Agent::new(mock_client, vec![ohc_builtin_agent_tools::Tool {
        name: "test_tool".to_string(),
        description: "dummy".to_string(),
        is_read_only: false,
        parameters: serde_json::json!({}),
        execute: Arc::new(DummyToolExecutor),
    }]);

    let mut cfg = AgentRunConfig::default();
    cfg.enable_visual_verification = true;
    cfg.visual_verification_command = "echo 'REJECT: visual bug'".to_string();
    cfg.max_iterations = 2; // Prevent 100 iteration timeout

    let mut events = vec![];
    let result = agent.run(&cfg, "Build UI", &mut |e| events.push(e)).await;

    // Test should assert successful error recovery (`Ok`) or specific event interception
    match result {
        Ok(out) => assert!(out.contains("Fixed"), "Expected 'Fixed', got: {}", out),
        Err(e) => {
            let caught_error = events.iter().any(|e| match e {
                AgentEvent::TextChunk { content } => content.contains("REJECT: visual bug") || content.contains("Visual verification rejected") || content.contains("Fixed"),
                AgentEvent::TaskError { error } => error.contains("REJECT: visual bug") || error.contains("Visual verification rejected"),
                _ => false,
            });
            assert!(caught_error, "Agent loop failed and did NOT intercept visual rejection: {}", e);
        }
    }
}

#[tokio::test]
async fn test_llm_judge_reject() {
    let mock_client = Arc::new(MockVerificationClient {
        responses: tokio::sync::Mutex::new(vec![
            "Initial attempt".to_string(), // Main loop generates this
            "REJECT: missing feature X".to_string(), // Judge rejects it
            "Fixed attempt".to_string(), // Main loop fixed it
            "APPROVE".to_string(), // Judge approves it
        ]),
    });

    let agent = Agent::new(mock_client, vec![]);
    let mut cfg = AgentRunConfig::default();
    cfg.enable_llm_judge = true;

    let mut events = vec![];
    let result = agent.run(&cfg, "Build logic", &mut |e| events.push(e)).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Fixed attempt");
}

struct ComputationalGuideClient {
    responses: tokio::sync::Mutex<Vec<ChatResponse>>,
}

#[async_trait::async_trait]
impl crate::llm::LlmClient for ComputationalGuideClient {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let has_reject_msg = req.messages.iter().any(|m| m.role == Role::User && m.content.contains("Computational guide verification failed"));
        if has_reject_msg {
             return Ok(ChatResponse {
                message: Message::assistant("Fixed guide issue"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id2".into()),
            });
        }

        let mut resps = self.responses.lock().await;
        if !resps.is_empty() {
            Ok(resps.remove(0))
        } else {
            Ok(ChatResponse {
                message: Message::assistant("Fixed guide issue"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }
}

#[tokio::test]
async fn test_computational_guide_reject() {
    let mock_client = Arc::new(ComputationalGuideClient {
        responses: tokio::sync::Mutex::new(vec![
            ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: "Generated output".to_string(),
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: Some("id1".into()),
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id1".into()),
            }
        ]),
    });

    let agent = Agent::new(mock_client, vec![ohc_builtin_agent_tools::Tool {
        name: "test_tool".to_string(),
        description: "dummy".to_string(),
        is_read_only: false,
        parameters: serde_json::json!({}),
        execute: Arc::new(DummyToolExecutor),
    }]);

    let mut cfg = AgentRunConfig::default();
    cfg.enable_computational_guides = true;
    cfg.computational_guide_command = "exit 1".to_string();
    cfg.max_iterations = 2; // Prevent 100 iteration timeout

    let mut events = vec![];
    let result = agent.run(&cfg, "Build script", &mut |e| events.push(e)).await;

    match result {
        Ok(out) => assert!(out.contains("Fixed guide issue"), "Expected 'Fixed guide issue', got: {}", out),
        Err(e) => {
            let caught_error = events.iter().any(|e| match e {
                AgentEvent::TaskError { error } => error.contains("Computational guide verification failed"),
                AgentEvent::TextChunk { content } => content.contains("Computational guide verification failed") || content.contains("Fixed guide issue"),
                _ => false,
            });
            assert!(caught_error, "Agent loop failed and did NOT intercept computational guide rejection: {}", e);
        }
    }
}
