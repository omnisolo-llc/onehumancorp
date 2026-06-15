#![allow(clippy::manual_clamp)]
use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Deserialize)]
struct BashArgs {
    command: String,
    #[serde(default = "default_timeout")]
    timeout: f64,
}

fn default_timeout() -> f64 {
    120.0
}

struct BashExecutor {
    working_dir: Option<std::path::PathBuf>,
    runner: Arc<dyn crate::runner::CommandRunner>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<BashArgs> for BashExecutor {
    async fn execute_typed(&self, args: BashArgs) -> Result<String, ToolError> {
        let command = args.command;
        let timeout_secs = args.timeout;
        let timeout = Duration::from_secs_f64(timeout_secs.max(1.0).min(600.0));

        let wd_ref = self.working_dir.as_deref();
        let output_res = tokio::time::timeout(timeout, self.runner.run("bash", &["-c", &command], wd_ref, vec![])).await;
        
        let output = output_res
            .map_err(|_| ToolError::LlmRecoverable(format!("bash: command timed out after {}s", timeout_secs)))?
            .map_err(|e| format!("bash: failed to execute: {}", e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

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
        execute: Arc::new(PydanticAdapter::new(BashExecutor { working_dir, runner })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::mock::MockCommandRunner;

    #[tokio::test]
    async fn test_bash_executor_success() {
        let runner = Arc::new(MockCommandRunner::new());
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "hello bash", "")));

        let executor = BashExecutor {
            working_dir: None,
            runner,
        };

        let args = BashArgs {
            command: "echo 'hello bash'".to_string(),
            timeout: 120.0,
        };

        let result = executor.execute_typed(args).await.unwrap();
        assert_eq!(result, "hello bash");
    }

    #[tokio::test]
    async fn test_bash_executor_stderr() {
        let runner = Arc::new(MockCommandRunner::new());
        runner.push_response(Ok(crate::runner::mock::mock_output(1, "output", "some error")));

        let executor = BashExecutor {
            working_dir: None,
            runner,
        };

        let args = BashArgs {
            command: "ls -z".to_string(),
            timeout: 120.0,
        };

        let result = executor.execute_typed(args).await.unwrap();
        assert!(result.contains("output"));
        assert!(result.contains("STDERR:\nsome error"));
    }

    #[tokio::test]
    async fn test_bash_executor_timeout() {
        struct HangingRunner;
        #[async_trait::async_trait]
        impl crate::runner::CommandRunner for HangingRunner {
            async fn run(&self, _prog: &str, _args: &[&str], _cwd: Option<&std::path::Path>, _env: Vec<(String, String)>) -> std::io::Result<std::process::Output> {
                tokio::time::sleep(Duration::from_secs(5)).await;
                Ok(crate::runner::mock::mock_output(0, "", ""))
            }
        }

        let runner = Arc::new(HangingRunner);
        let executor = BashExecutor {
            working_dir: None,
            runner,
        };

        let args = BashArgs {
            command: "sleep 5".to_string(),
            timeout: 1.0, // Should timeout quickly
        };

        let result = executor.execute_typed(args).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::LlmRecoverable(msg) => assert!(msg.contains("timed out")),
            _ => panic!("Expected LlmRecoverable timeout error"),
        }
    }

    #[tokio::test]
    async fn test_bash_executor_truncation() {
        let runner = Arc::new(MockCommandRunner::new());
        // generate a huge string, including multi-byte chars to cross boundaries
        let huge_stdout = "A".repeat(29999) + "🚀" + &"A".repeat(20000);
        let huge_stderr = "B".repeat(29999) + "🚀" + &"B".repeat(20000);
        runner.push_response(Ok(crate::runner::mock::mock_output(0, &huge_stdout, &huge_stderr)));

        let executor = BashExecutor {
            working_dir: None,
            runner,
        };

        let args = BashArgs {
            command: "echo 'huge'".to_string(),
            timeout: 120.0,
        };

        let result = executor.execute_typed(args).await.unwrap();

        assert!(result.contains("... [STDOUT TRUNCATED TO 30,000 CHARS]"));
        assert!(result.contains("... [STDERR TRUNCATED TO 30,000 CHARS]"));
        assert!(result.len() <= 70000); // 30k + 30k + padding/notes
    }
}
