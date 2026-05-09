use super::sandbox::{SandboxManager, SandboxAdapter};
use sqlx::PgPool;

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

        // In a real execution, we would run `wrapped_cmd` using `tokio::process::Command`
        // For the scope of this harness executor logic, we just return the wrapped command
        // or execute it if needed. Let's return the wrapped command as a success placeholder
        // to show interception logic.
        Ok(format!("Executing: {}", wrapped_cmd))
    }

    pub async fn execute_with_cloud(&self, cmd: &str, cloud_transport: Option<&crate::harness::cloud_transport::CloudTransport>) -> Result<String, String> {
        let wrapped_cmd = match self.manager.wrap_command(cmd).await {
            Ok(c) => c,
            Err(e) => return Err(self.manager.annotate_error(e, String::new())),
        };

        if let Some(transport) = cloud_transport {
            // Forward execution request over CloudTransport to prevent echo chamber
            use prost::Message as ProstMessage;
            let dispatch = crate::interop::protocol::proto::JobDispatch {
                job_id: uuid::Uuid::new_v4().to_string(),
                tenant_id: "local".to_string(),
                action_name: "harness_exec".to_string(),
                payload_json: wrapped_cmd.as_bytes().to_vec(),
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
            };
            let mut buf = Vec::new();
            if dispatch.encode(&mut buf).is_ok() {
                let _ = transport.dispatch_request(buf).await;
            }
        }

        Ok(format!("Executing: {}", wrapped_cmd))
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
        assert!(msg.contains("Executing: bash -c \"set -e; echo 'hello'\""));
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
