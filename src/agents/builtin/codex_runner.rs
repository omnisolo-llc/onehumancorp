use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use std::sync::Arc;
use tokio::sync::mpsc;


/// OpenAI Codex & Agents SDK Archetype:
/// Uses a `Runner` class with async, sync, and streamed modes.
pub struct Runner {
    pub agent: Arc<Agent>,
    pub cloud_manager: tokio::sync::Mutex<Option<std::sync::Arc<crate::scalable_cloud::CloudDeploymentManager>>>,
}

impl Runner {
    pub fn new(agent: Arc<Agent>) -> Self {
        Self { agent, cloud_manager: tokio::sync::Mutex::new(None) }
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

    /// Scalable Distributed Execution Mode (1000+ Agents)
    pub async fn run_distributed(&self, cfg: &AgentRunConfig, tasks: Vec<String>, mode: crate::scalable_cloud::DeploymentMode) -> Result<Vec<String>, String> {
        let runner = Arc::new(crate::tools::runner::SandboxedCommandRunner::new(None));
        let manager = std::sync::Arc::new(crate::scalable_cloud::CloudDeploymentManager::new(mode, self.agent.clone(), runner));
        let ids = manager.submit_jobs(cfg.clone(), tasks).await?;
        *self.cloud_manager.lock().await = Some(manager);
        Ok(ids)
    }

    pub async fn get_batch_status(&self, job_id: &str) -> Option<String> {
        if let Some(m) = self.cloud_manager.lock().await.as_ref() {
            m.get_status(job_id).await
        } else {
            None
        }
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
        } else if req.method == "run_distributed_batch" {
            let tasks: Vec<String> = match req.params.get("tasks").and_then(|v| v.as_array()) {
                Some(arr) => arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect(),
                None => Vec::new(),
            };

            let mode = if req.params.get("mode").and_then(|v| v.as_str()) == Some("cloud") {
                crate::scalable_cloud::DeploymentMode::CloudDistributed
            } else {
                crate::scalable_cloud::DeploymentMode::LocalCLI
            };

            let cfg = AgentRunConfig::default();
            match self.runner.run_distributed(&cfg, tasks, mode).await {
                Ok(ids) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: Some(serde_json::json!({ "job_ids": ids })),
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
        } else if req.method == "get_batch_status" {
            let job_id = req.params.get("job_id").and_then(|v| v.as_str()).unwrap_or("");
            if let Some(status) = self.runner.get_batch_status(job_id).await {
                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: Some(serde_json::json!({ "status": status })),
                    error: None,
                };
                serde_json::to_string(&resp).unwrap()
            } else {
                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: None,
                    error: Some(JsonRpcError { code: -32000, message: "Job not found".to_string() }),
                };
                serde_json::to_string(&resp).unwrap()
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

        // Test unknown method
        let req_json_bad = r#"{"jsonrpc": "2.0", "id": "2", "method": "unknown", "params": {}}"#;
        let resp_json_bad = app_server.handle_request(req_json_bad).await;
        let resp_bad: JsonRpcResponse = serde_json::from_str(&resp_json_bad).unwrap();
        assert_eq!(resp_bad.error.unwrap().code, -32601);
    }
}

#[tokio::test]
async fn test_app_server_distributed_batch() {
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage};

    struct MockLlmClientCloud2;
    #[async_trait::async_trait]
    impl LlmClient for MockLlmClientCloud2 {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id1".to_string()),
            })
        }
    }

    let client = Arc::new(MockLlmClientCloud2);
    let agent = Arc::new(Agent::new(client, vec![]));
    let runner = Arc::new(Runner::new(agent));
    let app_server = AppServer::new(runner);

    let req_json = r#"{"jsonrpc": "2.0", "id": "1", "method": "run_distributed_batch", "params": {"tasks": ["task1", "task2"], "mode": "cloud"}}"#;
    let resp_json = app_server.handle_request(req_json).await;

    let resp: JsonRpcResponse = serde_json::from_str(&resp_json).unwrap();
    assert_eq!(resp.id.unwrap(), serde_json::json!("1"));
    assert!(resp.error.is_none());

    let job_ids = resp.result.unwrap().get("job_ids").unwrap().as_array().unwrap().clone();
    assert_eq!(job_ids.len(), 2);
}
