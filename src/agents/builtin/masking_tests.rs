use ohc_builtin_agent::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent::tools::recall::recall_observation_tool;
use ohc_builtin_agent::types::{ChatRequest, ChatResponse, Message, Role, ToolCall, Usage};
use ohc_builtin_agent::llm::LlmClient;
use ohc_builtin_agent::tools::Tool;
use std::sync::Arc;
use tokio::sync::Mutex;
use dashmap::DashMap;

struct MockLlm {
    responses: Mutex<Vec<ChatResponse>>,
}

#[async_trait::async_trait]
impl LlmClient for MockLlm {
    async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut resps = self.responses.lock().await;
        if resps.is_empty() {
            Ok(ChatResponse {
                message: Message::assistant("Done"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id".to_string()),
            })
        } else {
            Ok(resps.remove(0))
        }
    }
}

struct SimpleTool;
#[async_trait::async_trait]
impl ohc_builtin_agent::tools::ToolExecutor for SimpleTool {
    async fn execute(&self, _args: serde_json::Value) -> Result<String, ohc_builtin_agent::types::ToolError> {
        Ok("This is a very long observation that should eventually be masked if it gets old enough and is large enough.".to_string())
    }
}

#[tokio::test]
async fn test_recency_aware_masking() {
    let observation_store = Arc::new(DashMap::new());
    let tool = Tool {
        name: "long_tool".to_string(),
        description: "returns long string".to_string(),
        is_read_only: true,
        parameters: serde_json::json!({}),
        execute: Arc::new(SimpleTool),
    };

    let client = Arc::new(MockLlm {
        responses: Mutex::new(vec![
            ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: "Calling tool".to_string(),
                    tool_calls: vec![ToolCall { id: "call_1".to_string(), name: "long_tool".to_string(), arguments: serde_json::Value::Null }],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                response_id: Some("id1".to_string()),
            },
            ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: "Turn 2".to_string(),
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id2".to_string()),
            },
        ]),
    });

    let mut agent = Agent::new(client, vec![tool]);
    agent.observation_store = observation_store.clone();

    let mut cfg = AgentRunConfig::default();
    cfg.enable_observation_masking = true;
    cfg.observation_masking_threshold = 1; // Mask very aggressively
    cfg.observation_masking_size_limit = 10; // Small limit

    let mut events = vec![];
    let res = agent.run(&cfg, "Start", &mut |e| events.push(e)).await;
    assert!(res.is_ok());

    assert!(observation_store.contains_key("call_1"));
    let full_content = observation_store.get("call_1").unwrap().clone();
    assert!(full_content.contains("very long observation"));
}

struct RecordingMockLlm {
    responses: Mutex<Vec<ChatResponse>>,
    requests: Mutex<Vec<ChatRequest>>,
}

#[async_trait::async_trait]
impl LlmClient for RecordingMockLlm {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.requests.lock().await.push(req);
        let mut resps = self.responses.lock().await;
        if resps.is_empty() {
            Ok(ChatResponse {
                message: Message::assistant("Done"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id".to_string()),
            })
        } else {
            Ok(resps.remove(0))
        }
    }
}

#[tokio::test]
async fn test_recall_observation_tool() {
    let observation_store = Arc::new(DashMap::new());
    observation_store.insert("secret_id".to_string(), "The lost city is at 42, 42".to_string());

    let tool = recall_observation_tool(observation_store.clone());
    let args = serde_json::json!({"tool_call_id": "secret_id"});

    let result = tool.execute.execute(args).await.unwrap();
    assert_eq!(result, "The lost city is at 42, 42");

    let args_bad = serde_json::json!({"tool_call_id": "wrong_id"});
    let result_bad = tool.execute.execute(args_bad).await;
    assert!(result_bad.is_err());
}

