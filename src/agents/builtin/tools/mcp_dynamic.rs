use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

use super::{Tool, ToolExecutor, pydantic::{PydanticAdapter, PydanticToolExecutor}};
use server_ohc::agent::service::{McpServerConfig, McpTransportType};
use serde::Deserialize;

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


#[derive(Deserialize)]
struct McpDiscoverArgs {
    query: String,
}

struct McpDynamicDiscoveryExecutor {
    gateway: Arc<McpGatewayClient>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<McpDiscoverArgs> for McpDynamicDiscoveryExecutor {
    async fn execute_typed(&self, args: McpDiscoverArgs) -> Result<String, ToolError> {
        let query = &args.query;

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
        execute: Arc::new(PydanticAdapter::new(McpDynamicDiscoveryExecutor {
            gateway: Arc::new(McpGatewayClient::new(gateway_url))
        })),
    }
}


#[derive(Deserialize)]
struct McpInvokeArgs {
    tool_name: String,
    #[serde(default = "default_invoke_args")]
    arguments: Value,
}

fn default_invoke_args() -> Value {
    serde_json::json!({})
}

struct McpDynamicInvokeExecutor {
    gateway: Arc<McpGatewayClient>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<McpInvokeArgs> for McpDynamicInvokeExecutor {
    async fn execute_typed(&self, args: McpInvokeArgs) -> Result<String, ToolError> {
        let tool_name = &args.tool_name;
        let tool_args = args.arguments;

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
        execute: Arc::new(PydanticAdapter::new(McpDynamicInvokeExecutor {
            gateway: Arc::new(McpGatewayClient::new(gateway_url))
        })),
    }
}

#[derive(Clone, Debug)]
struct McpToolSpec {
    server: McpServerConfig,
    exposed_name: String,
    raw_name: String,
    description: String,
    parameters: Value,
}

struct McpConfiguredToolExecutor {
    spec: McpToolSpec,
}

#[async_trait::async_trait]
impl ToolExecutor for McpConfiguredToolExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        match McpTransportType::try_from(self.spec.server.transport)
            .unwrap_or(McpTransportType::McpTransportUnspecified)
        {
            McpTransportType::McpTransportStdio => call_stdio_tool(&self.spec.server, &self.spec.raw_name, args)
                .await
                .map(format_mcp_result)
                .map_err(|e| ToolError::LlmRecoverable(format!("MCP {}: {}", self.spec.exposed_name, e))),
            McpTransportType::McpTransportSse => call_http_tool(&self.spec.server, &self.spec.raw_name, args)
                .await
                .map(format_mcp_result)
                .map_err(|e| ToolError::LlmRecoverable(format!("MCP {}: {}", self.spec.exposed_name, e))),
            McpTransportType::McpTransportUnspecified => Err(ToolError::LlmRecoverable(format!(
                "MCP {}: transport is required",
                self.spec.exposed_name
            ))),
        }
    }
}

pub async fn load_mcp_server_tools(servers: &[McpServerConfig]) -> Vec<Tool> {
    let mut tools = Vec::new();
    for server in servers {
        let specs = discover_mcp_tool_specs(server).await;
        for spec in specs {
            tools.push(Tool {
                name: spec.exposed_name.clone(),
                description: spec.description.clone(),
                is_read_only: false,
                parameters: spec.parameters.clone(),
                execute: Arc::new(McpConfiguredToolExecutor { spec }),
            });
        }
    }
    tools
}

async fn discover_mcp_tool_specs(server: &McpServerConfig) -> Vec<McpToolSpec> {
    let allowed = server
        .allowed_tools
        .iter()
        .filter(|tool| !tool.trim().is_empty())
        .cloned()
        .collect::<Vec<_>>();

    if !allowed.is_empty() {
        return allowed
            .into_iter()
            .map(|raw_name| generic_tool_spec(server, &raw_name))
            .collect();
    }

    let listed = match McpTransportType::try_from(server.transport).unwrap_or(McpTransportType::McpTransportUnspecified) {
        McpTransportType::McpTransportStdio => list_stdio_tools(server).await,
        McpTransportType::McpTransportSse => list_http_tools(server).await,
        McpTransportType::McpTransportUnspecified => Err("transport is required".to_string()),
    };

    match listed {
        Ok(tools) => tools
            .into_iter()
            .filter_map(|tool| {
                let raw_name = tool.get("name").and_then(Value::as_str)?.to_string();
                let mut spec = generic_tool_spec(server, &raw_name);
                if let Some(description) = tool.get("description").and_then(Value::as_str) {
                    spec.description = format!(
                        "Invoke MCP tool '{}' from server '{}'. {}",
                        raw_name, server.name, description
                    );
                }
                if let Some(schema) = tool.get("inputSchema").or_else(|| tool.get("parameters")) {
                    spec.parameters = schema.clone();
                }
                Some(spec)
            })
            .collect(),
        Err(e) => {
            tracing::warn!("Failed to list MCP tools for server '{}': {}", server.name, e); // pii-safe
            Vec::new()
        }
    }
}

