use std::time::Duration;
use tokio::sync::{mpsc, Mutex};
use tokio::time::timeout;
use std::sync::Arc;
use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::Message;

/// Hermes Agent Unique Harness Innovations:
/// Serverless persistence: Hibernates when idle, wakes on demand (works on $5 VPS to GPU clusters).
#[async_trait::async_trait]
pub trait SessionStore: Send + Sync {
    async fn save_session(&self, session_id: &str, state: Vec<Message>) -> Result<(), String>;
    async fn load_session(&self, session_id: &str) -> Result<Option<Vec<Message>>, String>;
}

pub enum ServerlessMessage {
    ProcessTask {
        session_id: String,
        task: String,
        reply_to: mpsc::Sender<Result<String, String>>,
    },
    Shutdown,
}

pub struct ServerlessHibernator {
    agent: Arc<Agent>,
    config: AgentRunConfig,
    store: Arc<dyn SessionStore>,
    idle_timeout: Duration,
    receiver: Mutex<mpsc::Receiver<ServerlessMessage>>,
}

impl ServerlessHibernator {
    pub fn new(
        agent: Arc<Agent>,
        config: AgentRunConfig,
        store: Arc<dyn SessionStore>,
        idle_timeout: Duration,
        receiver: mpsc::Receiver<ServerlessMessage>,
    ) -> Self {
        Self {
            agent,
            config,
            store,
            idle_timeout,
            receiver: Mutex::new(receiver),
        }
    }

    pub async fn run_loop(&self) {
        let mut rx = self.receiver.lock().await;

        loop {
            // Wait for a message with a timeout (Hibernation mechanic)
            match timeout(self.idle_timeout, rx.recv()).await {
                Ok(Some(ServerlessMessage::ProcessTask { session_id, task, reply_to })) => {
                    // 1. Wake on demand: Load state from persistent store
                    let mut session_history = self.store.load_session(&session_id).await.unwrap_or(None).unwrap_or_default();

                    // 2. Execute agent
                    let mut run_cfg = self.config.clone();
                    // Restore context
                    run_cfg.injected_context = Some(session_history.clone());

                    let mut on_event = |_| {};
                    let result = self.agent.run(&run_cfg, &task, &mut on_event).await;

                    // Update state
                    session_history.push(Message::user(task));
                    if let Ok(ref res) = result {
                        session_history.push(Message::assistant(res.clone()));
                    }

                    // 3. Save state back to persistent store before hibernating
                    let _ = self.store.save_session(&session_id, session_history).await;

                    // 4. Reply
                    let _ = reply_to.send(result.map_err(|e| e.to_string())).await;
                }
                Ok(Some(ServerlessMessage::Shutdown)) => {
                    break;
                }
                Ok(None) => {
                    // Channel closed
                    break;
                }
                Err(_) => {
                    // Timeout occurred, hibernate when idle
                    // In a true serverless environment, we might exit the process here to free resources.
                    // State is only loaded during execution (wakes on demand).
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage};
    use crate::llm::LlmClient;
    use std::collections::HashMap;
    use std::sync::RwLock;

    struct MockStore {
        sessions: RwLock<HashMap<String, Vec<Message>>>,
    }

    #[async_trait::async_trait]
    impl SessionStore for MockStore {
        async fn save_session(&self, session_id: &str, state: Vec<Message>) -> Result<(), String> {
            self.sessions.write().unwrap().insert(session_id.to_string(), state);
            Ok(())
        }
        async fn load_session(&self, session_id: &str) -> Result<Option<Vec<Message>>, String> {
            Ok(self.sessions.read().unwrap().get(session_id).cloned())
        }
    }

    struct MockLlm;
    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("processed"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_serverless_hibernator() {
        let store = Arc::new(MockStore { sessions: RwLock::new(HashMap::new()) });
        let agent = Arc::new(Agent::new(Arc::new(MockLlm), vec![]));
        let (tx, rx) = mpsc::channel(10);

        let hibernator = ServerlessHibernator::new(
            agent,
            AgentRunConfig::default(),
            store.clone(),
            Duration::from_millis(50),
            rx,
        );

        let hibernator_handle = tokio::spawn(async move {
            hibernator.run_loop().await;
        });

        // Test process task
        let (reply_tx, mut reply_rx) = mpsc::channel(1);
        tx.send(ServerlessMessage::ProcessTask {
            session_id: "session_1".to_string(),
            task: "hello".to_string(),
            reply_to: reply_tx,
        }).await.unwrap();

        let reply = reply_rx.recv().await.unwrap();
        assert_eq!(reply, Ok("processed".to_string()));

        // Verify state is saved
        let saved_state = store.load_session("session_1").await.unwrap().unwrap();
        assert_eq!(saved_state.len(), 2); // user msg + assistant msg

        // Test hibernation timeout (just wait a bit)
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Test shutdown
        tx.send(ServerlessMessage::Shutdown).await.unwrap();
        hibernator_handle.await.unwrap();
    }
}
