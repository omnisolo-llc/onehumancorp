use ohc_builtin_agent_core::types::ToolError;
use serde_json::Value;
use std::sync::Arc;
use tokio::process::Command;

use super::{Tool, ToolExecutor};

/// Ruflo Unique Harness Innovations: 32+ Claude Code plugins
/// Executes external tools/scripts via CLI.

pub struct ClaudePlugin {
    pub name: String,
    pub command: String,
}

impl ClaudePlugin {
    pub fn new(name: String, command: String) -> Self {
        Self { name, command }
    }
}

#[async_trait::async_trait]
impl ToolExecutor for ClaudePlugin {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let plugin_args = args.get("args")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
            .unwrap_or_default();

        let output = Command::new(&self.command)
            .args(&plugin_args)
            .output()
            .await
            .map_err(|e| ToolError::Unexpected(format!("Failed to execute plugin '{}': {}", self.name, e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            let error_msg = format!("Plugin '{}' failed with status: {}. Stderr: {}", self.name, output.status, stderr);
            return Err(ToolError::LlmRecoverable(error_msg));
        }

        Ok(stdout)
    }
}

pub fn claude_plugin_tool(name: String, description: String, command: String) -> Tool {
    Tool {
        name: name.clone(),
        description,
        is_read_only: false,
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "args": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Arguments to pass to the plugin command."
                }
            }
        }),
        execute: Arc::new(ClaudePlugin::new(name, command)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_claude_plugin_success() {
        let plugin = claude_plugin_tool("echo_plugin".to_string(), "Echoes input".to_string(), "echo".to_string());

        let args = serde_json::json!({
            "args": ["hello", "world"]
        });

        let result = plugin.execute.execute(args).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.trim(), "hello world");
    }

    #[tokio::test]
    async fn test_claude_plugin_failure() {
        // A command that always fails
        let plugin = claude_plugin_tool("fail_plugin".to_string(), "Always fails".to_string(), "false".to_string());

        let args = serde_json::json!({
            "args": []
        });

        let result = plugin.execute.execute(args).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::LlmRecoverable(msg) => {
                assert!(msg.contains("failed with status:"));
            }
            _ => panic!("Expected LlmRecoverable error"),
        }
    }
}
