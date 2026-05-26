use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;
use async_trait::async_trait;
use tokio::process::Command;
use ::server_telemetry::{record_bubblewrap_spawn, record_bubblewrap_execution_latency};
use std::time::Instant;
use std::sync::OnceLock;

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

pub struct SandboxedCommandRunner {
    pub sandbox_dir: Option<PathBuf>,
}

impl SandboxedCommandRunner {
    pub fn new(sandbox_dir: Option<PathBuf>) -> Self {
        Self { sandbox_dir }
    }

    fn execution_mode() -> String {
        std::env::var("OHC_AGENT_EXECUTION_MODE")
            .or_else(|_| std::env::var("OHC_EXECUTION_MODE"))
            .or_else(|_| std::env::var("OHC_SOURCE_MODE"))
            .unwrap_or_else(|_| "standalone".to_string())
            .to_lowercase()
    }

    // Hermes Agent Unique Harness Innovations: Multi-backend terminal: local, Docker, SSH, Singularity, Modal, Daytona, Vercal Sandbox
    fn should_use_container_backend() -> bool {
        let backend = std::env::var("OHC_AGENT_COMMAND_BACKEND")
            .unwrap_or_default()
            .to_lowercase();

        matches!(
            backend.as_str(),
            "container" | "docker" | "podman" | "ssh" | "singularity" | "modal" | "daytona" | "vercal"
        ) || matches!(Self::execution_mode().as_str(), "cluster" | "cloud")
    }

    fn find_container_runtime() -> Option<String> {
        static RUNTIME: OnceLock<Option<String>> = OnceLock::new();
        RUNTIME
            .get_or_init(|| {
                let backend_env = std::env::var("OHC_AGENT_COMMAND_BACKEND").unwrap_or_default().to_lowercase();

                // If a specific multi-backend is requested, try to use it if available
                let candidates = match backend_env.as_str() {
                    "ssh" => vec!["ssh"],
                    "singularity" => vec!["singularity"],
                    "modal" => vec!["modal"],
                    "daytona" => vec!["daytona"],
                    "vercal" => vec!["vercal"],
                    _ => vec!["docker", "podman"],
                };

                for candidate in candidates {
                    let mut cmd = std::process::Command::new(candidate);
                    let arg = if candidate == "ssh" { "-V" } else { "--version" };
                    let available = cmd.arg(arg)
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .status()
                        .map(|s| s.success())
                        .unwrap_or(false);
                    if available {
                        return Some(candidate.to_string());
                    }
                }
                None
            })
            .clone()
    }

    fn shell_command(program: &str, args: &[&str]) -> String {
        if (program == "bash" || program == "sh") && args.first() == Some(&"-c") {
            return args.get(1).copied().unwrap_or_default().to_string();
        }

        let mut parts = Vec::with_capacity(args.len() + 1);
        parts.push(shell_escape(program));
        parts.extend(args.iter().map(|arg| shell_escape(arg)));
        parts.join(" ")
    }

