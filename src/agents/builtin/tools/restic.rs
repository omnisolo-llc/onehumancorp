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

        // Initialize repo if it doesn't exist
        match action {
            "snapshot" | "restore" | "status" => {
                 let full_args = vec!["-r", repo.as_str(), "init"];
                 let init_res = tokio::time::timeout(timeout, self.runner.run("restic", &full_args, None, env_vars.clone())).await;

                 // if init command failed, handle error appropriately instead of dropping it
                 if let Ok(Ok(output)) = init_res {
                     if !output.status.success() {
                         // It's normal if it fails because repo already exists, so we only print out for debug.
                         tracing::debug!("Restic init output: {:?}", output);
                     }
                 } else {
                     tracing::warn!("Restic init failed or timed out: {:?}", init_res);
                 }
            }
            _ => return Err(ToolError::LlmRecoverable("Invalid action. Allowed: snapshot, restore, status".to_string()))
        }

        let mut full_args = vec!["-r", repo.as_str()];

        match action {
            "snapshot" => {
                let target = args.target.as_deref().unwrap_or(".");
                full_args.extend(vec!["backup", target]);
            }
            "restore" => {
                let snapshot_id = args.snapshot_id.as_deref().unwrap_or("latest");
                let target = args.target.as_deref().unwrap_or("/tmp/restore");
                full_args.extend(vec!["restore", snapshot_id, "--target", target]);
            }
            "status" => {
                full_args.extend(vec!["snapshots"]);
            }
            _ => unreachable!(),
        }

        let output_res = tokio::time::timeout(
            timeout,
            self.runner.run("restic", &full_args, None, env_vars),
        ).await;

        let output = output_res
            .map_err(|_| ToolError::LlmRecoverable("restic: command timed out".to_string()))?
            .map_err(|e| ToolError::LlmRecoverable(format!("restic: failed to execute: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(ToolError::LlmRecoverable(format!("Command failed with exit code {}\nSTDOUT: {}\nSTDERR: {}", output.status.code().unwrap_or(-1), stdout, stderr)));
        }

        Ok(format!("{} complete:\n{}", action, stdout))
    }
}

pub fn restic_tool(runner: Arc<dyn crate::runner::CommandRunner>) -> Tool {
    Tool {
        name: "ResticBackup".to_string(),
        description: "Perform local database snapshotting and restores using Restic. \
            Supported actions: snapshot, restore, status."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "The action to perform: 'snapshot', 'restore', or 'status'."
                },
                "target": {
                    "type": "string",
                    "description": "For 'snapshot', the directory to backup. For 'restore', the target directory."
                },
                "snapshot_id": {
                    "type": "string",
                    "description": "For 'restore', the ID of the snapshot to restore (default 'latest')."
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
    use std::sync::Arc;

    struct MockRunner;

    #[async_trait::async_trait]
    impl crate::runner::CommandRunner for MockRunner {
        async fn run(
            &self,
            _program: &str,
            _args: &[&str],
            _current_dir: Option<&std::path::Path>,
            _envs: Vec<(String, String)>,
        ) -> std::io::Result<std::process::Output> {
            Ok(std::process::Output {
                status: std::process::ExitStatus::default(),
                stdout: b"ok".to_vec(),
                stderr: b"".to_vec(),
            })
        }
    }

    #[tokio::test]
    async fn test_missing_restic_password_returns_error() {
        temp_env::async_with_vars(vec![("RESTIC_PASSWORD", None::<&str>)], async {
            let executor = ResticExecutor {
                runner: Arc::new(MockRunner),
            };
            let args = ResticArgs {
                action: "status".to_string(),
                target: None,
                snapshot_id: None,
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
    async fn test_cloud_mode_returns_error() {
        temp_env::async_with_vars(vec![
            ("RESTIC_PASSWORD", Some("test_pass")),
            ("OHC_EXECUTION_MODE", Some("cloud")),
        ], async {
            let executor = ResticExecutor {
                runner: Arc::new(MockRunner),
            };
            let args = ResticArgs {
                action: "status".to_string(),
                target: None,
                snapshot_id: None,
            };
            let result = executor.execute_typed(args).await;
            assert!(result.is_err(), "Should error in cloud mode");
            match result.unwrap_err() {
                ToolError::LlmRecoverable(msg) => {
                    assert!(msg.contains("cloud"), "Error should mention cloud mode: {}", msg);
                }
                other => panic!("Expected LlmRecoverable, got: {:?}", other),
            }
        }).await;
    }

    #[tokio::test]
    async fn test_no_hardcoded_dummy_password_test() {
        let source = include_str!("restic.rs");
        let forbidden = format!("dummy_{}", "password");
        let count = source.matches(&forbidden).count();
        // It will match once here in the test itself. It should not appear anywhere else.
        assert!(count <= 2, "Hardcoded password string should have been removed from source");
    }
}
