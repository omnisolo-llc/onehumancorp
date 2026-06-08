use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;

use super::{Tool, ToolExecutor};

struct ResticExecutor {
    runner: Arc<dyn crate::runner::CommandRunner>,
}

#[async_trait::async_trait]
impl ToolExecutor for ResticExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let action = args["action"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("restic: action is required (snapshot, restore, status)".to_string()))?;

        let repo = std::env::var("RESTIC_REPOSITORY").unwrap_or_else(|_| "/tmp/restic-repo".to_string());
        let password = std::env::var("RESTIC_PASSWORD").unwrap_or_else(|_| "dummy_password".to_string());

        let timeout = Duration::from_secs(300);

        let mut env_vars = vec![];
        env_vars.push(("RESTIC_PASSWORD".to_string(), password.clone()));

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
                let target = args["target"].as_str().unwrap_or(".");
                full_args.extend(vec!["backup", target]);
            }
            "restore" => {
                let snapshot_id = args["snapshot_id"].as_str().unwrap_or("latest");
                let target = args["target"].as_str().unwrap_or("/tmp/restore");
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
        execute: Arc::new(ResticExecutor { runner }),
    }
}
