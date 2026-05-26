use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::Duration;
use tempfile::{tempdir, TempDir};
use tokio::process::Command as AsyncCommand;
use tokio::time::timeout;
use anyhow::{Result, Context, anyhow};
use ::server_telemetry::{record_bubblewrap_spawn, record_bubblewrap_execution_latency, record_bubblewrap_violation};
use std::time::Instant;

#[async_trait::async_trait]
pub trait ExecutionEnvironment: Send + Sync {
    async fn execute_context(&self, command: String, work_dir: String) -> Result<String, anyhow::Error>;
}

pub struct LocalEnvironment {
    dir: TempDir,
}

impl LocalEnvironment {
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
}

#[async_trait::async_trait]
impl ExecutionEnvironment for LocalEnvironment {
    async fn execute_context(&self, command: String, work_dir: String) -> Result<String, anyhow::Error> {
        self.execute(&command, &work_dir, Duration::from_secs(30)).await
            .map(|out| String::from_utf8_lossy(&out.stdout).to_string())
    }
}

impl LocalEnvironment {
    pub async fn execute(&self, cmd: &str, work_dir: &str, timeout_dur: Duration) -> Result<Output> {
        // Wrap command for Bash execution to disable extended globs
        let wrapped_cmd = format!("shopt -u extglob 2>/dev/null || true; cd '{}'; {}", work_dir, cmd);

        let dir_str = self.dir.path().to_str()
            .ok_or_else(|| anyhow!("Failed to convert temp dir path to string"))?;

        let mut command = AsyncCommand::new("bash");
        command.arg("-c").arg(wrapped_cmd);

        // Force TMPDIR to sandbox directory
        command.env("TMPDIR", dir_str);

        // Override HOME to temporary directory for isolation
        let home_dir = self.dir.path().join(".agent-home");
        fs::create_dir_all(&home_dir).unwrap_or_default();
        command.env("HOME", home_dir.to_str().unwrap_or(dir_str));

        // Scrub sensitive environment variables
        command.env_remove("OHC_API_KEY");
        command.env_remove("GH_TOKEN");
        command.env_remove("GITHUB_TOKEN");
        command.env_remove("OTEL_EXPORTER_OTLP_HEADERS");

        record_bubblewrap_spawn("local_agent", "unknown_task");
        let start = Instant::now();
        let child = command.output();

        let result = match timeout(timeout_dur, child).await {
            Ok(output_result) => output_result.context("Failed to execute command"),
            Err(_) => {
                record_bubblewrap_violation("local_agent", "unknown_task", "execution_timeout");
                Err(anyhow!("Command execution timed out"))
            },
        };
        let latency = start.elapsed().as_secs_f64() * 1000.0;
        record_bubblewrap_execution_latency("local_agent", "unknown_task", latency);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_sandbox_execute_tmpdir() {
        let sm = LocalEnvironment::new().unwrap();
        let work_dir = sm.dir_path().to_str().unwrap().to_string();
        let output = sm.execute_context("echo $TMPDIR".to_string(), work_dir).await.unwrap();

        assert_eq!(output.trim(), sm.dir_path().to_str().unwrap());
    }

    #[tokio::test]
    async fn test_sandbox_execute_shopt() {
        let sm = LocalEnvironment::new().unwrap();
        let work_dir = sm.dir_path().to_str().unwrap().to_string();
        let output = sm.execute_context("shopt | grep extglob".to_string(), work_dir).await.unwrap();

        assert!(output.contains("extglob\toff") || output.contains("extglob        \toff") || output.contains("extglob\t off") || output.contains("extglob") && output.contains("off"));
    }

    #[tokio::test]
    async fn test_sandbox_execute_timeout() {
        let sm = LocalEnvironment::new().unwrap();
        let work_dir = sm.dir_path().to_str().unwrap().to_string();
        let result = sm.execute("sleep 1", &work_dir, Duration::from_millis(10)).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Command execution timed out");
    }

    #[tokio::test]
    async fn test_sandbox_environment_scrubbing() {
        let sm = LocalEnvironment::new().unwrap();
        let work_dir = sm.dir_path().to_str().unwrap().to_string();

        let output = sm.execute_context("echo ${GITHUB_TOKEN:-not_found}".to_string(), work_dir).await.unwrap();

        // It should output not_found because the environment variable is stripped out from the command context
        assert_eq!(output.trim(), "not_found");
    }

    #[tokio::test]
    async fn test_sandbox_home_override() {
        let sm = LocalEnvironment::new().unwrap();
        let work_dir = sm.dir_path().to_str().unwrap().to_string();

        let output = sm.execute_context("echo $HOME".to_string(), work_dir).await.unwrap();

        let expected_home = sm.dir_path().join(".agent-home");
        assert_eq!(output.trim(), expected_home.to_str().unwrap());
    }
}
