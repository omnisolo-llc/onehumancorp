use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::tools::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::ToolError;
use tokio::process::Command;
use std::process::Stdio;

/// Ruflo Unique Harness Innovations: 32+ Claude Code plugins
/// We implement a plugin architecture to integrate Claude Code's plugin ecosystem.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudePluginConfig {
    pub plugin_name: String,
    pub command: String, // The executable to run, e.g., "npx" or "node"
    pub sub_args: Vec<String>, // Additional arguments, e.g., ["@claude/code-plugin-xyz"]
}

pub struct ClaudePluginExecutor {
    pub config: ClaudePluginConfig,
}

#[async_trait::async_trait]
impl ToolExecutor for ClaudePluginExecutor {
    async fn execute(&self, args: serde_json::Value) -> Result<String, ToolError> {
        let args_str = serde_json::to_string(&args).unwrap_or_else(|_| "{}".to_string());

        let mut cmd = Command::new(&self.config.command);

        for arg in &self.config.sub_args {
            cmd.arg(arg);
        }

        // Safely pass the JSON string as an argument directly
        cmd.arg(&args_str);

        let mut child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| ToolError::Unexpected(format!("Failed to spawn plugin {}: {}", self.config.plugin_name, e)))?;

        let output = child.wait_with_output().await
            .map_err(|e| ToolError::Unexpected(format!("Failed to wait for plugin {}: {}", self.config.plugin_name, e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            Ok(stdout.to_string())
        } else {
            Err(ToolError::LlmRecoverable(format!("Plugin {} failed. Stdout: {} Stderr: {}", self.config.plugin_name, stdout, stderr)))
        }
    }
}

pub struct ClaudePluginManager {
    pub loaded_plugins: Vec<Tool>,
}

impl ClaudePluginManager {
    pub fn new() -> Self {
        Self {
            loaded_plugins: Vec::new(),
        }
    }

    pub fn load_plugin(&mut self, config: ClaudePluginConfig, parameters_schema: serde_json::Value, description: String) {
        let tool = Tool {
            name: config.plugin_name.clone(),
            description,
            is_read_only: false,
            parameters: parameters_schema,
            execute: Arc::new(ClaudePluginExecutor { config }),
        };
        self.loaded_plugins.push(tool);
    }

    pub fn get_tools(self) -> Vec<Tool> {
        self.loaded_plugins
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_claude_plugin_executor_success() {
        let config = ClaudePluginConfig {
            plugin_name: "echo_plugin".to_string(),
            command: "echo".to_string(),
            sub_args: vec![],
        };
        let executor = ClaudePluginExecutor { config };

        let res = executor.execute(serde_json::json!({"test": 123})).await;
        assert!(res.is_ok());
        assert!(res.unwrap().contains("test"));
    }

    #[tokio::test]
    async fn test_claude_plugin_manager_load() {
        let mut manager = ClaudePluginManager::new();
        manager.load_plugin(
            ClaudePluginConfig {
                plugin_name: "test_plugin".to_string(),
                command: "false".to_string(),
                sub_args: vec![],
            },
            serde_json::json!({}),
            "A test plugin".to_string(),
        );

        assert_eq!(manager.loaded_plugins.len(), 1);
        assert_eq!(manager.loaded_plugins[0].name, "test_plugin");
    }
}
