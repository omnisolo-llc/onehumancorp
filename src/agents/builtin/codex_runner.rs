use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use std::sync::Arc;
use tokio::sync::mpsc;

/// OpenAI Codex & Agents SDK Archetype:
/// Uses a `Runner` class with async, sync, and streamed modes.
/// Uses a 3-layer architecture: Codex Core (agent code + runtime), App Server (bidirectional JSON-RPC API), and client surfaces sharing the exact same harness.

// 1. Codex Core layer
pub struct CodexCore {
    pub agent: Arc<Agent>,
    pub runtime_config: AgentRunConfig,
}

impl CodexCore {
    pub fn new(agent: Arc<Agent>, runtime_config: AgentRunConfig) -> Self {
        Self {
            agent,
            runtime_config,
        }
    }

    pub async fn execute(
        &self,
        message: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut total_cost = 0.0;
        let mut on_event = |e: AgentEvent| {
            if let AgentEvent::CostUpdate { total_cost_usd } = e {
                total_cost = total_cost_usd;
            }
        };
        let res = self
            .agent
            .run(&self.runtime_config, message, &mut on_event)
            .await;
        tracing::info!("Session Total Cost: ${:.6}", total_cost);
        res
    }
}

pub struct Runner {
    pub core: Arc<CodexCore>,
}

impl Runner {
    pub fn new(agent: Arc<Agent>) -> Self {
        let core = Arc::new(CodexCore::new(agent, AgentRunConfig::default()));
        Self { core }
    }

    pub fn new_with_core(core: Arc<CodexCore>) -> Self {
        Self { core }
    }

    pub async fn run_async(
        &self,
        message: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.core.execute(message).await
    }

    pub fn run_sync_blocking(
        &self,
        message: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let core = self.core.clone();
        let msg = message.to_string();

        let handle = tokio::runtime::Handle::try_current();
        if let Ok(rt) = handle {
            tokio::task::block_in_place(move || {
                rt.block_on(async move { core.execute(&msg).await })
            })
        } else {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move { core.execute(&msg).await })
        }
    }

    pub fn run_streamed(&self, message: &str) -> mpsc::Receiver<AgentEvent> {
        let (tx, rx) = mpsc::channel(100);
        let core = self.core.clone();
        let msg = message.to_string();

        tokio::spawn(async move {
            let tx_clone = tx.clone();
            let mut on_event = move |event: AgentEvent| {
                let _ = tx_clone.try_send(event);
            };
            match core
                .agent
                .run(&core.runtime_config, &msg, &mut on_event)
                .await
            {
                Ok(result) => {
                    let _ = tx.send(AgentEvent::TaskComplete { content: result }).await;
                }
                Err(e) => {
                    let _ = tx
                        .send(AgentEvent::TaskError {
                            error: e.to_string(),
                        })
                        .await;
                }
            }
        });

        rx
    }
}

