use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::Mutex;
use super::proxy::authorizer::CapabilityAuthorizer;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Value,
    pub id: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub result: Option<Value>,
    pub error: Option<Value>,
    pub id: Value,
}

pub struct MCPClient {
    child: Mutex<Child>,
    authorizer: Arc<CapabilityAuthorizer>,
    session_id: String,
}

impl MCPClient {
    pub async fn spawn(
        command: &str,
        args: &[String],
        session_id: &str,
        authorizer: Arc<CapabilityAuthorizer>,
    ) -> Result<Self, String> {
        let child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("failed to spawn MCP server: {}", e))?;

        Ok(MCPClient {
            child: Mutex::new(child),
            authorizer,
            session_id: session_id.to_string(),
        })
    }

    pub async fn call_tool(&self, tool_name: &str, arguments: Value) -> Result<Value, String> {
        // Enforce policy via authorizer
        self.authorizer.authorize(&self.session_id, "call_tool", tool_name)?;

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "tools/call".to_string(),
            params: json!({
                "name": tool_name,
                "arguments": arguments,
            }),
            id: json!(1),
        };

        let mut child = self.child.lock().await;
        let req_text = serde_json::to_string(&request).map_err(|e| e.to_string())?;

        {
            let stdin = child.stdin.as_mut().ok_or("failed to open stdin")?;
            stdin.write_all(req_text.as_bytes()).await.map_err(|e| e.to_string())?;
            stdin.write_all(b"\n").await.map_err(|e| e.to_string())?;
            stdin.flush().await.map_err(|e| e.to_string())?;
        }

        let mut line = String::new();
        {
            let stdout = child.stdout.as_mut().ok_or("failed to open stdout")?;
            let mut reader = BufReader::new(stdout);
            reader.read_line(&mut line).await.map_err(|e| e.to_string())?;
        }

        let response: JsonRpcResponse = serde_json::from_str(&line).map_err(|e| e.to_string())?;

        if let Some(error) = response.error {
            return Err(format!("MCP error: {}", error));
        }

        response.result.ok_or_else(|| "missing result in MCP response".to_string())
    }

    pub async fn list_tools(&self) -> Result<Value, String> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "tools/list".to_string(),
            params: json!({}),
            id: json!(2),
        };

        let mut child = self.child.lock().await;
        let req_text = serde_json::to_string(&request).map_err(|e| e.to_string())?;

        {
            let stdin = child.stdin.as_mut().ok_or("failed to open stdin")?;
            stdin.write_all(req_text.as_bytes()).await.map_err(|e| e.to_string())?;
            stdin.write_all(b"\n").await.map_err(|e| e.to_string())?;
            stdin.flush().await.map_err(|e| e.to_string())?;
        }

        let mut line = String::new();
        {
            let stdout = child.stdout.as_mut().ok_or("failed to open stdout")?;
            let mut reader = BufReader::new(stdout);
            reader.read_line(&mut line).await.map_err(|e| e.to_string())?;
        }

        let response: JsonRpcResponse = serde_json::from_str(&line).map_err(|e| e.to_string())?;

        if let Some(error) = response.error {
            return Err(format!("MCP error: {}", error));
        }

        response.result.ok_or_else(|| "missing result in MCP response".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::proxy::authorizer::CapabilityProfile;

    #[tokio::test]
    async fn test_mcp_client_spawn_failure() {
        let authorizer = Arc::new(CapabilityAuthorizer::new(None));
        let res = MCPClient::spawn("non-existent-binary", &[], "session-1", authorizer).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_mcp_client_policy_enforcement() {
        let authorizer = Arc::new(CapabilityAuthorizer::new(None));
        let profile = CapabilityProfile {
            allowed_capabilities: vec![],
            denied_capabilities: vec!["*".to_string()],
        };
        authorizer.set_profile("session-1".to_string(), profile);

        // This should fail because of policy, even if the binary exists
        let client = MCPClient {
            child: Mutex::new(Command::new("true").spawn().unwrap()),
            authorizer,
            session_id: "session-1".to_string(),
        };

        let res = client.call_tool("some_tool", json!({})).await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("denied"));
    }
}
