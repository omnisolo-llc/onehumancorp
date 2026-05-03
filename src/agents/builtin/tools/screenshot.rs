use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::process::Command;

use super::{Tool, ToolExecutor};

struct ScreenshotExecutor {
    working_dir: Option<std::path::PathBuf>,
    #[cfg(test)]
    mock_command: Option<String>, // Allows overriding "npx" for testing
}

#[async_trait::async_trait]
impl ToolExecutor for ScreenshotExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let url = args["url"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("screenshot: url is required".to_string()))?;

        let path = args["path"]
            .as_str()
            .unwrap_or("screenshot.png");

        // Prevent directory traversal and absolute paths on Unix/Windows
        if path.contains("..") || path.starts_with('/') || path.contains(":\\") {
            return Err(ToolError::LlmRecoverable("screenshot: path cannot contain '..' or be absolute".to_string()));
        }

        #[cfg(not(test))]
        let base_cmd = "npx";

        #[cfg(test)]
        let base_cmd = self.mock_command.as_deref().unwrap_or("npx");

        let mut cmd = Command::new(base_cmd);
        // Force npx to be non-interactive
        cmd.env("npm_config_yes", "true");

        #[cfg(test)]
        if base_cmd == "echo" {
            // For testing success
            cmd.arg("success");
        } else if base_cmd == "false" {
            // For testing failure exit code
            // false takes no args effectively
        } else {
            cmd.arg("playwright").arg("screenshot").arg(url).arg(path);
        }

        #[cfg(not(test))]
        cmd.arg("playwright").arg("screenshot").arg(url).arg(path);

        if let Some(wd) = &self.working_dir {
            cmd.current_dir(wd);
        }

        let output = cmd.output().await
            .map_err(|e| ToolError::LlmRecoverable(format!("screenshot: failed to execute playwright: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(ToolError::LlmRecoverable(format!(
                "screenshot command failed with exit code {}\nStdout: {}\nStderr: {}",
                output.status.code().unwrap_or(-1),
                stdout,
                stderr
            )));
        }

        Ok(format!("Screenshot of {} successfully saved to {}", url, path))
    }
}

pub fn screenshot_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "Screenshot".to_string(),
        description: "Takes a screenshot of a web page at the specified URL using Playwright. Use this to visually verify web application functionality. \
            Returns the path to the saved screenshot.".to_string(),
        is_read_only: true, // It writes a file locally but doesn't change the remote state, treating as read-only or safe to run concurrently
        is_subagent: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL of the web page to screenshot."
                },
                "path": {
                    "type": "string",
                    "description": "The local file path where the screenshot will be saved (e.g., 'screenshot.png'). Must be a relative path without '..'."
                }
            },
            "required": ["url"]
        }),
        execute: Arc::new(ScreenshotExecutor {
            working_dir,
            #[cfg(test)]
            mock_command: None,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ToolError;
    use serde_json::json;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_screenshot_missing_url() {
        let executor = ScreenshotExecutor { working_dir: None, mock_command: None };
        let args = json!({ "path": "test.png" });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert_eq!(msg, "screenshot: url is required");
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }

    #[tokio::test]
    async fn test_screenshot_path_traversal() {
        let executor = ScreenshotExecutor { working_dir: None, mock_command: None };

        let args1 = json!({ "url": "https://example.com", "path": "../test.png" });
        let result1 = executor.execute(args1).await;
        assert!(result1.is_err());

        let args2 = json!({ "url": "https://example.com", "path": "/etc/shadow" });
        let result2 = executor.execute(args2).await;
        assert!(result2.is_err());

        let args3 = json!({ "url": "https://example.com", "path": "C:\\Windows" });
        let result3 = executor.execute(args3).await;
        assert!(result3.is_err());
    }

    #[tokio::test]
    async fn test_screenshot_tool_creation() {
        let wd = PathBuf::from("/tmp");
        let tool = screenshot_tool(Some(wd.clone()));
        assert_eq!(tool.name, "Screenshot");
        assert!(tool.is_read_only);
        assert_eq!(tool.parameters["required"][0], "url");
    }

    #[tokio::test]
    async fn test_screenshot_execute_success() {
        let executor = ScreenshotExecutor {
            working_dir: None,
            mock_command: Some("echo".to_string())
        };
        let args = json!({ "url": "https://example.com", "path": "test.png" });
        let result = executor.execute(args).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Screenshot of https://example.com successfully saved to test.png");
    }

    #[tokio::test]
    async fn test_screenshot_execute_failure() {
        let executor = ScreenshotExecutor {
            working_dir: None,
            mock_command: Some("false".to_string())
        };
        let args = json!({ "url": "https://example.com", "path": "test.png" });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("screenshot command failed with exit code"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }

    #[tokio::test]
    async fn test_screenshot_execute_bad_command() {
        let executor = ScreenshotExecutor {
            working_dir: None,
            mock_command: Some("this_command_does_not_exist_12345".to_string())
        };
        let args = json!({ "url": "https://example.com", "path": "test.png" });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("screenshot: failed to execute playwright"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }
}
