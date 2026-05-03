use super::permissions::PermissionEvaluator;
use super::wrapper::BashWrapper;
use crate::telemetry::buffer_metric;
use sqlx::PgPool;
use serde_json::json;

pub struct SandboxManager {
    evaluator: PermissionEvaluator,
    wrapper: BashWrapper,
    pool: Option<PgPool>,
}

impl SandboxManager {
    pub fn new(pool: Option<PgPool>) -> Self {
        SandboxManager {
            evaluator: PermissionEvaluator::new(),
            wrapper: BashWrapper::new(),
            pool,
        }
    }

    pub async fn wrap_command(&self, cmd: &str) -> Result<String, String> {
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
            // Record violation metrics
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

    pub fn annotate_error(&self, err: String, stdout: String) -> String {
        format!("SANDBOX_FAILURE: {}\nSTDOUT:\n{}", err, stdout)
    }
}
