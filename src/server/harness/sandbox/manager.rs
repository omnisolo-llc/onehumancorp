use std::sync::Arc;

use async_trait::async_trait;
use serde_json::json;
use sqlx::PgPool;

use super::ast::ASTParser;
use super::permissions::PermissionEvaluator;
use super::wrapper::BashWrapper;
use crate::telemetry::ViolationStore;


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SandboxPolicy {
    #[serde(default)]
    pub disabled_commands: Vec<String>,
    #[serde(default)]
    pub disabled_patterns: Vec<String>,
    #[serde(default)]
    pub read_only_paths: Vec<String>,
    #[serde(default)]
    pub blocked_domains: Vec<String>,
    #[serde(default)]
    pub allowed_domains: Vec<String>,
    #[serde(default)]
    pub seccomp_fd: Option<i32>,
    #[serde(default)]
    pub socat_socket_path: Option<String>,
    #[serde(default)]
    pub socat_proxy_port: Option<u16>,
    #[serde(default)]
    pub should_use_sandbox: bool,
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        Self {
            disabled_commands: Vec::new(),
            disabled_patterns: Vec::new(),
            read_only_paths: Vec::new(),
            blocked_domains: Vec::new(),
            allowed_domains: Vec::new(),
            seccomp_fd: None,
            socat_socket_path: None,
            socat_proxy_port: None,
            should_use_sandbox: true,
        }
    }
}



#[async_trait]
pub trait SandboxAdapter: Send + Sync {
    async fn wrap_command(&self, cmd: &str) -> Result<String, String>;
    async fn update_config(&mut self, policy_json: &str) -> Result<(), String>;
    fn annotate_error(&self, err: String, stdout: String) -> String;
}

pub struct SandboxManager {
    evaluator: PermissionEvaluator,
    wrapper: BashWrapper,
    violation_store: Arc<ViolationStore>,
    policy: SandboxPolicy,
}

impl SandboxManager {
    pub fn get_policy(&self) -> SandboxPolicy {
        self.policy.clone()
    }

    pub fn should_use_sandbox(&self) -> bool {
        self.policy.should_use_sandbox
    }


    pub fn new(pool: Option<PgPool>) -> Self {
        let violation_store = Arc::new(ViolationStore::new(pool.clone()));
        SandboxManager {
            evaluator: PermissionEvaluator::new(),
            wrapper: BashWrapper::new(),
            violation_store,
            policy: SandboxPolicy::default(),
        }
    }
}

#[async_trait]
impl SandboxAdapter for SandboxManager {

    async fn wrap_command(&self, cmd: &str) -> Result<String, String> {
        if !self.should_use_sandbox() {
            return Ok(cmd.to_string());
        }

        let mut ast_parser = ASTParser::new();
        if let Err(reason) = ast_parser.parse_for_security(cmd) {
            ::server_telemetry::record_harness_security_divergence(&reason, cmd);
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

        Ok(self.wrapper.wrap(cmd))
    }

    async fn update_config(&mut self, policy_json: &str) -> Result<(), String> {
        let policy: SandboxPolicy = serde_json::from_str(policy_json)
            .map_err(|e| format!("Invalid policy JSON: {}", e))?;

        self.evaluator.update_policy(policy.clone());
        self.wrapper.update_policy(policy.clone());
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
    async fn test_sandbox_bypass() {
        let mut task = SandboxManager::new(None);
        // By default should_use_sandbox is true
        assert!(task.should_use_sandbox());

        let policy = r#"{
            "should_use_sandbox": false
        }"#;

        task.update_config(policy).await.unwrap();

        assert!(!task.should_use_sandbox());

        // This would fail in sandbox but works when bypassed
        let result = task.wrap_command("sudo rm -rf /").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "sudo rm -rf /");
    }
}
