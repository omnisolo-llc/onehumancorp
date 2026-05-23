use std::io;
use std::path::{Path, PathBuf};
use std::process::Output;
use async_trait::async_trait;
use tokio::process::Command;
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

    fn should_use_container_backend() -> bool {
        matches!(
            std::env::var("OHC_AGENT_COMMAND_BACKEND")
                .unwrap_or_default()
                .to_lowercase()
                .as_str(),
            "container" | "docker" | "podman"
        ) || matches!(Self::execution_mode().as_str(), "cluster" | "cloud")
    }

    fn find_container_runtime() -> Option<String> {
        static RUNTIME: OnceLock<Option<String>> = OnceLock::new();
        RUNTIME
            .get_or_init(|| {
                for candidate in ["docker", "podman"] {
                    let available = std::process::Command::new(candidate)
                        .arg("--version")
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
    ) -> Vec<String> {
        let image = std::env::var("OHC_AGENT_CONTAINER_IMAGE")
            .unwrap_or_else(|_| "alpine:3.20".to_string());
        let workspace = sandbox_dir
            .or(current_dir)
            .map(Path::to_path_buf)
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        let command = Self::shell_command(program, args);

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

#[cfg(unix)]
fn create_exit_status(code: i32) -> std::process::ExitStatus {
    use std::os::unix::process::ExitStatusExt;
    std::process::ExitStatus::from_raw(code << 8)
}

#[cfg(windows)]
fn create_exit_status(code: i32) -> std::process::ExitStatus {
    use std::os::windows::process::ExitStatusExt;
    std::process::ExitStatus::from_raw(code as u32)
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
        let mut target_backend = None;
        let mut filtered_envs = Vec::new();
        for (k, v) in envs {
            if k == "__OHC_TARGET_BACKEND" {
                target_backend = Some(v);
            } else {
                filtered_envs.push((k, v));
            }
        }
        let envs = filtered_envs;

        let is_remote_backend = match target_backend.as_deref() {
            Some("ssh") | Some("singularity") | Some("modal") | Some("daytona") | Some("vercel") => true,
            _ => false,
        };

        if is_remote_backend {
            let backend_name = target_backend.unwrap();

            // To properly implement remote execution for these backends,
            // we would implement the integration using their respective APIs.
            // For now we return an error since the backends are not fully supported yet.
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!("The {} backend is defined in the schema but its runner logic is currently unsupported in this environment.", backend_name),
            ));
        }

        let use_container = match target_backend.as_deref() {
            Some("docker") | Some("container") => true,
            Some("local") => false,
            _ => Self::should_use_container_backend(),
        };

        if use_container {
            if let Some(runtime) = Self::find_container_runtime() {
                let container_args = Self::container_args(
                    program,
                    args,
                    current_dir,
                    self.sandbox_dir.as_deref(),
                    &envs,
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
            return bwrap_cmd.output().await;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sandboxed_runner_multi_backend_routing() {
        let runner = SandboxedCommandRunner::new(None);

        let envs = vec![
            ("__OHC_TARGET_BACKEND".to_string(), "modal".to_string())
        ];

        let out = runner.run("echo", &["hello", "world"], None, envs).await;

        assert!(out.is_err());
        let err_str = out.unwrap_err().to_string();

        // Assert the mock simulated output works for 'modal'
        assert!(err_str.contains("The modal backend is defined in the schema but its runner logic is currently unsupported"));
    }
}
