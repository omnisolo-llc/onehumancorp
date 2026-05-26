use crate::agent::{Agent, AgentRunConfig, AgentEvent};
use std::sync::Arc;
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};
use futures::StreamExt;

/// OpenHands/OpenDevin Unique Harness Innovations: Python SDK + CLI, MIT licensed
/// We'll implement an OpenHands-inspired Agent execution surface that exposes both a programmatic SDK-like
/// interface and an interactive CLI loop style interface, emphasizing an MIT-licensed open approach to execution.
/// A key OpenHands pattern is defining clear "Action" and "Observation" models at the API boundary.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "args")]
pub enum Action {
    RunCommand { command: String },
    WriteFile { path: String, content: String },
    Talk { message: String },
    Think { thought: String },
    Unknown { tool_name: String, args: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "observation", content = "content")]
pub enum Observation {
    CommandOutput { stdout: String, stderr: String, exit_code: i32 },
    FileWritten { success: bool },
    AgentMessage { message: String },
    Error { message: String },
}

pub struct OpenHandsAgent {
    pub inner: Arc<Agent>,
    pub run_config: AgentRunConfig,
}

impl OpenHandsAgent {
    pub fn new(inner: Arc<Agent>, run_config: AgentRunConfig) -> Self {
        Self { inner, run_config }
    }

    /// SDK programmatic execution returning a stream of mapped Actions and Observations
    pub fn run_task_stream(&self, task: &str) -> mpsc::UnboundedReceiver<Result<serde_json::Value, String>> {
        let mut cfg = self.run_config.clone();
        // Custom system prompt for OpenHands
        cfg.server_system_message = format!("You are an OpenHands agent. You perform Actions and observe Observations.\n\nTask: {}", task);

        let mut rx = self.inner.clone().query(cfg, task.to_string());

        let (tx, out_rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                match event {
                    AgentEvent::TextChunk { content } => {
                        let obs = Observation::AgentMessage { message: content };
                        let _ = tx.send(Ok(serde_json::to_value(obs).unwrap()));
                    }
                    AgentEvent::ToolCall { name, args_json, .. } => {
                        let action = match name.as_str() {
                            "Bash" => {
                                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&args_json) {
                                    if let Some(cmd) = v.get("command").and_then(|c| c.as_str()) {
                                        Action::RunCommand { command: cmd.to_string() }
                                    } else {
                                        Action::Unknown { tool_name: name, args: args_json }
                                    }
                                } else {
                                    Action::Unknown { tool_name: name, args: args_json }
                                }
                            },
                            "WriteFile" => {
                                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&args_json) {
                                    if let (Some(path), Some(content)) = (v.get("path").and_then(|p| p.as_str()), v.get("content").and_then(|c| c.as_str())) {
                                        Action::WriteFile { path: path.to_string(), content: content.to_string() }
                                    } else {
                                        Action::Unknown { tool_name: name, args: args_json }
                                    }
                                } else {
                                    Action::Unknown { tool_name: name, args: args_json }
                                }
                            },
                            _ => Action::Unknown { tool_name: name, args: args_json },
                        };
                        let _ = tx.send(Ok(serde_json::to_value(action).unwrap()));
                    }
                    AgentEvent::TaskError { error } => {
                        let obs = Observation::Error { message: error };
                        let _ = tx.send(Ok(serde_json::to_value(obs).unwrap()));
                    }
                    _ => {}
                }
            }
        });

        out_rx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage, ToolCall};
    use crate::llm::LlmClient;

    struct MockLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message {
                    role: ohc_builtin_agent_core::types::Role::Assistant,
                    content: "success".to_string(),
                    tool_calls: vec![
                        ToolCall {
                            id: "call_1".to_string(),
                            name: "Bash".to_string(),
                            arguments: serde_json::json!({
                                "command": "echo 'hello'"
                            }),
                        }
                    ],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_openhands_sdk() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![
            crate::tools::Tool {
                name: "Bash".to_string(),
                description: "Bash tool".to_string(),
                is_read_only: false,
                parameters: serde_json::json!({}),
                execute: crate::tools::bash::bash_tool(None, Arc::new(crate::tools::runner::SandboxedCommandRunner::new(None))).execute,
            }
        ]));
        let openhands_agent = OpenHandsAgent::new(agent, AgentRunConfig::default());

        let mut rx = openhands_agent.run_task_stream("test task");
        let mut events = vec![];
        while let Some(event) = rx.recv().await {
            events.push(event.unwrap());
        }

        assert!(!events.is_empty());
        let json_str = serde_json::to_string(&events).unwrap();
        assert!(json_str.contains("RunCommand"), "Output JSON should contain RunCommand action: {}", json_str);
    }
}
