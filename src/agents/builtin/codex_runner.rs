use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use ohc_builtin_agent_core::types::Message;
use std::sync::Arc;
use tokio::sync::mpsc;

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::io::BufReader;
use tokio::net::TcpListener;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    pub params: Option<serde_json::Value>,
    #[serde(default)]
    pub id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    #[serde(default)]
    pub id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct RunParams {
    pub initial_message: String,
    pub config: Option<serde_json::Value>,
}

/// OpenAI Codex & Agents SDK Archetype:
/// Uses a `Runner` class with async, sync, and streamed modes.
pub struct Runner {
    pub agent: Arc<Agent>,
}

impl Runner {
    pub fn new(agent: Arc<Agent>) -> Self {
        Self { agent }
    }

    /// Asynchronous execution mode
    pub async fn run_async(&self, cfg: &AgentRunConfig, initial_message: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut on_event = |_e| {};
        self.agent.run(cfg, initial_message, &mut on_event).await
    }

    /// Synchronous execution mode (blocks the current thread)
    pub fn run_sync_blocking(&self, cfg: &AgentRunConfig, initial_message: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let cfg = cfg.clone();
        let initial_message = initial_message.to_string();
        let agent = self.agent.clone();

        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(async move {
            let mut on_event = |_e| {};
            agent.run(&cfg, &initial_message, &mut on_event).await
        })
    }

    /// Streamed execution mode (returns a receiver for AgentEvents)
    pub fn run_streamed(&self, cfg: &AgentRunConfig, initial_message: &str) -> mpsc::UnboundedReceiver<AgentEvent> {
        self.agent.clone().query(cfg.clone(), initial_message.to_string())
    }

    pub async fn handle_json_rpc_request(&self, req: JsonRpcRequest) -> JsonRpcResponse {
        if req.method == "run_async" {
            if let Some(params) = req.params {
                if let Ok(run_params) = serde_json::from_value::<RunParams>(params) {
                    let cfg = AgentRunConfig::default(); // TODO: implement full config parsing
                    match self.run_async(&cfg, &run_params.initial_message).await {
                        Ok(res) => return JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            result: Some(serde_json::Value::String(res)),
                            error: None,
                            id: req.id,
                        },
                        Err(e) => return JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            result: None,
                            error: Some(JsonRpcError { code: -32000, message: format!("Execution error: {}", e) }),
                            id: req.id,
                        },
                    }
                }
            }
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError { code: -32602, message: "Invalid params".to_string() }),
                id: req.id,
            }
        } else {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError { code: -32601, message: "Method not found".to_string() }),
                id: req.id,
            }
        }
    }

    /// App Server layer: bidirectional JSON-RPC TCP server
    pub async fn start_json_rpc_server(self: Arc<Self>, addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(addr).await?;
        tracing::info!("JSON-RPC App Server listening on {}", addr);

        loop {
            let (mut socket, _) = match listener.accept().await {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("Accept failed: {}", e);
                    continue;
                }
            };
            let runner_clone = self.clone();

            tokio::spawn(async move {
                let (reader, mut writer) = socket.split();
                let mut reader = BufReader::new(reader);
                let mut line = String::new();

                while let Ok(bytes_read) = reader.read_line(&mut line).await {
                    if bytes_read == 0 {
                        break;
                    }
                    if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(&line) {
                        let res = runner_clone.handle_json_rpc_request(req).await;
                        if let Ok(res_str) = serde_json::to_string(&res) {
                            let _ = writer.write_all(format!("{}\n", res_str).as_bytes()).await;
                        }
                    } else {
                        let err_res = JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            result: None,
                            error: Some(JsonRpcError { code: -32700, message: "Parse error".to_string() }),
                            id: None,
                        };
                        if let Ok(res_str) = serde_json::to_string(&err_res) {
                            let _ = writer.write_all(format!("{}\n", res_str).as_bytes()).await;
                        }
                    }
                    line.clear();
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Usage};
    use std::sync::Arc;

    struct MockLlmClient {
        responses: tokio::sync::Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            if !resps.is_empty() {
                Ok(resps.remove(0))
            } else {
                Ok(ChatResponse {
                    message: Message::assistant("default output"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            }
        }
    }

    #[tokio::test]
    async fn test_runner_async() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message::assistant("async success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            }]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Runner::new(agent);
        let cfg = AgentRunConfig::default();
        let result = runner.run_async(&cfg, "test").await.unwrap();
        assert_eq!(result, "async success");
    }

    #[test]
    fn test_runner_sync() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message::assistant("sync success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            }]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Runner::new(agent);
        let cfg = AgentRunConfig::default();
        let result = runner.run_sync_blocking(&cfg, "test").unwrap();
        assert_eq!(result, "sync success");
    }

    #[tokio::test]
    async fn test_runner_streamed() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message::assistant("stream success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            }]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Runner::new(agent);
        let cfg = AgentRunConfig::default();
        let mut rx = runner.run_streamed(&cfg, "test");

        let mut events = vec![];
        while let Some(event) = rx.recv().await {
            events.push(event);
        }

        let has_complete = events.iter().any(|e| matches!(e, AgentEvent::TaskComplete { .. }));
        assert!(has_complete);
    }

    #[tokio::test]
    async fn test_json_rpc_server_success() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message::assistant("json rpc success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            }]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));

        let addr = "127.0.0.1:0"; // random port
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener); // free the port for start_json_rpc_server

        let runner_clone = runner.clone();
        let port_addr = format!("127.0.0.1:{}", port);

        tokio::spawn(async move {
            let _ = runner_clone.start_json_rpc_server(&port_addr).await;
        });

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let mut stream = tokio::net::TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "run_async",
            "params": {
                "initial_message": "test msg"
            },
            "id": 1
        });

        stream.write_all(format!("{}\n", req.to_string()).as_bytes()).await.unwrap();

        let mut reader = tokio::io::BufReader::new(stream);
        let mut line = String::new();
        reader.read_line(&mut line).await.unwrap();

        let res: JsonRpcResponse = serde_json::from_str(&line).unwrap();
        assert_eq!(res.jsonrpc, "2.0");
        assert_eq!(res.id.unwrap(), serde_json::json!(1));
        assert_eq!(res.result.unwrap(), serde_json::Value::String("json rpc success".to_string()));
        assert!(res.error.is_none());
    }

    #[tokio::test]
    async fn test_json_rpc_server_method_not_found() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Runner::new(agent);

        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "unknown_method".to_string(),
            params: None,
            id: Some(serde_json::json!(2)),
        };

        let res = runner.handle_json_rpc_request(req).await;
        assert_eq!(res.error.unwrap().code, -32601);
    }

    #[tokio::test]
    async fn test_json_rpc_server_invalid_params() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Runner::new(agent);

        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "run_async".to_string(),
            params: Some(serde_json::json!({"wrong_param": "test"})),
            id: Some(serde_json::json!(3)),
        };

        let res = runner.handle_json_rpc_request(req).await;
        assert_eq!(res.error.unwrap().code, -32602);
    }
}
