use super::permissions::PermissionEvaluator;
use super::wrapper::BashWrapper;
use crate::harness::telemetry::ViolationStore;
use crate::telemetry::buffer_metric;
use sqlx::PgPool;
use serde_json::json;
use async_trait::async_trait;
use std::sync::Arc;
use opentelemetry::{global, KeyValue};

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
}

pub struct SandboxManager {
    evaluator: PermissionEvaluator,
    wrapper: BashWrapper,
    pool: Option<PgPool>,
    violation_store: Arc<ViolationStore>,
}

impl SandboxManager {
    pub fn new(pool: Option<PgPool>) -> Self {
        let violation_store = Arc::new(ViolationStore::new(pool.clone()));
        SandboxManager {
            evaluator: PermissionEvaluator::new(),
            wrapper: BashWrapper::new(),
            pool,
            violation_store,
        }
    }
}

#[async_trait]
impl SandboxAdapter for SandboxManager {
    #[tracing::instrument(skip(self), name = "wrap_command")]
    async fn wrap_command(&self, cmd: &str) -> Result<String, String> {
        // Record wrapping metrics if pool available
        if let Some(pool) = &self.pool {
            let labels = json!({ "command": cmd });
            let _ = buffer_metric(
                pool,
                "harness.sandbox.wrapped_executions",
                "counter",
                1.0,
                labels,
            ).await;
        }

        if !self.evaluator.evaluate(cmd) {
            // Emit OpenTelemetry counter for sandbox violations
            let meter = global::meter("ohc.harness.telemetry");
            let violation_counter = meter.u64_counter("ohc_sandbox_violation_total").build();
            violation_counter.add(1, &[
                KeyValue::new("violation_type", "blocked_command"),
                KeyValue::new("harness_mode", "standalone"),
            ]);

            // Record violation via ViolationStore
            let details = json!({ "command": cmd });
            let _ = self.violation_store.record_violation(
                "system", // Tenant ID, default to system since manager doesn't have context
                "unknown_agent", // SandboxManager doesn't have agent context natively here yet
                "unknown_session",
                "command_execution",
                details
            ).await;

            // Legacy metric (keep for backwards compatibility if needed, or remove, but we'll keep for safety)
            if let Some(pool) = &self.pool {
                let labels = json!({ "command": cmd });
                let _ = buffer_metric(
                    pool,
                    "harness.sandbox.violations",
                    "counter",
                    1.0,
                    labels,
                ).await;
            }
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
