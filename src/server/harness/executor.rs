use super::sandbox::SandboxManager;
use sqlx::PgPool;
use std::collections::HashMap;

pub struct ExecutionRequest {
    pub env: HashMap<String, String>,
    pub cwd: String,
    pub command_args: Vec<String>,
}

#[derive(Debug)]
pub struct ExecutionResponse {
    pub stdout: String,
}


pub struct LocalShellTask {
    manager: SandboxManager,
}

impl LocalShellTask {
    pub fn new(pool: Option<PgPool>) -> Self {
        LocalShellTask {
            manager: SandboxManager::new(pool),
        }
    }

    pub async fn execute(&self, req: &ExecutionRequest) -> Result<ExecutionResponse, String> {
        let args = match self.manager.wrap_command_args(&req.command_args).await {
            Ok(c) => c,
            Err(e) => return Err(self.manager.annotate_error(e, String::new())),
        };

        // If we are in tests, return the wrapped command text so we can verify the interception logic.
        // In this harness testing scope we don't really want to fork true bwrap commands which will fail in normal unprivileged pods
        if cfg!(test) {
            return Ok(ExecutionResponse {
                stdout: format!("Executing: {}", args.join(" ")),
            });
        }

        if args.is_empty() {
            return Err("Command is empty".to_string());
        }

        let mut command = tokio::process::Command::new(&args[0]);
        command.args(&args[1..]);

        if !req.cwd.is_empty() {
            command.current_dir(&req.cwd);
        }
        command.envs(&req.env);

        let output = command.output().await.map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(format!(
                "Command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(ExecutionResponse {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_allowed_command_execution() {
        let task = LocalShellTask::new(None);
        let req = ExecutionRequest {
            env: HashMap::new(),
            cwd: "/tmp".to_string(),
            command_args: vec!["echo".to_string(), "hello".to_string()],
        };
        let result = task.execute(&req).await;
        assert!(result.is_ok());
        let res = result.unwrap();
        let msg = res.stdout;

        #[cfg(target_os = "linux")]
        assert!(msg.contains("Executing: bwrap --unshare-all --share-net --ro-bind / / --bind /tmp/agent_workspace /tmp/agent_workspace --dev /dev bash -c set -e; echo hello"));

        #[cfg(target_os = "macos")]
        assert!(msg.contains("Executing: sandbox-exec -p (version 1)(allow default)(deny file-write*) bash -c set -e; echo hello"));

        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        assert!(msg.contains("Executing: bash -c set -e; echo hello"));
    }

    #[tokio::test]
    async fn test_denied_command_execution() {
        let task = LocalShellTask::new(None);
        let req = ExecutionRequest {
            env: HashMap::new(),
            cwd: "/tmp".to_string(),
            command_args: vec!["sudo".to_string(), "rm".to_string(), "-rf".to_string(), "/".to_string()],
        };
        let result = task.execute(&req).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("SANDBOX_FAILURE"));
        assert!(err.contains("Command execution denied by sandbox policy"));
    }
}
