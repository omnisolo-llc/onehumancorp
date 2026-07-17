use ohc_builtin_agent_core::types::ToolError;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::fs;
use tokio::process::Command;
use tracing::warn;

use super::{Tool, pydantic::{PydanticAdapter, PydanticToolExecutor}};

/// Ruflo Unique Harness Innovations: 32+ Claude Code plugins
/// We load plugin definitions dynamically from a directory.
/// A plugin definition contains the tool name, description, parameters, and the bash command to execute.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCodePluginManifest {
    pub name: String,
    pub description: String,
    pub is_read_only: bool,
    pub parameters: Value,
    pub execute_command: String, // Command template, e.g., "python /path/to/script.py {{args}}"
}

struct ClaudeCodePluginExecutor {
    manifest: ClaudeCodePluginManifest,
    working_dir: Option<std::path::PathBuf>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<serde_json::Value> for ClaudeCodePluginExecutor {
    async fn execute_typed(&self, args: serde_json::Value) -> Result<String, ToolError> {
        let args_str = serde_json::to_string(&args)
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to serialize args: {}", e)))?;

        // Simple template replacement
        let command_str = self
            .manifest
            .execute_command
            .replace("{{args}}", &format!("'{}'", args_str.replace("'", "'\\''")));

        let mut cmd = Command::new("bash");
        cmd.arg("-c").arg(&command_str);

        if let Some(wd) = &self.working_dir {
            cmd.current_dir(wd);
        }

        let output = cmd.output().await.map_err(|e| {
            ToolError::Transient(format!("Failed to execute plugin command: {}", e))
        })?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            Err(ToolError::LlmRecoverable(format!(
                "Plugin execution failed: {}",
                stderr
            )))
        }
    }
}

pub async fn load_claude_code_plugins(
    plugins_dir: &std::path::Path,
    working_dir: Option<std::path::PathBuf>,
) -> Vec<Tool> {
    let mut tools = Vec::new();

    if !plugins_dir.exists() || !plugins_dir.is_dir() {
        return tools;
    }

    let mut entries = match fs::read_dir(plugins_dir).await {
        Ok(e) => e,
        Err(e) => {
            warn!("Failed to read plugins directory: {}", e);
            return tools;
        }
    };

    while let Ok(Some(entry)) = entries.next_entry().await {
        if entry.path().extension().and_then(|e| e.to_str()) == Some("json") {
            if let Ok(content) = fs::read_to_string(entry.path()).await {
                if let Ok(manifest) = serde_json::from_str::<ClaudeCodePluginManifest>(&content) {
                    let tool = Tool {
                        name: manifest.name.clone(),
                        description: manifest.description.clone(),
                        is_read_only: manifest.is_read_only,
                        parameters: manifest.parameters.clone(),
                        execute: Arc::new(PydanticAdapter::new(ClaudeCodePluginExecutor {
                            manifest,
                            working_dir: working_dir.clone(),
                        })),
                    };
                    tools.push(tool);
                } else {
                    warn!("Failed to parse plugin manifest: {:?}", entry.path());
                }
            }
        }
    }

    tools
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_load_claude_code_plugins() {
        let dir = tempdir().unwrap();
        let plugin_path = dir.path().join("test_plugin.json");

        let manifest = ClaudeCodePluginManifest {
            name: "TestPlugin".to_string(),
            description: "A test plugin".to_string(),
            is_read_only: true,
            parameters: json!({"type": "object", "properties": {"msg": {"type": "string"}}}),
            execute_command: "echo {{args}}".to_string(),
        };

        fs::write(&plugin_path, serde_json::to_string(&manifest).unwrap())
            .await
            .unwrap();

        let tools = load_claude_code_plugins(dir.path(), None).await;
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "TestPlugin");

        let result = tools[0]
            .execute
            .execute(json!({"msg": "hello"}))
            .await
            .unwrap();
        assert!(result.contains("hello"));
    }
}
