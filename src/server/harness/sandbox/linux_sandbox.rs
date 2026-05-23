use std::sync::Arc;

use async_trait::async_trait;
use serde_json::json;
use sqlx::PgPool;

use super::ast::ASTParser;
use super::manager::{SandboxAdapter, SandboxPolicy};
use super::permissions::PermissionEvaluator;
use crate::telemetry::ViolationStore;

pub struct LinuxSandbox {
    evaluator: PermissionEvaluator,
    policy: SandboxPolicy,
    violation_store: Arc<ViolationStore>,
}

impl LinuxSandbox {
    pub fn new(pool: Option<PgPool>) -> Self {
        let violation_store = Arc::new(ViolationStore::new(pool.clone()));
        LinuxSandbox {
            evaluator: PermissionEvaluator::new(),
            policy: SandboxPolicy::default(),
            violation_store,
        }
    }
}

#[async_trait]
impl SandboxAdapter for LinuxSandbox {
    async fn wrap_command(&self, cmd: &str) -> Result<String, String> {
        let mut ast_parser = ASTParser::new();
        if let Err(reason) = ast_parser.parse_for_security(cmd) {
            let details = json!({ "command": cmd, "reason": reason });
            let _ = self.violation_store.record_violation(
                "system",
                "unknown_agent",
                "unknown_session",
                "ast_security_violation",
                details
            ).await;
            return Err(reason);
        }

        if !self.evaluator.evaluate(cmd) {
            let details = json!({ "command": cmd });
            let _ = self.violation_store.record_violation(
                "system",
                "unknown_agent",
                "unknown_session",
                "command_execution",
                details
            ).await;
            return Err("Command execution denied by sandbox policy".to_string());
        }

        let escaped_cmd = cmd.replace("'", "'\\''");

        let mut bwrap_cmd = vec![
            "bwrap".to_string(),
            "--unshare-pid".to_string(),
            "--ro-bind / /".to_string(),
            "--dev /dev".to_string(),
            "--proc /proc".to_string(),
            "--unshare-net".to_string(),
            "--bind /tmp /tmp".to_string(),
        ];

        let args = bwrap_cmd.join(" ");

        Ok(format!("{} bash -c '{}'", args, escaped_cmd))
    }

    async fn update_config(&mut self, policy_json: &str) -> Result<(), String> {
        let policy: SandboxPolicy = serde_json::from_str(policy_json)
            .map_err(|e| format!("Invalid policy JSON: {}", e))?;

        self.evaluator.update_policy(policy.clone());
        self.policy = policy;

        Ok(())
    }

    fn annotate_error(&self, err: String, stdout: String) -> String {
        format!("SANDBOX_FAILURE: {}\nSTDOUT:\n{}", err, stdout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_linux_sandbox_wrap_command() {
        let sandbox = LinuxSandbox::new(None);
        let wrapped = sandbox.wrap_command("echo 'hello'").await.unwrap();
        assert!(wrapped.starts_with("bwrap --unshare-pid --ro-bind / / --dev /dev --proc /proc --unshare-net --bind /tmp /tmp bash -c 'echo '\\''hello'\\'''"));
    }

    #[tokio::test]
    async fn test_linux_sandbox_ast_denied() {
        let sandbox = LinuxSandbox::new(None);
        let result = sandbox.wrap_command("echo test > >(cat)").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Dangerous pattern detected: >() process substitution");
    }

    #[tokio::test]
    async fn test_linux_sandbox_evaluator_denied() {
        let sandbox = LinuxSandbox::new(None);
        let result = sandbox.wrap_command("rm -rf /").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Command execution denied by sandbox policy");
    }
}
