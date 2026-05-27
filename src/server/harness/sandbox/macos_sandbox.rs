use std::sync::Arc;

use async_trait::async_trait;
use serde_json::json;
use sqlx::PgPool;

use super::ast::ASTParser;
use super::manager::{SandboxAdapter, SandboxPolicy};
use super::permissions::PermissionEvaluator;
use super::super::telemetry::ViolationStore;

pub struct MacOsSandbox {
    evaluator: PermissionEvaluator,
    policy: SandboxPolicy,
    violation_store: Arc<ViolationStore>,
}

impl MacOsSandbox {
    pub fn new(pool: Option<PgPool>) -> Self {
        let violation_store = Arc::new(ViolationStore::new(pool.clone()));
        MacOsSandbox {
            evaluator: PermissionEvaluator::new(),
            policy: SandboxPolicy::default(),
            violation_store,
        }
    }

    fn generate_profile(&self) -> String {
        let mut profile = String::from("(version 1)\n");
        profile.push_str("(deny default)\n");
        profile.push_str("(allow file-read*)\n");
        profile.push_str("(allow process-exec)\n");
        profile.push_str("(allow network-bind)\n");
        profile.push_str("(allow network-outbound)\n");
        profile.push_str("(allow sysctl-read)\n");
        profile.push_str("(allow mach-lookup)\n");
        profile.push_str("(allow process-fork)\n");

        // allow writes except to read_only_paths
        profile.push_str("(allow file-write*)\n");

        for path in &self.policy.read_only_paths {
            profile.push_str(&format!("(deny file-write* (subpath \"{}\"))\n", path));
        }

        for domain in &self.policy.blocked_domains {
            profile.push_str(&format!("(deny network-outbound (remote ip \"{}\"))\n", domain));
        }

        profile
    }
}

#[async_trait]
impl SandboxAdapter for MacOsSandbox {
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

        let profile = self.generate_profile();
        let escaped_profile = profile.replace("'", "'\\''");
        let escaped_cmd = cmd.replace("'", "'\\''");

        Ok(format!("sandbox-exec -p '{}' bash -c '{}'", escaped_profile, escaped_cmd))
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
    fn test_generate_profile_default() {
        let sandbox = MacOsSandbox::new(None);
        let profile = sandbox.generate_profile();
        assert!(profile.contains("(version 1)"));
        assert!(profile.contains("(deny default)"));
        assert!(profile.contains("(allow file-read*)"));
        assert!(profile.contains("(allow file-write*)"));
        assert!(!profile.contains("(deny file-write*"));
    }

    #[tokio::test]
    async fn test_update_config_and_profile() {
        let mut sandbox = MacOsSandbox::new(None);
        let policy_json = r#"{
            "read_only_paths": ["/etc", "/var/log"],
            "blocked_domains": ["evil.com"]
        }"#;

        sandbox.update_config(policy_json).await.unwrap();
        let profile = sandbox.generate_profile();

        assert!(profile.contains("(deny file-write* (subpath \"/etc\"))"));
        assert!(profile.contains("(deny file-write* (subpath \"/var/log\"))"));
        assert!(profile.contains("(deny network-outbound (remote ip \"evil.com\"))"));
    }

    #[tokio::test]
    async fn test_wrap_command_allowed() {
        let sandbox = MacOsSandbox::new(None);
        let wrapped = sandbox.wrap_command("echo 'hello world'").await.unwrap();

        assert!(wrapped.starts_with("sandbox-exec -p '"));
        assert!(wrapped.contains("(version 1)"));
        assert!(wrapped.contains("bash -c 'echo '\\''hello world'\\'''"));
    }

    #[tokio::test]
    async fn test_wrap_command_ast_denied() {
        let sandbox = MacOsSandbox::new(None);
        let result = sandbox.wrap_command("echo test > >(cat)").await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Dangerous pattern detected: >() process substitution");
    }

    #[tokio::test]
    async fn test_wrap_command_evaluator_denied() {
        let sandbox = MacOsSandbox::new(None);
        let result = sandbox.wrap_command("rm -rf /").await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Command execution denied by sandbox policy");
    }
}
