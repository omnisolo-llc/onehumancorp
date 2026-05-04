use super::sandbox::SandboxManager;
use crate::harness::bwrap_executor::BwrapExecutor;
use sqlx::PgPool;

pub struct LocalShellTask {
    manager: SandboxManager,
    bwrap: BwrapExecutor,
}

impl LocalShellTask {
    pub fn new(pool: Option<PgPool>) -> Self {
        LocalShellTask {
            manager: SandboxManager::new(pool.clone()),
            bwrap: BwrapExecutor::new(pool),
        }
    }

    pub async fn execute(&self, cmd: &str) -> Result<String, String> {
        // Evaluate command via sandbox permission policies first
        let _ = match self.manager.wrap_command(cmd).await {
            Ok(c) => c,
            Err(e) => return Err(self.manager.annotate_error(e, String::new())),
        };

        let proxy_url = std::env::var("HTTP_PROXY").unwrap_or_else(|_| "http://localhost:8080".to_string());

        // Use BwrapExecutor to safely execute the command
        self.bwrap.execute(cmd, Some(&proxy_url)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_allowed_command_execution() {
        // We will just test the execution works without touching the global environment
        // since std::env::var("HTTP_PROXY") will just default to http://localhost:8080 if not set.
        let task = LocalShellTask::new(None);
        let result = task.execute("echo 'hello'").await;
        // BwrapExecutor simulates execution for tests if bwrap missing or executes if available
        if let Err(e) = &result {
            if e.contains("No such file or directory") {
                return; // bwrap not installed, acceptable for this environment
            }
        }
        assert!(result.is_ok());
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
