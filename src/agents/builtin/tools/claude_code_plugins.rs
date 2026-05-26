use super::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::ToolError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::runner::CommandRunner;

/// Ruflo Unique Harness Innovations: 32+ Claude Code plugins
/// Adaptor to load and execute Claude Code plugins natively within our harness.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCodePluginManifest {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub entrypoint: String,
}

pub struct ClaudeCodePluginExecutor {
    pub runner: Arc<dyn CommandRunner>,
    pub entrypoint: String,
}

#[async_trait::async_trait]
impl ToolExecutor for ClaudeCodePluginExecutor {
    async fn execute(&self, args: serde_json::Value) -> Result<String, ToolError> {
        let args_str = match serde_json::to_string(&args) {
            Ok(s) => s,
            Err(e) => return Err(ToolError::LlmRecoverable(format!("Failed to serialize arguments: {}", e))),
        };

        // Pass arguments via standard input (or as a CLI arg, but stdin is safer for large JSON)
        // Since our runner does not expose stdin directly, we pass it as a CLI argument safely.
        // Or better yet, we pass it via an environment variable `PLUGIN_ARGS`.
        let mut child_envs = vec![("PLUGIN_ARGS".to_string(), args_str.clone())];
        let parts: Vec<&str> = self.entrypoint.split_whitespace().collect();
        if parts.is_empty() {
            return Err(ToolError::Fatal("Plugin entrypoint is empty".to_string()));
        }

        let cmd = parts[0];
        let mut cmd_args: Vec<&str> = parts.into_iter().skip(1).collect();
        cmd_args.push(&args_str); // Also keep it as the last argument for backward compatibility

        match self.runner.run(cmd, &cmd_args, None, child_envs).await {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();

                if out.status.success() {
                    Ok(stdout)
                } else {
                    Err(ToolError::LlmRecoverable(format!("Plugin execution failed. Stdout: {}, Stderr: {}", stdout, stderr)))
                }
            },
            Err(e) => Err(ToolError::LlmRecoverable(format!("Failed to spawn plugin process: {}", e))),
        }
    }
}

pub async fn load_claude_code_plugins(directory: &str, runner: Arc<dyn CommandRunner>) -> Vec<Tool> {
    let mut tools = Vec::new();

    let mut dir = match tokio::fs::read_dir(directory).await {
        Ok(d) => d,
        Err(e) => {
            tracing::warn!("Failed to read Claude Code plugins directory {}: {}", directory, e);
            return tools;
        }
    };

    while let Ok(Some(entry)) = dir.next_entry().await {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            if let Ok(content) = tokio::fs::read_to_string(&path).await {
                match serde_json::from_str::<ClaudeCodePluginManifest>(&content) {
                    Ok(manifest) => {
                        let tool = Tool {
                            name: manifest.name.clone(),
                            description: manifest.description.clone(),
                            is_read_only: false, // Default to false to be safe, could be configurable
                            parameters: manifest.parameters.clone(),
                            execute: Arc::new(ClaudeCodePluginExecutor {
                                runner: runner.clone(),
                                entrypoint: manifest.entrypoint.clone(),
                            }),
                        };
                        tools.push(tool);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse Claude Code plugin manifest at {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    tools
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ToolError;
    use serde_json::json;

    #[tokio::test]
    async fn test_load_claude_code_plugins() {
        let temp_dir = tempfile::tempdir().unwrap();
        let plugin_dir = temp_dir.path().to_path_buf();

        let manifest = ClaudeCodePluginManifest {
            name: "test_plugin".to_string(),
            description: "A test plugin".to_string(),
            parameters: json!({"type": "object", "properties": {"arg1": {"type": "string"}}}),
            entrypoint: "./run_test.sh".to_string(),
        };

        let manifest_path = plugin_dir.join("test_plugin.json");
        tokio::fs::write(&manifest_path, serde_json::to_string(&manifest).unwrap()).await.unwrap();

        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let tools = load_claude_code_plugins(plugin_dir.to_str().unwrap(), runner.clone()).await;

        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "test_plugin");
        assert_eq!(tools[0].description, "A test plugin");
        assert_eq!(tools[0].is_read_only, false);
    }

    #[tokio::test]
    async fn test_claude_code_plugin_executor_success() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = ClaudeCodePluginExecutor {
            runner,
            entrypoint: "./run_test.sh".to_string(),
        };

        let result = executor.execute(json!({"arg1": "value1"})).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_claude_code_plugin_executor_empty_entrypoint() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = ClaudeCodePluginExecutor {
            runner,
            entrypoint: "".to_string(),
        };

        let result = executor.execute(json!({"arg1": "value1"})).await;
        assert!(result.is_err());
        match result {
            Err(ToolError::Fatal(e)) => assert_eq!(e, "Plugin entrypoint is empty"),
            _ => panic!("Expected Fatal error"),
        }
    }
}