#[tokio::test]
async fn test_masking_logic_depth() {
    let observation_store = Arc::new(DashMap::new());
    let client = Arc::new(RecordingMockLlm {
        requests: Mutex::new(vec![]),
        responses: Mutex::new(vec![
            ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: "Call 1".to_string(),
                    tool_calls: vec![ToolCall { id: "c1".to_string(), name: "t".to_string(), arguments: serde_json::Value::Null }],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                response_id: Some("r1".to_string()),
            },
            ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: "Call 2".to_string(),
                    tool_calls: vec![ToolCall { id: "c2".to_string(), name: "t".to_string(), arguments: serde_json::Value::Null }],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                response_id: Some("r2".to_string()),
            },
            ChatResponse {
                message: Message::assistant("Final"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("r3".to_string()),
            },
        ]),
    });

    struct FixedTool;
    #[async_trait::async_trait]
    impl ohc_builtin_agent::tools::ToolExecutor for FixedTool {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ohc_builtin_agent::types::ToolError> {
            Ok("Long output content here".to_string())
        }
    }

    let mut agent = Agent::new(client.clone(), vec![Tool {
        name: "t".to_string(),
        description: "t".to_string(),
        is_read_only: true,
        parameters: serde_json::json!({}),
        execute: Arc::new(FixedTool),
    }]);
    agent.observation_store = observation_store;

    let mut cfg = AgentRunConfig::default();
    cfg.enable_observation_masking = true;
    cfg.observation_masking_threshold = 1;
    cfg.observation_masking_size_limit = 5;

    let _ = agent.run(&cfg, "Start", &mut |_| {}).await;

    let reqs = client.requests.lock().await;
    let turn3_msgs = &reqs[2].messages;
    let tr1 = &turn3_msgs[2];
    assert_eq!(tr1.role, Role::Tool);
    assert!(tr1.tool_results[0].content.contains("Observation Masked"), "Expected Result 1 to be masked in turn 3");

    let tr2 = &turn3_msgs[4];
    assert_eq!(tr2.role, Role::Tool);
    assert!(tr2.tool_results[0].content.contains("Long output content"), "Expected Result 2 NOT to be masked in turn 3");
}

#[tokio::test]
async fn test_masking_element_limit() {
    let observation_store = Arc::new(DashMap::new());
    let client = Arc::new(RecordingMockLlm {
        requests: Mutex::new(vec![]),
        responses: Mutex::new(vec![
            ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: "Call 1".to_string(),
                    tool_calls: vec![ToolCall { id: "c1".to_string(), name: "json_tool".to_string(), arguments: serde_json::Value::Null }],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                response_id: Some("r1".to_string()),
            },
            ChatResponse {
                message: Message::assistant("Final"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("r2".to_string()),
            },
        ]),
    });

    struct JsonTool;
    #[async_trait::async_trait]
    impl ohc_builtin_agent::tools::ToolExecutor for JsonTool {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ohc_builtin_agent::types::ToolError> {
            let large_array: Vec<usize> = (0..20).collect();
            Ok(serde_json::to_string(&large_array).unwrap())
        }
    }

    let mut agent = Agent::new(client.clone(), vec![Tool {
        name: "json_tool".to_string(),
        description: "json_tool".to_string(),
        is_read_only: true,
        parameters: serde_json::json!({}),
        execute: Arc::new(JsonTool),
    }]);
    agent.observation_store = observation_store;

    let mut cfg = AgentRunConfig::default();
    cfg.enable_observation_masking = true;
    cfg.observation_masking_threshold = 0; // Immediate masking
    cfg.observation_masking_size_limit = 10;
    cfg.observation_masking_element_limit = 5;

    let _ = agent.run(&cfg, "Start", &mut |_| {}).await;

    let reqs = client.requests.lock().await;
    let turn2_msgs = &reqs[1].messages;
    let tr1 = &turn2_msgs[2];
    assert_eq!(tr1.role, Role::Tool);

    let content = &tr1.tool_results[0].content;
    let parsed: serde_json::Value = serde_json::from_str(content).expect("Should be valid JSON");
    let arr = parsed.as_array().expect("Should be an array");
    assert_eq!(arr.len(), 6); // 5 elements + 1 summary
    let masked_element = arr.iter().find(|v| v.as_str().map_or(false, |s| s.contains("elements truncated") || s.contains("Masked string"))).map_or("", |v| v.as_str().unwrap());
    println!("MASKED ELEMENT: {}", masked_element);
    println!("ARRAY CONTENT: {:?}", arr);
    assert!(masked_element.contains("elements truncated") || masked_element.contains("Masked string"));
}
