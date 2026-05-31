use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};

use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::{Message, ToolCall};
use ohc_builtin_agent_tools::Tool;

/// SOTA Harness Patterns (2025-2026): 1. Actor-model message passing -> replacing classic ReAct loops
#[derive(Debug, Clone)]
pub enum ActorMessagePayload {
    UserTask(String),
    LlmResponse(String),
    ToolCallsRequest(Vec<ToolCall>),
    ToolResults(Vec<ohc_builtin_agent_core::types::ToolResult>),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct ActorMessage {
    pub sender: String,
    pub recipient: String,
    pub payload: ActorMessagePayload,
}

pub trait Actor: Send + Sync {
    fn name(&self) -> String;
    fn start(
        &self,
        receiver: mpsc::Receiver<ActorMessage>,
        system: Arc<ActorSystem>,
    ) -> tokio::task::JoinHandle<()>;
}

pub struct ActorSystem {
    mailboxes: Mutex<HashMap<String, mpsc::Sender<ActorMessage>>>,
}

impl ActorSystem {
    pub fn new() -> Self {
        Self {
            mailboxes: Mutex::new(HashMap::new()),
        }
    }

    pub async fn register(&self, name: String, sender: mpsc::Sender<ActorMessage>) {
        let mut mb = self.mailboxes.lock().await;
        mb.insert(name, sender);
    }

    pub async fn send(&self, msg: ActorMessage) -> Result<(), String> {
        let sender = {
            let mb = self.mailboxes.lock().await;
            mb.get(&msg.recipient).cloned()
        };

        if let Some(sender) = sender {
            sender
                .send(msg)
                .await
                .map_err(|e| format!("Failed to send message: {}", e))
        } else {
            Err(format!("Recipient {} not found", msg.recipient))
        }
    }
}

pub struct LlmActor {
    pub name: String,
    pub agent: Arc<Agent>,
    pub config: AgentRunConfig,
}

impl Actor for LlmActor {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn start(
        &self,
        mut receiver: mpsc::Receiver<ActorMessage>,
        system: Arc<ActorSystem>,
    ) -> tokio::task::JoinHandle<()> {
        let name = self.name.clone();
        let agent = self.agent.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            info!("LlmActor {} started", name);
            let mut messages = Vec::new();
            let mut initiator = String::new();

            while let Some(msg) = receiver.recv().await {
                debug!("LlmActor {} received message from {}", name, msg.sender);

                match msg.payload {
                    ActorMessagePayload::UserTask(task) => {
                        initiator = msg.sender.clone();
                        messages.push(Message::user(task));
                    }
                    ActorMessagePayload::ToolResults(results) => {
                        messages.push(Message {
                            role: ohc_builtin_agent_core::types::Role::Tool,
                            content: String::new(),
                            tool_calls: Vec::new(),
                            tool_results: results,
                            response_id: None,
                            previous_response_id: None,
                        });
                    }
                    _ => {
                        error!("LlmActor received unsupported payload type from {}", msg.sender);
                        continue;
                    }
                }

                let req = crate::types::ChatRequest {
                    model: config.model.clone(),
                    system: config.server_system_message.clone(),
                    messages: messages.clone(),
                    tools: agent.tools.iter().map(|t| crate::types::ToolDefinition {
                        name: t.name.clone(),
                        description: t.description.clone(),
                        parameters: t.parameters.clone(),
                    }).collect(),
                    max_tokens: config.max_tokens,
                    temperature: config.temperature,
                };

                match agent.llm.chat(req).await {
                    Ok(resp) => {
                        messages.push(resp.message.clone());

                        if resp.message.tool_calls.is_empty() {
                            let reply_msg = ActorMessage {
                                sender: name.clone(),
                                recipient: initiator.clone(),
                                payload: ActorMessagePayload::LlmResponse(resp.message.content),
                            };
                            let _ = system.send(reply_msg).await;
                        } else {
                            let reply_msg = ActorMessage {
                                sender: name.clone(),
                                recipient: "ToolActor".to_string(), // Convention
                                payload: ActorMessagePayload::ToolCallsRequest(resp.message.tool_calls),
                            };
                            let _ = system.send(reply_msg).await;
                        }
                    }
                    Err(e) => {
                        let err_msg = ActorMessage {
                            sender: name.clone(),
                            recipient: initiator.clone(),
                            payload: ActorMessagePayload::Error(format!("LLM Error: {}", e)),
                        };
                        let _ = system.send(err_msg).await;
                    }
                }
            }
            info!("LlmActor {} stopped", name);
        })
    }
}

pub struct ToolActor {
    pub name: String,
    pub tools: Vec<Tool>,
}

