#![allow(dead_code)]

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



pub struct HybridContextTool {
    pool: sqlx::PgPool,
}

impl HybridContextTool {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn execute(&self, arguments: Value) -> Result<Value, String> {
        let metric_name = arguments.get("metric_name").and_then(|v| v.as_str()).unwrap_or("hybrid_action");
        let metric_type = arguments.get("metric_type").and_then(|v| v.as_str()).unwrap_or("event");
        let value = arguments.get("value").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32;
        let labels = arguments.get("labels").cloned().unwrap_or(json!({}));

        crate::telemetry::buffer_metric(&self.pool, metric_name, metric_type, value, labels)
            .await
            .map_err(|e| e.to_string())?;

        Ok(json!({"status": "success"}))
    }
}

pub struct MCPClient {
    child: Mutex<Child>,
    authorizer: Arc<CapabilityAuthorizer>,
    session_id: String,
    hybrid_tool: Option<HybridContextTool>,
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
            hybrid_tool: None,
        })
    }

    pub fn with_hybrid_tool(mut self, pool: sqlx::PgPool) -> Self {
        self.hybrid_tool = Some(HybridContextTool::new(pool));
        self
    }

    pub async fn call_tool(&self, tool_name: &str, arguments: Value) -> Result<Value, String> {
        // Enforce policy via authorizer
        self.authorizer.authorize(&self.session_id, "call_tool", tool_name)?;

        if tool_name == "RecordHybridContext" {
            if let Some(tool) = &self.hybrid_tool {
                return tool.execute(arguments).await;
            } else {
                return Err("RecordHybridContext tool not configured on this MCPClient".to_string());
            }
        }

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

    #[tokio::test]
    async fn test_hybrid_context_tool_success() {
        // This tests the success path if a database is actually available.
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc?statement_cache_capacity=0".to_string());
        if let Ok(pool) = sqlx::PgPool::connect_lazy(&db_url) {
            // Check if connection is actually alive
            if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
                return;
            }
            let tool = HybridContextTool::new(pool);
            let args = json!({
                "metric_name": "test_action_success",
                "metric_type": "event",
                "value": 1.0,
                "labels": {"source": "test_success"}
            });
            let res = tool.execute(args).await;
            assert!(res.is_ok());
            let val = res.unwrap();
            assert_eq!(val["status"], "success");
        }
    }


    #[tokio::test]
    async fn test_hybrid_context_tool() {
        // We test with a dummy pool URL, expecting execution to attempt buffering
        // and return an error because the pool is disconnected/invalid.
        // This gives us 100% execution coverage on the tool's mapping logic.
        let pool = sqlx::PgPool::connect_lazy("postgres://invalid:invalid@localhost/invalid?statement_cache_capacity=0").unwrap();
        let tool = HybridContextTool::new(pool);

        let args = json!({
            "metric_name": "test_action",
            "metric_type": "event",
            "value": 1.0,
            "labels": {"source": "test"}
        });

        let res = tool.execute(args).await;

        // Assert that we reached the execute phase and it attempted to run buffer_metric
        // Since the DB is invalid, we expect an error related to connection/execution.
        assert!(res.is_err());
        let err_msg = res.unwrap_err();
        assert!(err_msg.contains("pool") || err_msg.contains("connect") || err_msg.contains("error") || err_msg.contains("closed"), "Unexpected error: {}", err_msg);
    }

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
            hybrid_tool: None,
        };

        let res = client.call_tool("some_tool", json!({})).await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("denied"));
    }
}
