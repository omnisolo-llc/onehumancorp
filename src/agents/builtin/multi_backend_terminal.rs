/// Hermes Agent Unique Harness Innovations: Multi-backend terminal: local, Docker, SSH, Singularity, Modal, Daytona, Vercal Sandbox
use async_trait::async_trait;
use std::process::Stdio;
use tokio::process::Command;

/// Represents the execution result of a command run within a terminal backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// A trait abstracting terminal execution backends.
#[async_trait]
pub trait TerminalBackend: Send + Sync {
    /// Executes a command string and returns its result.
    async fn execute_command(&self, command: &str) -> Result<CommandResult, String>;

    /// Returns the name of the backend (e.g., "local", "docker", "ssh").
    fn name(&self) -> &'static str;
}

/// Local terminal execution backend.
pub struct LocalTerminal {
    working_dir: std::path::PathBuf,
}

impl LocalTerminal {
    pub fn new(working_dir: std::path::PathBuf) -> Self {
        Self { working_dir }
    }
}

#[async_trait]
impl TerminalBackend for LocalTerminal {
    async fn execute_command(&self, command: &str) -> Result<CommandResult, String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(&self.working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| format!("Failed to execute local command: {}", e))?;

        Ok(CommandResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }

    fn name(&self) -> &'static str {
        "local"
    }
}

/// Docker terminal execution backend.
pub struct DockerTerminal {
    container_name: String,
}

impl DockerTerminal {
    pub fn new(container_name: &str) -> Self {
        Self {
            container_name: container_name.to_string(),
        }
    }
}

#[async_trait]
impl TerminalBackend for DockerTerminal {
    async fn execute_command(&self, command: &str) -> Result<CommandResult, String> {
        // execute via: docker exec <container_name> sh -c <command>
        let output = Command::new("docker")
            .args(["exec", &self.container_name, "sh", "-c", command])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        Ok(CommandResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }

    fn name(&self) -> &'static str {
        "docker"
    }
}

/// SSH terminal execution backend.
pub struct SshTerminal {
    host: String,
    user: String,
}

impl SshTerminal {
    pub fn new(host: &str, user: &str) -> Self {
        Self {
            host: host.to_string(),
            user: user.to_string(),
        }
    }
}

#[async_trait]
impl TerminalBackend for SshTerminal {
    async fn execute_command(&self, command: &str) -> Result<CommandResult, String> {
        let destination = format!("{}@{}", self.user, self.host);
        let output = Command::new("ssh")
            .args([&destination, command])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| format!("Failed to execute ssh command: {}", e))?;

        Ok(CommandResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }

    fn name(&self) -> &'static str {
        "ssh"
    }
}

/// Singularity, Modal, Daytona, Vercel Sandbox stubs could be implemented similarly.
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_local_terminal_execution() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "hello terminal").unwrap();

        let terminal = LocalTerminal::new(dir.path().to_path_buf());
        let result = terminal.execute_command("cat test.txt").await.unwrap();

        assert_eq!(result.exit_code, 0);
        assert_eq!(result.stdout.trim(), "hello terminal");
        assert_eq!(result.stderr, "");
        assert_eq!(terminal.name(), "local");
    }

    #[tokio::test]
    async fn test_local_terminal_failure() {
        let dir = tempdir().unwrap();
        let terminal = LocalTerminal::new(dir.path().to_path_buf());
        let result = terminal.execute_command("ls non_existent_file_12345").await.unwrap();

        assert_ne!(result.exit_code, 0);
        assert!(result.stderr.contains("non_existent_file_12345") || result.stderr.contains("No such file"));
    }

    // Note: In a real environment, we'd mock the `tokio::process::Command` calls for Docker and SSH testing,
    // or run integration tests. For unit tests, we'll verify they correctly instantiate and return names.
    #[test]
    fn test_docker_terminal_init() {
        let terminal = DockerTerminal::new("my-container");
        assert_eq!(terminal.name(), "docker");
        assert_eq!(terminal.container_name, "my-container");
    }

    #[test]
    fn test_ssh_terminal_init() {
        let terminal = SshTerminal::new("192.168.1.10", "admin");
        assert_eq!(terminal.name(), "ssh");
        assert_eq!(terminal.host, "192.168.1.10");
        assert_eq!(terminal.user, "admin");
    }
}
