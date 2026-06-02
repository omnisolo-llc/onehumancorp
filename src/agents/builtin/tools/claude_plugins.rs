use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::process::Command;
use std::process::Stdio;

use super::{Tool, ToolExecutor};

/// Ruflo Unique Harness Innovations: 32+ Claude Code plugins
/// Implements a generic executor for Claude Code compatible plugins.

pub struct ClaudePluginExecutor {
    plugin_name: String,
    plugin_path: String,
}

impl ClaudePluginExecutor {
    pub fn new(plugin_name: String, plugin_path: String) -> Self {
        Self { plugin_name, plugin_path }
    }
}

#[async_trait::async_trait]
impl ToolExecutor for ClaudePluginExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let args_str = serde_json::to_string(&args).map_err(|e| ToolError::Fatal(format!("Failed to serialize args: {}", e)))?;

        let output = Command::new(&self.plugin_path)
            .arg(&self.plugin_name)
            .arg(args_str)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| ToolError::Fatal(format!("Failed to execute plugin: {}", e)))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
            Err(ToolError::LlmRecoverable(format!("Plugin execution failed: {}", error_msg)))
        }
    }
}

pub fn claude_plugin_tool(name: String, description: String, schema: Value, plugin_path: String) -> Tool {
    Tool {
        name: name.clone(),
        description: format!("{} (Ruflo Unique Harness Innovations: 32+ Claude Code plugins)", description),
        is_read_only: false,
        parameters: schema,
        execute: Arc::new(ClaudePluginExecutor::new(name, plugin_path)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_claude_plugin_execution() {
        let mut script = NamedTempFile::new().unwrap();
        writeln!(script, "#!/bin/bash\necho \"Plugin Output: $1 $2\"").unwrap();

        let script_path = script.path().to_str().unwrap().to_string();

        // Make script executable
        Command::new("chmod")
            .arg("+x")
            .arg(&script_path)
            .output()
            .await
            .unwrap();

        let executor = ClaudePluginExecutor::new("test_plugin".to_string(), script_path);
        let args = json!({"key": "value"});
        let result = executor.execute(args.clone()).await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Plugin Output: test_plugin {\"key\":\"value\"}"));
    }
}