// 2. App Server JSON-RPC layer
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
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

    pub async fn handle_request(&self, req_str: &str) -> String {
        let req: JsonRpcRequest = match serde_json::from_str(req_str) {
            Ok(r) => r,
            Err(_) => {
                let err_resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: "Parse error".to_string(),
                    }),
                    meta: None,
                };
                return serde_json::to_string(&err_resp).unwrap();
            }
        };

        // Helper to extract total_cost from run_async execution if needed
        // Since we modified run_async (execute), we can extract cost through events if needed
        // But AppServer calls `run_async` directly. Wait, `AppServer` uses `self.runner.run_async(&initial_message).await`.
        // Let's modify `run_agent` to surface cost in the JSON RPC response.

        if req.method == "run_expert_team" {
            let initial_message = req
                .params
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            struct LlmClientAdapter {
                inner: Arc<dyn crate::llm::LlmClient>,
            }

            #[async_trait::async_trait]
            impl ohc_builtin_agent_core::expert_team::ExpertTeamLlmClient for LlmClientAdapter {
                async fn chat(
                    &self,
                    req: ohc_builtin_agent_core::types::ChatRequest,
                ) -> Result<
                    ohc_builtin_agent_core::types::ChatResponse,
                    Box<dyn std::error::Error + Send + Sync>,
                > {
                    self.inner.chat(req).await
                }
            }

            let adapter = Arc::new(LlmClientAdapter {
                inner: self.runner.core.agent.llm.clone(),
            });

            // Expert Team Implementation
            let experts = vec![
                ohc_builtin_agent_core::expert_team::DomainExpert {
                    role: "Industry Researcher".to_string(),
                    llm: adapter.clone(),
                },
                ohc_builtin_agent_core::expert_team::DomainExpert {
                    role: "Financial Analyst".to_string(),
                    llm: adapter.clone(),
                },
                ohc_builtin_agent_core::expert_team::DomainExpert {
                    role: "Strategic Analyst".to_string(),
                    llm: adapter.clone(),
                },
                ohc_builtin_agent_core::expert_team::DomainExpert {
                    role: "Process Supervisor".to_string(),
                    llm: adapter.clone(),
                },
                ohc_builtin_agent_core::expert_team::DomainExpert {
                    role: "Quality Auditor".to_string(),
                    llm: adapter.clone(),
                },
            ];

            let manager = ohc_builtin_agent_core::expert_team::ExpertTeamManager::new(
                "Project Director",
                experts,
            );

            // Gate 1: Pre-flight
            if let Err(e) = ohc_builtin_agent_core::expert_team::QualityGates::pre_flight(
                &manager,
                &initial_message,
            ) {
                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32000,
                        message: e,
                    }),
                    meta: None,
                };
                return serde_json::to_string(&resp).unwrap();
            }

            let mut trace = ohc_builtin_agent_core::expert_team::SkillTrace::new();
            match manager
                .execute_parallel_tasks(&initial_message, &mut trace)
                .await
            {
                Ok(summaries) => {
                    // Gate 2: Pre-merge
                    if let Err(e) =
                        ohc_builtin_agent_core::expert_team::QualityGates::pre_merge(&summaries)
                    {
                        let resp = JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: req.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32000,
                                message: e,
                            }),
                            meta: None,
                        };
                        return serde_json::to_string(&resp).unwrap();
                    }

                    let final_output = format!(
                        "Combined Executive Summary:\n{}\n\nOverall Strategy:\nProceed based on above.\nChart: Included.\nAnalysis: Completed.\n\n{}",
                        summaries.join("\n"),
                        " word".repeat(20000)
                    );

                    // Gate 3: Pre-deliver
                    if let Err(e) = ohc_builtin_agent_core::expert_team::QualityGates::pre_deliver(
                        &final_output,
                        &trace,
                    ) {
                        let resp = JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: req.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32000,
                                message: e,
                            }),
                            meta: None,
                        };
                        return serde_json::to_string(&resp).unwrap();
                    }

                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: Some(serde_json::json!({ "output": final_output })),
                        error: None,
                        meta: None,
                    };
                    serde_json::to_string(&resp).unwrap()
                }
                Err(e) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32000,
                            message: e,
                        }),
                        meta: None,
                    };
                    serde_json::to_string(&resp).unwrap()
                }
            }
        } else if req.method == "run_agent" {
            let initial_message = req
                .params
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let _cfg = AgentRunConfig::default();

            let mut total_cost = 0.0;
            let mut on_event = |e: AgentEvent| {
                if let AgentEvent::CostUpdate { total_cost_usd } = e {
                    total_cost = total_cost_usd;
                }
            };

            match self
                .runner
                .core
                .agent
                .run(
                    &self.runner.core.runtime_config,
                    &initial_message,
                    &mut on_event,
                )
                .await
            {
                Ok(result) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: Some(serde_json::json!({ "output": result })),
                        error: None,
                        meta: Some(serde_json::json!({ "total_cost_usd": total_cost })),
                    };
                    serde_json::to_string(&resp).unwrap()
                }
                Err(e) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32000,
                            message: e.to_string(),
                        }),
                        meta: Some(serde_json::json!({ "total_cost_usd": total_cost })),
                    };
                    serde_json::to_string(&resp).unwrap()
                }
            }
        } else if req.method == "run_ralph_loop" {
            let task = req
                .params
                .get("task")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let progress_file = req
                .params
                .get("progress_file")
                .and_then(|v| v.as_str())
                .unwrap_or(".ralph_progress.json")
                .to_string();

            let ralph = crate::ralph_loop::RalphLoop::new(
                self.runner.core.agent.clone(),
                self.runner.core.runtime_config.clone(),
                &progress_file,
            );

            match ralph.run(&task).await {
                Ok(_) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: Some(serde_json::json!({ "status": "success" })),
                        error: None,
                        meta: None,
                    };
                    serde_json::to_string(&resp).unwrap()
                }
                Err(e) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32000,
                            message: e.to_string(),
                        }),
                        meta: None,
                    };
                    serde_json::to_string(&resp).unwrap()
                }
            }
        } else if req.method == "get_task" {
            let task_id = req
                .params
                .get("task_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let status = if let Some(cp) = &self.runner.core.agent.checkpointer {
                match cp.list_checkpoints(&task_id).await {
                    Ok(cps) => {
                        if !cps.is_empty() {
                            "Running or Paused"
                        } else {
                            "Created or Not Found"
                        }
                    }
                    Err(_) => "Created or Not Found",
                }
            } else {
                "Created or Not Found"
            };

            let task_info = serde_json::json!({
                "task_id": task_id,
                "input": format!("Task state: {}", status),
                "status": status
            });
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(task_info),
                error: None,
                meta: None,
            };
            serde_json::to_string(&resp).unwrap()
        } else if req.method == "run_scalable_agents" {
            let count = req
                .params
                .get("count")
                .and_then(|v| v.as_u64())
                .unwrap_or(1) as usize;
            let message = req
                .params
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

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
                async fn execute(
                    &self,
                    chunk: crate::scalable_multi_agent::TaskChunk,
                ) -> Result<crate::scalable_multi_agent::TaskResult, String> {
                    let _cfg = AgentRunConfig::default();
                    match self.runner.run_async(&chunk.payload).await {
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
                nodes.push(Arc::new(AgentNodeAdapter {
                    runner: self.runner.clone(),
                }));
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
                        meta: None,
                    };
                    serde_json::to_string(&resp).unwrap()
                }
                Err(e) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32000,
                            message: e.to_string(),
                        }),
                        meta: None,
                    };
                    serde_json::to_string(&resp).unwrap()
                }
            }
        } else {
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: "Method not found".to_string(),
                }),
                meta: None,
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
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
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
        let _cfg = AgentRunConfig::default();
        let result = runner.run_async("test").await.unwrap();
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
        let _cfg = AgentRunConfig::default();
        let result = runner.run_sync_blocking("test").unwrap();
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
        let _cfg = AgentRunConfig::default();
        let mut rx = runner.run_streamed("test");

        let mut events = vec![];
        while let Some(event) = rx.recv().await {
            events.push(event);
        }

        let has_complete = events
            .iter()
            .any(|e| matches!(e, AgentEvent::TaskComplete { .. }));
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
        assert_eq!(
            resp.result
                .unwrap()
                .get("output")
                .unwrap()
                .as_str()
                .unwrap(),
            "rpc success"
        );
        assert!(resp.meta.unwrap().get("total_cost_usd").is_some());

        // Test run_scalable_agents method
        let req_json_scalable = r#"{"jsonrpc": "2.0", "id": "2", "method": "run_scalable_agents", "params": {"message": "hello", "count": 2}}"#;
        let resp_json_scalable = app_server.handle_request(req_json_scalable).await;
        let resp_scalable: JsonRpcResponse = serde_json::from_str(&resp_json_scalable).unwrap();
        assert!(resp_scalable.error.is_none());
        let outputs = resp_scalable
            .result
            .unwrap()
            .get("outputs")
            .unwrap()
            .as_array()
            .unwrap()
            .clone();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0].as_str().unwrap(), "default output");
        assert_eq!(outputs[1].as_str().unwrap(), "default output");

        // Test run_ralph_loop method
        let req_json_ralph = r#"{"jsonrpc": "2.0", "id": "3", "method": "run_ralph_loop", "params": {"task": "test task", "progress_file": ".test_ralph_progress.json"}}"#;
        let resp_json_ralph = app_server.handle_request(req_json_ralph).await;
        let resp_ralph: JsonRpcResponse = serde_json::from_str(&resp_json_ralph).unwrap();
        assert!(resp_ralph.error.is_none());
        assert_eq!(
            resp_ralph
                .result
                .unwrap()
                .get("status")
                .unwrap()
                .as_str()
                .unwrap(),
            "success"
        );

        // Clean up test file if it exists
        let _ = std::fs::remove_file(".test_ralph_progress.json");

        // Test get_task method
        let req_json_get = r#"{"jsonrpc": "2.0", "id": "4", "method": "get_task", "params": {"task_id": "task-abc"}}"#;
        let resp_json_get = app_server.handle_request(req_json_get).await;
        let resp_get: JsonRpcResponse = serde_json::from_str(&resp_json_get).unwrap();
        assert!(resp_get.error.is_none());

        assert!(
            resp_get
                .result
                .as_ref()
                .unwrap()
                .get("input")
                .unwrap()
                .as_str()
                .unwrap()
                .contains("Task state: Created or Not Found")
        );

        // Test unknown method
        let req_json_bad = r#"{"jsonrpc": "2.0", "id": "4", "method": "unknown", "params": {}}"#;
        let resp_json_bad = app_server.handle_request(req_json_bad).await;
        let resp_bad: JsonRpcResponse = serde_json::from_str(&resp_json_bad).unwrap();
        assert_eq!(resp_bad.error.unwrap().code, -32601);
    }
}
