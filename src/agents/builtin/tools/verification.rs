use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::process::Command;
use std::process::Output;
use base64::{engine::general_purpose, Engine as _};

use super::{Tool, ToolExecutor};

#[async_trait::async_trait]
pub trait CommandRunner: Send + Sync {
    async fn run_cmd(&self, cmd: &str, args: &[&str], cwd: Option<&std::path::PathBuf>) -> std::io::Result<Output>;
    async fn read_file(&self, path: &str) -> std::io::Result<Vec<u8>>;
}

pub struct RealCommandRunner;

#[async_trait::async_trait]
impl CommandRunner for RealCommandRunner {
    async fn run_cmd(&self, cmd: &str, args: &[&str], cwd: Option<&std::path::PathBuf>) -> std::io::Result<Output> {
        let mut command = Command::new(cmd);
        command.args(args);
        if let Some(wd) = cwd {
            command.current_dir(wd);
        }
        command.output().await
    }

    async fn read_file(&self, path: &str) -> std::io::Result<Vec<u8>> {
        tokio::fs::read(path).await
    }
}

struct PlaywrightScreenshotExecutor {
    working_dir: Option<std::path::PathBuf>,
    runner: Arc<dyn CommandRunner>,
}

#[async_trait::async_trait]
impl ToolExecutor for PlaywrightScreenshotExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let url = args["url"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("playwright_screenshot: url is required".to_string()))?;

        let output_path = args["output_path"]
            .as_str()
            .unwrap_or("screenshot.png");

        if output_path.contains("..") || output_path.starts_with("/") {
             return Err(ToolError::LlmRecoverable("playwright_screenshot: invalid output_path".to_string()));
        }

        let output = self.runner.run_cmd("npx", &["playwright", "screenshot", url, output_path], self.working_dir.as_ref())
            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to run playwright: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ToolError::LlmRecoverable(format!("Playwright failed: {}", stderr)));
        }


        let actual_output_path = if let Some(wd) = &self.working_dir {
            wd.join(output_path).to_string_lossy().to_string()
        } else {
            output_path.to_string()
        };

        // Read file and encode to base64
        let img_bytes = self.runner.read_file(&actual_output_path)

            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to read screenshot: {}", e)))?;

        let b64 = general_purpose::STANDARD.encode(&img_bytes);

        Ok(format!("Screenshot saved to {}. Base64 image data: data:image/png;base64,{}", output_path, b64))
    }
}

pub fn playwright_screenshot_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "playwright_screenshot".to_string(),
        description: "Visual Verification Loop: Take a screenshot of a URL or local HTML file using Playwright to visually verify the frontend changes. It returns the base64 encoded image.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL or file path to take a screenshot of."
                },
                "output_path": {
                    "type": "string",
                    "description": "The file path to save the screenshot to (e.g., screenshot.png)."
                }
            },
            "required": ["url"]
        }),
        execute: Arc::new(PlaywrightScreenshotExecutor { working_dir, runner: Arc::new(RealCommandRunner) }),
    }
}

struct LinterCheckExecutor {
    working_dir: Option<std::path::PathBuf>,
    runner: Arc<dyn CommandRunner>,
}

#[async_trait::async_trait]
impl ToolExecutor for LinterCheckExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("linter_check: path is required".to_string()))?;

        let (cmd, cmd_args) = if path.ends_with(".rs") || path == "." {
            ("cargo", vec!["check"])
        } else if path.ends_with(".go") {
            ("golangci-lint", vec!["run", path])
        } else if path.ends_with(".ts") || path.ends_with(".js") {
            ("npx", vec!["eslint", path])
        } else {
            return Err(ToolError::LlmRecoverable("Unsupported file type for linting".to_string()));
        };

        let output = self.runner.run_cmd(cmd, &cmd_args, self.working_dir.as_ref())
            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to run linter: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            return Ok(format!("Linter found issues:\nSTDOUT:\n{}\nSTDERR:\n{}", stdout, stderr));
        }

        Ok("Linter passed successfully. No issues found.".to_string())
    }
}

