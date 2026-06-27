use std::path::PathBuf;
use tokio::process::Command;
use async_trait::async_trait;
use crate::sandbox::session::ShellSession;

/// Hermes Agent Unique Harness Innovations: Multi-backend terminal: local, Docker, SSH, Singularity, Modal, Daytona, Vercal Sandbox
#[async_trait]
pub trait TerminalBackend: Send + Sync {
    async fn execute_command(&self, command: &str) -> Result<String, String>;
    fn name(&self) -> &'static str;
}

pub struct LocalTerminal {
    session: ShellSession,
}

impl LocalTerminal {
    pub fn new(session: ShellSession) -> Self {
        Self { session }
    }
}

#[async_trait]
impl TerminalBackend for LocalTerminal {
    async fn execute_command(&self, command: &str) -> Result<String, String> {
        let current_dir = self.session.current_cwd.read().await.clone();
        let output = Command::new("bash")
            .arg("-c")
            .arg(command)
            .current_dir(current_dir)
            .output()
            .await
            .map_err(|e| e.to_string())?;

        let mut result = String::new();
        if !output.stdout.is_empty() {
            result.push_str(&String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            result.push_str(&String::from_utf8_lossy(&output.stderr));
        }

        Ok(result)
    }

    fn name(&self) -> &'static str {
        "Local"
    }
}

pub struct DockerTerminal {
    container_name: String,
}

impl DockerTerminal {
    pub fn new(container_name: String) -> Self {
        Self { container_name }
    }
}

#[async_trait]
impl TerminalBackend for DockerTerminal {
    async fn execute_command(&self, command: &str) -> Result<String, String> {
        let output = Command::new("docker")
            .arg("exec")
            .arg(&self.container_name)
            .arg("bash")
            .arg("-c")
            .arg(command)
            .output()
            .await
            .map_err(|e| e.to_string())?;

        let mut result = String::new();
        if !output.stdout.is_empty() {
            result.push_str(&String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            result.push_str(&String::from_utf8_lossy(&output.stderr));
        }

        Ok(result)
    }

    fn name(&self) -> &'static str {
        "Docker"
    }
}
