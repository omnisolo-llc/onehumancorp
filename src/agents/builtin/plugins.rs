use ohc_builtin_agent_core::types::ToolError;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use ohc_builtin_agent_tools::{Tool, ToolExecutor};

/// Ruflo Unique Harness Innovations: 32+ Claude Code plugins
/// A registry to dynamically manage and execute external plugins as tools.

pub trait ClaudeCodePlugin: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn parameters(&self) -> Value;
    fn is_read_only(&self) -> bool;
}

#[async_trait::async_trait]
pub trait ClaudeCodePluginExecutor: ClaudeCodePlugin {
    async fn execute(&self, args: Value) -> Result<String, ToolError>;
}

struct PluginAdapter {
    plugin: Arc<dyn ClaudeCodePluginExecutor>,
}

#[async_trait::async_trait]
impl ToolExecutor for PluginAdapter {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        self.plugin.execute(args).await
    }
}

pub struct PluginRegistry {
    plugins: HashMap<String, Arc<dyn ClaudeCodePluginExecutor>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register(&mut self, plugin: Arc<dyn ClaudeCodePluginExecutor>) {
        self.plugins.insert(plugin.name(), plugin);
    }

    pub fn get_tools(&self) -> Vec<Tool> {
        self.plugins
            .values()
            .map(|plugin| Tool {
                name: plugin.name(),
                description: plugin.description(),
                is_read_only: plugin.is_read_only(),
                parameters: plugin.parameters(),
                execute: Arc::new(PluginAdapter { plugin: plugin.clone() }),
            })
            .collect()
    }
}

// Example Mock Plugin to demonstrate the 32+ Claude Code plugins capability
pub struct MathPlugin;

impl ClaudeCodePlugin for MathPlugin {
    fn name(&self) -> String {
        "math_plugin".to_string()
    }

    fn description(&self) -> String {
        "A Claude Code plugin for performing basic math operations.".to_string()
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "description": "The math operation to perform (add, subtract)"
                },
                "a": {
                    "type": "number"
                },
                "b": {
                    "type": "number"
                }
            },
            "required": ["operation", "a", "b"]
        })
    }

    fn is_read_only(&self) -> bool {
        true
    }
}

#[async_trait::async_trait]
impl ClaudeCodePluginExecutor for MathPlugin {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let op = args["operation"].as_str().unwrap_or("add");
        let a = args["a"].as_f64().unwrap_or(0.0);
        let b = args["b"].as_f64().unwrap_or(0.0);

        let result = match op {
            "add" => a + b,
            "subtract" => a - b,
            _ => return Err(ToolError::LlmRecoverable(format!("Unsupported operation: {}", op))),
        };

        Ok(format!("Result: {}", result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_registry() {
        let mut registry = PluginRegistry::new();
        registry.register(Arc::new(MathPlugin));

        let tools = registry.get_tools();
        assert_eq!(tools.len(), 1);

        let math_tool = &tools[0];
        assert_eq!(math_tool.name, "math_plugin");

        let args = serde_json::json!({
            "operation": "add",
            "a": 5,
            "b": 3
        });

        let result = math_tool.execute.execute(args).await.unwrap();
        assert_eq!(result, "Result: 8");
    }
}