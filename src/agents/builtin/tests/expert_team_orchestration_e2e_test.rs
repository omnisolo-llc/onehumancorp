use ohc_builtin_agent::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent::types::{ChatRequest, ChatResponse, Message, Role, Usage, ToolCall};
use ohc_builtin_agent_llm::LlmClient;
use std::sync::Arc;
use tokio::sync::Mutex;

struct MockLlmClient {
    responses: Mutex<Vec<ChatResponse>>,
}

#[async_trait::async_trait]
impl LlmClient for MockLlmClient {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut resps = self.responses.lock().await;

        // Let's identify the request type based on the system prompt
        if req.system.contains("Project Director") {
            // This is the lead agent synthesizing the result
            return Ok(ChatResponse {
                message: Message::assistant("Final synthesis output with Chart: included. Word count is quite large... ".repeat(4000)), // Needs >20k words and chart
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id-final".to_string()),
            });
        }

        if req.system.contains("You are an expert in") {
             // This is a domain expert responding
             return Ok(ChatResponse {
                message: Message::assistant("Expert analysis output. Valid words... ".repeat(200)),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id-expert".to_string()),
             });
        }

        // Otherwise it's the main orchestrator agent
        // If this is the initial agent prompt, generate the tool call
        if req.messages.len() == 1 {
            let tc = ToolCall {
                id: "call_expert_team".to_string(),
                name: "expert_team_orchestration".to_string(),
                arguments: serde_json::json!({
                    "task": "E2E expert team task testing the full pattern"
                }),
            };

            return Ok(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: "Invoking expert team tool...".to_string(),
                    tool_calls: vec![tc],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                response_id: Some("mock-id-1".to_string()),
            });
        }

        // If we have responses ready, return them
        if !resps.is_empty() {
            return Ok(resps.remove(0));
        }

        Ok(ChatResponse {
            message: Message::assistant("Task Complete"), // main agent finishes
            usage: Usage::default(),
            stop_reason: "stop".to_string(),
            response_id: Some("mock-id-complete".to_string()),
        })
    }
}

#[tokio::test]
async fn test_expert_team_orchestration_e2e() {
    let client = Arc::new(MockLlmClient {
        responses: Mutex::new(vec![]),
    });

    let tools = ohc_builtin_agent::tools::all_tools(
        Some(client.clone()), // Provide the agent_llm so the tool gets registered
        None,
        None,
        Arc::new(tokio::sync::RwLock::new(ohc_builtin_agent::tools::task::TaskStore::default())),
        Arc::new(tokio::sync::RwLock::new(ohc_builtin_agent::tools::sendmessage::Mailbox::default())),
        None,
        None,
        Arc::new(dashmap::DashMap::new()),
    );

    let has_expert_tool = tools.iter().any(|t| t.name == "expert_team_orchestration");
    assert!(has_expert_tool, "expert_team_orchestration tool must be registered");

    let mut cfg = AgentRunConfig::default();
    cfg.max_iterations = 2; // initial call, tool response, final stop

    let agent = Agent::new(client, tools);

    let result = agent.run(&cfg, "E2E expert team task testing the full pattern", &mut |_| ()).await;

    assert!(result.is_ok(), "Agent execution failed: {:?}", result.err());
}
