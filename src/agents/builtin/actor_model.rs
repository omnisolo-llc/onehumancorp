use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use crate::agent::{Agent, AgentRunConfig};
use crate::tool_executor_engine::ToolExecutionEngine;
use ohc_builtin_agent_core::types::{ChatRequest, Message, ToolCall, ToolResult};

/// SOTA Harness Patterns (2025-2026): 1. Actor-model message passing -> replacing classic ReAct loops
#[derive(Debug, Clone)]
pub struct ActorMessage {
    pub sender: String,
    pub recipient: String,
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
    pub tool_results: Vec<ToolResult>,
    pub correlation_id: String,  // Tracks the original request ID
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
    dead_letters: Mutex<Vec<ActorMessage>>,
}

impl Default for ActorSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ActorSystem {
    pub fn new() -> Self {
        Self {
            mailboxes: Mutex::new(HashMap::new()),
            dead_letters: Mutex::new(Vec::new()),
        }
    }

    /// SOTA Harness Patterns (2025-2026): 1. Actor-model message passing
    /// Spawns a new actor and returns its handle, encapsulating the channel setup
    /// and registration logic required by the actor framework.
    pub async fn spawn<A: Actor + 'static>(
        self: &Arc<Self>,
        actor: A,
    ) -> tokio::task::JoinHandle<()> {
        let (tx, rx) = mpsc::channel(100);
        let name = actor.name();

        self.register(name.clone(), tx).await;

        actor.start(rx, self.clone())
    }

    pub async fn get_dead_letters(&self) -> Vec<ActorMessage> {
        let dlq = self.dead_letters.lock().await;
        dlq.clone()
    }

    pub async fn register(&self, name: String, sender: mpsc::Sender<ActorMessage>) {
        let mut mb = self.mailboxes.lock().await;
        mb.insert(name, sender);
    }

    pub async fn broadcast(&self, msg: ActorMessage) -> Result<(), String> {
        let senders = {
            let mb = self.mailboxes.lock().await;
            mb.clone()
        };

        let futures = senders.into_iter().map(|(name, sender)| {
            let mut clone_msg = msg.clone();
            clone_msg.recipient = name.clone();
            async move {
                if let Err(e) = sender.send(clone_msg).await {
                    Some(format!("Failed to send to {}: {}", name, e))
                } else {
                    None
                }
            }
        });

        let errors: Vec<String> = futures::future::join_all(futures)
            .await
            .into_iter()
            .flatten()
            .collect();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join(", "))
        }
    }

    pub async fn unregister(&self, name: &str) {
        let mut mb = self.mailboxes.lock().await;
        mb.remove(name);
    }

    pub async fn ask(
        &self,
        mut msg: ActorMessage,
        timeout: std::time::Duration,
    ) -> Result<ActorMessage, String> {
        let reply_to = format!("ask-{}", uuid::Uuid::new_v4());
        let (tx, mut rx) = mpsc::channel(1);

        self.register(reply_to.clone(), tx).await;
        msg.sender = reply_to.clone();

        let send_res = self.send(msg).await;
        if let Err(e) = send_res {
            self.unregister(&reply_to).await;
            return Err(e);
        }

        let result = match tokio::time::timeout(timeout, rx.recv()).await {
            Ok(Some(reply)) => Ok(reply),
            Ok(None) => Err("Channel closed before receiving reply".to_string()),
            Err(_) => Err("Ask timed out".to_string()),
        };

        self.unregister(&reply_to).await;
        result
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
            let recipient = msg.recipient.clone();
            let mut dlq = self.dead_letters.lock().await;
            dlq.push(msg);
            Err(format!("Recipient {} not found", recipient))
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
            let mut error_counts: std::collections::HashMap<String, u64> =
                std::collections::HashMap::new();
            while let Some(msg) = receiver.recv().await {
                debug!(
                    "Actor {} received message from {}: executing tools",
                    name, msg.sender
                );

                let mut tool_results = Vec::new();
                let mut read_only_calls = Vec::new();
                let mut mutating_calls = Vec::new();

                for tc in &msg.tool_calls {
                    let is_read_only = agent.tools.iter().find(|t| t.name == tc.name).map(|t| t.is_read_only).unwrap_or(false);
                    if is_read_only {
                        read_only_calls.push(tc.clone());
                    } else {
                        mutating_calls.push(tc.clone());
                    }
                }

                // Master Catalog B.2: Tools (The Agent's Hands): Read-only operations run concurrently; mutating operations run serially.
                let mut ro_futures = Vec::new();
                for tc in read_only_calls {
                    let agent_tools = agent.tools.clone();
                    let tc_clone = tc.clone();

                    ro_futures.push(async move {
                        let tool = agent_tools.iter().find(|t| t.name == tc_clone.name);
                        match tool {
                            Some(t) => {
                                let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(
                                    t,
                                    &tc_clone,
                                    2,
                                    &crate::agent::AgentRunConfig::default(),
                                )
                                .await;
                                (tc_clone, Some(res))
                            }
                            None => (tc_clone, None),
                        }
                    });
                }

                let ro_results = futures::future::join_all(ro_futures).await;
                for (tc, res_opt) in ro_results {
                    match res_opt {
                        Some(res) => {
                            match res {
                                Ok(content) => {
                                    error_counts.insert(tc.name.clone(), 0);
                                    tool_results.push(ToolResult {
                                        tool_call_id: tc.id.clone(),
                                        content,
                                        error: String::new(),
                                    });
                                }
                                Err(e) => {
                                    let error_str = match e {
                                        ohc_builtin_agent_core::types::ToolError::LlmRecoverable(msg) => {
                                            let count = *error_counts.entry(tc.name.clone()).or_insert(0) + 1;
                                            error_counts.insert(tc.name.clone(), count);
                                            if count > 2 {
                                                format!("Fatal tool error: Tool '{}' failed consecutively beyond max_retries limit with recoverable errors. Escalating to Fatal to prevent compounding error loops. Last error: {}", tc.name, msg)
                                            } else {
                                                ohc_builtin_agent_core::types::ToolResult::new_llm_recoverable("".to_string(), &tc.name, &msg).error
                                            }
                                        },
                                        ohc_builtin_agent_core::types::ToolError::UserFixable(msg) => msg,
                                        ohc_builtin_agent_core::types::ToolError::Fatal(msg) => format!("Fatal Error: {}", msg),
                                        ohc_builtin_agent_core::types::ToolError::Transient(msg) => format!("Transient Error: {}", msg),
                                        ohc_builtin_agent_core::types::ToolError::Unexpected(msg) => format!("Unexpected Error: {}", msg),
                                        ohc_builtin_agent_core::types::ToolError::HandoffRequested(msg) => format!("Handoff Requested: {}", msg),
                                    };
                                    tool_results.push(ToolResult {
                                        tool_call_id: tc.id.clone(),
                                        content: String::new(),
                                        error: error_str,
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

                // Mutating tools serially
                for tc in mutating_calls {
                    let tool = agent.tools.iter().find(|t| t.name == tc.name);
                    match tool {
                        Some(t) => {
                            let res = ToolExecutionEngine::execute_tool_with_langgraph_mechanics(
                                t,
                                &tc,
                                2,
                                &crate::agent::AgentRunConfig::default(),
                            )
                            .await;
                            match res {
                                Ok(content) => {
                                    error_counts.insert(tc.name.clone(), 0);
                                    tool_results.push(ToolResult {
                                        tool_call_id: tc.id.clone(),
                                        content,
                                        error: String::new(),
                                    });
                                }
                                Err(e) => {
                                    let error_str = match e {
                                        ohc_builtin_agent_core::types::ToolError::LlmRecoverable(msg) => {
                                            let count = *error_counts.entry(tc.name.clone()).or_insert(0) + 1;
                                            error_counts.insert(tc.name.clone(), count);
                                            if count > 2 {
                                                format!("Fatal tool error: Tool '{}' failed consecutively beyond max_retries limit with recoverable errors. Escalating to Fatal to prevent compounding error loops. Last error: {}", tc.name, msg)
                                            } else {
                                                ohc_builtin_agent_core::types::ToolResult::new_llm_recoverable("".to_string(), &tc.name, &msg).error
                                            }
                                        },
                                        ohc_builtin_agent_core::types::ToolError::UserFixable(msg) => msg,
                                        ohc_builtin_agent_core::types::ToolError::Fatal(msg) => format!("Fatal Error: {}", msg),
                                        ohc_builtin_agent_core::types::ToolError::Transient(msg) => format!("Transient Error: {}", msg),
                                        ohc_builtin_agent_core::types::ToolError::Unexpected(msg) => format!("Unexpected Error: {}", msg),
                                        ohc_builtin_agent_core::types::ToolError::HandoffRequested(msg) => format!("Handoff Requested: {}", msg),
                                    };
                                    tool_results.push(ToolResult {
                                        tool_call_id: tc.id.clone(),
                                        content: String::new(),
                                        error: error_str,
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
                    error!(
                        "Actor {} failed to send reply to {}: {}",
                        name, msg.sender, e
                    );
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

        let tool_defs: Vec<_> = agent
            .tools
            .iter()
            .map(|t| ohc_builtin_agent_core::types::ToolDefinition {
                name: t.name.clone(),
                description: t.description.clone(),
                parameters: t.parameters.clone(),
            })
            .collect();

        tokio::spawn(async move {
            info!("Actor {} started", name);
            // We maintain a map of correlation ID to message history, and use the correlation_id
            // in incoming messages to pick up the conversational thread
            let mut threads: HashMap<String, Vec<Message>> = HashMap::new();

            while let Some(msg) = receiver.recv().await {
                debug!(
                    "Actor {} received message from {}: {}",
                    name, msg.sender, msg.content
                );

                // Track conversation thread using correlation_id
                let messages = threads.entry(msg.correlation_id.clone()).or_default();

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
                                error!(
                                    "Actor {} failed to send tool calls to ToolActor: {}",
                                    name, e
                                );
                                let error_reply = ActorMessage {
                                    sender: name.clone(),
                                    recipient: msg.original_sender.clone(),
                                    content: format!(
                                        "Error: failed to delegate tool calls to ToolActor: {}",
                                        e
                                    ),
                                    tool_calls: vec![],
                                    tool_results: vec![],
                                    correlation_id: msg.correlation_id.clone(),
                                    original_sender: name.clone(),
                                };
                                if let Err(send_err) = system.send(error_reply).await {
                                    error!(
                                        "Actor {} failed to send fallback error reply: {}",
                                        name, send_err
                                    );
                                }
                            }
                        } else {
                            // No tool calls means it's a final reply to the original sender
                            let mut actual_content = assistant_msg.content.clone();
                            let mut target_recipient = msg.original_sender.clone();

                            // Routing convention: if the response starts with "@ActorName ", route it to that actor.
                            if actual_content.starts_with('@')
                                && let Some(space_idx) = actual_content.find(' ')
                            {
                                target_recipient = actual_content[1..space_idx].to_string();
                                actual_content = actual_content[space_idx + 1..].to_string();
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
                                error!(
                                    "Actor {} failed to send reply to {}: {}",
                                    name, target_recipient, e
                                );
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
                            error!(
                                "Actor {} failed to send error reply to {}: {}",
                                name, msg.original_sender, e
                            );
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
    use crate::llm::LlmClient;
    use crate::tools::{Tool, ToolExecutor};
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, ToolError, Usage};

    struct MockLlm {
        pub response_text: String,
        pub calls: Arc<Mutex<usize>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
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
        system
            .register("ProductionHarness".to_string(), test_tx)
            .await;

        system
            .send(ActorMessage {
                sender: "ProductionHarness".to_string(),
                recipient: "Coordinator".to_string(),
                content: "Please do this task".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                correlation_id: "tx-123".to_string(),
                original_sender: "ProductionHarness".to_string(),
            })
            .await
            .unwrap();

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
        system
            .register("ProductionHarness".to_string(), test_tx)
            .await;

        system
            .send(ActorMessage {
                sender: "ProductionHarness".to_string(),
                recipient: "Coordinator".to_string(),
                content: "Hello there".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                correlation_id: "tx-pure".to_string(),
                original_sender: "ProductionHarness".to_string(),
            })
            .await
            .unwrap();

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
        system
            .register("ProductionHarness".to_string(), test_tx)
            .await;

        let tool_results = vec![ohc_builtin_agent_core::types::ToolResult {
            tool_call_id: "call_1".to_string(),
            content: "Tool completed successfully".to_string(),
            error: "".to_string(),
        }];

        system
            .send(ActorMessage {
                sender: "ToolActor".to_string(),
                recipient: "Coordinator".to_string(),
                content: "Tool execution completed".to_string(),
                tool_calls: vec![],
                tool_results,
                correlation_id: "tx-tools".to_string(),
                original_sender: "ProductionHarness".to_string(),
            })
            .await
            .unwrap();

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

        system
            .send(ActorMessage {
                sender: "ProductionHarness".to_string(),
                recipient: "Coordinator".to_string(),
                content: "Hello".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                correlation_id: "tx-route".to_string(),
                original_sender: "ProductionHarness".to_string(),
            })
            .await
            .unwrap();

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
            async fn chat(
                &self,
                _req: ChatRequest,
            ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
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
        system
            .register("ProductionHarness".to_string(), test_tx)
            .await;

        system
            .send(ActorMessage {
                sender: "ProductionHarness".to_string(),
                recipient: "Coordinator".to_string(),
                content: "Hello".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                correlation_id: "tx-err".to_string(),
                original_sender: "ProductionHarness".to_string(),
            })
            .await
            .unwrap();

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
        system
            .register("ProductionHarness".to_string(), test_tx)
            .await;

        system
            .send(ActorMessage {
                sender: "ProductionHarness".to_string(),
                recipient: "Coordinator".to_string(),
                content: "Task".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                correlation_id: "tx-lifecycle".to_string(),
                original_sender: "ProductionHarness".to_string(),
            })
            .await
            .unwrap();

        if let Some(reply) = test_rx.recv().await {
            assert_eq!(
                reply.content,
                "Error: failed to delegate tool calls to ToolActor: Recipient ToolActor not found"
            );
        } else {
            panic!("Did not receive reply");
        }
    }
    #[tokio::test]
    async fn test_actor_model_dead_letter_queue() {
        let system = Arc::new(ActorSystem::new());

        let msg = ActorMessage {
            sender: "ProductionHarness".to_string(),
            recipient: "NonExistentActor".to_string(),
            content: "Lost message".to_string(),
            tool_calls: vec![],
            tool_results: vec![],
            correlation_id: "tx-dlq".to_string(),
            original_sender: "ProductionHarness".to_string(),
        };

        let result = system.send(msg).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Recipient NonExistentActor not found");

        let dlq = system.get_dead_letters().await;
        assert_eq!(dlq.len(), 1);
        assert_eq!(dlq[0].content, "Lost message");
        assert_eq!(dlq[0].recipient, "NonExistentActor");
    }

    #[tokio::test]
    async fn test_actor_system_ask_success() {
        let system = Arc::new(ActorSystem::new());

        let (test_tx, mut test_rx) = mpsc::channel(10);
        system.register("EchoActor".to_string(), test_tx).await;

        let system_clone = system.clone();
        tokio::spawn(async move {
            if let Some(msg) = test_rx.recv().await {
                let reply = ActorMessage {
                    sender: "EchoActor".to_string(),
                    recipient: msg.sender.clone(),
                    content: format!("Reply to: {}", msg.content),
                    tool_calls: vec![],
                    tool_results: vec![],
                    correlation_id: msg.correlation_id,
                    original_sender: msg.original_sender,
                };
                system_clone.send(reply).await.unwrap();
            }
        });

        let msg = ActorMessage {
            sender: "Requester".to_string(),
            recipient: "EchoActor".to_string(),
            content: "Ping".to_string(),
            tool_calls: vec![],
            tool_results: vec![],
            correlation_id: "tx-ask".to_string(),
            original_sender: "Requester".to_string(),
        };

        let result = system.ask(msg, std::time::Duration::from_secs(1)).await;
        assert!(result.is_ok());
        let reply = result.unwrap();
        assert_eq!(reply.sender, "EchoActor");
        assert_eq!(reply.content, "Reply to: Ping");
    }

    #[tokio::test]
    async fn test_actor_system_ask_timeout() {
        let system = Arc::new(ActorSystem::new());

        let (test_tx, mut test_rx) = mpsc::channel(10);
        system.register("SlowActor".to_string(), test_tx).await;

        tokio::spawn(async move {
            if let Some(_msg) = test_rx.recv().await {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        });

        let msg = ActorMessage {
            sender: "Requester".to_string(),
            recipient: "SlowActor".to_string(),
            content: "Ping".to_string(),
            tool_calls: vec![],
            tool_results: vec![],
            correlation_id: "tx-ask-timeout".to_string(),
            original_sender: "Requester".to_string(),
        };

        let result = system.ask(msg, std::time::Duration::from_millis(10)).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Ask timed out");
    }

    #[tokio::test]
    async fn test_actor_system_broadcast() {
        let system = Arc::new(ActorSystem::new());

        let (tx1, mut rx1) = mpsc::channel(10);
        let (tx2, mut rx2) = mpsc::channel(10);

        system.register("Actor1".to_string(), tx1).await;
        system.register("Actor2".to_string(), tx2).await;

        let msg = ActorMessage {
            sender: "Broadcaster".to_string(),
            recipient: "Unknown".to_string(),
            content: "Broadcast message".to_string(),
            tool_calls: vec![],
            tool_results: vec![],
            correlation_id: "tx-bcast".to_string(),
            original_sender: "Broadcaster".to_string(),
        };

        let result = system.broadcast(msg).await;
        assert!(result.is_ok());

        let received1 = rx1.recv().await.expect("Actor1 should receive message");
        assert_eq!(received1.recipient, "Actor1");
        assert_eq!(received1.content, "Broadcast message");

        let received2 = rx2.recv().await.expect("Actor2 should receive message");
        assert_eq!(received2.recipient, "Actor2");
        assert_eq!(received2.content, "Broadcast message");
    }

    #[tokio::test]
    async fn test_actor_system_spawn_convenience() {
        let system = Arc::new(ActorSystem::new());

        struct DummyActor;
        impl Actor for DummyActor {
            fn name(&self) -> String {
                "dummy".to_string()
            }
            fn start(
                &self,
                mut rx: mpsc::Receiver<ActorMessage>,
                _sys: Arc<ActorSystem>,
            ) -> tokio::task::JoinHandle<()> {
                tokio::spawn(async move { while rx.recv().await.is_some() {} })
            }
        }

        let handle = system.spawn(DummyActor).await;

        assert!(system.mailboxes.lock().await.contains_key("dummy"));
        system.unregister("dummy").await;
        handle.abort();
    }
}