    fn container_args(
        program: &str,
        args: &[&str],
        current_dir: Option<&Path>,
        sandbox_dir: Option<&Path>,
        envs: &[(String, String)],
        runtime: &str,
    ) -> Vec<String> {
        let command = Self::shell_command(program, args);

        // Multi-backend argument mapping based on selected runtime
        match runtime {
            "ssh" => {
                let target = std::env::var("OHC_AGENT_SSH_TARGET").unwrap_or_else(|_| "localhost".to_string());
                let mut ssh_args = vec![target];
                let mut env_prefix = String::new();
                for (key, value) in envs {
                    env_prefix.push_str(&format!("{}={} ", key, value));
                }
                ssh_args.push(format!("{} {}", env_prefix, command));
                ssh_args
            },
            "singularity" | "modal" | "daytona" | "vercal" => {
                // Placeholder mappings for these advanced multi-backends
                // They generally wrap the command in their own execution context
                let mut exec_args = vec!["exec".to_string()];

                // Add environments
                for (key, value) in envs {
                    exec_args.push("-e".to_string());
                    exec_args.push(format!("{}={}", key, value));
                }

                exec_args.push(command);
                exec_args
            },
            _ => {
                // Default Docker / Podman mapping
                let image = std::env::var("OHC_AGENT_CONTAINER_IMAGE")
                    .unwrap_or_else(|_| "alpine:3.20".to_string());
                let workspace = sandbox_dir
                    .or(current_dir)
                    .map(Path::to_path_buf)
                    .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

                let mut docker_args = vec![
                    "run".to_string(),
                    "--rm".to_string(),
                    "--network".to_string(),
                    std::env::var("OHC_AGENT_CONTAINER_NETWORK").unwrap_or_else(|_| "none".to_string()),
                    "-v".to_string(),
                    format!("{}:/workspace", workspace.display()),
                    "-w".to_string(),
                    "/workspace".to_string(),
                ];

                for (key, value) in envs {
                    docker_args.push("-e".to_string());
                    docker_args.push(format!("{}={}", key, value));
                }

                docker_args.push(image);
                docker_args.push("/bin/sh".to_string());
                docker_args.push("-lc".to_string());
                docker_args.push(command);
                docker_args
            }
        }
    }
}

fn shell_escape(value: &str) -> String {
    if value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '.' | '_' | '-' | ':' | '='))
    {
        value.to_string()
    } else {
        format!("'{}'", value.replace('\'', "'\\''"))
    }
}

#[async_trait]
impl CommandRunner for SandboxedCommandRunner {
    async fn run(
        &self,
        program: &str,
        args: &[&str],
        current_dir: Option<&Path>,
        envs: Vec<(String, String)>,
    ) -> io::Result<Output> {
        if Self::should_use_container_backend() {
            if let Some(runtime) = Self::find_container_runtime() {
                let container_args = Self::container_args(
                    program,
                    args,
                    current_dir,
                    self.sandbox_dir.as_deref(),
                    &envs,
                    &runtime,
                );
                let mut cmd = Command::new(runtime);
                cmd.args(&container_args);
                return cmd.output().await;
            }
        }

        static BWRAP_AVAILABLE: OnceLock<bool> = OnceLock::new();
        let is_bwrap_available = *BWRAP_AVAILABLE.get_or_init(|| {
            if std::env::var("TEST_WORKSPACE").is_ok() || std::env::var("BAZEL_TEST").is_ok() {
                false
            } else {
                std::process::Command::new("bwrap")
                    .arg("--version")
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false)
            }
        });

        if is_bwrap_available && self.sandbox_dir.is_some() {
            let sandbox_dir_str = self.sandbox_dir.as_ref().unwrap().to_string_lossy().to_string();

            let mut bwrap_args = vec![
                "--unshare-pid".to_string(),
                "--unshare-uts".to_string(),
                "--unshare-ipc".to_string(),
                "--unshare-cgroup".to_string(),
                "--proc".to_string(), "/proc".to_string(),
                "--dev".to_string(), "/dev".to_string(),
                "--tmpfs".to_string(), "/tmp".to_string(),
                "--ro-bind".to_string(), "/".to_string(), "/".to_string(),
                "--bind".to_string(), sandbox_dir_str.clone(), sandbox_dir_str,
                "--".to_string(),
                program.to_string(),
            ];

            for arg in args {
                bwrap_args.push(arg.to_string());
            }

            let mut bwrap_cmd = Command::new("bwrap");
            bwrap_cmd.args(&bwrap_args);

            if let Some(dir) = current_dir {
                bwrap_cmd.current_dir(dir);
            }
            bwrap_cmd.env_clear();
            bwrap_cmd.env("PATH", "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin");
            for (k, v) in envs {
                bwrap_cmd.env(k, v);
            }
            record_bubblewrap_spawn("local_agent", "unknown_task");
            let start = Instant::now();
            let output = bwrap_cmd.output().await;
            let latency = start.elapsed().as_secs_f64() * 1000.0;
            record_bubblewrap_execution_latency("local_agent", "unknown_task", latency);
            return output;
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
