use super::sandbox::{SandboxManager, SandboxAdapter};
use sqlx::PgPool;
use std::time::Instant;
use ::server_telemetry::{record_bubblewrap_spawn, record_bubblewrap_execution_latency, record_bubblewrap_violation, record_harness_execution_latency};


use super::network_proxy::NetworkProxy;

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

        let policy = self.manager.get_policy();
        let proxy = NetworkProxy::new(policy.allowed_domains.clone(), agent_id.to_string(), task_id.to_string());
        let (proxy_port, _shutdown_tx) = proxy.start(0).await.map_err(|e| format!("Failed to start proxy: {}", e))?;

        record_bubblewrap_spawn(agent_id, task_id);

        let start = Instant::now();

        let output = tokio::process::Command::new("bash")
            .arg("-c")
            .arg(&wrapped_cmd)
            .env("HTTP_PROXY", format!("http://127.0.0.1:{}", proxy_port))
            .env("HTTPS_PROXY", format!("http://127.0.0.1:{}", proxy_port))
            .output()
            .await
            .map_err(|e| format!("Failed to spawn process: {}", e))?;

        let exit_code = output.status.code().unwrap_or(-1);

        let latency_ms = start.elapsed().as_secs_f64() * 1000.0;
        record_bubblewrap_execution_latency(agent_id, task_id, latency_ms);

        let latency_seconds = start.elapsed().as_secs_f64();
        record_harness_execution_latency(latency_seconds);

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
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        assert!(msg.contains("Executing: bash -c \"echo 'hello'\""));
        #[cfg(target_os = "linux")]
        assert!(msg.contains("bwrap --unshare-all"));
        #[cfg(target_os = "macos")]
        assert!(msg.contains("sandbox-exec -p '"));
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

        #[cfg(target_os = "linux")]
        {
            assert!(msg.contains("export BLOCKED_DOMAINS='evil.com'"));
        }

        // MacOS translates this to subpath profile rules rather than exports
        #[cfg(target_os = "macos")]
        {
            assert!(msg.contains("sandbox-exec -p '"));
        }
    }

    #[tokio::test]
    async fn test_proxy_injection() {
        let mut task = LocalShellTask::new(None);
        // We configure a socat proxy explicitly to test proxy injection which was done by BashWrapper via this
        let policy_json = r#"{
            "socat_socket_path": "/tmp/test.sock",
            "socat_proxy_port": 8080
        }"#;

        task.update_config(policy_json).await.unwrap();

        let result = task.execute("echo 'hello'").await;
        assert!(result.is_ok());
        let output = result.unwrap();

        #[cfg(target_os = "linux")]
        {
            assert!(output.contains("socat UNIX-LISTEN:'/tmp/test.sock',fork TCP:127.0.0.1:8080 & \n"));
        }
        // MacOS doesn't currently use socat in its sandbox implementation wrapper
    }
}
