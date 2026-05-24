use std::sync::Arc;
use std::os::unix::io::AsRawFd;

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
    seccomp_fd: Option<i32>,
}

impl LinuxSandbox {
    pub fn new(pool: Option<PgPool>) -> Self {
        let violation_store = Arc::new(ViolationStore::new(pool.clone()));
        LinuxSandbox {
            evaluator: PermissionEvaluator::new(),
            policy: SandboxPolicy::default(),
            violation_store,
            seccomp_fd: None,
        }
    }

    pub fn with_seccomp_fd(mut self, fd: i32) -> Self {
        self.seccomp_fd = Some(fd);
        self
    }

    fn generate_bwrap_args(&self, cmd: &str) -> Vec<String> {
        let mut args = vec![
            "--ro-bind".to_string(), "/".to_string(), "/".to_string(),
            "--dev".to_string(), "/dev".to_string(),
            "--proc".to_string(), "/proc".to_string(),
            "--unshare-all".to_string(),
            "--share-net".to_string(),
            "--tmpfs".to_string(), "/tmp".to_string(),
        ];

        for path in &self.policy.read_only_paths {
            args.push("--ro-bind".to_string());
            args.push(path.to_string());
            args.push(path.to_string());
        }

        if let Some(fd) = self.seccomp_fd {
            args.push("--seccomp".to_string());
            args.push(fd.to_string());
        }

        args.push("--".to_string());
        args.push("bash".to_string());
        args.push("-c".to_string());
        args.push(cmd.to_string());

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

        let args = self.generate_bwrap_args(cmd);
        let escaped_args: Vec<String> = args.into_iter().map(|arg| {
            if arg.contains(' ') || arg.contains('\'') {
                format!("'{}'", arg.replace("'", "'\\''"))
            } else {
                arg
            }
        }).collect();

        Ok(format!("bwrap {}", escaped_args.join(" ")))
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
        let args = sandbox.generate_bwrap_args("echo 'hello world'");
        let args_str = args.join(" ");
        assert!(args_str.contains("--ro-bind / /"));
        assert!(args_str.contains("--dev /dev"));
        assert!(args_str.contains("--proc /proc"));
        assert!(args_str.contains("--unshare-all"));
        assert!(args_str.contains("--share-net"));
        assert!(args_str.contains("--tmpfs /tmp"));
        assert!(args_str.ends_with("-- bash -c echo 'hello world'"));
        assert!(!args_str.contains("--seccomp"));
    }

    #[test]
    fn test_generate_bwrap_args_with_seccomp() {
        let sandbox = LinuxSandbox::new(None).with_seccomp_fd(11);
        let args = sandbox.generate_bwrap_args("echo 'hello world'");
        let args_str = args.join(" ");
        assert!(args_str.contains("--seccomp 11"));
    }

    #[tokio::test]
    async fn test_update_config_and_args() {
        let mut sandbox = LinuxSandbox::new(None);
        let policy_json = r#"{
            "read_only_paths": ["/etc", "/var/log"],
            "blocked_domains": ["evil.com"]
        }"#;

        sandbox.update_config(policy_json).await.unwrap();
        let args = sandbox.generate_bwrap_args("ls");
        let args_str = args.join(" ");

        assert!(args_str.contains("--ro-bind /etc /etc"));
        assert!(args_str.contains("--ro-bind /var/log /var/log"));
    }

    #[tokio::test]
    async fn test_wrap_command_allowed() {
        let sandbox = LinuxSandbox::new(None);
        let wrapped = sandbox.wrap_command("echo 'hello world'").await.unwrap();

        assert!(wrapped.starts_with("bwrap "));
        assert!(wrapped.contains("--ro-bind / /"));
        assert!(wrapped.contains("-- bash -c 'echo '\\''hello world'\\'''"));
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
