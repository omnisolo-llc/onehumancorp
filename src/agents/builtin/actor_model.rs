use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};
use ohc_builtin_agent_core::types::{Message, ToolCall, ToolDefinition};
use crate::agent::{Agent, AgentRunConfig, AgentEvent};

/// SOTA Harness Patterns (2025-2026): 1. Actor-model message passing -> replacing classic ReAct loops
#[derive(Debug, Clone)]
pub struct ActorMessage {
    pub sender: String,
    pub recipient: String,
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
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

pub struct LLMActor {
    pub name: String,
    pub agent: Arc<Agent>,
    pub config: AgentRunConfig,
}

impl Actor for LLMActor {
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
        let mut config = self.config.clone();

        config.enable_tao_orchestration_loop = false;

        tokio::spawn(async move {
            info!("LLMActor {} started", name);

            let mut conversation_history: Vec<Message> = Vec::new();
            if !config.server_system_message.is_empty() {
                conversation_history.push(Message::system(config.server_system_message.clone()));
            }

            while let Some(msg) = receiver.recv().await {
                debug!("LLMActor {} received message from {}: {}", name, msg.sender, msg.content);

                if msg.sender == "ToolExecutor" {
                    // It's a tool result
                    let tool_results: Vec<ohc_builtin_agent_core::types::ToolResult> = msg.tool_calls.iter().map(|tc| {
                        ohc_builtin_agent_core::types::ToolResult {
                            tool_call_id: tc.id.clone(),
                            content: msg.content.clone(),
                            error: String::new(),
                        }
                    }).collect();

                    conversation_history.push(Message {
                        role: ohc_builtin_agent_core::types::Role::Tool,
                        content: String::new(),
                        tool_calls: vec![],
                        tool_results,
                        response_id: None,
                        previous_response_id: None,
                    });
                } else {
                    // Normal user message
                    conversation_history.push(Message::user(msg.content.clone()));
                }

                use ohc_builtin_agent_core::types::ChatRequest;

                let tool_defs: Vec<ToolDefinition> = agent.tools.iter().map(|t| ToolDefinition {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    parameters: t.parameters.clone(),
                }).collect();

                let req = ChatRequest {
                    model: config.model.clone(),
                    system: String::new(),
                    messages: conversation_history.clone(),
                    tools: tool_defs,
                    max_tokens: config.max_tokens,
                    temperature: config.temperature,
                };

                match agent.llm.chat(req).await {
                    Ok(resp) => {
                        // Add assistant response to history
                        let mut asst_msg = Message::assistant(resp.message.content.clone());
                        asst_msg.tool_calls = resp.message.tool_calls.clone();
                        conversation_history.push(asst_msg);

                        if !resp.message.tool_calls.is_empty() {
                            let tool_msg = ActorMessage {
                                sender: name.clone(),
                                recipient: "ToolExecutor".to_string(),
                                content: resp.message.content.clone(),
                                tool_calls: resp.message.tool_calls.clone(),
                            };
                            let _ = system.send(tool_msg).await;
                        } else {
                            let final_msg = ActorMessage {
                                sender: name.clone(),
                                recipient: "Harness".to_string(),
                                content: resp.message.content,
                                tool_calls: vec![],
                            };
                            let _ = system.send(final_msg).await;
                        }
                    }
                    Err(e) => {
                        let error_msg = ActorMessage {
                            sender: name.clone(),
                            recipient: "Harness".to_string(),
                            content: format!("Error: {}", e),
                            tool_calls: vec![],
                        };
                        let _ = system.send(error_msg).await;
                    }
                }
            }
            info!("LLMActor {} stopped", name);
        })
    }
}

pub struct ToolExecutorActor {
    pub name: String,
    pub agent: Arc<Agent>,
    pub config: AgentRunConfig,
}

impl Actor for ToolExecutorActor {
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
            info!("ToolExecutorActor {} started", name);
            while let Some(msg) = receiver.recv().await {
                debug!("ToolExecutorActor {} received tool execution request", name);

                let mut results_content = String::new();
                for tc in &msg.tool_calls {
                    if let Some(tool) = agent.tools.iter().find(|t| t.name == tc.name) {
                        match crate::tool_executor_engine::ToolExecutionEngine::execute_tool_with_langgraph_mechanics(tool, tc, config.max_retries).await {
                            Ok(res) => {
                                results_content.push_str(&res);
                            }
                            Err(e) => {
                                results_content.push_str(&format!("{:?}", e));
                            }
                        }
                    } else {
                        results_content.push_str(&format!("Tool {} not found", tc.name));
                    }
                }

                let reply_msg = ActorMessage {
                    sender: name.clone(),
                    recipient: msg.sender.clone(),
                    content: results_content,
                    tool_calls: msg.tool_calls.clone(),
                };
                let _ = system.send(reply_msg).await;
            }
            info!("ToolExecutorActor {} stopped", name);
        })
    }
}

pub struct AgentActor {
    pub name: String,
    pub agent: Arc<Agent>,
    pub config: AgentRunConfig,
}