fn generic_tool_spec(server: &McpServerConfig, raw_name: &str) -> McpToolSpec {
    let exposed_name = format!(
        "Mcp_{}_{}",
        super::skill::sanitize_tool_suffix(&server.name),
        super::skill::sanitize_tool_suffix(raw_name)
    );

    McpToolSpec {
        server: server.clone(),
        exposed_name,
        raw_name: raw_name.to_string(),
        description: format!("Invoke MCP tool '{}' from server '{}'.", raw_name, server.name),
        parameters: json!({
            "type": "object",
            "additionalProperties": true,
            "description": "Arguments passed through to the MCP tool."
        }),
    }
}

fn format_mcp_result(value: Value) -> String {
    if let Some(content) = value.get("content") && let Some(items) = content.as_array() {
            let mut out = Vec::new();
            for item in items {
                if let Some(text) = item.get("text").and_then(Value::as_str) {
                    out.push(text.to_string());
                } else {
                    out.push(item.to_string());
                }
            }
            return out.join("\n");
    }
    value.to_string()
}

async fn list_stdio_tools(server: &McpServerConfig) -> Result<Vec<Value>, String> {
    let result = stdio_rpc(
        server,
        vec![
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": {"name": "ohc-builtin-agent", "version": "1.0.0"}
                }
            }),
            json!({"jsonrpc": "2.0", "method": "notifications/initialized", "params": {}}),
            json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}),
        ],
        Some(2),
    )
    .await?;
    parse_tools_list(result)
}

async fn call_stdio_tool(server: &McpServerConfig, tool_name: &str, args: Value) -> Result<Value, String> {
    stdio_rpc(
        server,
        vec![
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": {"name": "ohc-builtin-agent", "version": "1.0.0"}
                }
            }),
            json!({"jsonrpc": "2.0", "method": "notifications/initialized", "params": {}}),
            json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "tools/call",
                "params": {"name": tool_name, "arguments": args}
            }),
        ],
        Some(2),
    )
    .await
}

async fn stdio_rpc(
    server: &McpServerConfig,
    requests: Vec<Value>,
    wanted_id: Option<i64>,
) -> Result<Value, String> {
    if server.command.is_empty() {
        return Err("stdio MCP server command is empty".to_string());
    }

    let mut child = Command::new(&server.command[0])
        .args(server.command.iter().skip(1))
        .envs(server.env.clone())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("failed to spawn MCP server: {}", e))?;

    let mut stdin = child.stdin.take().ok_or_else(|| "failed to open MCP stdin".to_string())?;
    let stdout = child.stdout.take().ok_or_else(|| "failed to open MCP stdout".to_string())?;
    let mut reader = BufReader::new(stdout).lines();

    let mut wanted_response = None;
    for request in requests {
        let request_id = request.get("id").and_then(Value::as_i64);
        let text = serde_json::to_string(&request).map_err(|e| e.to_string())?;
        stdin.write_all(text.as_bytes()).await.map_err(|e| e.to_string())?;
        stdin.write_all(b"\n").await.map_err(|e| e.to_string())?;
        stdin.flush().await.map_err(|e| e.to_string())?;

        if request_id.is_none() {
            continue;
        }

        loop {
            let line = tokio::time::timeout(Duration::from_secs(5), reader.next_line())
                .await
                .map_err(|_| "timed out waiting for MCP response".to_string())?
                .map_err(|e| e.to_string())?
                .ok_or_else(|| "MCP server closed stdout".to_string())?;
            let response: Value = serde_json::from_str(&line).map_err(|e| e.to_string())?;
            if let Some(error) = response.get("error") {
                let _ = child.kill().await;
                return Err(format!("MCP error: {}", error));
            }
            if response.get("id").and_then(Value::as_i64) == request_id {
                if wanted_id.map(|id| Some(id) == request_id).unwrap_or(true) {
                    wanted_response = response.get("result").cloned();
                }
                break;
            }
        }
    }

    let _ = child.kill().await;
    wanted_response.ok_or_else(|| "missing result in MCP response".to_string())
}

async fn list_http_tools(server: &McpServerConfig) -> Result<Vec<Value>, String> {
    let result = http_rpc(
        server,
        json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": {}}),
    )
    .await?;
    parse_tools_list(result)
}

async fn call_http_tool(server: &McpServerConfig, tool_name: &str, args: Value) -> Result<Value, String> {
    http_rpc(
        server,
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {"name": tool_name, "arguments": args}
        }),
    )
    .await
}

async fn http_rpc(server: &McpServerConfig, request: Value) -> Result<Value, String> {
    if server.endpoint.trim().is_empty() {
        return Err("HTTP/SSE MCP endpoint is empty".to_string());
    }
    let response: Value = reqwest::Client::new()
        .post(&server.endpoint)
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    if let Some(error) = response.get("error") {
        return Err(format!("MCP error: {}", error));
    }
    response
        .get("result")
        .cloned()
        .ok_or_else(|| "missing result in MCP response".to_string())
}

fn parse_tools_list(result: Value) -> Result<Vec<Value>, String> {
    result
        .get("tools")
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| "MCP tools/list response did not contain a tools array".to_string())
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
