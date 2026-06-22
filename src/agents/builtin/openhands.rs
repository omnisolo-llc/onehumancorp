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
    ReadUrl { url: String },
    SearchFiles { query: String },
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
    UrlContent { url: String, text: String },
    FilesFound { paths: Vec<String> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    Action(Action),
    Observation(Observation),
}

pub struct EventStream {
    sender: broadcast::Sender<EventType>,
}

impl EventStream {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<EventType> {
        self.sender.subscribe()
    }

    pub fn publish(
        &self,
        event: EventType,
    ) -> Result<usize, broadcast::error::SendError<EventType>> {
        self.sender.send(event)
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
        stream.publish(action.clone()).unwrap();

        let recv1 = rx1.recv().await.unwrap();
        let recv2 = rx2.recv().await.unwrap();

        assert_eq!(recv1, action);
        assert_eq!(recv2, action);

        let obs = EventType::Observation(Observation::CommandOutput {
            exit_code: 0,
            stdout: "total 0".to_string(),
            stderr: "".to_string(),
        });

        stream.publish(obs.clone()).unwrap();

        assert_eq!(rx1.recv().await.unwrap(), obs);
        assert_eq!(rx2.recv().await.unwrap(), obs);
    }

    #[tokio::test]
    async fn test_openhands_event_stream_new_variants() {
        let stream = EventStream::new(10);
        let mut rx1 = stream.subscribe();
        let mut rx2 = stream.subscribe();

        let action_url = EventType::Action(Action::ReadUrl {
            url: "https://example.com".to_string(),
        });
        stream.publish(action_url.clone()).unwrap();

        assert_eq!(rx1.recv().await.unwrap(), action_url);
        assert_eq!(rx2.recv().await.unwrap(), action_url);

        let action_search = EventType::Action(Action::SearchFiles {
            query: "*.rs".to_string(),
        });
        stream.publish(action_search.clone()).unwrap();

        assert_eq!(rx1.recv().await.unwrap(), action_search);
        assert_eq!(rx2.recv().await.unwrap(), action_search);

        let obs_url = EventType::Observation(Observation::UrlContent {
            url: "https://example.com".to_string(),
            text: "<html></html>".to_string(),
        });
        stream.publish(obs_url.clone()).unwrap();

        assert_eq!(rx1.recv().await.unwrap(), obs_url);
        assert_eq!(rx2.recv().await.unwrap(), obs_url);

        let obs_search = EventType::Observation(Observation::FilesFound {
            paths: vec!["src/main.rs".to_string(), "src/lib.rs".to_string()],
        });
        stream.publish(obs_search.clone()).unwrap();

        assert_eq!(rx1.recv().await.unwrap(), obs_search);
        assert_eq!(rx2.recv().await.unwrap(), obs_search);
    }
}
