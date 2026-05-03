use super::permissions::PermissionEvaluator;
use super::wrapper::{BashWrapper, PowerShellWrapper};
use crate::telemetry::buffer_metric;
use sqlx::PgPool;
use serde_json::json;
use std::time::Duration;
use tempfile::TempDir;

pub struct SandboxManager {
    evaluator: PermissionEvaluator,
    bash_wrapper: BashWrapper,
    powershell_wrapper: PowerShellWrapper,
    pool: Option<PgPool>,
}

pub enum ShellType {
    Bash,
    PowerShell,
}

impl SandboxManager {
    pub fn new(pool: Option<PgPool>) -> Self {
        SandboxManager {
            evaluator: PermissionEvaluator::new(),
            bash_wrapper: BashWrapper::new(),
            powershell_wrapper: PowerShellWrapper::new(),
            pool,
        }
    }

    pub async fn execute(
        &self,
        cmd: &str,
        shell_type: ShellType,
        timeout_duration: Duration,
    ) -> Result<String, String> {
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
            return Err(self.annotate_error("Command execution denied by sandbox policy".to_string(), String::new()));
        }

        let temp_dir = TempDir::new().map_err(|e| format!("Failed to create temp directory: {}", e))?;

        let execution_result = match shell_type {
            ShellType::Bash => self.bash_wrapper.execute(cmd, &temp_dir, timeout_duration).await,
            ShellType::PowerShell => self.powershell_wrapper.execute(cmd, &temp_dir, timeout_duration).await,
        };

        match execution_result {
            Ok(output) => Ok(output),
            Err(e) => Err(self.annotate_error(e, String::new())),
        }
    }

    pub fn annotate_error(&self, err: String, stdout: String) -> String {
        format!("SANDBOX_FAILURE: {}\nSTDOUT:\n{}", err, stdout)
    }
}
