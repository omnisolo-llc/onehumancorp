use std::sync::Arc;

use async_trait::async_trait;
use serde_json::json;
use sqlx::PgPool;

use super::ast::ASTParser;
use super::permissions::PermissionEvaluator;
use super::wrapper::BashWrapper;
use crate::telemetry::ViolationStore;
use redis::AsyncCommands;

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
}

#[async_trait]
pub trait SandboxAdapter: Send + Sync {
    async fn wrap_command(&self, cmd: &str) -> Result<String, String>;
    async fn update_config(&mut self, policy_json: &str) -> Result<(), String>;
    fn annotate_error(&self, err: String, stdout: String) -> String;
    async fn should_use_sandbox(&self, cmd: &str) -> bool;
}

pub struct SandboxManager {
    evaluator: PermissionEvaluator,
    wrapper: BashWrapper,
    pool: Option<PgPool>,
    violation_store: Arc<ViolationStore>,
    redis_client: Option<redis::Client>,
}

impl SandboxManager {
    pub fn new(pool: Option<PgPool>) -> Self {
        let violation_store = Arc::new(ViolationStore::new(pool.clone()));

        let redis_client = match std::env::var("REDIS_URL") {
            Ok(url) => {
                match redis::Client::open(url) {
                    Ok(client) => Some(client),
                    Err(e) => {
                        eprintln!("Failed to open redis client: {}", e);
                        None
                    }
                }
            },
            Err(_) => None,
        };

        SandboxManager {
            evaluator: PermissionEvaluator::new(),
            wrapper: BashWrapper::new(),
            pool,
            violation_store,
            redis_client,
        }
    }
}

#[async_trait]
impl SandboxAdapter for SandboxManager {
    async fn should_use_sandbox(&self, _cmd: &str) -> bool {
        if let Some(client) = &self.redis_client {
            if let Ok(mut con) = client.get_multiplexed_async_connection().await {
                let enabled: Result<bool, redis::RedisError> = con.get("ohc:harness:sandbox_enabled").await;
                if let Ok(is_enabled) = enabled {
                    return is_enabled;
                }
            }
        }
        true
    }

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

            // Record OpenTelemetry metric
            // ohc_harness_security_divergence_total
            if let Some(pool) = &self.pool {
                let labels = json!({
                    "reason": reason.clone(),
                });
                let _ = crate::telemetry::buffer_metric(
                    pool,
                    "ohc_harness_security_divergence_total",
                    "counter",
                    1.0,
                    labels,
                ).await;
            }

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
        self.wrapper.update_policy(policy);

        Ok(())
    }

    fn annotate_error(&self, err: String, stdout: String) -> String {
        format!("SANDBOX_FAILURE: {}\nSTDOUT:\n{}", err, stdout)
    }
}
