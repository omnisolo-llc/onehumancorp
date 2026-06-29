use std::path::PathBuf;
use tokio::process::Command;
use async_trait::async_trait;
use crate::sandbox::session::ShellSession;

/// Master Catalog Harness Innovation: Multi-backend terminal: local, Docker, SSH, Singularity, Modal, Daytona, Vercal Sandbox
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

pub struct SshTerminal {
    pub host: String,
    pub user: String,
}

impl SshTerminal {
    pub fn new(host: String, user: String) -> Self {
        Self { host, user }
    }
}

#[async_trait]
impl TerminalBackend for SshTerminal {
    async fn execute_command(&self, command: &str) -> Result<String, String> {
        let output = Command::new("ssh")
            .arg(format!("{}@{}", self.user, self.host))
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
        "SSH"
    }
}

pub struct SingularityTerminal {
    pub image_path: String,
}

impl SingularityTerminal {
    pub fn new(image_path: String) -> Self {
        Self { image_path }
    }
}

#[async_trait]
impl TerminalBackend for SingularityTerminal {
    async fn execute_command(&self, command: &str) -> Result<String, String> {
        let output = Command::new("singularity")
            .arg("exec")
            .arg(&self.image_path)
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
        "Singularity"
    }
}

pub struct ModalTerminal {
    pub app_name: String,
}

impl ModalTerminal {
    pub fn new(app_name: String) -> Self {
        Self { app_name }
    }
}

#[async_trait]
impl TerminalBackend for ModalTerminal {
    async fn execute_command(&self, command: &str) -> Result<String, String> {
        let output = Command::new("modal")
            .arg("run")
            .arg(&self.app_name)
            .arg("--cmd")
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
        "Modal"
    }
}

pub struct DaytonaTerminal {
    pub workspace: String,
}

impl DaytonaTerminal {
    pub fn new(workspace: String) -> Self {
        Self { workspace }
    }
}

#[async_trait]
impl TerminalBackend for DaytonaTerminal {
    async fn execute_command(&self, command: &str) -> Result<String, String> {
        let output = Command::new("daytona")
            .arg("run")
            .arg("-w")
            .arg(&self.workspace)
            .arg("--")
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
        "Daytona"
    }
}

pub struct VercelSandboxTerminal {
    pub project_id: String,
}

impl VercelSandboxTerminal {
    pub fn new(project_id: String) -> Self {
        Self { project_id }
    }
}

#[async_trait]
impl TerminalBackend for VercelSandboxTerminal {
    async fn execute_command(&self, command: &str) -> Result<String, String> {
        let output = Command::new("vercel")
            .arg("exec")
            .arg(&self.project_id)
            .arg("--")
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
        "VercelSandbox"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docker_terminal() {
        let terminal = DockerTerminal::new("my-container".to_string());
        assert_eq!(terminal.name(), "Docker");
        assert_eq!(terminal.container_name, "my-container");
    }

    #[test]
    fn test_ssh_terminal() {
        let terminal = SshTerminal::new("example.com".to_string(), "root".to_string());
        assert_eq!(terminal.name(), "SSH");
        assert_eq!(terminal.host, "example.com");
        assert_eq!(terminal.user, "root");
    }

    #[test]
    fn test_singularity_terminal() {
        let terminal = SingularityTerminal::new("/path/to/image.sif".to_string());
        assert_eq!(terminal.name(), "Singularity");
        assert_eq!(terminal.image_path, "/path/to/image.sif");
    }

    #[test]
    fn test_modal_terminal() {
        let terminal = ModalTerminal::new("my-app".to_string());
        assert_eq!(terminal.name(), "Modal");
        assert_eq!(terminal.app_name, "my-app");
    }

    #[test]
    fn test_daytona_terminal() {
        let terminal = DaytonaTerminal::new("my-workspace".to_string());
        assert_eq!(terminal.name(), "Daytona");
        assert_eq!(terminal.workspace, "my-workspace");
    }

    #[test]
    fn test_vercel_sandbox_terminal() {
        let terminal = VercelSandboxTerminal::new("my-project-id".to_string());
        assert_eq!(terminal.name(), "VercelSandbox");
        assert_eq!(terminal.project_id, "my-project-id");
    }
}
