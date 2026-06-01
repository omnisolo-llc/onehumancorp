use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;
use ::server_telemetry::record_mcp_tool_call;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalTool {
    pub id: String,
    pub name: String,
    pub description: String,
    pub config: Value,
}

pub mod mcp {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub struct Tool {
        pub name: String,
        pub description: String,
        #[serde(rename = "inputSchema")]
        pub input_schema: Value,
    }
}

pub fn convert_to_mcp_tool(t: &InternalTool) -> mcp::Tool {
    mcp::Tool {
        name: t.name.clone(),
        description: t.description.clone(),
        input_schema: t.config.clone(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

pub struct McpClientManager {
    config: ServerConfig,
    child: Arc<Mutex<Option<Child>>>,
    request_id_counter: Arc<Mutex<u64>>,
    response_channels: Arc<Mutex<HashMap<u64, tokio::sync::oneshot::Sender<JsonRpcResponse>>>>,
    tx: Option<mpsc::Sender<String>>,
}

impl McpClientManager {
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            child: Arc::new(Mutex::new(None)),
            request_id_counter: Arc::new(Mutex::new(1)),
            response_channels: Arc::new(Mutex::new(HashMap::new())),
            tx: None,
        }
    }

    pub async fn spawn(&mut self) -> Result<(), String> {
        let mut cmd = Command::new(&self.config.command);
        cmd.args(&self.config.args)
           .envs(&self.config.env)
           .stdin(Stdio::piped())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| format!("Failed to spawn child process: {}", e))?;

        let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
        let stdin = child.stdin.take().ok_or("Failed to open stdin")?;

        *self.child.lock().await = Some(child);

        let (tx, mut rx) = mpsc::channel::<String>(32);
        self.tx = Some(tx);

        // Writer task
        tokio::spawn(async move {
            let mut stdin = stdin;
            while let Some(msg) = rx.recv().await {
                if let Err(e) = stdin.write_all(format!("{}\n", msg).as_bytes()).await {
                    eprintln!("Failed to write to MCP stdin: {}", e);
                    break;
                }
                let _ = stdin.flush().await;
            }
        });

        // Reader task
        let response_channels = self.response_channels.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                if let Ok(response) = serde_json::from_str::<JsonRpcResponse>(&line) {
                    let mut channels = response_channels.lock().await;
                    if let Some(sender) = channels.remove(&response.id) {
                        let _ = sender.send(response);
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn call_tool(&self, name: &str, params: Value) -> Result<Value, String> {
        record_mcp_tool_call(name, "started");

        let id = {
            let mut counter = self.request_id_counter.lock().await;
            let current = *counter;
            *counter += 1;
            current
        };

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id,
            method: "callTool".to_string(),
            params: Some(serde_json::json!({
                "name": name,
                "arguments": params
            })),
        };

        let req_str = serde_json::to_string(&request).map_err(|e| e.to_string())?;

        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
        self.response_channels.lock().await.insert(id, resp_tx);

        if let Some(tx) = &self.tx {
            tx.send(req_str).await.map_err(|e| e.to_string())?;
        } else {
            record_mcp_tool_call(name, "failed_not_spawned");
            return Err("Process not spawned".to_string());
        }

        match tokio::time::timeout(std::time::Duration::from_secs(30), resp_rx).await {
            Ok(Ok(response)) => {
                if let Some(result) = response.result {
                    record_mcp_tool_call(name, "success");
                    Ok(result)
                } else if let Some(error) = response.error {
                    record_mcp_tool_call(name, "error");
                    Err(serde_json::to_string(&error).unwrap_or_else(|_| "Unknown error".to_string()))
                } else {
                    record_mcp_tool_call(name, "error_invalid_response");
                    Err("Invalid response".to_string())
                }
            }
            Ok(Err(_)) => {
                record_mcp_tool_call(name, "channel_closed");
                Err("Response channel closed".to_string())
            }
            Err(_) => {
                record_mcp_tool_call(name, "timeout");
                Err("Request timed out".to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_mcp_tool() {
        let internal = InternalTool {
            id: "1".to_string(),
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            config: serde_json::json!({"type": "object"}),
        };

        let mcp_tool = convert_to_mcp_tool(&internal);
        assert_eq!(mcp_tool.name, "test_tool");
        assert_eq!(mcp_tool.description, "A test tool");
        assert_eq!(mcp_tool.input_schema, serde_json::json!({"type": "object"}));
    }

    #[tokio::test]
    async fn test_mcp_client_manager_spawn_and_call() {
        // We'll mock the stdio server using a simple bash script that reads JSON-RPC and writes a response.
        let config = ServerConfig {
            name: "mock".to_string(),
            command: "bash".to_string(),
            args: vec!["-c".to_string(), "while read line; do echo '{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"success\":true}}'; done".to_string()],
            env: HashMap::new(),
        };

        let mut manager = McpClientManager::new(config);

        let spawn_result = manager.spawn().await;
        assert!(spawn_result.is_ok(), "Should spawn successfully");

        let result = manager.call_tool("test_tool", serde_json::json!({})).await;
        assert!(result.is_ok(), "Should receive successful response");
        assert_eq!(result.unwrap(), serde_json::json!({"success": true}));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServeMcpRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: Option<Value>,
}

/// Expose the OHC Agent Harness itself as an MCP Server
pub struct HarnessMcpServer {
    // The agent harness MCP Server could hold configuration here
}

impl HarnessMcpServer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn serve(&self, req_str: &str) -> Result<String, String> {
        let req: ServeMcpRequest = serde_json::from_str(req_str).map_err(|e| e.to_string())?;

        if req.method == "listTools" {
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(serde_json::json!({
                    "tools": [
                        {
                            "name": "harness_action",
                            "description": "Execute an action in the harness",
                            "inputSchema": { "type": "object" }
                        }
                    ]
                })),
                error: None,
            };
            return serde_json::to_string(&resp).map_err(|e| e.to_string());
        }

        let err_resp = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: req.id,
            result: None,
            error: Some(serde_json::json!({"code": -32601, "message": "Method not found"})),
        };
        serde_json::to_string(&err_resp).map_err(|e| e.to_string())
    }
}
