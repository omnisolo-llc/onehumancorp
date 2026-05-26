use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::process::Command;
use ohc_builtin_agent_core::types::ToolError;
use ohc_builtin_agent_tools::{Tool, ToolExecutor};

/// Ruflo Unique Harness Innovations: 32+ Claude Code plugins
/// A dynamic plugin manager that loads Claude Code compatible plugins and exposes them as tools.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCodePluginManifest {
    pub name: String,
    pub description: String,
    pub version: String,
    pub commands: Vec<PluginCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCommand {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub executable_path: String,
}

pub struct PluginExecutor {
    executable_path: String,
}

#[async_trait::async_trait]
impl ToolExecutor for PluginExecutor {
    async fn execute(&self, args: serde_json::Value) -> Result<String, ToolError> {
        let args_str = serde_json::to_string(&args)
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to serialize args: {}", e)))?;

        let output = Command::new(&self.executable_path)
            .arg(&args_str)
            .output()
            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to execute plugin: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ToolError::LlmRecoverable(format!("Plugin execution failed: {}", stderr)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }
}

pub struct ClaudeCodePluginManager {
    plugins: Vec<ClaudeCodePluginManifest>,
}

impl ClaudeCodePluginManager {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn register_plugin(&mut self, manifest: ClaudeCodePluginManifest) {
        self.plugins.push(manifest);
    }

    pub fn get_tools(&self) -> Vec<Tool> {
        let mut tools = Vec::new();

        for plugin in &self.plugins {
            for cmd in &plugin.commands {
                let exposed_name = format!("{}_{}", plugin.name, cmd.name);
                tools.push(Tool {
                    name: exposed_name.clone(),
                    description: format!("Claude Code Plugin '{}': {}", plugin.name, cmd.description),
                    is_read_only: false, // Defaulting to mutating, could be enhanced
                    parameters: cmd.parameters.clone(),
                    execute: Arc::new(PluginExecutor {
                        executable_path: cmd.executable_path.clone(),
                    }),
                });
            }
        }

        tools
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_claude_code_plugin_registration() {
        let mut manager = ClaudeCodePluginManager::new();

        let manifest = ClaudeCodePluginManifest {
            name: "fs_utils".to_string(),
            description: "File system utilities".to_string(),
            version: "1.0.0".to_string(),
            commands: vec![
                PluginCommand {
                    name: "list_files".to_string(),
                    description: "List files in a directory".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "path": { "type": "string" }
                        }
                    }),
                    executable_path: "/usr/bin/ls".to_string(),
                }
            ]
        };

        manager.register_plugin(manifest);
        let tools = manager.get_tools();

        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "fs_utils_list_files");
        assert!(tools[0].description.contains("Claude Code Plugin"));
    }
}
