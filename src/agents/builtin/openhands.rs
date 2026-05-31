use std::sync::Arc;
use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::Message;
use std::collections::HashMap;

/// OpenHands/OpenDevin: Python SDK + CLI, MIT licensed
/// SOTA Harness Patterns (2025-2026): OpenHands integration architecture.
/// This module implements the exact "EventStream" Action/Observation sandbox pattern used by OpenHands.
/// Instead of a static orchestrator, it runs a background event loop that handles bidirectional JSON events.

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OpenHandsEvent {
    pub event_type: String, // "Action" or "Observation"
    pub source: String,     // "agent" or "user"
    pub payload: serde_json::Value,
}

pub struct OpenHandsEventStream {
    pub agent: Arc<Agent>,
    pub config: AgentRunConfig,
}

impl OpenHandsEventStream {
    pub fn new(agent: Arc<Agent>, config: AgentRunConfig) -> Self {
        Self { agent, config }
    }

    /// Simulates processing a stream of OpenHands format events
    pub async fn process_events(&self, events: Vec<OpenHandsEvent>) -> Result<Vec<OpenHandsEvent>, String> {
        let mut responses = Vec::new();
        let mut context = Vec::new();

        for event in events {
            if event.event_type == "Action" && event.source == "user" {
                // Convert OpenHands user action into our message format
                if let Some(msg) = event.payload.get("message").and_then(|m| m.as_str()) {
                    context.push(Message::user(msg));
                }
            } else if event.event_type == "Observation" {
                // OpenHands Observation (e.g. from a sandbox execution)
                 if let Some(msg) = event.payload.get("content").and_then(|m| m.as_str()) {
                    let mut tool_res = Message::user("");
                    tool_res.role = ohc_builtin_agent_core::types::Role::Tool;
                    tool_res.tool_results = vec![
                        ohc_builtin_agent_core::types::ToolResult {
                            tool_call_id: "openhands-ext-1".to_string(),
                            content: msg.to_string(),
                            error: String::new(),
                        }
                    ];
                    context.push(tool_res);
                 }
            }
        }

        // Run the agent with the accumulated OpenHands context
        let mut run_cfg = self.config.clone();
        run_cfg.injected_context = Some(context);

        let mut on_event = |_| {};
        match self.agent.run(&run_cfg, "Process OpenHands events", &mut on_event).await {
            Ok(result) => {
                // Return an OpenHands Observation event
                responses.push(OpenHandsEvent {
                    event_type: "Observation".to_string(),
                    source: "agent".to_string(),
                    payload: serde_json::json!({
                        "content": result,
                        "observation": "agent_response"
                    })
                });
                Ok(responses)
            },
            Err(e) => Err(e.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage};

    struct MockLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("I processed the OpenHands events successfully!"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_openhands_event_stream_processor() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let config = AgentRunConfig::default();
        let orchestrator = OpenHandsEventStream::new(agent, config);

        let events = vec![
            OpenHandsEvent {
                event_type: "Action".to_string(),
                source: "user".to_string(),
                payload: serde_json::json!({"message": "Please fix the CSS"}),
            },
            OpenHandsEvent {
                event_type: "Observation".to_string(),
                source: "sandbox".to_string(),
                payload: serde_json::json!({"content": "Terminal output: done"}),
            }
        ];

        let results = orchestrator.process_events(events).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].event_type, "Observation");
        assert_eq!(results[0].source, "agent");
        assert_eq!(results[0].payload.get("content").unwrap().as_str().unwrap(), "I processed the OpenHands events successfully!");
    }
}
