use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};

use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::{ChatRequest, Message, ToolCall, ToolResult};

/// SOTA Harness Patterns (2025-2026): 1. Actor-model message passing -> replacing classic ReAct loops
#[derive(Debug, Clone)]
pub struct ActorMessage {
    pub sender: String,
    pub recipient: String,
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
    pub tool_results: Vec<ToolResult>,
    pub correlation_id: String, // Tracks the original request ID
    pub original_sender: String, // Tracks the original sender across delegations
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

pub struct ToolActor {
    pub name: String,
    pub agent: Arc<Agent>,
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
        let agent = self.agent.clone();

        tokio::spawn(async move {
            info!("Actor {} started", name);
            while let Some(msg) = receiver.recv().await {
                debug!("Actor {} received message from {}: executing tools", name, msg.sender);

                let mut tool_results = Vec::new();
                for tc in &msg.tool_calls {
                    let tool = agent.tools.iter().find(|t| t.name == tc.name);
                    match tool {
                        Some(t) => {
                            let res = t.execute.execute(tc.arguments.clone()).await;
                            match res {
                                Ok(content) => {
                                    tool_results.push(ToolResult {
                                        tool_call_id: tc.id.clone(),
                                        content,
                                        error: String::new(),
                                    });
                                }
                                Err(e) => {
                                    tool_results.push(ToolResult {
                                        tool_call_id: tc.id.clone(),
                                        content: String::new(),
                                        error: e.to_string(),
                                    });
                                }
                            }
                        }
                        None => {
                            tool_results.push(ToolResult {
                                tool_call_id: tc.id.clone(),
                                content: String::new(),
                                error: format!("Tool {} not found", tc.name),
                            });
                        }
                    }
                }

                let reply_msg = ActorMessage {
                    sender: name.clone(),
                    recipient: msg.sender.clone(), // reply back to caller
                    content: "Tool execution completed".to_string(),
                    tool_calls: vec![],
                    tool_results,
                    correlation_id: msg.correlation_id,
                    original_sender: msg.original_sender,
                };

                if let Err(e) = system.send(reply_msg).await {
                    error!("Actor {} failed to send reply to {}: {}", name, msg.sender, e);
                }
            }
            info!("Actor {} stopped", name);
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

        let tool_defs: Vec<_> = agent.tools.iter().map(|t| ohc_builtin_agent_core::types::ToolDefinition {
            name: t.name.clone(),
            description: t.description.clone(),
            parameters: t.parameters.clone(),
        }).collect();

        tokio::spawn(async move {
            info!("Actor {} started", name);
            // We maintain a map of correlation ID to message history, and use the correlation_id
            // in incoming messages to pick up the conversational thread
            let mut threads: HashMap<String, Vec<Message>> = HashMap::new();

            while let Some(msg) = receiver.recv().await {
                debug!("Actor {} received message from {}: {}", name, msg.sender, msg.content);

                // Track conversation thread using correlation_id
                let messages = threads.entry(msg.correlation_id.clone()).or_insert_with(Vec::new);

                // Is it a tool result coming back from the ToolActor?
                if !msg.tool_results.is_empty() {
                     messages.push(Message {
                        role: ohc_builtin_agent_core::types::Role::Tool,
                        content: String::new(),
                        tool_calls: vec![],
                        tool_results: msg.tool_results.clone(),
                        response_id: None,
                        previous_response_id: None,
                    });
                } else if !msg.content.is_empty() {
                    // Treat incoming message content as user instruction (unless we wanted a different routing logic)
                     messages.push(Message::user(msg.content.clone()));
                }

                // Construct and send Request to LLM
                let req = ChatRequest {
                    model: config.model.clone(),
                    system: config.server_system_message.clone(),
                    messages: messages.clone(),
                    tools: tool_defs.clone(),
                    max_tokens: config.max_tokens,
                    temperature: config.temperature,
                };

                let result = agent.llm.chat(req).await;

                match result {
                    Ok(resp) => {
                        let assistant_msg = resp.message;
                        messages.push(assistant_msg.clone());

                        // If it has tool calls, send to ToolActor
                        if !assistant_msg.tool_calls.is_empty() {
                            let tool_msg = ActorMessage {
                                sender: name.clone(),
                                recipient: "ToolActor".to_string(), // Convention: The ToolActor should be registered as ToolActor
                                content: "Please execute these tools".to_string(),
                                tool_calls: assistant_msg.tool_calls.clone(),
                                tool_results: vec![],
                                correlation_id: msg.correlation_id.clone(),
                                original_sender: msg.original_sender.clone(),
                            };

                            if let Err(e) = system.send(tool_msg).await {
                                error!("Actor {} failed to send tool calls to ToolActor: {}", name, e);
                                let error_reply = ActorMessage {
                                    sender: name.clone(),
                                    recipient: msg.original_sender.clone(),
                                    content: format!("Error: failed to delegate tool calls to ToolActor: {}", e),
                                    tool_calls: vec![],
                                    tool_results: vec![],
                                    correlation_id: msg.correlation_id.clone(),
                                    original_sender: name.clone(),
                                };
                                if let Err(send_err) = system.send(error_reply).await {
                                    error!("Actor {} failed to send fallback error reply: {}", name, send_err);
                                }
                            }
                        } else {
                            // No tool calls means it's a final reply to the original sender
                            let mut actual_content = assistant_msg.content.clone();
                            let mut target_recipient = msg.original_sender.clone();

                            // Routing convention: if the response starts with "@ActorName ", route it to that actor.
                            if actual_content.starts_with('@') {
                                if let Some(space_idx) = actual_content.find(' ') {
                                    target_recipient = actual_content[1..space_idx].to_string();
                                    actual_content = actual_content[space_idx+1..].to_string();
                                }
                            }

                            let reply_msg = ActorMessage {
                                sender: name.clone(),
                                recipient: target_recipient.clone(),
                                content: actual_content,
                                tool_calls: vec![],
                                tool_results: vec![],
                                correlation_id: msg.correlation_id.clone(),
                                original_sender: name.clone(), // We are the sender for the next hop
                            };

                            if let Err(e) = system.send(reply_msg).await {
                                error!("Actor {} failed to send reply to {}: {}", name, target_recipient, e);
                            }

                            // Thread completed, we can remove it (in a real system we might keep it longer or persist it)
                            threads.remove(&msg.correlation_id);
                        }
                    }
                    Err(e) => {
                        let reply_msg = ActorMessage {
                            sender: name.clone(),
                            recipient: msg.original_sender.clone(),
                            content: format!("Error: {}", e),
                            tool_calls: vec![],
                            tool_results: vec![],
                            correlation_id: msg.correlation_id.clone(),
                            original_sender: name.clone(),
                        };

                        if let Err(e) = system.send(reply_msg).await {
                            error!("Actor {} failed to send error reply to {}: {}", name, msg.original_sender, e);
                        }
                    }
                }
            }
            info!("Actor {} stopped", name);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage, ToolError};
    use crate::llm::LlmClient;
    use crate::tools::{Tool, ToolExecutor};

    struct MockLlm {
        pub response_text: String,
        pub calls: Arc<Mutex<usize>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut count = self.calls.lock().await;
            *count += 1;

            // If it's the first call, return a tool call. If second, return final answer.
            if *count == 1 {
                Ok(ChatResponse {
                    message: Message {
                        role: ohc_builtin_agent_core::types::Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ohc_builtin_agent_core::types::ToolCall {
                            id: "call_1".to_string(),
                            name: "echo".to_string(),
                            arguments: serde_json::json!({"val": "test"}),
                        }],
                        tool_results: vec![],
                        response_id: None,
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("mock-id-1".to_string()),
                })
            } else {
                 Ok(ChatResponse {
                    message: Message::assistant(&self.response_text),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id-2".to_string()),
                })
            }
        }
    }

    struct MockEchoTool;
    #[async_trait::async_trait]
    impl ToolExecutor for MockEchoTool {
        async fn execute(&self, args: serde_json::Value) -> Result<String, ToolError> {
            Ok(format!("Echo: {}", args["val"].as_str().unwrap_or("")))
        }
    }

    #[tokio::test]
    async fn test_actor_model_message_passing() {
        let system = Arc::new(ActorSystem::new());

        let tools = vec![Tool {
            name: "echo".to_string(),
            description: "echo tool".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: Arc::new(MockEchoTool),
        }];

        let coord_llm = Arc::new(MockLlm {
            response_text: "Coordinator final response".to_string(),
            calls: Arc::new(Mutex::new(0)),
        });

        let coord_agent = Arc::new(Agent::new(coord_llm, tools));

        let coord_actor = AgentActor {
            name: "Coordinator".to_string(),
            agent: coord_agent.clone(),
            config: AgentRunConfig::default(),
        };

        let tool_actor = ToolActor {
            name: "ToolActor".to_string(),
            agent: coord_agent.clone(),
        };

        let (coord_tx, coord_rx) = mpsc::channel(10);
        let (tool_tx, tool_rx) = mpsc::channel(10);

        system.register(coord_actor.name(), coord_tx).await;
        system.register(tool_actor.name(), tool_tx).await;

        coord_actor.start(coord_rx, system.clone());
        tool_actor.start(tool_rx, system.clone());

        let (test_tx, mut test_rx) = mpsc::channel(10);
        system.register("ProductionHarness".to_string(), test_tx).await;

        system.send(ActorMessage {
            sender: "ProductionHarness".to_string(),
            recipient: "Coordinator".to_string(),
            content: "Please do this task".to_string(),
            tool_calls: vec![],
            tool_results: vec![],
            correlation_id: "tx-123".to_string(),
            original_sender: "ProductionHarness".to_string(),
        }).await.unwrap();

        // The sequence:
        // 1. ProductionHarness -> Coordinator (content: Please do this task, correlation_id: tx-123)
        // 2. Coordinator LLM returns tool_call "echo"
        // 3. Coordinator -> ToolActor (tool_calls: ["echo"], original_sender: ProductionHarness)
        // 4. ToolActor executes, returns to Coordinator (tool_results: ["Echo: test"], original_sender: ProductionHarness)
        // 5. Coordinator LLM returns final string "Coordinator final response"
        // 6. Coordinator -> ProductionHarness (uses original_sender instead of hardcoded route)

        if let Some(reply) = test_rx.recv().await {
            assert_eq!(reply.sender, "Coordinator");
            assert_eq!(reply.content, "Coordinator final response");
            assert_eq!(reply.correlation_id, "tx-123");
        } else {
            panic!("Did not receive reply from Coordinator");
        }
    }

    #[tokio::test]
    async fn test_actor_model_pure_conversation() {
        let system = Arc::new(ActorSystem::new());
        let coord_llm = Arc::new(MockLlm {
            response_text: "Just conversation, no tools".to_string(),
            calls: Arc::new(Mutex::new(1)), // Start at 1 so MockLlm bypasses the tool_call logic
        });

        let coord_agent = Arc::new(Agent::new(coord_llm, vec![]));

        let coord_actor = AgentActor {
            name: "Coordinator".to_string(),
            agent: coord_agent.clone(),
            config: AgentRunConfig::default(),
        };

        let (coord_tx, coord_rx) = mpsc::channel(10);
        system.register(coord_actor.name(), coord_tx).await;
        coord_actor.start(coord_rx, system.clone());

        let (test_tx, mut test_rx) = mpsc::channel(10);
        system.register("ProductionHarness".to_string(), test_tx).await;

        system.send(ActorMessage {
            sender: "ProductionHarness".to_string(),
            recipient: "Coordinator".to_string(),
            content: "Hello there".to_string(),
            tool_calls: vec![],
            tool_results: vec![],
            correlation_id: "tx-pure".to_string(),
            original_sender: "ProductionHarness".to_string(),
        }).await.unwrap();

        if let Some(reply) = test_rx.recv().await {
            assert_eq!(reply.sender, "Coordinator");
            assert_eq!(reply.content, "Just conversation, no tools");
            assert_eq!(reply.recipient, "ProductionHarness");
        } else {
            panic!("Did not receive reply");
        }
    }

    #[tokio::test]
    async fn test_actor_model_tool_results_routing() {
        let system = Arc::new(ActorSystem::new());
        let coord_llm = Arc::new(MockLlm {
            response_text: "Final response after tool result".to_string(),
            calls: Arc::new(Mutex::new(1)), // Start at 1 so MockLlm bypasses tool_calls
        });

        let coord_agent = Arc::new(Agent::new(coord_llm, vec![]));

        let coord_actor = AgentActor {
            name: "Coordinator".to_string(),
            agent: coord_agent.clone(),
            config: AgentRunConfig::default(),
        };

        let (coord_tx, coord_rx) = mpsc::channel(10);
        system.register(coord_actor.name(), coord_tx).await;
        coord_actor.start(coord_rx, system.clone());

        let (test_tx, mut test_rx) = mpsc::channel(10);
        system.register("ProductionHarness".to_string(), test_tx).await;

        let tool_results = vec![
            ohc_builtin_agent_core::types::ToolResult {
                tool_call_id: "call_1".to_string(),
                content: "Tool completed successfully".to_string(),
                error: "".to_string(),
            }
        ];

        system.send(ActorMessage {
            sender: "ToolActor".to_string(),
            recipient: "Coordinator".to_string(),
            content: "Tool execution completed".to_string(),
            tool_calls: vec![],
            tool_results,
            correlation_id: "tx-tools".to_string(),
            original_sender: "ProductionHarness".to_string(),
        }).await.unwrap();

        if let Some(reply) = test_rx.recv().await {
            assert_eq!(reply.sender, "Coordinator");
            assert_eq!(reply.content, "Final response after tool result");
            assert_eq!(reply.recipient, "ProductionHarness");
        } else {
            panic!("Did not receive reply");
        }
    }

    #[tokio::test]
    async fn test_actor_model_routing_convention() {
        let system = Arc::new(ActorSystem::new());
        let coord_llm = Arc::new(MockLlm {
            response_text: "@OtherActor Can you handle this?".to_string(),
            calls: Arc::new(Mutex::new(1)), // bypass tool calls
        });

        let coord_agent = Arc::new(Agent::new(coord_llm, vec![]));

        let coord_actor = AgentActor {
            name: "Coordinator".to_string(),
            agent: coord_agent.clone(),
            config: AgentRunConfig::default(),
        };

        let (coord_tx, coord_rx) = mpsc::channel(10);
        system.register(coord_actor.name(), coord_tx).await;
        coord_actor.start(coord_rx, system.clone());

        let (test_tx, mut test_rx) = mpsc::channel(10);
        // Register the *target* actor to intercept the routed message
        system.register("OtherActor".to_string(), test_tx).await;

        system.send(ActorMessage {
            sender: "ProductionHarness".to_string(),
            recipient: "Coordinator".to_string(),
            content: "Hello".to_string(),
            tool_calls: vec![],
            tool_results: vec![],
            correlation_id: "tx-route".to_string(),
            original_sender: "ProductionHarness".to_string(),
        }).await.unwrap();

        if let Some(reply) = test_rx.recv().await {
            assert_eq!(reply.sender, "Coordinator");
            assert_eq!(reply.content, "Can you handle this?"); // @ prefix removed
            assert_eq!(reply.recipient, "OtherActor");
        } else {
            panic!("Did not receive reply to OtherActor");
        }
    }

    #[tokio::test]
    async fn test_actor_model_llm_error() {
        let system = Arc::new(ActorSystem::new());

        struct FailingLlm;
        #[async_trait::async_trait]
        impl LlmClient for FailingLlm {
            async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                Err("Simulated LLM Failure".into())
            }
        }

        let coord_llm = Arc::new(FailingLlm);
        let coord_agent = Arc::new(Agent::new(coord_llm, vec![]));

        let coord_actor = AgentActor {
            name: "Coordinator".to_string(),
            agent: coord_agent.clone(),
            config: AgentRunConfig::default(),
        };

        let (coord_tx, coord_rx) = mpsc::channel(10);
        system.register(coord_actor.name(), coord_tx).await;
        coord_actor.start(coord_rx, system.clone());

        let (test_tx, mut test_rx) = mpsc::channel(10);
        system.register("ProductionHarness".to_string(), test_tx).await;

        system.send(ActorMessage {
            sender: "ProductionHarness".to_string(),
            recipient: "Coordinator".to_string(),
            content: "Hello".to_string(),
            tool_calls: vec![],
            tool_results: vec![],
            correlation_id: "tx-err".to_string(),
            original_sender: "ProductionHarness".to_string(),
        }).await.unwrap();

        if let Some(reply) = test_rx.recv().await {
            assert_eq!(reply.sender, "Coordinator");
            assert_eq!(reply.content, "Error: Simulated LLM Failure");
            assert_eq!(reply.recipient, "ProductionHarness");
        } else {
            panic!("Did not receive reply from Coordinator");
        }
    }

    #[tokio::test]
    async fn test_actor_model_lifecycle() {
        let system = Arc::new(ActorSystem::new());
        let coord_llm = Arc::new(MockLlm {
            response_text: "Final".to_string(),
            calls: Arc::new(Mutex::new(0)),
        });

        let coord_agent = Arc::new(Agent::new(coord_llm, vec![]));

        let coord_actor = AgentActor {
            name: "Coordinator".to_string(),
            agent: coord_agent.clone(),
            config: AgentRunConfig::default(),
        };

        let (coord_tx, coord_rx) = mpsc::channel(10);
        system.register(coord_actor.name(), coord_tx).await;

        coord_actor.start(coord_rx, system.clone());

        let (test_tx, mut test_rx) = mpsc::channel(10);
        system.register("ProductionHarness".to_string(), test_tx).await;

        system.send(ActorMessage {
            sender: "ProductionHarness".to_string(),
            recipient: "Coordinator".to_string(),
            content: "Task".to_string(),
            tool_calls: vec![],
            tool_results: vec![],
            correlation_id: "tx-lifecycle".to_string(),
            original_sender: "ProductionHarness".to_string(),
        }).await.unwrap();

        if let Some(reply) = test_rx.recv().await {
            assert_eq!(reply.content, "Error: failed to delegate tool calls to ToolActor: Recipient ToolActor not found");
        } else {
            panic!("Did not receive reply");
        }
    }
}
