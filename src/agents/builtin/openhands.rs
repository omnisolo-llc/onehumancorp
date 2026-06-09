use tokio::sync::broadcast;

/// OpenHands/OpenDevin Unique Harness Innovations: EventStream Architecture
/// (Inspired by OpenHands / OpenDevin Python SDK + CLI).
/// The event stream acts as a single source of truth for all actions and observations
/// occurring between the Agent, LLM, and the Environment (Browser/Terminal).

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    RunCommand { command: String },
    WriteFile { path: String, content: String },
    AgentMessage { content: String },
    ToolExecutionStarted { tool_name: String, args: String },
    ToolExecutionCompleted { tool_name: String, result: String },
    LlmRequestStarted { prompt_length: usize },
    LlmResponseReceived { response: String, usage: Option<usize> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Observation {
    CommandOutput {
        exit_code: i32,
        stdout: String,
        stderr: String,
    },
    FileWritten {
        path: String,
    },
    AgentReply {
        content: String,
    },
    GuardrailTriggered {
        reason: String,
    },
    CheckpointSaved {
        checkpoint_id: String,
    },
    AgentStateChanged {
        new_state: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    Action(Action),
    Observation(Observation),
}

pub struct EventStream {
    sender: broadcast::Sender<EventType>,
    history: std::sync::Arc<tokio::sync::RwLock<std::collections::VecDeque<EventType>>>,
    max_history: usize,
}

impl EventStream {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self {
            sender,
            history: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::VecDeque::new())),
            max_history: 1000,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<EventType> {
        self.sender.subscribe()
    }

    pub async fn publish(
        &self,
        event: EventType,
    ) -> Result<usize, broadcast::error::SendError<EventType>> {
        let mut hist = self.history.write().await;
        if hist.len() >= self.max_history {
            hist.pop_front();
        }
        hist.push_back(event.clone());
        self.sender.send(event)
    }

    pub async fn get_history(&self) -> Vec<EventType> {
        self.history.read().await.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_openhands_event_stream() {
        let stream = EventStream::new(10);
        let mut rx1 = stream.subscribe();
        let mut rx2 = stream.subscribe();

        let action = EventType::Action(Action::RunCommand {
            command: "ls -la".to_string(),
        });
        stream.publish(action.clone()).await.unwrap();

        let recv1 = rx1.recv().await.unwrap();
        let recv2 = rx2.recv().await.unwrap();

        assert_eq!(recv1, action);
        assert_eq!(recv2, action);

        let obs = EventType::Observation(Observation::CommandOutput {
            exit_code: 0,
            stdout: "total 0".to_string(),
            stderr: "".to_string(),
        });

        stream.publish(obs.clone()).await.unwrap();

        assert_eq!(rx1.recv().await.unwrap(), obs);
        assert_eq!(rx2.recv().await.unwrap(), obs);

        let history = stream.get_history().await;
        assert_eq!(history.len(), 2);
        assert_eq!(history[0], action);
        assert_eq!(history[1], obs);
    }
}
