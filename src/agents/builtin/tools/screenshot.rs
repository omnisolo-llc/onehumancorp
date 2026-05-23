use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Deserialize)]
struct ScreenshotArgs {
    url: String,
    #[serde(default = "default_screenshot_path")]
    path: String,
}

fn default_screenshot_path() -> String {
    "screenshot.png".to_string()
}

struct ScreenshotExecutor {
    working_dir: Option<std::path::PathBuf>,
    runner: Arc<dyn crate::runner::CommandRunner>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<ScreenshotArgs> for ScreenshotExecutor {
    async fn execute_typed(&self, args: ScreenshotArgs) -> Result<String, ToolError> {
        let url = &args.url;
        let path = &args.path;

        // Prevent directory traversal and absolute paths on Unix/Windows
        if path.contains("..") || path.starts_with('/') || path.contains(":\\") {
            return Err(ToolError::LlmRecoverable("screenshot: path cannot contain '..' or be absolute".to_string()));
        }

        let wd_ref = self.working_dir.as_deref();
        
        let output = self.runner.run("npx", &["playwright", "screenshot", url, path], wd_ref, vec![("npm_config_yes".to_string(), "true".to_string())]).await
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

pub fn screenshot_tool(working_dir: Option<std::path::PathBuf>, runner: Arc<dyn crate::runner::CommandRunner>) -> Tool {
    Tool {
        name: "Screenshot".to_string(),
        description: "Takes a screenshot of a web page at the specified URL using Playwright. Use this to visually verify web application functionality. \
            Returns the path to the saved screenshot.".to_string(),
        is_read_only: true, // It writes a file locally but doesn't change the remote state, treating as read-only or safe to run concurrently
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
        execute: Arc::new(PydanticAdapter::new(ScreenshotExecutor {
            working_dir,
            runner,
        })),
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
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = PydanticAdapter::new(ScreenshotExecutor { working_dir: None, runner });
        let args = json!({ "path": "test.png" });
        let result = super::super::ToolExecutor::execute(&executor, args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }

    #[tokio::test]
    async fn test_screenshot_path_traversal() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = PydanticAdapter::new(ScreenshotExecutor { working_dir: None, runner });

        let args1 = json!({ "url": "https://example.com", "path": "../test.png" });
        let result1 = super::super::ToolExecutor::execute(&executor, args1).await;
        assert!(result1.is_err());

        let args2 = json!({ "url": "https://example.com", "path": "/etc/shadow" });
        let result2 = super::super::ToolExecutor::execute(&executor, args2).await;
        assert!(result2.is_err());

        let args3 = json!({ "url": "https://example.com", "path": "C:\\Windows" });
        let result3 = super::super::ToolExecutor::execute(&executor, args3).await;
        assert!(result3.is_err());
    }

    #[tokio::test]
    async fn test_screenshot_tool_creation() {
        let wd = PathBuf::from("/tmp");
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let tool = screenshot_tool(Some(wd.clone()), runner);
        assert_eq!(tool.name, "Screenshot");
        assert!(tool.is_read_only);
        assert_eq!(tool.parameters["required"][0], "url");
    }

    #[tokio::test]
    async fn test_screenshot_execute_success() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = PydanticAdapter::new(ScreenshotExecutor {
            working_dir: None,
            runner,
        });
        let args = json!({ "url": "https://example.com", "path": "test.png" });
        let result = super::super::ToolExecutor::execute(&executor, args).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Screenshot of https://example.com successfully saved to test.png");
    }

    #[tokio::test]
    async fn test_screenshot_execute_failure() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        runner.push_response(Ok(crate::runner::mock::mock_output(1, "", "Error!")));
        
        let executor = PydanticAdapter::new(ScreenshotExecutor {
            working_dir: None,
            runner,
        });
        let args = json!({ "url": "https://example.com", "path": "test.png" });
        let result = super::super::ToolExecutor::execute(&executor, args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("screenshot command failed with exit code"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }

    #[tokio::test]
    async fn test_screenshot_execute_bad_command() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        // Simulate binary missing error
        runner.push_response(Err(std::io::Error::new(std::io::ErrorKind::NotFound, "not found")));

        let executor = PydanticAdapter::new(ScreenshotExecutor {
            working_dir: None,
            runner,
        });
        let args = json!({ "url": "https://example.com", "path": "test.png" });
        let result = super::super::ToolExecutor::execute(&executor, args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("screenshot: failed to execute playwright"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }
}
