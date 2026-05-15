use crate::agent::AgentRunConfig;
use crate::codex_runner::Runner;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: serde_json::Value,
    pub id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
    pub id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

pub struct AppServer {
    runner: Arc<Runner>,
}

impl AppServer {
    pub fn new(runner: Arc<Runner>) -> Self {
        Self { runner }
    }

    pub async fn handle_request(&self, req_str: &str) -> String {
        let req: Result<RpcRequest, _> = serde_json::from_str(req_str);
        match req {
            Ok(rpc_req) => {
                if rpc_req.method == "run_agent_sync" {
                    let initial_message = rpc_req.params.get("message").and_then(|v| v.as_str()).unwrap_or("");
                    let cfg = AgentRunConfig::default(); // In a real app, parse from params

                    match self.runner.run_async(&cfg, initial_message).await {
                        Ok(res) => {
                            let resp = RpcResponse {
                                jsonrpc: "2.0".to_string(),
                                result: Some(serde_json::Value::String(res)),
                                error: None,
                                id: rpc_req.id,
                            };
                            serde_json::to_string(&resp).unwrap_or_default()
                        }
                        Err(e) => {
                            let resp = RpcResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(RpcError { code: -32603, message: e.to_string() }),
                                id: rpc_req.id,
                            };
                            serde_json::to_string(&resp).unwrap_or_default()
                        }
                    }
                } else {
                    let resp = RpcResponse {
                        jsonrpc: "2.0".to_string(),
                        result: None,
                        error: Some(RpcError { code: -32601, message: "Method not found".to_string() }),
                        id: rpc_req.id,
                    };
                    serde_json::to_string(&resp).unwrap_or_default()
                }
            }
            Err(_) => {
                let resp = RpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(RpcError { code: -32700, message: "Parse error".to_string() }),
                    id: None,
                };
                serde_json::to_string(&resp).unwrap_or_default()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::Agent;
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage, Message};

    struct MockLlmClient;
    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("RPC Success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_json_rpc_server() {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AppServer::new(runner);

        let req_json = r#"{"jsonrpc": "2.0", "method": "run_agent_sync", "params": {"message": "Hello"}, "id": 1}"#;
        let res_json = server.handle_request(req_json).await;

        let res: RpcResponse = serde_json::from_str(&res_json).unwrap();
        assert_eq!(res.result, Some(serde_json::Value::String("RPC Success".to_string())));

        let req_bad_method = r#"{"jsonrpc": "2.0", "method": "unknown", "params": {}, "id": 2}"#;
        let res_bad_json = server.handle_request(req_bad_method).await;
        let res_bad: RpcResponse = serde_json::from_str(&res_bad_json).unwrap();
        assert!(res_bad.error.is_some());
    }
}
