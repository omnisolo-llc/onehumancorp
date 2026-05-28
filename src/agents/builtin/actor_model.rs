use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};

use crate::agent::{Agent, AgentRunConfig};

/// SOTA Harness Patterns (2025-2026): 1. Actor-model message passing -> replacing classic ReAct loops
#[derive(Debug, Clone)]
pub struct ActorMessage {
    pub sender: String,
    pub recipient: String,
    pub content: String,
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

                // Replace classic ReAct loop with message-based trigger
                let mut on_event = |_e| {};
                let result = agent.run(&config, &msg.content, &mut on_event).await;

                let reply_content = match result {
                    Ok(res) => res,
                    Err(e) => format!("Error: {}", e),
                };

                // Send reply back to the sender
                let reply_msg = ActorMessage {
                    sender: name.clone(),
                    recipient: msg.sender.clone(),
                    content: reply_content,
                };

                if let Err(e) = system.send(reply_msg).await {
                    error!("Actor {} failed to send reply: {}", name, e);
                }
            }
            info!("Actor {} stopped", name);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};
    use crate::llm::LlmClient;

    struct MockLlm {
        response_text: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(&self.response_text),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_actor_model_message_passing() {
        let system = Arc::new(ActorSystem::new());

        // Create coordinator actor
        let coord_llm = Arc::new(MockLlm {
            response_text: "Coordinator received your response!".to_string(),
        });
        let coord_agent = Arc::new(Agent::new(coord_llm, vec![]));
        let coord_actor = AgentActor {
            name: "Coordinator".to_string(),
            agent: coord_agent,
            config: AgentRunConfig::default(),
        };

        // Create worker actor
        let worker_llm = Arc::new(MockLlm {
            response_text: "Worker processed the task".to_string(),
        });
        let worker_agent = Arc::new(Agent::new(worker_llm, vec![]));
        let worker_actor = AgentActor {
            name: "Worker".to_string(),
            agent: worker_agent,
            config: AgentRunConfig::default(),
        };

        // Channels
        let (coord_tx, coord_rx) = mpsc::channel(10);
        let (worker_tx, worker_rx) = mpsc::channel(10);

        // Register
        system.register(coord_actor.name(), coord_tx).await;
        system.register(worker_actor.name(), worker_tx).await;

        // Start
        coord_actor.start(coord_rx, system.clone());
        worker_actor.start(worker_rx, system.clone());

        // We can manually send a message to the coordinator, but instead let's create a "TestHarness" actor
        // to receive the final result, or just use a raw mpsc channel.
        let (test_tx, mut test_rx) = mpsc::channel(10);
        system.register("TestHarness".to_string(), test_tx).await;

        // Send task to Worker from TestHarness
        system.send(ActorMessage {
            sender: "TestHarness".to_string(),
            recipient: "Worker".to_string(),
            content: "Please do this task".to_string(),
        }).await.unwrap();

        // Expect worker to reply to TestHarness
        if let Some(reply) = test_rx.recv().await {
            assert_eq!(reply.sender, "Worker");
            assert_eq!(reply.content, "Worker processed the task");
        } else {
            panic!("Did not receive reply from Worker");
        }
    }
}