impl Actor for ToolActor {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn start(
        &self,
        mut receiver: mpsc::Receiver<ActorMessage>,
        system: Arc<ActorSystem>,
    ) -> tokio::task::JoinHandle<()> {
        let name = self.name.clone();
        let tools = self.tools.clone();

        tokio::spawn(async move {
            info!("ToolActor {} started", name);
            while let Some(msg) = receiver.recv().await {
                debug!("ToolActor {} received message from {}", name, msg.sender);

                if let ActorMessagePayload::ToolCallsRequest(tool_calls) = msg.payload {
                    let mut results = Vec::new();
                    for tc in tool_calls {
                        let mut tc_result = ohc_builtin_agent_core::types::ToolResult {
                            tool_call_id: tc.id.clone(),
                            content: String::new(),
                            error: String::new(),
                        };

                        if let Some(tool) = tools.iter().find(|t| t.name == tc.name) {
                            match tool.execute.execute(tc.arguments).await {
                                Ok(res) => {
                                    tc_result.content = res;
                                }
                                Err(e) => {
                                    tc_result.error = e.to_string();
                                }
                            }
                        } else {
                            tc_result.error = format!("Tool {} not found", tc.name);
                        }
                        results.push(tc_result);
                    }

                    let reply_msg = ActorMessage {
                        sender: name.clone(),
                        recipient: msg.sender.clone(),
                        payload: ActorMessagePayload::ToolResults(results),
                    };
                    let _ = system.send(reply_msg).await;
                } else {
                    error!("ToolActor received unsupported payload type from {}", msg.sender);
                }
            }
            info!("ToolActor {} stopped", name);
        })
    }
}

pub struct UserActor {
    pub name: String,
    pub tx: tokio::sync::mpsc::Sender<Result<String, String>>,
}

impl Actor for UserActor {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn start(
        &self,
        mut receiver: mpsc::Receiver<ActorMessage>,
        _system: Arc<ActorSystem>,
    ) -> tokio::task::JoinHandle<()> {
        let name = self.name.clone();
        let tx = self.tx.clone();

        tokio::spawn(async move {
            info!("UserActor {} started", name);
            while let Some(msg) = receiver.recv().await {
                match msg.payload {
                    ActorMessagePayload::LlmResponse(res) => {
                        let _ = tx.send(Ok(res)).await;
                        break;
                    }
                    ActorMessagePayload::Error(err) => {
                        let _ = tx.send(Err(err)).await;
                        break;
                    }
                    _ => {
                        let _ = tx.send(Err(format!("UserActor received unexpected payload from {}", msg.sender))).await;
                        break;
                    }
                }
            }
            info!("UserActor {} stopped", name);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage, Role, ToolResult};
    use crate::llm::LlmClient;

    struct MockLlm {
        response_text: String,
        invoke_tool: bool,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            // First pass: if we need to invoke a tool and there are no tool results in context
            let has_tool_results = req.messages.iter().any(|m| m.role == Role::Tool);
            if self.invoke_tool && !has_tool_results {
                Ok(ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: String::new(),
                        tool_calls: vec![
                            ToolCall {
                                id: "call_1".to_string(),
                                name: "dummy_tool".to_string(),
                                arguments: serde_json::json!({}),
                            }
                        ],
                        tool_results: vec![],
                        response_id: None,
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            } else {
                Ok(ChatResponse {
                    message: Message::assistant(&self.response_text),
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
            Ok("Tool execution success".to_string())
        }
    }

    #[tokio::test]
    async fn test_actor_model_success() {
        let system = Arc::new(ActorSystem::new());

        // Setup LLM
        let llm = Arc::new(MockLlm {
            response_text: "Final answer after tool".to_string(),
            invoke_tool: true,
        });

        // Setup Agent
        let mut agent = Agent::new(llm, vec![]);
        agent.tools.push(Tool {
            name: "dummy_tool".to_string(),
            description: "A dummy tool".to_string(),
            parameters: serde_json::json!({}),
            is_read_only: false,
            execute: Arc::new(DummyToolExecutor),
        });
        let arc_agent = Arc::new(agent);

        let llm_actor = LlmActor {
            name: "LlmActor".to_string(),
            agent: arc_agent.clone(),
            config: AgentRunConfig::default(),
        };

        let tool_actor = ToolActor {
            name: "ToolActor".to_string(),
            tools: arc_agent.tools.clone(),
        };

        let (out_tx, mut out_rx) = tokio::sync::mpsc::channel(1);
        let user_actor = UserActor {
            name: "UserActor".to_string(),
            tx: out_tx,
        };

        // Channels
        let (llm_tx, llm_rx) = mpsc::channel(10);
        let (tool_tx, tool_rx) = mpsc::channel(10);
        let (user_tx, user_rx) = mpsc::channel(10);

        system.register(llm_actor.name(), llm_tx).await;
        system.register(tool_actor.name(), tool_tx).await;
        system.register(user_actor.name(), user_tx).await;

        llm_actor.start(llm_rx, system.clone());
        tool_actor.start(tool_rx, system.clone());
        user_actor.start(user_rx, system.clone());

        // Send initial task
        system.send(ActorMessage {
            sender: "UserActor".to_string(),
            recipient: "LlmActor".to_string(),
            payload: ActorMessagePayload::UserTask("Do the thing".to_string()),
        }).await.unwrap();

        // Wait for result
        let result = out_rx.recv().await.unwrap();
        assert_eq!(result.unwrap(), "Final answer after tool");
    }
}
