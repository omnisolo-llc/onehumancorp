use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DynamicToolSchema {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    pub endpoint_url: String,
    pub required_spiffe_id: Option<String>,
}

pub struct McpGateway {
    registry: RwLock<std::collections::HashMap<String, DynamicToolSchema>>,
}

impl McpGateway {
    pub fn new() -> Self {
        Self {
            registry: RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub async fn register_tool(&self, spiffe_id: &str, schema: DynamicToolSchema) -> Result<(), String> {
        // Authenticate/Authorize via SPIFFE
        if !spiffe_id.starts_with("spiffe://") {
            return Err("Invalid SPIFFE ID".to_string());
        }

        let mut reg = self.registry.write().await;
        reg.insert(schema.name.clone(), schema);
        Ok(())
    }

    pub async fn discover_tools(&self, query: &str) -> Vec<DynamicToolSchema> {
        let reg = self.registry.read().await;
        reg.values()
            .filter(|schema| schema.name.contains(query) || schema.description.contains(query))
            .cloned()
            .collect()
    }

    pub async fn invoke_tool(&self, spiffe_id: &str, tool_name: &str, args: Value) -> Result<String, String> {
        // Find the tool
        let reg = self.registry.read().await;
        let tool = match reg.get(tool_name) {
            Some(t) => t,
            None => return Err(format!("Tool {} not found", tool_name)),
        };

        // SPIFFE authorization check
        if let Some(required_id) = &tool.required_spiffe_id {
            if spiffe_id != required_id && required_id != "*" {
                return Err("Unauthorized SPIFFE ID for this tool".to_string());
            }
        }

        // Simulate invoking the tool via HTTP/RPC
        // In a real implementation, this would make an actual network call to tool.endpoint_url
        Ok(format!("Successfully invoked dynamic tool {} with args: {}", tool_name, args))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_gateway_registration_and_discovery() {
        let gateway = McpGateway::new();

        let schema = DynamicToolSchema {
            name: "weather_api".to_string(),
            description: "Get local weather".to_string(),
            parameters: serde_json::json!({"type": "object", "properties": {"location": {"type": "string"}}}),
            endpoint_url: "http://weather-service.svc.cluster.local".to_string(),
            required_spiffe_id: Some("*".to_string()),
        };

        let result = gateway.register_tool("spiffe://example.org/agent-1", schema).await;
        assert!(result.is_ok());

        let discovered = gateway.discover_tools("weather").await;
        assert_eq!(discovered.len(), 1);
        assert_eq!(discovered[0].name, "weather_api");
    }

    #[tokio::test]
    async fn test_mcp_gateway_invocation_auth() {
        let gateway = McpGateway::new();

        let schema = DynamicToolSchema {
            name: "secure_finance".to_string(),
            description: "Access secure finance data".to_string(),
            parameters: serde_json::json!({}),
            endpoint_url: "http://finance-service.svc.cluster.local".to_string(),
            required_spiffe_id: Some("spiffe://example.org/finance-agent".to_string()),
        };

        gateway.register_tool("spiffe://example.org/admin", schema).await.unwrap();

        // Should fail due to wrong SPIFFE ID
        let res_fail = gateway.invoke_tool("spiffe://example.org/random-agent", "secure_finance", serde_json::json!({})).await;
        assert!(res_fail.is_err());

        // Should succeed with correct SPIFFE ID
        let res_ok = gateway.invoke_tool("spiffe://example.org/finance-agent", "secure_finance", serde_json::json!({})).await;
        assert!(res_ok.is_ok());
    }
}
