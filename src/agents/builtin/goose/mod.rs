//! goose (Agentic AI) Implementation Pattern
//! SOTA Harness Innovations: Rust + TypeScript UI, 70+ MCP extensions
//!
//! This module implements a basic bridge and UI stub pattern that enables
//! the agent to interact with a theoretical TypeScript UI and host MCP extensions.

use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionSpec {
    pub id: String,
    pub name: String,
    pub description: String,
    pub ui_hint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiMessage {
    pub message_type: String,
    pub content: String,
}

#[async_trait::async_trait]
pub trait McpExtension: Send + Sync {
    fn spec(&self) -> ExtensionSpec;
    async fn execute(&self, args: serde_json::Value) -> Result<serde_json::Value, String>;
}

/// A registry mapping UI events/extensions back to Rust execution logic.
pub struct GooseMcpRegistry {
    extensions: std::collections::HashMap<String, Arc<dyn McpExtension>>,
}

impl Default for GooseMcpRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl GooseMcpRegistry {
    pub fn new() -> Self {
        Self {
            extensions: std::collections::HashMap::new(),
        }
    }

    pub fn register(&mut self, extension: Arc<dyn McpExtension>) {
        self.extensions.insert(extension.spec().id, extension);
    }

    pub fn get_specs(&self) -> Vec<ExtensionSpec> {
        self.extensions.values().map(|ext| ext.spec()).collect()
    }

    pub async fn execute_extension(
        &self,
        id: &str,
        args: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        if let Some(ext) = self.extensions.get(id) {
            ext.execute(args).await
        } else {
            Err(format!("Extension '{}' not found", id))
        }
    }

    /// Simulate sending a message to a TypeScript UI bridge
    pub fn send_to_ui(&self, msg: UiMessage) -> Result<(), String> {
        // In a real implementation this would serialize to JSON and send over IPC/WebSocket.
        let json = serde_json::to_string(&msg).map_err(|e| e.to_string())?;
        tracing::debug!("Goose UI Bridge: Sending to TS -> {}", json);
        Ok(())
    }
}

// Example Minimal Extension
pub struct SampleExtension;

#[async_trait::async_trait]
impl McpExtension for SampleExtension {
    fn spec(&self) -> ExtensionSpec {
        ExtensionSpec {
            id: "sample_mcp".to_string(),
            name: "Sample MCP".to_string(),
            description: "A minimal MCP extension example".to_string(),
            ui_hint: Some("button".to_string()),
        }
    }

    async fn execute(&self, args: serde_json::Value) -> Result<serde_json::Value, String> {
        let echo = args
            .get("echo")
            .and_then(|v| v.as_str())
            .unwrap_or("no_echo");
        Ok(serde_json::json!({
            "result": format!("Sample executed with echo: {}", echo)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_goose_mcp_registry() {
        let mut registry = GooseMcpRegistry::new();
        registry.register(Arc::new(SampleExtension));

        let specs = registry.get_specs();
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].id, "sample_mcp");

        let args = serde_json::json!({"echo": "hello goose"});
        let result = registry
            .execute_extension("sample_mcp", args)
            .await
            .unwrap();
        assert_eq!(result["result"], "Sample executed with echo: hello goose");

        let msg = UiMessage {
            message_type: "notification".to_string(),
            content: "Update complete".to_string(),
        };
        assert!(registry.send_to_ui(msg).is_ok());
    }

    #[tokio::test]
    async fn test_goose_mcp_not_found() {
        let registry = GooseMcpRegistry::new();
        let result = registry
            .execute_extension("non_existent", serde_json::json!({}))
            .await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Extension 'non_existent' not found");
    }
}
