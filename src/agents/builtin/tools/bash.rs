use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;

use super::{Tool, ToolExecutor};

struct BashExecutor {
    working_dir: Option<std::path::PathBuf>,
    runner: Arc<dyn crate::runner::CommandRunner>,
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

        if let Err(e) = super::bash_security::validate_bash_command(&command) {
            return Err(ToolError::LlmRecoverable(format!("bash: {}", e)));
        }

        let timeout_secs = args["timeout"].as_f64().unwrap_or(120.0);
        let timeout = Duration::from_secs_f64(timeout_secs.max(1.0).min(600.0));

        let wd_ref = self.working_dir.as_deref();
        let output_res = tokio::time::timeout(timeout, self.runner.run("bash", &["-c", &command], wd_ref, vec![])).await;
        
        let output = output_res
            .map_err(|_| ToolError::LlmRecoverable(format!("bash: command timed out after {}s", timeout_secs)))?
            .map_err(|e| format!("bash: failed to execute: {}", e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

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

        if !output.status.success() && result.is_empty() {
            result = format!(
                "Command failed with exit code {}",
                output.status.code().unwrap_or(-1)
            );
        }

        Ok(result)
    }
}

pub fn bash_tool(working_dir: Option<std::path::PathBuf>, runner: Arc<dyn crate::runner::CommandRunner>) -> Tool {
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
        execute: Arc::new(BashExecutor { working_dir, runner }),
    }
}
