use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::Duration;
use tempfile::{tempdir, TempDir};
use tokio::process::Command as AsyncCommand;
use tokio::time::timeout;
use anyhow::{Result, Context, anyhow};

pub struct SandboxManager {
    dir: TempDir,
}

impl SandboxManager {
    pub fn new() -> Result<Self> {
        let dir = tempdir().context("Failed to create temp directory for sandbox")?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700))
                .context("Failed to set permissions on sandbox directory")?;
        }

        Ok(Self { dir })
    }

    pub fn dir_path(&self) -> &std::path::Path {
        self.dir.path()
    }

    pub async fn execute(&self, cmd: &str, timeout_dur: Duration) -> Result<Output> {
        // Wrap command for Bash execution to disable extended globs
        let wrapped_cmd = format!("shopt -u extglob 2>/dev/null || true; {}", cmd);

        let dir_str = self.dir.path().to_str()
            .ok_or_else(|| anyhow!("Failed to convert temp dir path to string"))?;

        let mut command = AsyncCommand::new("bash");
        command.arg("-c").arg(wrapped_cmd);

        // Force TMPDIR to sandbox directory
        command.env("TMPDIR", dir_str);

        let child = command.output();

        match timeout(timeout_dur, child).await {
            Ok(output_result) => output_result.context("Failed to execute command"),
            Err(_) => Err(anyhow!("Command execution timed out")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_sandbox_execute_tmpdir() {
        let sm = SandboxManager::new().unwrap();
        let output = sm.execute("echo $TMPDIR", Duration::from_secs(5)).await.unwrap();

        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert_eq!(result, sm.dir_path().to_str().unwrap());
    }

    #[tokio::test]
    async fn test_sandbox_execute_shopt() {
        let sm = SandboxManager::new().unwrap();
        let output = sm.execute("shopt | grep extglob", Duration::from_secs(5)).await.unwrap();

        let result = String::from_utf8_lossy(&output.stdout).to_string();
        assert!(result.contains("extglob\toff") || result.contains("extglob        \toff") || result.contains("extglob\t off") || result.contains("extglob") && result.contains("off"));
    }

    #[tokio::test]
    async fn test_sandbox_execute_timeout() {
        let sm = SandboxManager::new().unwrap();
        let result = sm.execute("sleep 1", Duration::from_millis(10)).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Command execution timed out");
    }
}
