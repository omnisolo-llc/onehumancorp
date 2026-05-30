use super::sandbox::{SandboxManager, SandboxAdapter, LinuxSandbox, MacOsSandbox};
use sqlx::PgPool;
use std::time::Instant;
use ::server_telemetry::{record_bubblewrap_spawn, record_bubblewrap_execution_latency, record_bubblewrap_violation, record_harness_execution_latency};
use std::collections::HashMap;

use super::network_proxy::NetworkProxy;

pub struct ExecutionRequest {
    pub cmd: String,
    pub env: HashMap<String, String>,
    pub working_dir: Option<String>,
}

pub struct ExecutionResponse {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub wrapped_cmd: String,
}

pub struct LocalShellTask {
    manager: Box<dyn SandboxAdapter>,
}

impl LocalShellTask {
    pub fn new(pool: Option<PgPool>) -> Self {
        #[cfg(target_os = "linux")]
        let manager = Box::new(LinuxSandbox::new(pool.clone()));

        #[cfg(target_os = "macos")]
        let manager = Box::new(MacOsSandbox::new(pool.clone()));

        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        let manager = Box::new(SandboxManager::new(pool.clone()));

        LocalShellTask { manager }
    }

    pub async fn update_config(&mut self, policy_json: &str) -> Result<(), String> {
        self.manager.update_config(policy_json).await
    }

    pub async fn execute(&self, req: &ExecutionRequest) -> Result<ExecutionResponse, String> {
        let wrapped_cmd = match self.manager.wrap_command(&req.cmd).await {
            Ok(c) => c,
            Err(e) => return Err(self.manager.annotate_error(e, String::new())),
        };

        // The task_id and agent_id should be dynamic in reality, but for context we use defaults if not available here
        let task_id = "unknown_task";
        let agent_id = "unknown_agent";

        let policy = self.manager.get_policy();
        let proxy = NetworkProxy::new(policy.allowed_domains.clone(), agent_id.to_string(), task_id.to_string());
        let (proxy_port, _shutdown_tx) = proxy.start(0).await.map_err(|e| format!("Failed to start proxy: {}", e))?;

        record_bubblewrap_spawn(agent_id, task_id);

        let start = Instant::now();

        let mut cmd = tokio::process::Command::new("bash");
        cmd.arg("-c").arg(&wrapped_cmd);

        cmd.env("HTTP_PROXY", format!("http://127.0.0.1:{}", proxy_port));
        cmd.env("HTTPS_PROXY", format!("http://127.0.0.1:{}", proxy_port));

        for (k, v) in &req.env {
            cmd.env(k, v);
        }

        if let Some(wd) = &req.working_dir {
            cmd.current_dir(wd);
        }

        let output = cmd.output().await.map_err(|e| format!("Failed to spawn process: {}", e))?;

        let exit_code = output.status.code().unwrap_or(-1);

        let latency_ms = start.elapsed().as_secs_f64() * 1000.0;
        record_bubblewrap_execution_latency(agent_id, task_id, latency_ms);

        let latency_seconds = start.elapsed().as_secs_f64();
        record_harness_execution_latency(latency_seconds);

        if exit_code == 13 || exit_code == 126 { // Permission denied related exit codes
            record_bubblewrap_violation(agent_id, task_id, "permission_denied");
        }

        let response = ExecutionResponse {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code,
            wrapped_cmd: wrapped_cmd.clone(),
        };

        if !output.status.success() {
            return Err(format!("Process exited with error: {}
{}", exit_code, response.stderr));
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_allowed_command_execution() {
        let task = LocalShellTask::new(None);
        let req = ExecutionRequest {
            cmd: "echo 'hello'".to_string(),
            env: HashMap::new(),
            working_dir: None,
        };
        let result = task.execute(&req).await;
        assert!(result.is_ok());
        let res = result.unwrap();
        // Just assert it succeeds, specific wrapped command depends on OS
    }

    #[tokio::test]
    async fn test_denied_command_execution() {
        let task = LocalShellTask::new(None);
        let req = ExecutionRequest {
            cmd: "sudo rm -rf /".to_string(),
            env: HashMap::new(),
            working_dir: None,
        };
        let result = task.execute(&req).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("SANDBOX_FAILURE"));
        assert!(err.contains("Command execution denied by sandbox policy"));
    }

    #[tokio::test]
    async fn test_dynamic_config_update() {
        let mut task = LocalShellTask::new(None);

        let req = ExecutionRequest {
            cmd: "curl http://example.com".to_string(),
            env: HashMap::new(),
            working_dir: None,
        };

        let result1 = task.execute(&req).await;
        assert!(result1.is_ok());

        let policy = r#"{
            "disabled_commands": ["curl"]
        }"#;

        task.update_config(policy).await.unwrap();

        let result2 = task.execute(&req).await;
        assert!(result2.is_err());

        let msg = result2.unwrap_err();
        assert!(msg.contains("Command execution denied by sandbox policy"));
    }

    #[tokio::test]
    async fn test_proxy_injection() {
        let mut task = LocalShellTask::new(None);
        let policy_json = r#"{
            "allowed_domains": ["example.com"]
        }"#;

        task.update_config(policy_json).await.unwrap();

        let req = ExecutionRequest {
            cmd: "echo $HTTP_PROXY".to_string(),
            env: HashMap::new(),
            working_dir: None,
        };

        let result = task.execute(&req).await;
        assert!(result.is_ok());
        let res = result.unwrap();
        assert!(res.stdout.contains("http://127.0.0.1:"));
    }
}
