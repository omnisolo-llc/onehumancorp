use std::sync::Arc;
use tokio::sync::broadcast;
use ohc_builtin_agent_tools::runner::CommandRunner;

/// OpenHands/OpenDevin Unique Harness Innovations: EventStream Architecture
/// (Inspired by OpenHands / OpenDevin Python SDK + CLI).
/// The event stream acts as a single source of truth for all actions and observations
/// occurring between the Agent, LLM, and the Environment (Browser/Terminal).

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    RunCommand { command: String },
    WriteFile { path: String, content: String },
    AgentMessage { content: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Observation {
    CommandOutput { exit_code: i32, stdout: String, stderr: String },
    FileWritten { path: String },
    AgentReply { content: String },
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

    pub fn publish(&self, event: EventType) -> Result<usize, broadcast::error::SendError<EventType>> {
        self.sender.send(event)
    }
}

/// The Environment worker listens to the EventStream for actions (like RunCommand),
/// executes them using the provided CommandRunner, and publishes observations back.
pub struct Environment {
    pub stream: Arc<EventStream>,
    pub runner: Arc<dyn CommandRunner>,
}

impl Environment {
    pub fn new(stream: Arc<EventStream>, runner: Arc<dyn CommandRunner>) -> Self {
        Self { stream, runner }
    }

    /// Spawns a background task that listens to the event stream and processes actions.
    /// Returns a JoinHandle so the caller can await or abort the worker.
    pub fn start(&self) -> tokio::task::JoinHandle<()> {
        let mut rx = self.stream.subscribe();
        let stream = self.stream.clone();
        let runner = self.runner.clone();

        tokio::spawn(async move {
            loop {
                let event = match rx.recv().await {
                    Ok(evt) => evt,
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                };
                if let EventType::Action(Action::RunCommand { command }) = event {
                    // Execute the command using the provided runner
                    let result = runner.run("sh", &["-c", &command], None, vec![]).await;

                    let observation = match result {
                        Ok(output) => {
                            let exit_code = output.status.code().unwrap_or(if output.status.success() { 0 } else { 1 });
                            Observation::CommandOutput {
                                exit_code,
                                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                            }
                        }
                        Err(e) => {
                            Observation::CommandOutput {
                                exit_code: -1,
                                stdout: String::new(),
                                stderr: e.to_string(),
                            }
                        }
                    };

                    // Publish the observation back to the stream
                    let _ = stream.publish(EventType::Observation(observation));
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    pub struct MockCommandRunner {
        pub last_command: Arc<Mutex<Option<(String, Vec<String>)>>>,
        pub exit_code: i32,
        pub stdout: String,
        pub stderr: String,
    }

    #[async_trait::async_trait]
    impl CommandRunner for MockCommandRunner {
        async fn run(
            &self,
            program: &str,
            args: &[&str],
            _current_dir: Option<&std::path::Path>,
            _envs: Vec<(String, String)>,
        ) -> std::io::Result<std::process::Output> {
            *self.last_command.lock().unwrap() = Some((program.to_string(), args.iter().map(|s| s.to_string()).collect()));

            #[cfg(unix)]
            let status = {
                use std::os::unix::process::ExitStatusExt;
                std::process::ExitStatus::from_raw(self.exit_code << 8)
            };

            #[cfg(windows)]
            let status = {
                use std::os::windows::process::ExitStatusExt;
                std::process::ExitStatus::from_raw(self.exit_code as u32)
            };

            Ok(std::process::Output {
                status,
                stdout: self.stdout.as_bytes().to_vec(),
                stderr: self.stderr.as_bytes().to_vec(),
            })
        }
    }

    #[tokio::test]
    async fn test_openhands_event_stream() {
        let stream = EventStream::new(10);
        let mut rx1 = stream.subscribe();
        let mut rx2 = stream.subscribe();

        let action = EventType::Action(Action::RunCommand { command: "ls -la".to_string() });
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
    async fn test_openhands_environment_worker() {
        let stream = Arc::new(EventStream::new(10));

        let mock_runner = Arc::new(MockCommandRunner {
            last_command: Arc::new(Mutex::new(None)),
            exit_code: 0,
            stdout: "mock stdout".to_string(),
            stderr: "mock stderr".to_string(),
        });

        let env = Environment::new(stream.clone(), mock_runner.clone());
        let _worker_handle = env.start();

        // Subscribe to receive the observation
        let mut rx = stream.subscribe();

        // Publish a RunCommand action
        let action = EventType::Action(Action::RunCommand { command: "echo test".to_string() });
        stream.publish(action).unwrap();

        // The first event we receive will be the Action we just published
        let recv_action = rx.recv().await.unwrap();
        assert!(matches!(recv_action, EventType::Action(_)));

        // The second event should be the Observation from the Environment worker
        let recv_obs = rx.recv().await.unwrap();

        match recv_obs {
            EventType::Observation(Observation::CommandOutput { exit_code, stdout, stderr }) => {
                assert_eq!(exit_code, 0);
                assert_eq!(stdout, "mock stdout");
                assert_eq!(stderr, "mock stderr");
            }
            _ => panic!("Expected Observation::CommandOutput"),
        }

        // Verify that the command runner was called with the correct arguments
        let last_cmd = mock_runner.last_command.lock().unwrap().clone().unwrap();
        assert_eq!(last_cmd.0, "sh");
        assert_eq!(last_cmd.1, vec!["-c", "echo test"]);
    }
}
