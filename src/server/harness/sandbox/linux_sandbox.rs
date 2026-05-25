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

    fn generate_bwrap_args(&self) -> Vec<String> {
        let mut args = vec![
            "--unshare-pid".to_string(),
            "--bind".to_string(),
            "/".to_string(),
            "/".to_string(),
            "--dev".to_string(),
            "/dev".to_string(),
            "--proc".to_string(),
            "/proc".to_string(),
        ];

        for path in &self.policy.read_only_paths {
            args.push("--ro-bind".to_string());
            args.push(path.clone());
            args.push(path.clone());
        }

        if !self.policy.blocked_domains.is_empty() {
            args.push("--unshare-net".to_string());
        }

        args
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

        let args = self.generate_bwrap_args();
        let args_str = args.join(" ");

        // Escaping cmd string might be needed here, simplified for now
        let escaped_cmd = cmd.replace("'", "'\\''");

        Ok(format!("bwrap {} bash -c '{}'", args_str, escaped_cmd))
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
    fn test_generate_bwrap_args_default() {
        let sandbox = LinuxSandbox::new(None);
        let args = sandbox.generate_bwrap_args();
        assert!(args.contains(&"--unshare-pid".to_string()));
        assert!(args.contains(&"--bind".to_string()));
        assert!(!args.contains(&"--unshare-net".to_string()));
    }

    #[tokio::test]
    async fn test_update_config_and_args() {
        let mut sandbox = LinuxSandbox::new(None);
        let policy_json = r#"{
            "read_only_paths": ["/etc", "/var/log"],
            "blocked_domains": ["evil.com"]
        }"#;

        sandbox.update_config(policy_json).await.unwrap();
        let args = sandbox.generate_bwrap_args();

        let etc_pos = args.iter().position(|r| r == "/etc").unwrap();
        assert_eq!(args[etc_pos - 1], "--ro-bind");

        let var_log_pos = args.iter().position(|r| r == "/var/log").unwrap();
        assert_eq!(args[var_log_pos - 1], "--ro-bind");

        assert!(args.contains(&"--unshare-net".to_string()));
    }

    #[tokio::test]
    async fn test_wrap_command_allowed() {
        let sandbox = LinuxSandbox::new(None);
        let wrapped = sandbox.wrap_command("echo 'hello world'").await.unwrap();

        assert!(wrapped.starts_with("bwrap --unshare-pid --bind / / --dev /dev --proc /proc bash -c 'echo '\\''hello world'\\'''"));
    }

    #[tokio::test]
    async fn test_wrap_command_ast_denied() {
        let sandbox = LinuxSandbox::new(None);
        let result = sandbox.wrap_command("echo test > >(cat)").await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Dangerous pattern detected: >() process substitution");
    }

    #[tokio::test]
    async fn test_wrap_command_evaluator_denied() {
        let sandbox = LinuxSandbox::new(None);
        let result = sandbox.wrap_command("rm -rf /").await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Command execution denied by sandbox policy");
    }
}
