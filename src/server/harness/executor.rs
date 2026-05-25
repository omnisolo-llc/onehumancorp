use super::sandbox::{SandboxManager, SandboxAdapter, MacOsSandbox, BwrapSandbox};
use sqlx::PgPool;
use std::time::Instant;
use ::server_telemetry::{record_bubblewrap_spawn, record_bubblewrap_execution_latency, record_bubblewrap_violation};

pub struct LocalShellTask {
    manager: Box<dyn SandboxAdapter>,
}

impl LocalShellTask {
    pub fn new(pool: Option<PgPool>) -> Self {
        #[cfg(target_os = "linux")]
        let manager = Box::new(BwrapSandbox::new(pool));

        #[cfg(target_os = "macos")]
        let manager = Box::new(MacOsSandbox::new(pool));

        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        let manager = Box::new(SandboxManager::new(pool));

        LocalShellTask { manager }
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

        let mut output_str = String::new();
        let mut err_str = String::new();

        // Use a mock exit code fallback for tests if tokio command isn't meant to run.
        // We'll run the command here. If it fails to spawn or is killed by a signal, exit_code will be -1.
        let exit_code = match tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&wrapped_cmd)
            .output()
            .await
        {
            Ok(out) => {
                output_str = String::from_utf8_lossy(&out.stdout).to_string();
                err_str = String::from_utf8_lossy(&out.stderr).to_string();
                out.status.code().unwrap_or(-1)
            },
            Err(e) => {
                err_str = e.to_string();
                -1
            }
        };

        let latency = start.elapsed().as_secs_f64() * 1000.0;
        record_bubblewrap_execution_latency(agent_id, task_id, latency);

        if exit_code == 13 || exit_code == 126 { // Permission denied related exit codes
            record_bubblewrap_violation(agent_id, task_id, "permission_denied");
            return Err(self.manager.annotate_error("Permission denied by sandbox".into(), format!("{}\n{}", output_str, err_str)));
        }

        if exit_code != 0 {
             return Err(self.manager.annotate_error(format!("Command failed with code {}", exit_code), format!("{}\n{}", output_str, err_str)));
        }

        Ok(output_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock command execution tests for SandboxManager
    // In CI environments without bwrap, running an actual command wrapped in bwrap will fail.
    // However, the test only checks the AST parser/Sandbox logic by injecting restricted commands
    // and ensuring `execute` returns the correct err.

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
        // Fallback test for SandboxManager injection
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            let mut task = LocalShellTask::new(None);

            let policy = r#"{
                "read_only_paths": ["/etc", "/var"],
                "blocked_domains": ["evil.com"]
            }"#;

            task.update_config(policy).await.unwrap();

            let result = task.execute("echo 'hello'").await;
            assert!(result.is_ok());
        }
    }
}

#[cfg(test)]
mod e2e_tests {
    use super::*;

    #[tokio::test]
    async fn test_restricted_command_fails_gracefully() {
        let mut task = LocalShellTask::new(None);

        let policy = r#"{
            "disabled_commands": ["cat /etc/shadow", "rm -rf /"]
        }"#;

        task.update_config(policy).await.unwrap();

        let result = task.execute("cat /etc/shadow").await;

        assert!(result.is_err(), "Expected restricted command to fail gracefully");

        let err_msg = result.unwrap_err();
        // Here we just test that the policy denied it, and execute wrapped it in a SANDBOX_FAILURE via annotate_error
        assert!(err_msg.contains("SANDBOX_FAILURE"));
        assert!(err_msg.contains("Command execution denied by sandbox policy"));
    }
}
