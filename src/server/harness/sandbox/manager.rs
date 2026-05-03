use super::permissions::PermissionEvaluator;
use super::wrapper::BashWrapper;
use crate::telemetry::{record_bubblewrap_spawn_total, record_bubblewrap_violation};
use sqlx::PgPool;

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
            let _ = record_bubblewrap_spawn_total(pool, 1.0).await;
        }

        if !self.evaluator.evaluate(cmd) {
            // Record violation metrics
            if let Some(pool) = &self.pool {
                let _ = record_bubblewrap_violation(pool, 1.0, cmd).await;
            }
            return Err("Command execution denied by sandbox policy".to_string());
        }

        Ok(self.wrapper.wrap(cmd))
    }

    pub fn annotate_error(&self, err: String, stdout: String) -> String {
        format!("SANDBOX_FAILURE: {}\nSTDOUT:\n{}", err, stdout)
    }
}
