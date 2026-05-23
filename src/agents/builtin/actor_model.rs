use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use ohc_builtin_agent_core::types::{ChatRequest, Message, ToolCall, ToolResult};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;

/// Harness: Actor-model message passing -> replacing classic ReAct loops

#[derive(Debug, Clone)]
pub enum ActorMessage {
    RunTask { initial_message: String },
    LlmResponse { message: Message },
    ToolRequest { tool_calls: Vec<ToolCall> },
    ToolResult { results: Vec<ToolResult> },
    EndTask { final_output: String },
    Error { error: String },
}

pub struct OrchestratorActor {
    rx: mpsc::Receiver<ActorMessage>,
    llm_tx: mpsc::Sender<ActorMessage>,
    tool_tx: mpsc::Sender<ActorMessage>,
    event_tx: mpsc::Sender<AgentEvent>,
}

impl OrchestratorActor {
    pub async fn run(mut self, mut messages: Vec<Message>) -> Result<String, String> {
        let mut final_output = String::new();
        let mut error_output = None;

        while let Some(msg) = self.rx.recv().await {
            match msg {
                ActorMessage::RunTask { initial_message } => {
                    self.event_tx.send(AgentEvent::RunStarted { iteration: 0 }).await.unwrap_or_default();
                    messages.push(Message::user(initial_message));

                    self.event_tx.send(AgentEvent::IterationStarted { iteration: 0, message_count: messages.len() }).await.unwrap_or_default();
                    let req = ActorMessage::LlmResponse { message: Message::system("please start") }; // trigger LlmActor
                    self.llm_tx.send(req).await.unwrap_or_default();
                }
                ActorMessage::LlmResponse { message } => {
                    if message.role != ohc_builtin_agent_core::types::Role::System {
                        messages.push(message.clone());
                    }
                    if message.tool_calls.is_empty() && message.role != ohc_builtin_agent_core::types::Role::System {
                        // Task Complete
                        self.event_tx.send(AgentEvent::TaskComplete { content: message.content.clone() }).await.unwrap_or_default();
                        final_output = message.content;
                        break;
                    } else if !message.tool_calls.is_empty() {
                        self.tool_tx.send(ActorMessage::ToolRequest { tool_calls: message.tool_calls }).await.unwrap_or_default();
                    }
                }
                ActorMessage::ToolResult { results } => {
                    messages.push(Message {
                        role: ohc_builtin_agent_core::types::Role::Tool,
                        content: String::new(),
                        tool_calls: vec![],
                        tool_results: results,
                        response_id: None,
                        previous_response_id: None,
                    });

                    self.event_tx.send(AgentEvent::IterationStarted { iteration: 0, message_count: messages.len() }).await.unwrap_or_default();
                    self.llm_tx.send(ActorMessage::LlmResponse { message: Message::system("trigger next") }).await.unwrap_or_default();
                }
                ActorMessage::EndTask { final_output: output } => {
                    final_output = output;
                    break;
                }
                ActorMessage::Error { error } => {
                    self.event_tx.send(AgentEvent::TaskError { error: error.clone() }).await.unwrap_or_default();
                    error_output = Some(error);
                    break;
                }
                _ => {}
            }
        }

        if let Some(e) = error_output {
            Err(e)
        } else {
            Ok(final_output)
        }
    }
}

pub struct LlmActor {
    agent: Arc<Agent>,
    config: AgentRunConfig,
    rx: mpsc::Receiver<ActorMessage>,
    orchestrator_tx: mpsc::Sender<ActorMessage>,
    messages: Vec<Message>,
}

impl LlmActor {
    pub async fn run(mut self, session_tools: Vec<crate::tools::Tool>) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                ActorMessage::LlmResponse { message } => {
                    if message.role != ohc_builtin_agent_core::types::Role::System {
                        self.messages.push(message);
                    }
                    let req = ChatRequest {
                        model: self.config.model.clone(),
                        system: self.config.server_system_message.clone(),
                        messages: self.messages.clone(),
                        tools: session_tools.iter().map(|t| ohc_builtin_agent_core::types::ToolDefinition {
                            name: t.name.clone(),
                            description: t.description.clone(),
                            parameters: t.parameters.clone(),
                        }).collect(),
                        max_tokens: self.config.max_tokens,
                        temperature: self.config.temperature,
                    };
                    match self.agent.llm.chat(req).await {
                        Ok(resp) => {
                            let msg = resp.message.clone();
                            self.messages.push(resp.message);
                            self.orchestrator_tx.send(ActorMessage::LlmResponse { message: msg }).await.unwrap_or_default();
                        }
                        Err(e) => {
                            self.orchestrator_tx.send(ActorMessage::Error { error: format!("Error: {:?}", e) }).await.unwrap_or_default();
                        }
                    }
                }
                ActorMessage::ToolResult { results } => {
                    self.messages.push(Message {
                        role: ohc_builtin_agent_core::types::Role::Tool,
                        content: String::new(),
                        tool_calls: vec![],
                        tool_results: results,
                        response_id: None,
                        previous_response_id: None,
                    });
                }
                ActorMessage::RunTask { initial_message } => {
                    self.messages.push(Message::user(initial_message));
                }
                _ => {}
            }
        }
    }
}

