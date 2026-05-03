use super::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::process::Command;

pub struct BrowserExecutor {
    working_dir: Option<std::path::PathBuf>,
}

#[async_trait::async_trait]
impl ToolExecutor for BrowserExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let url = args["url"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("browser: url is required".to_string()))?;

        let output_path = args["output_path"]
            .as_str()
            .unwrap_or(".agent-task/screenshots/screenshot.png");

        let mut cmd = Command::new("npx");
        cmd.arg("--yes").arg("playwright").arg("screenshot").arg(url).arg(output_path);

        if let Some(true) = args["full_page"].as_bool() {
            cmd.arg("--full-page");
        }

        if let Some(wd) = &self.working_dir {
            cmd.current_dir(wd);
        }

        let output = cmd.output().await.map_err(|e| ToolError::LlmRecoverable(format!("Failed to execute playwright: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ToolError::LlmRecoverable(format!("Playwright failed: {}", stderr)));
        }

        Ok(format!("Screenshot successfully captured and saved to {}", output_path))
    }
}

pub fn browser_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "Browser".to_string(),
        description: "Captures a screenshot of a given URL using Playwright for visual verification of frontend/UI state.".to_string(),
        is_read_only: true, // While it writes to disk, it's considered an observation/read operation for the harness logic
        parameters: json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to screenshot."
                },
                "output_path": {
                    "type": "string",
                    "description": "The file path where the screenshot will be saved (e.g., .agent-task/screenshots/output.png)."
                },
                "full_page": {
                    "type": "boolean",
                    "description": "Optional flag to take a full page screenshot instead of just the viewport."
                }
            },
            "required": ["url"]
        }),
        execute: Arc::new(BrowserExecutor { working_dir }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_browser_missing_url() {
        let executor = BrowserExecutor { working_dir: None };
        let args = json!({});
        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert_eq!(msg, "browser: url is required");
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }

    #[tokio::test]
    async fn test_browser_playwright_failure() {
        let executor = BrowserExecutor { working_dir: None };
        // We'll pass a definitely invalid URL so Playwright fails
        let args = json!({
            "url": "http://this-url-definitely-does-not-exist-123456789.com"
        });

        let result = executor.execute(args).await;

        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Playwright failed") || msg.contains("Failed to execute playwright"));
        } else {
            panic!("Expected LlmRecoverable error for failing playwright command");
        }
    }
}
