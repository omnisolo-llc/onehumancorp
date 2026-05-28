/// OpenHands/OpenDevin Unique Harness Innovations: Python SDK + CLI, MIT licensed
/// This implements the OpenHands event-driven architecture, using an AgentController
/// that steps through Actions and Observations via an event stream.

use std::sync::Arc;
use tokio::sync::mpsc;
use crate::agent::Agent;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    CmdRun(String),
    Message(String),
    Finish,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Observation {
    CmdOutput(String),
    AgentMessage(String),
}

#[derive(Debug)]
pub enum Event {
    Action(Action),
    Observation(Observation),
}

pub struct AgentController {
    agent: Arc<Agent>,
    event_tx: mpsc::Sender<Event>,
    event_rx: mpsc::Receiver<Event>,
}

impl AgentController {
    pub fn new(agent: Arc<Agent>) -> Self {
        let (event_tx, event_rx) = mpsc::channel(100);
        Self {
            agent,
            event_tx,
            event_rx,
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<Event> {
        self.event_tx.clone()
    }

    pub async fn step(&mut self) -> Result<Option<Action>, String> {
        if let Some(event) = self.event_rx.recv().await {
            match event {
                Event::Action(action) => {
                    return Ok(Some(action));
                }
                Event::Observation(_) => {
                    // Update agent state with observation
                    // For now, we simulate agent deciding to finish after an observation
                    return Ok(Some(Action::Finish));
                }
            }
        }
        Ok(None)
    }

    pub async fn run_loop(&mut self) -> Result<(), String> {
        loop {
            match self.step().await? {
                Some(Action::Finish) => break,
                Some(Action::CmdRun(cmd)) => {
                    // Simulate running a command
                    let obs = Observation::CmdOutput(format!("Executed: {}", cmd));
                    let _ = self.event_tx.send(Event::Observation(obs)).await;
                }
                Some(Action::Message(msg)) => {
                    let obs = Observation::AgentMessage(msg);
                    let _ = self.event_tx.send(Event::Observation(obs)).await;
                }
                None => break,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage};
    use tokio::sync::Mutex;

    struct MockLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("mock"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_openhands_agent_controller() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let mut controller = AgentController::new(agent);
        let tx = controller.get_sender();

        tx.send(Event::Action(Action::CmdRun("echo hello".to_string()))).await.unwrap();
        tx.send(Event::Action(Action::Finish)).await.unwrap();

        controller.run_loop().await.unwrap();
    }
}
