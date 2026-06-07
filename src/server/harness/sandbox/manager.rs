use std::sync::Arc;

use async_trait::async_trait;
use serde_json::json;
use sqlx::PgPool;

use super::bash_security::ParsedCommand as ASTParser;
use super::permissions::PermissionEvaluator;
use super::wrapper::BashWrapper;
use crate::telemetry::ViolationStore;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
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

    pub fn new(pool: Option<PgPool>) -> Self {
        let violation_store = Arc::new(ViolationStore::new(pool.clone()));
        SandboxManager {
            evaluator: PermissionEvaluator::new(),
            wrapper: BashWrapper::new(),
            violation_store,
            policy: SandboxPolicy::default(),
        }
    }

    /// Sandbox determination endpoint
    /// Checks OHC-SIP Redis configurations to enable or disable sandboxing dynamically.
    pub async fn should_use_sandbox(&self, _cmd: &str) -> bool {
        // Here we could query Redis, but since this module might run without Redis
        // we can check environment variables or rely on a Redis client.
        // For OHC-SIP dynamic toggling, if OHC_SANDBOX_ENABLED is false, we bypass.
        if std::env::var("OHC_SANDBOX_ENABLED").unwrap_or_else(|_| "true".to_string()) == "false" {
            return false;
        }

        // We can also connect to Redis directly if REDIS_URL is provided
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                if let Ok(mut con) = client.get_multiplexed_async_connection().await {
                    let enabled: Result<String, _> = redis::cmd("GET")
                        .arg("ohc:sandbox:enabled")
                        .query_async(&mut con)
                        .await;
                    if let Ok(val) = enabled {
                        if val == "false" {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

#[async_trait]
impl SandboxAdapter for SandboxManager {
    async fn wrap_command(&self, cmd: &str) -> Result<String, String> {
        let mut ast_parser = ASTParser::new();
        let ast_result = ast_parser.parse_for_security(cmd);
        let evaluator_result = self.evaluator.evaluate(cmd);

        // Record divergence if AST validation and Evaluator validation disagree
        if ast_result.is_err() != !evaluator_result {
            self.violation_store.record_divergence();
        }

        if let Err(reason) = ast_result {
            let details = json!({ "command": cmd, "reason": reason });
            let _ = self
                .violation_store
                .record_violation(
                    "system",
                    "unknown_agent",
                    "unknown_session",
                    "ast_security_violation",
                    details,
                )
                .await;
            return Err(reason);
        }

        if !evaluator_result {
            let details = json!({ "command": cmd });
            let _ = self
                .violation_store
                .record_violation(
                    "system",
                    "unknown_agent",
                    "unknown_session",
                    "command_execution",
                    details,
                )
                .await;
            return Err("Command execution denied by sandbox policy".to_string());
        }

        Ok(self.wrapper.wrap(cmd))
    }

    async fn update_config(&mut self, policy_json: &str) -> Result<(), String> {
        let policy: SandboxPolicy =
            serde_json::from_str(policy_json).map_err(|e| format!("Invalid policy JSON: {}", e))?;

        self.evaluator.update_policy(policy.clone());
        self.wrapper.update_policy(policy.clone());
        self.policy = policy;

        Ok(())
    }

    fn annotate_error(&self, err: String, stdout: String) -> String {
        format!("SANDBOX_FAILURE: {}\nSTDOUT:\n{}", err, stdout)
    }
}
