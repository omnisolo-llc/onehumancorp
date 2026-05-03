use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::process::Command;

use super::{Tool, ToolExecutor};

struct BrowserExecutor;

#[async_trait::async_trait]
impl ToolExecutor for BrowserExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let url = args["url"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("browser: url is required".to_string()))?;
        let output_path = args["output_path"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("browser: output_path is required".to_string()))?;

        // Use npx playwright screenshot to capture the visual output
        // Example: npx playwright screenshot https://example.com example.png
        let mut cmd = Command::new("npx");
        cmd.arg("playwright").arg("screenshot").arg(url).arg(output_path);

        let output = cmd.output().await.map_err(|e| {
            ToolError::LlmRecoverable(format!("Failed to execute playwright: {}", e))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ToolError::LlmRecoverable(format!(
                "Playwright failed: {}",
                stderr
            )));
        }

        Ok(format!(
            "Successfully captured screenshot of {} to {}",
            url, output_path
        ))
    }
}

pub fn browser_tool() -> Tool {
    Tool {
        name: "Browser".to_string(),
        description: "Capture a screenshot of a webpage using Playwright. Useful for visual verification of UI changes.".to_string(),
        is_read_only: true, // It captures a screenshot, doesn't mutate the core state
        parameters: json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL of the webpage to capture."
                },
                "output_path": {
                    "type": "string",
                    "description": "The path where the screenshot PNG should be saved."
                }
            },
            "required": ["url", "output_path"]
        }),
        execute: Arc::new(BrowserExecutor),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ToolError;

    #[test]
    fn test_browser_tool_definition() {
        let tool = browser_tool();
        assert_eq!(tool.name, "Browser");
        assert_eq!(tool.is_read_only, true);
        assert_eq!(tool.parameters["required"], json!(["url", "output_path"]));
    }

    #[tokio::test]
    async fn test_browser_executor_missing_url() {
        let executor = BrowserExecutor;
        let args = json!({ "output_path": "test.png" });
        let res = executor.execute(args).await;
        assert!(res.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = res {
            assert_eq!(msg, "browser: url is required");
        } else {
            panic!("Expected LlmRecoverable");
        }
    }

    #[tokio::test]
    async fn test_browser_executor_missing_output() {
        let executor = BrowserExecutor;
        let args = json!({ "url": "http://example.com" });
        let res = executor.execute(args).await;
        assert!(res.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = res {
            assert_eq!(msg, "browser: output_path is required");
        } else {
            panic!("Expected LlmRecoverable");
        }
    }

    #[tokio::test]
    async fn test_browser_executor_execution() {
        // This test runs the actual executor logic.
        // In a sandboxed test environment without playwright installed,
        // it should attempt execution but gracefully fail with a format string.
        let executor = BrowserExecutor;
        let args = json!({ "url": "http://example.com", "output_path": "test.png" });
        let res = executor.execute(args).await;

        // It's acceptable for it to fail as long as it handles the error properly
        // without crashing, because we lack a mocked runtime injection here.
        if let Err(ToolError::LlmRecoverable(msg)) = res {
            assert!(msg.contains("Failed to execute playwright") || msg.contains("Playwright failed"));
        } else {
            // Or if it miraculously succeeds locally
            assert_eq!(res.unwrap(), "Successfully captured screenshot of http://example.com to test.png");
        }
    }
}
