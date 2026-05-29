use ohc_builtin_agent_core::types::ToolError;
use crate::{Tool, ToolExecutor};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

/// Configuration for a Claude Code plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudePluginConfig {
    pub name: String,
    pub description: String,
    pub command: String,
    pub args: Vec<String>,
    pub parameters: Value,
}

/// Executor for a Claude Code plugin.
pub struct ClaudePluginExecutor {
    pub config: ClaudePluginConfig,
}

#[async_trait::async_trait]
impl ToolExecutor for ClaudePluginExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let mut cmd = tokio::process::Command::new(&self.config.command);
        cmd.args(&self.config.args);

        // Pass arguments as environment variables for the plugin
        if let Some(obj) = args.as_object() {
            for (k, v) in obj {
                let v_str = match v {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => v.to_string(),
                };
                cmd.env(format!("PLUGIN_ARG_{}", k.to_uppercase()), v_str);
            }
        }

        let output = cmd.output().await.map_err(|e| ToolError::Unexpected(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(ToolError::Unexpected(format!(
                "Plugin {} failed: {}",
                self.config.name, stderr
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// Manager for Claude Code plugins.
pub struct ClaudePluginManager {
    plugins: Vec<ClaudePluginConfig>,
}

impl ClaudePluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn load_plugin(&mut self, config: ClaudePluginConfig) {
        self.plugins.push(config);
    }

    pub fn get_tools(&self) -> Vec<Tool> {
        self.plugins
            .iter()
            .map(|config| {
                let executor = Arc::new(ClaudePluginExecutor {
                    config: config.clone(),
                });
                Tool {
                    name: config.name.clone(),
                    description: config.description.clone(),
                    is_read_only: false, // Assume external plugins might be mutating
                    parameters: config.parameters.clone(),
                    execute: executor,
                }
            })
            .collect()
    }
}