impl Actor for AgentActor {
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
            info!("Actor {} started", name);
            while let Some(msg) = receiver.recv().await {
                debug!("Actor {} received message from {}: {}", name, msg.sender, msg.content);

                let mut on_event = |_e: AgentEvent| {};
                let result = agent.run(&config, &msg.content, &mut on_event).await;

                let reply_content = match result {
                    Ok(res) => res,
                    Err(e) => format!("Error: {}", e),
                };

                let reply_msg = ActorMessage {
                    sender: name.clone(),
                    recipient: msg.sender.clone(),
                    content: reply_content,
                    tool_calls: vec![],
                };

                if let Err(e) = system.send(reply_msg).await {
                    error!("Actor {} failed to send reply: {}", name, e);
                }
            }
            info!("Actor {} stopped", name);
        })
    }
}


pub struct MessagePassingHarness {
    pub system: Arc<ActorSystem>,
    pub main_agent: Arc<Agent>,
    pub config: AgentRunConfig,
}

impl MessagePassingHarness {
    pub fn new(main_agent: Arc<Agent>, config: AgentRunConfig) -> Self {
        Self {
            system: Arc::new(ActorSystem::new()),
            main_agent,
            config,
        }
    }

    pub async fn start_and_run(&self, task: &str) -> Result<String, String> {
        let (agent_tx, agent_rx) = mpsc::channel(100);
        let (harness_tx, mut harness_rx) = mpsc::channel(100);

        self.system.register("MainAgent".to_string(), agent_tx).await;
        self.system.register("Harness".to_string(), harness_tx).await;

        let agent_actor = AgentActor {
            name: "MainAgent".to_string(),
            agent: self.main_agent.clone(),
            config: self.config.clone(),
        };

        agent_actor.start(agent_rx, self.system.clone());

        self.system.send(ActorMessage {
            sender: "Harness".to_string(),
            recipient: "MainAgent".to_string(),
            content: task.to_string(),
            tool_calls: vec![],
        }).await?;

        if let Some(msg) = harness_rx.recv().await {
            Ok(msg.content)
        } else {
            Err("No response received".to_string())
        }
    }

    pub async fn start_and_run_react_replacement(&self, task: &str) -> Result<String, String> {
        let (llm_tx, llm_rx) = mpsc::channel(100);
        let (tool_tx, tool_rx) = mpsc::channel(100);
        let (harness_tx, mut harness_rx) = mpsc::channel(100);

        self.system.register("LLM".to_string(), llm_tx).await;
        self.system.register("ToolExecutor".to_string(), tool_tx).await;
        self.system.register("Harness".to_string(), harness_tx).await;

        let llm_actor = LLMActor {
            name: "LLM".to_string(),
            agent: self.main_agent.clone(),
            config: self.config.clone(),
        };
        let tool_actor = ToolExecutorActor {
            name: "ToolExecutor".to_string(),
            agent: self.main_agent.clone(),
            config: self.config.clone(),
        };

        llm_actor.start(llm_rx, self.system.clone());
        tool_actor.start(tool_rx, self.system.clone());

        self.system.send(ActorMessage {
            sender: "Harness".to_string(),
            recipient: "LLM".to_string(),
            content: task.to_string(),
            tool_calls: vec![],
        }).await?;

        if let Some(msg) = harness_rx.recv().await {
            Ok(msg.content)
        } else {
            Err("No response received".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage, ToolCall};
    use crate::llm::LlmClient;
    use crate::tools::{Tool, ToolExecutor};

    struct TestLlmClient {
        call_count: tokio::sync::Mutex<usize>,
    }

    #[async_trait::async_trait]
    impl LlmClient for TestLlmClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut count = self.call_count.lock().await;
            *count += 1;

            // Check that tools are passed
            assert!(!req.tools.is_empty(), "Tools must be provided in ChatRequest");

            if *count == 1 {
                return Ok(ChatResponse {
                    message: Message {
                        role: ohc_builtin_agent_core::types::Role::Assistant,
                        content: "Let me check the tool".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_123".to_string(),
                            name: "test_tool".to_string(),
                            arguments: serde_json::json!({}),
                        }],
                        tool_results: vec![],
                        response_id: Some("id1".to_string()),
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("id1".to_string()),
                });
            }

            // For turn 2, check history is maintained: System, User, Assistant(with tool call), ToolResult
            assert_eq!(req.messages.len(), 4, "Conversation history must be maintained");
            assert_eq!(req.messages[2].role, ohc_builtin_agent_core::types::Role::Assistant);
            assert_eq!(req.messages[3].role, ohc_builtin_agent_core::types::Role::Tool);

            let last_msg = req.messages.last().unwrap().tool_results[0].content.clone();
            Ok(ChatResponse {
                message: Message::assistant(format!("Final answer based on: {}", last_msg)),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id2".to_string()),
            })
        }
    }

    struct MockToolExecutor;
    #[async_trait::async_trait]
    impl ToolExecutor for MockToolExecutor {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ohc_builtin_agent_core::types::ToolError> {
            Ok("Tool Success!".to_string())
        }
    }

    #[tokio::test]
    async fn test_react_replacement_loop_with_real_logic() {
        let llm = Arc::new(TestLlmClient {
            call_count: tokio::sync::Mutex::new(0),
        });

        let tool = Tool {
            name: "test_tool".to_string(),
            description: "Test tool".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: Arc::new(MockToolExecutor),
        };

        let agent = Arc::new(Agent::new(llm, vec![tool]));
        let mut config = AgentRunConfig::default();
        config.server_system_message = "Test System".to_string();

        let harness = MessagePassingHarness::new(agent, config);

        let result = harness.start_and_run_react_replacement("Do something").await.unwrap();
        assert!(result.contains("Final answer based on:"));
        assert!(result.contains("Tool Success!"));
    }
}
