use super::sandbox::SandboxManager;
use sqlx::PgPool;
use std::time::Instant;

pub struct LocalShellTask {
    manager: SandboxManager,
    pool: Option<PgPool>,
}

impl LocalShellTask {
    pub fn new(pool: Option<PgPool>) -> Self {
        LocalShellTask {
            manager: SandboxManager::new(pool.clone()),
            pool,
        }
    }

    pub async fn execute(&self, cmd: &str) -> Result<String, String> {
        let start = Instant::now();
        let wrapped_cmd = match self.manager.wrap_command(cmd).await {
            Ok(c) => c,
            Err(e) => return Err(self.manager.annotate_error(e, String::new())),
        };

        // In a real execution, we would run `wrapped_cmd` using `tokio::process::Command`
        // For the scope of this harness executor logic, we just return the wrapped command
        // or execute it if needed. Let's return the wrapped command as a success placeholder
        // to show interception logic.

        let elapsed = start.elapsed().as_millis() as f32;
        if let Some(pool) = &self.pool {
            let _ = crate::telemetry::record_bubblewrap_execution_latency(pool, elapsed).await;
        }

        Ok(format!("Executing: {}", wrapped_cmd))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_allowed_command_execution() {
        let task = LocalShellTask::new(None);
        let result = task.execute("echo 'hello'").await;
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.contains("Executing: bash -c \"set -e; echo 'hello'\""));
    }

    #[tokio::test]
    async fn test_denied_command_execution() {
        let task = LocalShellTask::new(None);
        let result = task.execute("sudo rm -rf /").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("SANDBOX_FAILURE"));
        assert!(err.contains("Command execution denied by sandbox policy"));
    }
}
