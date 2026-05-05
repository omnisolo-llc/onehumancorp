use std::io;
use std::path::Path;
use std::process::Output;
use async_trait::async_trait;
use tokio::process::Command;

#[async_trait]
pub trait CommandInterceptor: Send + Sync {
    async fn check_permission(&self, tool_name: &str, command: &str) -> Result<(), String>;
}

#[async_trait]
pub trait CommandRunner: Send + Sync {
    async fn run(
        &self,
        program: &str,
        args: &[&str],
        current_dir: Option<&Path>,
        envs: Vec<(String, String)>,
    ) -> io::Result<Output>;
}

pub struct RealCommandRunner {
    interceptor: Option<std::sync::Arc<dyn CommandInterceptor>>,
}

impl RealCommandRunner {
    pub fn new(interceptor: Option<std::sync::Arc<dyn CommandInterceptor>>) -> Self {
        RealCommandRunner { interceptor }
    }
}

#[async_trait]
impl CommandRunner for RealCommandRunner {
    async fn run(
        &self,
        program: &str,
        args: &[&str],
        current_dir: Option<&Path>,
        envs: Vec<(String, String)>,
    ) -> io::Result<Output> {
        if let Some(interceptor) = &self.interceptor {
            let full_command = std::iter::once(program).chain(args.iter().copied()).collect::<Vec<_>>().join(" ");
            if let Err(e) = interceptor.check_permission("shell", &full_command).await {
                return Err(io::Error::new(io::ErrorKind::PermissionDenied, e));
            }
        }

        let mut cmd = Command::new(program);
        cmd.args(args);
        if let Some(dir) = current_dir {
            cmd.current_dir(dir);
        }
        for (k, v) in envs {
            cmd.env(k, v);
        }
        cmd.output().await
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::collections::VecDeque;

    #[derive(Clone)]
    pub struct MockCommandRunner {
        pub last_command: Arc<Mutex<Option<(String, Vec<String>)>>>,
        pub next_responses: Arc<Mutex<VecDeque<io::Result<Output>>>>,
    }

    impl MockCommandRunner {
        pub fn new() -> Self {
            Self {
                last_command: Arc::new(Mutex::new(None)),
                next_responses: Arc::new(Mutex::new(VecDeque::new())),
            }
        }

        pub fn push_response(&self, response: io::Result<Output>) {
            self.next_responses.lock().unwrap().push_back(response);
        }
    }

    #[async_trait]
    impl CommandRunner for MockCommandRunner {
        async fn run(
            &self,
            program: &str,
            args: &[&str],
            _current_dir: Option<&Path>,
            _envs: Vec<(String, String)>,
        ) -> io::Result<Output> {
            *self.last_command.lock().unwrap() = Some((program.to_string(), args.iter().map(|s| s.to_string()).collect()));
            self.next_responses.lock().unwrap().pop_front().unwrap_or_else(|| {
                // Default to success
                Ok(Output {
                    status: mock_exit_status(0),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                })
            })
        }
    }

    #[cfg(unix)]
    fn mock_exit_status(code: i32) -> std::process::ExitStatus {
        use std::os::unix::process::ExitStatusExt;
        std::process::ExitStatus::from_raw(code << 8)
    }

    #[cfg(windows)]
    fn mock_exit_status(code: i32) -> std::process::ExitStatus {
        use std::os::windows::process::ExitStatusExt;
        std::process::ExitStatus::from_raw(code as u32)
    }

    pub fn mock_output(code: i32, stdout: &str, stderr: &str) -> Output {
        Output {
            status: mock_exit_status(code),
            stdout: stdout.as_bytes().to_vec(),
            stderr: stderr.as_bytes().to_vec(),
        }
    }
}
