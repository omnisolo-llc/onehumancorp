use super::bridge::transport::PermissionInterceptor;
use super::sandbox::{SandboxManager, SandboxAdapter};
use sqlx::PgPool;
use std::sync::Arc;

pub struct LocalShellTask {
    manager: SandboxManager,
    interceptor: Option<Arc<PermissionInterceptor>>,
}

impl LocalShellTask {
    pub fn new(pool: Option<PgPool>) -> Self {
        LocalShellTask {
            manager: SandboxManager::new(pool),
            interceptor: None,
        }
    }

    pub fn with_interceptor(mut self, interceptor: Arc<PermissionInterceptor>) -> Self {
        self.interceptor = Some(interceptor);
        self
    }

    pub async fn update_config(&mut self, policy_json: &str) -> Result<(), String> {
        self.manager.update_config(policy_json).await
    }

    pub async fn execute(&self, cmd: &str) -> Result<String, String> {
        if let Some(interceptor) = &self.interceptor {
            use ohc_builtin_agent::tools::runner::CommandInterceptor;
            interceptor.check_permission("shell", cmd).await?;
        }

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

    use super::super::bridge::permission::{BridgeTransport, PermissionRequest, AuthorizationResponse};
    use async_trait::async_trait;

    struct MockBridgeTransport {
        authorize: bool,
    }

    #[async_trait]
    impl BridgeTransport for MockBridgeTransport {
        async fn request_permission(&self, _req: PermissionRequest) -> Result<AuthorizationResponse, String> {
            Ok(AuthorizationResponse {
                authorized: self.authorize,
                reason: if self.authorize { None } else { Some("User denied permission".to_string()) },
            })
        }
    }

    #[tokio::test]
    async fn test_executor_with_interceptor_authorized() {
        let transport = Arc::new(MockBridgeTransport { authorize: true });
        let interceptor = Arc::new(PermissionInterceptor::new(transport, "session_1".to_string()));

        let task = LocalShellTask::new(None).with_interceptor(interceptor);

        let result = task.execute("echo 'allowed'").await;
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.contains("Executing: bash -c \"set -e; echo 'allowed'\""));
    }

    #[tokio::test]
    async fn test_executor_with_interceptor_denied() {
        let transport = Arc::new(MockBridgeTransport { authorize: false });
        let interceptor = Arc::new(PermissionInterceptor::new(transport, "session_1".to_string()));

        let task = LocalShellTask::new(None).with_interceptor(interceptor);

        let result = task.execute("echo 'denied'").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "User denied permission");
    }
}
