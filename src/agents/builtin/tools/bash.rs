use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;

use super::{Tool, ToolExecutor};

struct BashExecutor {
    working_dir: Option<std::path::PathBuf>,
    manager: Arc<ohc_builtin_agent_core::harness::Manager>,
}

#[async_trait::async_trait]
impl ToolExecutor for BashExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let command = args["command"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("bash: command is required".to_string()))?
            .to_string();
        let timeout_secs = args["timeout"].as_f64().unwrap_or(120.0);
        let timeout = Duration::from_secs_f64(timeout_secs.max(1.0).min(600.0));

        let mut policy = ohc_builtin_agent_core::harness::Config::default().default_policy;
        if let Some(wd) = &self.working_dir {
            policy.allowed_paths.push(wd.to_string_lossy().to_string());
            policy.working_dir = Some(wd.to_string_lossy().to_string());
            policy.allow_read.push(wd.to_string_lossy().to_string());
        }

        // Use harness manager with LocalBackend for sandboxed execution
        let output_res = tokio::time::timeout(
            timeout,
            self.manager.execute_with_policy(&command, Some(&policy), ohc_builtin_agent_core::harness::BackendType::Local)
        ).await;

        let output = output_res
            .map_err(|_| ToolError::LlmRecoverable(format!("bash: command timed out after {}s", timeout_secs)))?
            .map_err(|e| ToolError::LlmRecoverable(format!("bash: failed to execute in harness: {}", e)))?;

        let stdout = output.stdout;
        let stderr = output.stderr;

        let mut result = String::new();
        if !stdout.is_empty() {
            result.push_str(&stdout);
        }
        if !stderr.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(&format!("STDERR:\n{}", stderr));
        }

        if output.exit_code != 0 && result.is_empty() {
            result = format!(
                "Command failed with exit code {}",
                output.exit_code
            );
        }

        Ok(result)
    }
}

pub fn bash_tool(working_dir: Option<std::path::PathBuf>, manager: Arc<ohc_builtin_agent_core::harness::Manager>) -> Tool {
    Tool {
        name: "Bash".to_string(),
        description: "Execute a bash command and return its output. \
            Use for build/test/git/shell operations. \
            Commands run in the repository root."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The bash command to execute."
                },
                "timeout": {
                    "type": "number",
                    "description": "Timeout in seconds (default 120, max 600)."
                }
            },
            "required": ["command"]
        }),
        execute: Arc::new(BashExecutor { working_dir, manager }),
    }
}