pub fn linter_check_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "linter_check".to_string(),
        description: "Computational Verification Loop: Run a linter (cargo check, eslint, etc.) on a specific path to verify code correctness before executing further actions.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The file or directory path to lint."
                }
            },
            "required": ["path"]
        }),
        execute: Arc::new(LinterCheckExecutor { working_dir, runner: Arc::new(RealCommandRunner) }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
        use std::process::ExitStatus;

    struct MockCommandRunner {
        pub fail_cmd: bool,
        pub status_success: bool,
        pub stdout: String,
        pub stderr: String,
        pub fail_read: bool,
        pub file_data: Vec<u8>,
    }

    #[async_trait::async_trait]
    impl CommandRunner for MockCommandRunner {
        async fn run_cmd(&self, _cmd: &str, _args: &[&str], _cwd: Option<&std::path::PathBuf>) -> std::io::Result<Output> {
            if self.fail_cmd {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "cmd failed"));
            }
            Ok(Output {

                status: if self.status_success {
                    std::process::Command::new("true").status().unwrap()
                } else {
                    std::process::Command::new("false").status().unwrap()
                },
                    stdout: self.stdout.as_bytes().to_vec(),
                stderr: self.stderr.as_bytes().to_vec(),
            })
        }

        async fn read_file(&self, _path: &str) -> std::io::Result<Vec<u8>> {
            if self.fail_read {
                return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"));
            }
            Ok(self.file_data.clone())
        }
    }

    #[tokio::test]
    async fn test_playwright_screenshot_missing_url() {
        let runner = Arc::new(MockCommandRunner {
            fail_cmd: false, status_success: true, stdout: "".into(), stderr: "".into(), fail_read: false, file_data: vec![]
        });
        let executor = PlaywrightScreenshotExecutor { working_dir: None, runner };
        let result = executor.execute(json!({})).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ToolError::LlmRecoverable("playwright_screenshot: url is required".to_string()));
    }

    #[tokio::test]
    async fn test_playwright_screenshot_invalid_path() {
        let runner = Arc::new(MockCommandRunner {
            fail_cmd: false, status_success: true, stdout: "".into(), stderr: "".into(), fail_read: false, file_data: vec![]
        });
        let executor = PlaywrightScreenshotExecutor { working_dir: None, runner };
        let result = executor.execute(json!({"url": "http://test", "output_path": "../out.png"})).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ToolError::LlmRecoverable("playwright_screenshot: invalid output_path".to_string()));

        let result2 = executor.execute(json!({"url": "http://test", "output_path": "/root/out.png"})).await;
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err(), ToolError::LlmRecoverable("playwright_screenshot: invalid output_path".to_string()));
    }

    #[tokio::test]
    async fn test_playwright_screenshot_cmd_fails() {
        let runner = Arc::new(MockCommandRunner {
            fail_cmd: true, status_success: false, stdout: "".into(), stderr: "".into(), fail_read: false, file_data: vec![]
        });
        let executor = PlaywrightScreenshotExecutor { working_dir: None, runner };
        let result = executor.execute(json!({"url": "http://test"})).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ToolError::LlmRecoverable("Failed to run playwright: cmd failed".to_string()));
    }

    #[tokio::test]
    async fn test_playwright_screenshot_cmd_status_err() {
        let runner = Arc::new(MockCommandRunner {
            fail_cmd: false, status_success: false, stdout: "".into(), stderr: "pw error".into(), fail_read: false, file_data: vec![]
        });
        let executor = PlaywrightScreenshotExecutor { working_dir: None, runner };
        let result = executor.execute(json!({"url": "http://test"})).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ToolError::LlmRecoverable("Playwright failed: pw error".to_string()));
    }

    #[tokio::test]
    async fn test_playwright_screenshot_read_fails() {
        let runner = Arc::new(MockCommandRunner {
            fail_cmd: false, status_success: true, stdout: "".into(), stderr: "".into(), fail_read: true, file_data: vec![]
        });
        let executor = PlaywrightScreenshotExecutor { working_dir: None, runner };
        let result = executor.execute(json!({"url": "http://test"})).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ToolError::LlmRecoverable("Failed to read screenshot: file not found".to_string()));
    }

    #[tokio::test]
    async fn test_playwright_screenshot_success() {
        let runner = Arc::new(MockCommandRunner {
            fail_cmd: false, status_success: true, stdout: "".into(), stderr: "".into(), fail_read: false, file_data: vec![0, 1, 2, 3]
        });
        let executor = PlaywrightScreenshotExecutor { working_dir: None, runner };
        let result = executor.execute(json!({"url": "http://test"})).await;
        assert!(result.is_ok());
        let res = result.unwrap();
        assert!(res.contains("data:image/png;base64,AAECAw=="));
    }

    #[tokio::test]
    async fn test_linter_check_missing_path() {
        let runner = Arc::new(MockCommandRunner {
            fail_cmd: false, status_success: true, stdout: "".into(), stderr: "".into(), fail_read: false, file_data: vec![]
        });
        let executor = LinterCheckExecutor { working_dir: None, runner };
        let result = executor.execute(json!({})).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ToolError::LlmRecoverable("linter_check: path is required".to_string()));
    }

    #[tokio::test]
    async fn test_linter_check_unsupported_file() {
        let runner = Arc::new(MockCommandRunner {
            fail_cmd: false, status_success: true, stdout: "".into(), stderr: "".into(), fail_read: false, file_data: vec![]
        });
        let executor = LinterCheckExecutor { working_dir: None, runner };
        let result = executor.execute(json!({"path": "file.txt"})).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ToolError::LlmRecoverable("Unsupported file type for linting".to_string()));
    }

    #[tokio::test]
    async fn test_linter_check_cmd_fails() {
        let runner = Arc::new(MockCommandRunner {
            fail_cmd: true, status_success: true, stdout: "".into(), stderr: "".into(), fail_read: false, file_data: vec![]
        });
        let executor = LinterCheckExecutor { working_dir: None, runner };
        let result = executor.execute(json!({"path": "file.rs"})).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ToolError::LlmRecoverable("Failed to run linter: cmd failed".to_string()));
    }

    #[tokio::test]
    async fn test_linter_check_status_err() {
        let runner = Arc::new(MockCommandRunner {
            fail_cmd: false, status_success: false, stdout: "".into(), stderr: "linter error".into(), fail_read: false, file_data: vec![]
        });
        let executor = LinterCheckExecutor { working_dir: None, runner };
        let result = executor.execute(json!({"path": "file.rs"})).await;
        assert!(result.is_ok());
        let res = result.unwrap();
        assert!(res.contains("Linter found issues"));
        assert!(res.contains("linter error"));
    }

    #[tokio::test]
    async fn test_linter_check_success_rs() {
        let runner = Arc::new(MockCommandRunner {
            fail_cmd: false, status_success: true, stdout: "".into(), stderr: "".into(), fail_read: false, file_data: vec![]
        });
        let executor = LinterCheckExecutor { working_dir: None, runner };
        let result = executor.execute(json!({"path": "file.rs"})).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Linter passed successfully. No issues found.".to_string());
    }

    #[tokio::test]
    async fn test_linter_check_success_go() {
        let runner = Arc::new(MockCommandRunner {
            fail_cmd: false, status_success: true, stdout: "".into(), stderr: "".into(), fail_read: false, file_data: vec![]
        });
        let executor = LinterCheckExecutor { working_dir: None, runner };
        let result = executor.execute(json!({"path": "main.go"})).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Linter passed successfully. No issues found.".to_string());
    }

    #[tokio::test]
    async fn test_linter_check_success_js() {
        let runner = Arc::new(MockCommandRunner {
            fail_cmd: false, status_success: true, stdout: "".into(), stderr: "".into(), fail_read: false, file_data: vec![]
        });
        let executor = LinterCheckExecutor { working_dir: None, runner };
        let result = executor.execute(json!({"path": "index.ts"})).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Linter passed successfully. No issues found.".to_string());
    }
}
