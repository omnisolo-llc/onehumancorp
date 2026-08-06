use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;

use super::{pydantic::{PydanticAdapter, PydanticToolExecutor}, Tool};

#[derive(Deserialize)]
struct ResticArgs {
    action: String,
    target: Option<String>,
    snapshot_id: Option<String>,
}

struct ResticExecutor {
    runner: Arc<dyn crate::runner::CommandRunner>,
}

impl ResticExecutor {
    async fn run_cmd(&self, repo: &str, cmd_args: Vec<&str>, timeout: Duration, env_vars: Vec<(String, String)>) -> Result<String, ToolError> {
        let mut final_args = vec!["-r", repo];
        final_args.extend(cmd_args);
        let res = tokio::time::timeout(timeout, self.runner.run("restic", &final_args, None, env_vars)).await;
        let output = res
            .map_err(|_| ToolError::LlmRecoverable("restic command timed out".to_string()))?
            .map_err(|e| ToolError::LlmRecoverable(format!("restic execution failed: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ToolError::LlmRecoverable(format!("restic failed: {}", stderr)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[async_trait::async_trait]
impl PydanticToolExecutor<ResticArgs> for ResticExecutor {
    async fn execute_typed(&self, args: ResticArgs) -> Result<String, ToolError> {
        let action = args.action.as_str();

        let repo = std::env::var("RESTIC_REPOSITORY").unwrap_or_else(|_| "/tmp/restic-repo".to_string());
        let password = std::env::var("RESTIC_PASSWORD").map_err(|_| {
            ToolError::LlmRecoverable(
                "RESTIC_PASSWORD environment variable is not set. \
                 Set RESTIC_PASSWORD to enable backup operations."
                    .to_string(),
            )
        })?;

        let timeout = Duration::from_secs(300);

        let env_vars = vec![("RESTIC_PASSWORD".to_string(), password.clone())];

        let mode = std::env::var("OHC_EXECUTION_MODE").unwrap_or_else(|_| "standalone".to_string());
        if mode == "cloud" {
            return Err(ToolError::LlmRecoverable("restic: unsupported in cloud mode".to_string()));
        }

        match action {
            "status" | "snapshots" => {
                self.run_cmd(&repo, vec!["snapshots"], timeout, env_vars).await
            }
            "backup" => {
                let target = args.target.as_deref().ok_or_else(|| ToolError::LlmRecoverable("Target is required for backup action".to_string()))?;
                let output = self.run_cmd(&repo, vec!["backup", target], timeout, env_vars).await?;
                Ok(format!("Backup successful:\n{}", output))
            }
            "restore" => {
                let target = args.target.as_deref().ok_or_else(|| ToolError::LlmRecoverable("Target is required for restore action".to_string()))?;
                let snapshot_id = args.snapshot_id.as_deref().unwrap_or("latest");
                let output = self.run_cmd(&repo, vec!["restore", snapshot_id, "--target", target], timeout, env_vars).await?;
                Ok(format!("Restore successful:\n{}", output))
            }
            _ => Err(ToolError::LlmRecoverable(format!("restic: unknown action '{}'", action))),
        }
    }
}

pub fn restic_tool(runner: Arc<dyn crate::runner::CommandRunner>) -> Tool {
    Tool {
        name: "Restic".to_string(),
        description: "Perform local repository backup and restore operations using restic. \
            Supported actions: 'status' (list snapshots), 'backup' (create a backup of a target directory), \
            'restore' (restore a snapshot to a target directory)."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["status", "snapshots", "backup", "restore"],
                    "description": "The restic action to perform."
                },
                "target": {
                    "type": "string",
                    "description": "The file or directory to backup, or the target directory for restore."
                },
                "snapshot_id": {
                    "type": "string",
                    "description": "The ID of the snapshot to restore. Defaults to 'latest' if omitted."
                }
            },
            "required": ["action"]
        }),
        execute: Arc::new(PydanticAdapter::new(ResticExecutor { runner })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::CommandRunner;
    use std::path::Path;

    struct MockRunner;

    #[async_trait::async_trait]
    impl CommandRunner for MockRunner {
        async fn run(
            &self,
            _program: &str,
            _args: &[&str],
            _current_dir: Option<&Path>,
            _envs: Vec<(String, String)>,
        ) -> std::io::Result<std::process::Output> {
            Ok(std::process::Output {
                status: std::os::unix::process::ExitStatusExt::from_raw(0),
                stdout: b"mock output".to_vec(),
                stderr: b"".to_vec(),
            })
        }
    }

    #[tokio::test]
    async fn test_restic_missing_password() {
        let args = ResticArgs {
            action: "status".to_string(),
            target: None,
            snapshot_id: None,
        };

        temp_env::async_with_vars(vec![("RESTIC_PASSWORD", None::<&str>)], async {
            let executor = ResticExecutor {
                runner: Arc::new(MockRunner),
            };
            let result = executor.execute_typed(args).await;
            assert!(result.is_err(), "Should error when RESTIC_PASSWORD is not set");
            match result.unwrap_err() {
                ToolError::LlmRecoverable(msg) => {
                    assert!(msg.contains("RESTIC_PASSWORD"), "Error should mention RESTIC_PASSWORD: {}", msg);
                }
                other => panic!("Expected LlmRecoverable, got: {:?}", other),
            }
        }).await;
    }

    #[tokio::test]
    async fn test_restic_cloud_mode() {
        let args = ResticArgs {
            action: "status".to_string(),
            target: None,
            snapshot_id: None,
        };

        temp_env::async_with_vars(vec![
            ("RESTIC_PASSWORD", Some("test_pass")),
            ("OHC_EXECUTION_MODE", Some("cloud")),
        ], async {
            let executor = ResticExecutor {
                runner: Arc::new(MockRunner),
            };
            let result = executor.execute_typed(args).await;
            assert!(result.is_err(), "Should error in cloud mode");
            match result.unwrap_err() {
                ToolError::LlmRecoverable(msg) => {
                    assert!(msg.contains("unsupported in cloud mode"), "Error should mention cloud mode: {}", msg);
                }
                other => panic!("Expected LlmRecoverable, got: {:?}", other),
            }
        }).await;
    }
}
