use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use std::sync::Arc;
use tokio::sync::mpsc;


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
}

// App Server (bidirectional JSON-RPC API) layer
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

pub struct AppServer {
    pub runner: Arc<Runner>,
}

impl AppServer {
    pub fn new(runner: Arc<Runner>) -> Self {
        Self { runner }
    }

    pub async fn handle_request(&self, req_json: &str) -> String {
        let req: JsonRpcRequest = match serde_json::from_str(req_json) {
            Ok(r) => r,
            Err(_) => {
                let err_resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(JsonRpcError { code: -32700, message: "Parse error".to_string() }),
                };
                return serde_json::to_string(&err_resp).unwrap();
            }
        };

        if req.method == "run_agent" {
            let initial_message = req.params.get("message").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let cfg = AgentRunConfig::default();
            match self.runner.run_async(&cfg, &initial_message).await {
                Ok(result) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: Some(serde_json::json!({ "output": result })),
                        error: None,
                    };
                    serde_json::to_string(&resp).unwrap()
                }
                Err(e) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: None,
                        error: Some(JsonRpcError { code: -32000, message: e.to_string() }),
                    };
                    serde_json::to_string(&resp).unwrap()
                }
            }
        } else if req.method == "run_scalable_agents" {
            let count = req.params.get("count").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
            let message = req.params.get("message").and_then(|v| v.as_str()).unwrap_or("").to_string();

            // Integrate the scalable multi-agent cloud orchestrator
            let mode = if count > 10 {
                crate::scalable_multi_agent::DeploymentMode::CloudDistributed
            } else {
                crate::scalable_multi_agent::DeploymentMode::LocalCli
            };

            // We adapt Agent to AgentNode
            struct AgentNodeAdapter {
                runner: Arc<Runner>,
            }
            #[async_trait::async_trait]
            impl crate::scalable_multi_agent::AgentNode for AgentNodeAdapter {
                async fn execute(&self, chunk: crate::scalable_multi_agent::TaskChunk) -> Result<crate::scalable_multi_agent::TaskResult, String> {
                    let cfg = AgentRunConfig::default();
                    match self.runner.run_async(&cfg, &chunk.payload).await {
                        Ok(res) => Ok(crate::scalable_multi_agent::TaskResult {
                            chunk_id: chunk.id,
                            output: res,
                        }),
                        Err(e) => Err(e.to_string()),
                    }
                }
            }

            let mut nodes: Vec<Arc<dyn crate::scalable_multi_agent::AgentNode>> = Vec::new();
            // In a real cloud setup, these nodes would be distributed endpoints. Here we mock instances.
            for _ in 0..count {
                nodes.push(Arc::new(AgentNodeAdapter { runner: self.runner.clone() }));
            }

            let orchestrator = crate::scalable_multi_agent::CloudOrchestrator::new(mode, nodes);
            let mut tasks = Vec::new();
            for i in 0..count {
                tasks.push(crate::scalable_multi_agent::TaskChunk {
                    id: format!("chunk_{}", i),
                    payload: format!("{} (chunk {})", message, i),
                });
            }

            match orchestrator.distribute_and_execute(tasks).await {
                Ok(results) => {
                    let outputs: Vec<String> = results.into_iter().map(|r| r.output).collect();
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: Some(serde_json::json!({ "outputs": outputs })),
                        error: None,
                    };
                    serde_json::to_string(&resp).unwrap()
                }
                Err(e) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: None,
                        error: Some(JsonRpcError { code: -32000, message: e.to_string() }),
                    };
                    serde_json::to_string(&resp).unwrap()
                }
            }
        } else {
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: None,
                error: Some(JsonRpcError { code: -32601, message: "Method not found".to_string() }),
            };
            serde_json::to_string(&resp).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage};
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
    async fn test_app_server_json_rpc() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message::assistant("rpc success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            }]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        let app_server = AppServer::new(runner);

        let req_json = r#"{"jsonrpc": "2.0", "id": "1", "method": "run_agent", "params": {"message": "hello"}}"#;
        let resp_json = app_server.handle_request(req_json).await;

        let resp: JsonRpcResponse = serde_json::from_str(&resp_json).unwrap();
        assert_eq!(resp.id.unwrap(), serde_json::json!("1"));
        assert!(resp.error.is_none());
        assert_eq!(resp.result.unwrap().get("output").unwrap().as_str().unwrap(), "rpc success");

        // Test run_scalable_agents method
        let req_json_scalable = r#"{"jsonrpc": "2.0", "id": "2", "method": "run_scalable_agents", "params": {"message": "hello", "count": 2}}"#;
        let resp_json_scalable = app_server.handle_request(req_json_scalable).await;
        let resp_scalable: JsonRpcResponse = serde_json::from_str(&resp_json_scalable).unwrap();
        assert!(resp_scalable.error.is_none());
        let outputs = resp_scalable.result.unwrap().get("outputs").unwrap().as_array().unwrap().clone();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0].as_str().unwrap(), "default output");
        assert_eq!(outputs[1].as_str().unwrap(), "default output");

        // Test unknown method
        let req_json_bad = r#"{"jsonrpc": "2.0", "id": "3", "method": "unknown", "params": {}}"#;
        let resp_json_bad = app_server.handle_request(req_json_bad).await;
        let resp_bad: JsonRpcResponse = serde_json::from_str(&resp_json_bad).unwrap();
        assert_eq!(resp_bad.error.unwrap().code, -32601);
    }
}
