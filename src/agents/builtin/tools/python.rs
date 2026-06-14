use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Deserialize)]
struct PythonArgs {
    code: String,
    #[serde(default = "default_timeout")]
    timeout: f64,
}

fn default_timeout() -> f64 {
    30.0
}

struct PythonExecutor {
    working_dir: Option<std::path::PathBuf>,
    runner: Arc<dyn crate::runner::CommandRunner>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<PythonArgs> for PythonExecutor {
    async fn execute_typed(&self, args: PythonArgs) -> Result<String, ToolError> {
        let code = args.code;
        let timeout_secs = args.timeout;
        let timeout = Duration::from_secs_f64(timeout_secs.max(1.0).min(600.0));

        let wd = self.working_dir.clone().unwrap_or_else(std::env::temp_dir);

        let temp_file_path = wd.join(format!(".tmp_py_{}.py", uuid::Uuid::new_v4()));

        if let Err(e) = fs::write(&temp_file_path, &code).await {
            return Err(ToolError::LlmRecoverable(format!("python: failed to write code to file: {}", e)));
        }

        let wd_ref = self.working_dir.as_deref();

        // Execute python3 script
        let output_res = tokio::time::timeout(timeout, self.runner.run("python3", &[temp_file_path.to_str().unwrap()], wd_ref, vec![])).await;

        // Clean up the temp file
        let _ = fs::remove_file(&temp_file_path).await;

        let output = output_res
            .map_err(|_| ToolError::LlmRecoverable(format!("python: execution timed out after {}s", timeout_secs)))?
            .map_err(|e| format!("python: failed to execute: {}", e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        let mut stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let mut stderr = String::from_utf8_lossy(&output.stderr).to_string();

        let max_len = 30000;
        if stdout.len() > max_len {
            let mut end_idx = max_len;
            while end_idx > 0 && !stdout.is_char_boundary(end_idx) {
                end_idx -= 1;
            }
            stdout.truncate(end_idx);
            stdout.push_str("\n... [STDOUT TRUNCATED TO 30,000 CHARS]");
        }
        if stderr.len() > max_len {
            let mut end_idx = max_len;
            while end_idx > 0 && !stderr.is_char_boundary(end_idx) {
                end_idx -= 1;
            }
            stderr.truncate(end_idx);
            stderr.push_str("\n... [STDERR TRUNCATED TO 30,000 CHARS]");
        }

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

pub fn python_tool(working_dir: Option<std::path::PathBuf>, runner: Arc<dyn crate::runner::CommandRunner>) -> Tool {
    Tool {
        name: "Python".to_string(),
        description: "Execute Python code and return its output. \
            Commands run in the repository root or the provided working directory."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "code": {
                    "type": "string",
                    "description": "The Python code to execute."
                },
                "timeout": {
                    "type": "number",
                    "description": "Timeout in seconds (default 30, max 600)."
                }
            },
            "required": ["code"]
        }),
        execute: Arc::new(PydanticAdapter::new(PythonExecutor { working_dir, runner })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::mock::MockCommandRunner;

    #[tokio::test]
    async fn test_python_executor_success() {
        let runner = Arc::new(MockCommandRunner::new());
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "hello python", "")));

        let executor = PythonExecutor {
            working_dir: None,
            runner,
        };

        let args = PythonArgs {
            code: "print('hello python')".to_string(),
            timeout: 30.0,
        };

        let result = executor.execute_typed(args).await.unwrap();
        assert_eq!(result, "hello python");
    }

    #[tokio::test]
    async fn test_python_executor_stderr() {
        let runner = Arc::new(MockCommandRunner::new());
        runner.push_response(Ok(crate::runner::mock::mock_output(1, "output", "ZeroDivisionError")));

        let executor = PythonExecutor {
            working_dir: None,
            runner,
        };

        let args = PythonArgs {
            code: "1/0".to_string(),
            timeout: 30.0,
        };

        let result = executor.execute_typed(args).await.unwrap();
        assert!(result.contains("output"));
        assert!(result.contains("STDERR:\nZeroDivisionError"));
    }
}
