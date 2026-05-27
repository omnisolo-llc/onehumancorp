use ::server_telemetry::{record_harness_execution_latency};
use super::sandbox::{SandboxManager, SandboxAdapter};
use sqlx::PgPool;
use std::time::Instant;
use ::server_telemetry::{record_bubblewrap_spawn, record_bubblewrap_execution_latency, record_bubblewrap_violation};

pub struct LocalShellTask {
    manager: SandboxManager,
}

impl LocalShellTask {
    pub fn new(pool: Option<PgPool>) -> Self {
        LocalShellTask {
            manager: SandboxManager::new(pool),
        }
    }

    pub async fn update_config(&mut self, policy_json: &str) -> Result<(), String> {
        self.manager.update_config(policy_json).await
    }

    pub async fn execute(&self, cmd: &str) -> Result<String, String> {
        let wrapped_cmd = match self.manager.wrap_command(cmd).await {
            Ok(c) => c,
            Err(e) => return Err(self.manager.annotate_error(e, String::new())),
        };

        // The task_id and agent_id should be dynamic in reality, but for context we use defaults if not available here
        let task_id = "unknown_task";
        let agent_id = "unknown_agent";

        record_bubblewrap_spawn(agent_id, task_id);
        let start = Instant::now();

        let output = tokio::process::Command::new("bash")
            .arg("-c")
            .arg(&wrapped_cmd)
            .output()
            .await
            .map_err(|e| format!("Failed to spawn process: {}", e))?;

        let exit_code = output.status.code().unwrap_or(-1);

        let latency = start.elapsed().as_secs_f64() * 1000.0;
        let latency_seconds = start.elapsed().as_secs_f64();
        record_harness_execution_latency(::server_telemetry::get_deployment_mode(), latency_seconds);
        record_bubblewrap_execution_latency(agent_id, task_id, latency);

        if exit_code == 13 || exit_code == 126 { // Permission denied related exit codes
            record_bubblewrap_violation(agent_id, task_id, "permission_denied");
        }

        if !output.status.success() {
            return Err(format!("Process exited with error: {}\n{}", exit_code, String::from_utf8_lossy(&output.stderr)));
        }

        Ok(format!("Executing: {}\n{}", wrapped_cmd, String::from_utf8_lossy(&output.stdout)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_allowed_command_execution() {
        let task = LocalShellTask::new(None);
        let result = task.execute("echo 'hello'").await;
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.contains("Executing: bash -c \"set -e; umask 077; echo 'hello'\""));
    }

    #[tokio::test]
    async fn test_denied_command_execution() {
        let task = LocalShellTask::new(None);
        let result = task.execute("sudo rm -rf /").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("SANDBOX_FAILURE"));
        assert!(err.contains("Command execution denied by sandbox policy"));
    }

    #[tokio::test]
    async fn test_dynamic_config_update() {
        let mut task = LocalShellTask::new(None);

        let result1 = task.execute("curl http://example.com").await;
        assert!(result1.is_ok());

        let policy = r#"{
            "disabled_commands": ["curl"]
        }"#;

        task.update_config(policy).await.unwrap();

        let result2 = task.execute("curl http://example.com").await;
        assert!(result2.is_err());

        let msg = result2.unwrap_err();
        assert!(msg.contains("Command execution denied by sandbox policy"));
    }

    #[tokio::test]
    async fn test_dynamic_config_wrapper_update() {
        let mut task = LocalShellTask::new(None);

        let policy = r#"{
            "read_only_paths": ["/etc", "/var"],
            "blocked_domains": ["evil.com"]
        }"#;

        task.update_config(policy).await.unwrap();

        let result = task.execute("echo 'hello'").await;
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.contains("export READ_ONLY_PATHS='/etc:/var'"));
        assert!(msg.contains("export BLOCKED_DOMAINS='evil.com'"));
    }
}
