use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use ohc_builtin_agent_core::types::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

/// App Server layer of the 3-layer architecture for the OpenAI Codex archetype.
pub struct AppServer {
    pub runner: Arc<Runner>,
}

/// OpenAI Codex & Agents SDK Archetype:
/// Uses a `Runner` class with async, sync, and streamed modes.
pub struct Runner {
    pub agent: Arc<Agent>,
}

impl AppServer {
    pub fn new(runner: Arc<Runner>) -> Self {
        Self { runner }
    }

    /// Serves a bidirectional JSON-RPC API over an input and output channel.
    pub async fn serve(
        &self,
        mut input_rx: mpsc::UnboundedReceiver<String>,
        output_tx: mpsc::UnboundedSender<String>,
    ) {
        while let Some(line) = input_rx.recv().await {
            let req: Result<JsonRpcRequest, _> = serde_json::from_str(&line);
            match req {
                Ok(request) => {
                    let method = request.method.clone();

                    if method == "run_streamed" {
                        let cfg: AgentRunConfig = serde_json::from_value(request.params.get("cfg").cloned().unwrap_or_default()).unwrap_or_default();
                        let initial_message = request.params.get("initial_message").and_then(|v| v.as_str()).unwrap_or("");

                        let mut rx = self.runner.run_streamed(&cfg, initial_message);
                        let output_tx_clone = output_tx.clone();

                        tokio::spawn(async move {
                            while let Some(event) = rx.recv().await {
                                let event_val = match serde_json::to_value(&event) {
                                    Ok(v) => v,
                                    Err(_) => continue,
                                };
                                let notif = JsonRpcNotification {
                                    jsonrpc: "2.0".to_string(),
                                    method: "agent_event".to_string(),
                                    params: event_val,
                                };
                                if let Ok(notif_str) = serde_json::to_string(&notif) {
                                    let _ = output_tx_clone.send(notif_str);
                                }
                            }
                        });

                        let resp = JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id.clone(),
                            result: Some(serde_json::json!("stream_started")),
                            error: None,
                        };
                        if let Ok(resp_str) = serde_json::to_string(&resp) {
                            let _ = output_tx.send(resp_str);
                        }

                    } else if method == "run_async" || method == "run_sync" {
                        let cfg: AgentRunConfig = serde_json::from_value(request.params.get("cfg").cloned().unwrap_or_default()).unwrap_or_default();
                        let initial_message = request.params.get("initial_message").and_then(|v| v.as_str()).unwrap_or("").to_string();

                        let runner_clone = self.runner.clone();
                        let output_tx_clone = output_tx.clone();
                        let method_clone = method.clone();
                        let req_id = request.id.clone();
                        tokio::spawn(async move {
                            let result = if method_clone == "run_async" {
                                runner_clone.run_async(&cfg, &initial_message).await
                            } else {
                                let cfg_clone = cfg.clone();
                                let init_msg = initial_message.clone();
                                tokio::task::spawn_blocking(move || runner_clone.run_sync_blocking(&cfg_clone, &init_msg)).await.unwrap_or(Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "spawn_blocking failed"))))
                            };

                            let (res_val, err_val) = match result {
                                Ok(text) => (Some(serde_json::json!(text)), None),
                                Err(e) => (None, Some(serde_json::json!(e.to_string()))),
                            };

                            let resp = JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: req_id,
                                result: res_val,
                                error: err_val,
                            };
                            if let Ok(resp_str) = serde_json::to_string(&resp) {
                                let _ = output_tx_clone.send(resp_str);
                            }
                        });

                    } else {
                        let resp = JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id.clone(),
                            result: None,
                            error: Some(serde_json::json!("Method not found")),
                        };
                        if let Ok(resp_str) = serde_json::to_string(&resp) {
                            let _ = output_tx.send(resp_str);
                        }
                    }
                }
                Err(e) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: None,
                        result: None,
                        error: Some(serde_json::json!(format!("Parse error: {}", e))),
                    };
                    if let Ok(resp_str) = serde_json::to_string(&resp) {
                        let _ = output_tx.send(resp_str);
                    }
                }
            }
        }
    }
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
    async fn test_app_server_rpc() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message::assistant("app server success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            }]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let server = AppServer::new(runner);

        let (input_tx, input_rx) = mpsc::unbounded_channel();
        let (output_tx, mut output_rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            server.serve(input_rx, output_tx).await;
        });

        // Test run_async method
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: "run_async".to_string(),
            params: serde_json::json!({
                "cfg": AgentRunConfig::default(),
                "initial_message": "test async"
            }),
        };
        input_tx.send(serde_json::to_string(&req).unwrap()).unwrap();

        if let Some(resp_str) = output_rx.recv().await {
            let resp: JsonRpcResponse = serde_json::from_str(&resp_str).unwrap();
            assert_eq!(resp.id, Some(serde_json::json!(1)));
            assert_eq!(resp.result, Some(serde_json::json!("app server success")));
        } else {
            panic!("Expected response");
        }

        // Test method not found
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(2)),
            method: "unknown_method".to_string(),
            params: serde_json::json!({}),
        };
        input_tx.send(serde_json::to_string(&req).unwrap()).unwrap();

        if let Some(resp_str) = output_rx.recv().await {
            let resp: JsonRpcResponse = serde_json::from_str(&resp_str).unwrap();
            assert_eq!(resp.id, Some(serde_json::json!(2)));
            assert!(resp.error.is_some());
            assert_eq!(resp.error.unwrap().as_str().unwrap(), "Method not found");
        } else {
            panic!("Expected response");
        }
    }
}
