use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

// Simulated MCP Client Gateway
struct McpGatewayClient {
    base_url: String,
}

impl McpGatewayClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn discover_tools(&self, query: &str) -> Result<Vec<Value>, String> {
        // In a real implementation, this would make an HTTP request to the MCP Gateway
        // For the scope of this proof-of-concept, we simulate a response
        if query.to_lowercase().contains("weather") {
            Ok(vec![
                json!({
                    "name": "weather_api",
                    "description": "Get local weather",
                    "parameters": {"type": "object", "properties": {"location": {"type": "string"}}},
                    "endpoint_url": format!("{}/tools/weather", self.base_url)
                })
            ])
        } else {
            Ok(vec![])
        }
    }

    pub async fn invoke_tool(&self, tool_name: &str, args: Value) -> Result<String, String> {
        // Simulated network call
        Ok(format!("Successfully invoked dynamic tool {} via MCP Gateway with args: {}", tool_name, args))
    }
}


struct McpDynamicDiscoveryExecutor {
    gateway: Arc<McpGatewayClient>,
}

#[async_trait::async_trait]
impl ToolExecutor for McpDynamicDiscoveryExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("mcp_discover: query is required".to_string()))?;

        match self.gateway.discover_tools(query).await {
            Ok(tools) => {
                if tools.is_empty() {
                    Ok(format!("No dynamic tools found matching '{}'", query))
                } else {
                    let mut res = String::from("Found dynamic tools:\n");
                    for t in tools {
                        res.push_str(&format!("- Name: {}\n  Description: {}\n  Schema: {}\n", t["name"], t["description"], t["parameters"]));
                    }
                    Ok(res)
                }
            }
            Err(e) => Err(ToolError::LlmRecoverable(format!("Failed to query MCP Gateway: {}", e))),
        }
    }
}

pub fn mcp_discover_tool(gateway_url: String) -> Tool {
    Tool {
        name: "McpDiscoverTools".to_string(),
        description: "Query the MCP Gateway to dynamically discover external tools based on a search query.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search term to find external capabilities."
                }
            },
            "required": ["query"]
        }),
        execute: Arc::new(McpDynamicDiscoveryExecutor {
            gateway: Arc::new(McpGatewayClient::new(gateway_url))
        }),
    }
}


struct McpDynamicInvokeExecutor {
    gateway: Arc<McpGatewayClient>,
}

#[async_trait::async_trait]
impl ToolExecutor for McpDynamicInvokeExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let tool_name = args["tool_name"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("mcp_invoke: tool_name is required".to_string()))?;

        let tool_args = args.get("arguments").cloned().unwrap_or(json!({}));

        match self.gateway.invoke_tool(tool_name, tool_args).await {
            Ok(res) => Ok(res),
            Err(e) => Err(ToolError::LlmRecoverable(format!("Failed to invoke MCP tool: {}", e))),
        }
    }
}

pub fn mcp_invoke_tool(gateway_url: String) -> Tool {
    Tool {
        name: "McpInvokeTool".to_string(),
        description: "Invoke an external tool dynamically discovered via the MCP Gateway.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "tool_name": {
                    "type": "string",
                    "description": "The exact name of the tool to invoke."
                },
                "arguments": {
                    "type": "object",
                    "description": "The JSON arguments required by the tool's schema."
                }
            },
            "required": ["tool_name", "arguments"]
        }),
        execute: Arc::new(McpDynamicInvokeExecutor {
            gateway: Arc::new(McpGatewayClient::new(gateway_url))
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_discover_tool() {
        let tool = mcp_discover_tool("http://mcp-gateway".to_string());

        let res = tool.execute.execute(json!({"query": "weather"})).await.unwrap();
        assert!(res.contains("weather_api"));

        let res2 = tool.execute.execute(json!({"query": "random"})).await.unwrap();
        assert!(res2.contains("No dynamic tools found"));
    }

    #[tokio::test]
    async fn test_mcp_invoke_tool() {
        let tool = mcp_invoke_tool("http://mcp-gateway".to_string());

        let res = tool.execute.execute(json!({"tool_name": "weather_api", "arguments": {"location": "Seattle"}})).await.unwrap();
        assert!(res.contains("Successfully invoked dynamic tool weather_api"));
        assert!(res.contains("Seattle"));
    }
}
