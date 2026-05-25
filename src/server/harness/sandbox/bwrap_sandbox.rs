use std::sync::Arc;
use std::env;

use async_trait::async_trait;
use serde_json::json;
use sqlx::PgPool;

use super::ast::ASTParser;
use super::manager::{SandboxAdapter, SandboxPolicy};
use super::permissions::PermissionEvaluator;
use crate::telemetry::ViolationStore;

pub struct BwrapSandbox {
    evaluator: PermissionEvaluator,
    policy: SandboxPolicy,
    violation_store: Arc<ViolationStore>,
}

impl BwrapSandbox {
    pub fn new(pool: Option<PgPool>) -> Self {
        let violation_store = Arc::new(ViolationStore::new(pool.clone()));
        BwrapSandbox {
            evaluator: PermissionEvaluator::new(),
            policy: SandboxPolicy::default(),
            violation_store,
        }
    }
}

#[async_trait]
impl SandboxAdapter for BwrapSandbox {
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

        let cwd = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/"));
        let cwd_str = cwd.to_str().unwrap_or("/");

        // e.g. bwrap --unshare-net --unshare-pid --dev /dev --ro-bind / / ...
        let mut args = vec![
            "bwrap".to_string(),
            "--unshare-net".to_string(),
            "--unshare-pid".to_string(),
            "--dev".to_string(), "/dev".to_string(),
            "--ro-bind".to_string(), "/".to_string(), "/".to_string(),
            // Allow writing to the current directory using absolute path
            "--bind".to_string(), cwd_str.to_string(), cwd_str.to_string(),
            // Allow writing to /tmp
            "--bind".to_string(), "/tmp".to_string(), "/tmp".to_string(),
        ];

        for path in &self.policy.read_only_paths {
            let escaped_path = path.replace("'", "'\\''");
            args.push("--ro-bind".to_string());
            args.push(format!("'{}'", escaped_path));
            args.push(format!("'{}'", escaped_path));
        }

        args.push("--".to_string());
        args.push("bash".to_string());
        args.push("-c".to_string());

        let escaped_cmd = cmd.replace("'", "'\\''");
        args.push(format!("'{}'", escaped_cmd));

        Ok(args.join(" "))
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

    #[test]
    fn test_bwrap_sandbox_new() {
        let sandbox = BwrapSandbox::new(None);
        assert!(sandbox.policy.read_only_paths.is_empty());
    }

    #[tokio::test]
    async fn test_wrap_command_allowed() {
        let sandbox = BwrapSandbox::new(None);
        let wrapped = sandbox.wrap_command("echo 'hello world'").await.unwrap();

        assert!(wrapped.starts_with("bwrap --unshare-net --unshare-pid --dev /dev --ro-bind / / --bind "));
        assert!(wrapped.contains("--bind /tmp /tmp"));
        assert!(wrapped.contains("bash -c 'echo '\\''hello world'\\'''"));
    }

    #[tokio::test]
    async fn test_wrap_command_ast_denied() {
        let sandbox = BwrapSandbox::new(None);
        let result = sandbox.wrap_command("echo test > >(cat)").await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Dangerous pattern detected: >() process substitution");
    }

    #[tokio::test]
    async fn test_wrap_command_evaluator_denied() {
        let sandbox = BwrapSandbox::new(None);
        let result = sandbox.wrap_command("rm -rf /").await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Command execution denied by sandbox policy");
    }

    #[tokio::test]
    async fn test_update_config() {
        let mut sandbox = BwrapSandbox::new(None);
        let policy_json = r#"{
            "read_only_paths": ["/etc", "/var/log"]
        }"#;

        sandbox.update_config(policy_json).await.unwrap();
        let wrapped = sandbox.wrap_command("ls").await.unwrap();

        assert!(wrapped.contains("--ro-bind '/etc' '/etc'"));
        assert!(wrapped.contains("--ro-bind '/var/log' '/var/log'"));
    }
}