pub struct ToolExecutorActor {
    agent: Arc<Agent>,
    config: AgentRunConfig,
    session_tools: Vec<crate::tools::Tool>,
    rx: mpsc::Receiver<ActorMessage>,
    orchestrator_tx: mpsc::Sender<ActorMessage>,
    llm_tx: mpsc::Sender<ActorMessage>,
    event_tx: mpsc::Sender<AgentEvent>,
}

impl ToolExecutorActor {
    pub async fn run(mut self) {
        while let Some(msg) = self.rx.recv().await {
            if let ActorMessage::ToolRequest { tool_calls } = msg {
                let mut results = Vec::new();
                for tc in tool_calls {
                    let mut tc_result = ToolResult {
                        tool_call_id: tc.id.clone(),
                        content: String::new(),
                        error: String::new(),
                    };

                    // Simple gating validation mock
                    let gating_res = crate::tools_gating::ToolGater::check_gating(&tc, false, &self.config);
                    match gating_res {
                        Ok(_) => {
                            let res = self.agent.execute_tool(&tc, &self.session_tools, &[]).await;
                            match res {
                                Ok(content) => tc_result.content = content,
                                Err(e) => tc_result.error = format!("Error: {:?}", e),
                            }
                        }
                        Err(e) => tc_result.error = format!("Error: {:?}", e),
                    }

                    self.event_tx.send(AgentEvent::ToolCall {
                        name: tc.name.clone(),
                        args_json: tc.arguments.to_string(),
                        result: if !tc_result.error.is_empty() { tc_result.error.clone() } else { tc_result.content.clone() },
                        iteration: 0,
                    }).await.unwrap_or_default();

                    results.push(tc_result);
                }
                self.orchestrator_tx.send(ActorMessage::ToolResult { results: results.clone() }).await.unwrap_or_default();
                self.llm_tx.send(ActorMessage::ToolResult { results }).await.unwrap_or_default();
            }
        }
    }
}

pub async fn run_actor_model<F>(
    agent: Arc<Agent>,
    cfg: &AgentRunConfig,
    initial_message: &str,
    session_tools: Vec<crate::tools::Tool>,
    on_event: &mut F,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
where
    F: FnMut(AgentEvent) + Send + Sync,
{
    let (orch_tx, orch_rx) = mpsc::channel(100);
    let (llm_tx, llm_rx) = mpsc::channel(100);
    let (tool_tx, tool_rx) = mpsc::channel(100);
    let (event_tx, mut event_rx) = mpsc::channel(100);

    let orchestrator = OrchestratorActor {
        rx: orch_rx,
        llm_tx: llm_tx.clone(),
        tool_tx,
        event_tx: event_tx.clone(),
    };

    let llm_actor = LlmActor {
        agent: agent.clone(),
        config: cfg.clone(),
        rx: llm_rx,
        orchestrator_tx: orch_tx.clone(),
        messages: Vec::new(),
    };

    let tool_actor = ToolExecutorActor {
        agent: agent.clone(),
        config: cfg.clone(),
        session_tools: session_tools.clone(),
        rx: tool_rx,
        orchestrator_tx: orch_tx.clone(),
        llm_tx: llm_tx.clone(),
        event_tx: event_tx.clone(),
    };

    tokio::spawn(async move {
        llm_actor.run(session_tools).await;
    });

    tokio::spawn(async move {
        tool_actor.run().await;
    });

    let orch_handle = tokio::spawn(async move {
        orchestrator.run(Vec::new()).await
    });

    orch_tx.send(ActorMessage::RunTask { initial_message: initial_message.to_string() }).await.unwrap_or_default();
    llm_tx.send(ActorMessage::RunTask { initial_message: initial_message.to_string() }).await.unwrap_or_default();

    let mut event_drain_handle = tokio::spawn(async move {
        let mut collected_events = Vec::new();
        while let Some(evt) = event_rx.recv().await {
            collected_events.push(evt);
        }
        collected_events
    });

    let res: Result<String, String> = orch_handle.await.unwrap();

    // Drop transmitters to close channels
    drop(orch_tx);
    drop(llm_tx);
    drop(event_tx);

    let collected_events: Vec<AgentEvent> = event_drain_handle.await.unwrap();
    for evt in collected_events {
        on_event(evt);
    }

    match res {
        Ok(output) => Ok(output),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage, ToolCall};
    use crate::llm::LlmClient;

    struct MockLlmClientActor {
        responses: tokio::sync::Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClientActor {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            if !resps.is_empty() {
                Ok(resps.remove(0))
            } else {
                Ok(ChatResponse {
                    message: Message::assistant("Final answer"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            }
        }
    }

    #[tokio::test]
    async fn test_actor_model_message_passing() {
        let client = Arc::new(MockLlmClientActor {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: ohc_builtin_agent_core::types::Role::Assistant,
                        content: "I need to call a tool".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "test_tool".to_string(),
                            arguments: serde_json::json!({}),
                        }],
                        tool_results: vec![],
                        response_id: Some("mock-id-1".to_string()),
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("mock-id-1".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("Task is complete via actors!"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id-2".to_string()),
                },
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();
        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let res = run_actor_model(agent, &cfg, "Hello actors", vec![], &mut on_event).await;

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "Task is complete via actors!");
    }
}
