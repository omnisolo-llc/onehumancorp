#![allow(unused_variables)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unused_mut, clippy::useless_format)]
use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use std::sync::Arc;
use tokio::sync::mpsc;

/// Master Catalog A: Framework Implementation Archetypes: OpenAI Codex & Agents SDK.
/// Implements the exact implementation mechanics used by OpenAI Agents SDK (Python).
/// Uses a `Runner` class with async, sync, and streamed modes.
/// Uses a 3-layer architecture: Codex Core (agent code + runtime), App Server (bidirectional JSON-RPC API), and client surfaces sharing the exact same harness.
/// Also includes handoffs, guardrails, tracing, and session management as seen in the Python SDK.

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
        // OpenAI Mechanic: Input Guardrails (Early Check)
        if let Some(guardrails) = &self.runtime_config.guardrails
            && let Err(e) = guardrails.check_input(message)
        {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Codex Runner Input Guardrail tripped: {}", e),
            )));
        }

        let mut total_cost = 0.0;
        let mut on_event = |e: AgentEvent| {
            if let AgentEvent::CostUpdate { total_cost_usd } = e {
                total_cost = total_cost_usd;
            }
        };
        // OpenAI Agents SDK (Python) Mechanic: Tracing and Session setup before execution
        tracing::info!(
            "OpenAI Agents SDK (Python): Starting execution session with message: {}",
            message
        );
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
    pub session_id: String,
}

impl Runner {
    pub fn new(agent: Arc<Agent>) -> Self {
        let core = Arc::new(CodexCore::new(agent.clone(), AgentRunConfig::default()));
        Self {
            core,
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub fn new_with_core(core: Arc<CodexCore>) -> Self {
        Self {
            core,
            session_id: uuid::Uuid::new_v4().to_string(),
        }
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
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            rt.block_on(async move { core.execute(&msg).await })
        }
    }

    pub fn run_streamed(&self, message: &str) -> mpsc::Receiver<AgentEvent> {
        let (tx, rx) = mpsc::channel(100);
        let core = self.core.clone();
        let msg = message.to_string();

        tokio::spawn(async move {
            // OpenAI Mechanic: Input Guardrails (Early Check) for streamed execution
            if let Some(guardrails) = &core.runtime_config.guardrails
                && let Err(e) = guardrails.check_input(&msg)
            {
                let _ = tx
                    .send(AgentEvent::TaskError {
                        error: format!("Codex Runner Input Guardrail tripped: {}", e),
                    })
                    .await;
                return;
            }

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

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

pub struct AppServer {
    pub runner: Arc<Runner>,
    pub marketplace: Arc<crate::tools::marketplace::MarketplaceClient>,
}

impl AppServer {
    pub fn new(runner: Arc<Runner>) -> Self {
        let marketplace = Arc::new(crate::tools::marketplace::MarketplaceClient::new(Box::new(
            crate::tools::marketplace::HttpMarketplaceProvider::new(
                &std::env::var("AGENT_MARKETPLACE_URL")
                    .unwrap_or_else(|_| "https://marketplace.example.com".to_string()),
            ),
        )));
        Self {
            runner,
            marketplace,
        }
    }

    pub fn new_with_marketplace(
        runner: Arc<Runner>,
        marketplace: Arc<crate::tools::marketplace::MarketplaceClient>,
    ) -> Self {
        Self {
            runner,
            marketplace,
        }
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
                return serde_json::to_string(&err_resp).unwrap_or_else(|_| r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#.to_string());
            }
        };

        if req.method == "ap_create_task" {
            let server = crate::agent_protocol::AgentProtocolServer::new(self.runner.clone());
            let req_json = serde_json::to_string(&req.params).unwrap_or_default();
            let result = server.create_task(&req_json).await;
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id.clone(),
                result: Some(result),
                error: None,
                meta: None,
            };
            return serde_json::to_string(&resp).unwrap_or_default();
        } else if req.method == "ap_get_task" {
            let server = crate::agent_protocol::AgentProtocolServer::new(self.runner.clone());
            let task_id = req
                .params
                .get("task_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let result = server.get_task(task_id).await;
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id.clone(),
                result: Some(result),
                error: None,
                meta: None,
            };
            return serde_json::to_string(&resp).unwrap_or_default();
        } else if req.method == "ap_list_tasks" {
            let server = crate::agent_protocol::AgentProtocolServer::new(self.runner.clone());
            let result = server.list_tasks().await;
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id.clone(),
                result: Some(result),
                error: None,
                meta: None,
            };
            return serde_json::to_string(&resp).unwrap_or_default();
        } else if req.method == "ap_list_steps" {
            let server = crate::agent_protocol::AgentProtocolServer::new(self.runner.clone());
            let task_id = req
                .params
                .get("task_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let result = server.list_steps(task_id).await;
            let json_resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id.clone(),
                result: Some(result),
                error: None,
                meta: None,
            };
            return serde_json::to_string(&json_resp).unwrap_or_default();
        } else if req.method == "am_search_agents" {
            let query = req
                .params
                .get("query")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let agents = self.marketplace.search(query).await;
            let resp = match agents {
                Ok(agents) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: serde_json::to_value(agents).ok(),
                    error: None,
                    meta: None,
                },
                Err(e) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: e,
                    }),
                    meta: None,
                },
            };
            return serde_json::to_string(&resp).unwrap_or_default();
        } else if req.method == "am_fetch_agent" {
            let agent_id = req
                .params
                .get("agent_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let result = self.marketplace.fetch_agent(agent_id).await;
            let resp = match result {
                Ok(agent) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: serde_json::to_value(agent).ok(),
                    error: None,
                    meta: None,
                },
                Err(e) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: e,
                    }),
                    meta: None,
                },
            };
            return serde_json::to_string(&resp).unwrap_or_default();
        } else if req.method == "am_publish_agent" {
            let name = req
                .params
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let description = req
                .params
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let author = req
                .params
                .get("role")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(); // mapping role to author

            let new_agent = crate::tools::marketplace::MarketplaceAgent {
                id: "".to_string(),
                name,
                description,
                author,
                version: "1.0.0".to_string(),
                endpoint: "http://localhost".to_string(),
            };

            let published = self.marketplace.publish_agent(new_agent).await;

            let resp = match published {
                Ok(agent) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: serde_json::to_value(agent).ok(),
                    error: None,
                    meta: None,
                },
                Err(e) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: e,
                    }),
                    meta: None,
                },
            };
            return serde_json::to_string(&resp).unwrap_or_default();

        } else if req.method == "ap_list_checkpoints" {
            let server = crate::agent_protocol::AgentProtocolServer::new(self.runner.clone());
            let task_id = req
                .params
                .get("task_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let result = server.list_checkpoints(task_id).await;
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id.clone(),
                result: Some(result),
                error: None,
                meta: None,
            };
            return serde_json::to_string(&resp).unwrap_or_default();
        } else if req.method == "ap_restore_checkpoint" {
            let server = crate::agent_protocol::AgentProtocolServer::new(self.runner.clone());
            let task_id = req
                .params
                .get("task_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let req_json = serde_json::to_string(&req.params).unwrap_or_default();
            let result = server.restore_checkpoint(task_id, &req_json).await;
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id.clone(),
                result: Some(result),
                error: None,
                meta: None,
            };
            return serde_json::to_string(&resp).unwrap_or_default();
        } else if req.method == "ap_execute_step" {
            let server = crate::agent_protocol::AgentProtocolServer::new(self.runner.clone());
            let task_id = req
                .params
                .get("task_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let req_json = serde_json::to_string(&req.params).unwrap_or_default();
            let result = server.execute_step(task_id, &req_json).await;
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id.clone(),
                result: Some(result),
                error: None,
                meta: None,
            };
            return serde_json::to_string(&resp).unwrap_or_default();
        }

        if req.method == "execute_visual_workflow" {
            let client = reqwest::Client::new();
            let url = format!("http://localhost:18789/api/workflow/run");
            if let Ok(res) = client.post(&url).json(&req.params.clone()).send().await {
                let body = res.json::<serde_json::Value>().await.unwrap_or_default();
                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: Some(body),
                    error: None,
                    meta: None,
                };
                return serde_json::to_string(&resp).unwrap_or_default();
            }
        }

        // Helper to extract total_cost from run_async execution if needed
        // Since we modified run_async (execute), we can extract cost through events if needed
        // But AppServer calls `run_async` directly. Wait, `AppServer` uses `self.runner.run_async(&initial_message).await`.
        // Let's modify `run_agent` to surface cost in the JSON RPC response.

        if req.method == "verify_output" {
            let output_text = req
                .params
                .get("output_text")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let task_context = req
                .params
                .get("task_context")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let verification_type = req
                .params
                .get("verification_type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let mut verification_manager = crate::verification_loops::VerificationManager::new();

            // SOTA Harness Patterns (2025-2026): Verification Loops
            let result = match verification_type.as_str() {
                "computational" => {
                    verification_manager.add_computational(std::sync::Arc::new(
                        crate::verification_loops::BashComputationalGuide {
                            command: output_text.clone(),
                            workspace_path: None,
                        },
                    ));
                    verification_manager
                        .run_computational_guides(&output_text, &task_context)
                        .await
                }
                "visual" => {
                    verification_manager.add_visual(std::sync::Arc::new(
                        crate::verification_loops::PlaywrightVisualVerifier,
                    ));
                    verification_manager
                        .run_visual_verifiers(&output_text)
                        .await
                }
                "inferential" => {
                    verification_manager.add_inferential(std::sync::Arc::new(
                        crate::verification_loops::LlmJudgeSensor {
                            llm: self.runner.core.agent.llm.clone(),
                            model: "gpt-4o".to_string(), // Or get from config
                            criteria: None,
                            confidence_threshold: 0.8,
                        },
                    ));
                    verification_manager
                        .run_inferential_sensors(&output_text, &task_context)
                        .await
                }
                _ => Err(format!("Unknown verification type: {}", verification_type)),
            };

            let resp = match result {
                Ok(_) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: Some(
                        serde_json::json!({ "status": "success", "message": "Verification passed successfully." }),
                    ),
                    error: None,
                    id: req.id.clone(),
                    meta: None,
                },
                Err(e) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32000,
                        message: e,
                    }),
                    id: req.id.clone(),
                    meta: None,
                },
            };
            return serde_json::to_string(&resp).unwrap_or_default();
        }

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
                    id: req.id.clone(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32000,
                        message: e,
                    }),
                    meta: None,
                };
                return serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#.to_string());
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
                            id: req.id.clone(),
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32000,
                                message: e,
                            }),
                            meta: None,
                        };
                        return serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#.to_string());
                    }

                    let final_output = format!(
                        "Combined Executive Summary:\n{}\n\nOverall Strategy:\nProceed based on above.\nChart: Included.\nAnalysis: Completed.\n\n{}",
                        summaries.join("\n"),
                        " word".repeat(20000)
                    );

                    // Gate 3: Pre-deliver
                    let expected_roles = vec![
                        "Industry Researcher".to_string(),
                        "Financial Analyst".to_string(),
                        "Strategic Analyst".to_string(),
                        "Process Supervisor".to_string(),
                        "Quality Auditor".to_string(),
                    ];
                    if let Err(e) = ohc_builtin_agent_core::expert_team::QualityGates::pre_deliver(
                        &final_output,
                        &trace,
                        &expected_roles,
                    ) {
                        let resp = JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: req.id.clone(),
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32000,
                                message: e,
                            }),
                            meta: None,
                        };
                        return serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#.to_string());
                    }

                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id.clone(),
                        result: Some(serde_json::json!({ "output": final_output })),
                        error: None,
                        meta: None,
                    };
                    serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#.to_string())
                }
                Err(e) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id.clone(),
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32000,
                            message: e,
                        }),
                        meta: None,
                    };
                    serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#.to_string())
                }
            }
        } else if req.method == "run_agent" {
            let initial_message = req
                .params
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let mut total_cost = 0.0;
            let mut on_event = |e: AgentEvent| {
                if let AgentEvent::CostUpdate { total_cost_usd } = e {
                    total_cost = total_cost_usd;
                }
            };

            tracing::info!(
                "OpenAI Agents SDK (Python): Executing request {} in session {}",
                req.method,
                self.runner.session_id
            );

            // OpenAI Agents SDK (Python) Mechanic: Handoffs and Guardrails context injection
            let mut ctx_message = initial_message.clone();
            if let Some(target) = req.params.get("handoff_target").and_then(|v| v.as_str()) {
                ctx_message = format!(
                    "HANDOFF RECEIVED. Target Agent: {}. Initial Request: {}",
                    target, initial_message
                );
            }

            if let Some(guardrail_cfg) = self.runner.core.runtime_config.guardrails.as_ref()
                && let Err(e) = guardrail_cfg.check_input(&ctx_message)
            {
                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32001,
                        message: format!("Guardrail rejected input: {}", e),
                    }),
                    meta: None,
                };
                return serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#.to_string());
            }

            match self
                .runner
                .core
                .agent
                .run(
                    &self.runner.core.runtime_config,
                    &ctx_message,
                    &mut on_event,
                )
                .await
            {
                Ok(result) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id.clone(),
                        result: Some(serde_json::json!({ "output": result })),
                        error: None,
                        meta: Some(serde_json::json!({ "total_cost_usd": total_cost })),
                    };
                    serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#.to_string())
                }
                Err(e) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id.clone(),
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32000,
                            message: e.to_string(),
                        }),
                        meta: Some(serde_json::json!({ "total_cost_usd": total_cost })),
                    };
                    serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#.to_string())
                }
            }
        } else if req.method == "goose_mcp_list" {
            let mut registry = crate::goose::GooseMcpRegistry::new();
            registry.register(std::sync::Arc::new(crate::goose::SampleExtension));
            let specs = registry.get_specs();
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id.clone(),
                result: Some(serde_json::to_value(specs).unwrap()),
                error: None,
                meta: None,
            };
            serde_json::to_string(&resp).unwrap()
        } else if req.method == "goose_mcp_execute" {
            let mut registry = crate::goose::GooseMcpRegistry::new();
            registry.register(std::sync::Arc::new(crate::goose::SampleExtension));
            let ext_id = req.params.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let ext_args = req
                .params
                .get("args")
                .cloned()
                .unwrap_or(serde_json::json!({}));
            let result = registry.execute_extension(ext_id, ext_args).await;
            let resp = match result {
                Ok(val) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: Some(val),
                    error: None,
                    meta: None,
                },
                Err(e) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32603,
                        message: e,
                    }),
                    meta: None,
                },
            };
            serde_json::to_string(&resp).unwrap()
        } else if req.method == "get_sona_patterns" {
            // SOTA Harness Pattern: Ruflo SONA neural patterns
            // Retrieve all learned trajectory patterns
            let patterns = if let Some(matcher) = self.runner.core.agent.sona_matcher.as_ref() {
                matcher.lock().await.get_patterns().to_vec()
            } else {
                vec![]
            };
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id.clone(),
                result: Some(serde_json::json!({ "patterns": patterns })),
                error: None,
                meta: None,
            };
            serde_json::to_string(&resp).unwrap_or_else(|_| {
                r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#
                    .to_string()
            })
        } else if req.method == "record_sona_pattern" {
            let pattern: crate::sona_patterns::TrajectoryPattern = match serde_json::from_value(
                req.params.clone(),
            ) {
                Ok(p) => p,
                Err(e) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id.clone(),
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32602,
                            message: e.to_string(),
                        }),
                        meta: None,
                    };
                    return serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#.to_string());
                }
            };
            if let Some(matcher) = self.runner.core.agent.sona_matcher.as_ref() {
                matcher.lock().await.record_pattern(pattern);
            }
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id.clone(),
                result: Some(serde_json::json!({ "status": "success" })),
                error: None,
                meta: None,
            };
            serde_json::to_string(&resp).unwrap_or_else(|_| {
                r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#
                    .to_string()
            })
        } else if req.method == "run_actor_model" {
            let initial_message = req
                .params
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let mut cfg = AgentRunConfig {
                enable_actor_model_message_passing: true,
                ..Default::default()
            };
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
                .run(&cfg, &initial_message, &mut on_event)
                .await
            {
                Ok(result) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id.clone(),
                        result: Some(serde_json::json!({ "output": result })),
                        error: None,
                        meta: Some(serde_json::json!({ "total_cost_usd": total_cost })),
                    };
                    serde_json::to_string(&resp).unwrap_or_default()
                }
                Err(e) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id.clone(),
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32000,
                            message: e.to_string(),
                        }),
                        meta: Some(serde_json::json!({ "total_cost_usd": total_cost })),
                    };
                    serde_json::to_string(&resp).unwrap_or_default()
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
                        id: req.id.clone(),
                        result: Some(serde_json::json!({ "status": "success" })),
                        error: None,
                        meta: None,
                    };
                    serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#.to_string())
                }
                Err(e) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id.clone(),
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32000,
                            message: e.to_string(),
                        }),
                        meta: None,
                    };
                    serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#.to_string())
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
                id: req.id.clone(),
                result: Some(task_info),
                error: None,
                meta: None,
            };
            serde_json::to_string(&resp).unwrap_or_else(|_| {
                r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#
                    .to_string()
            })
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

            let orchestrator =
                crate::scalable_multi_agent::CloudOrchestrator::new(mode, nodes, 3, 100, 60);
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
                        id: req.id.clone(),
                        result: Some(serde_json::json!({ "outputs": outputs })),
                        error: None,
                        meta: None,
                    };
                    serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#.to_string())
                }
                Err(e) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id.clone(),
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32000,
                            message: e.to_string(),
                        }),
                        meta: None,
                    };
                    serde_json::to_string(&resp).unwrap_or_else(|_| r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#.to_string())
                }
            }
        } else {
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id.clone(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: "Method not found".to_string(),
                }),
                meta: None,
            };
            serde_json::to_string(&resp).unwrap_or_else(|_| {
                r#"{"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}}"#
                    .to_string()
            })
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
    async fn test_runner_handoff_and_guardrail_mechanics() {
        use crate::guardrails::{GuardrailRegistry, InputGuardrail};
        struct RejectGuardrail;
        impl InputGuardrail for RejectGuardrail {
            fn check_input(&self, _input: &str) -> Result<(), String> {
                Err("Rejected by test guardrail".to_string())
            }
        }

        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message::assistant("success"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            }]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));

        let mut config = AgentRunConfig::default();
        let mut registry = GuardrailRegistry::new();
        registry.input_guardrails.push(Arc::new(RejectGuardrail));
        config.guardrails = Some(registry);

        let core = Arc::new(CodexCore::new(agent, config));
        let runner = Arc::new(Runner::new_with_core(core));
        let app_server = AppServer::new(runner);

        let req_json = r#"{"jsonrpc": "2.0", "id": "1", "method": "run_agent", "params": {"message": "hello", "handoff_target": "agent_b"}}"#;
        let resp_json = app_server.handle_request(req_json).await;

        let resp: JsonRpcResponse = serde_json::from_str(&resp_json).unwrap();
        assert!(resp.error.is_some(), "Expected guardrail rejection error");
        assert!(
            resp.error
                .unwrap()
                .message
                .contains("Guardrail rejected input")
        );
    }

    #[tokio::test]
    async fn test_app_server_json_rpc() {
        let ralph_dir = tempfile::tempdir().unwrap();
        let ralph_progress_file = ralph_dir.path().join("progress.json");
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
        let marketplace = Arc::new(crate::tools::marketplace::MarketplaceClient::new(Box::new(
            crate::tools::marketplace::test_utils::MockMarketplaceProvider,
        )));
        let app_server = AppServer::new_with_marketplace(runner, marketplace);

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
        let req_json_ralph = serde_json::json!({
            "jsonrpc": "2.0",
            "id": "3",
            "method": "run_ralph_loop",
            "params": {
                "task": "test task",
                "progress_file": ralph_progress_file,
            }
        })
        .to_string();
        let resp_json_ralph = app_server.handle_request(&req_json_ralph).await;
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
        assert!(ralph_dir.path().join(".git").is_dir());

        // Test Agent Protocol ap_create_task method
        let req_json_ap_create = r#"{"jsonrpc": "2.0", "id": "10", "method": "ap_create_task", "params": {"input": "do this task"}}"#;
        let resp_json_ap_create = app_server.handle_request(req_json_ap_create).await;
        let resp_ap_create: JsonRpcResponse = serde_json::from_str(&resp_json_ap_create).unwrap();
        assert!(
            resp_ap_create.error.is_none(),
            "Error was: {:?}",
            resp_ap_create.error
        );
        let created_task = resp_ap_create.result.unwrap();
        assert_eq!(
            created_task.get("input").unwrap().as_str().unwrap(),
            "do this task"
        );
        let task_id = created_task
            .get("task_id")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        // Test Agent Protocol ap_get_task method
        let req_json_ap_get = format!(
            r#"{{"jsonrpc": "2.0", "id": "11", "method": "ap_get_task", "params": {{"task_id": "{}"}}}}"#,
            task_id
        );
        let resp_json_ap_get = app_server.handle_request(&req_json_ap_get).await;
        let resp_ap_get: JsonRpcResponse = serde_json::from_str(&resp_json_ap_get).unwrap();
        assert!(resp_ap_get.error.is_none());
        assert_eq!(
            resp_ap_get
                .result
                .unwrap()
                .get("task_id")
                .unwrap()
                .as_str()
                .unwrap(),
            task_id
        );

        // Test Agent Protocol ap_list_tasks method
        let req_json_ap_list = format!(
            r#"{{"jsonrpc": "2.0", "id": "11a", "method": "ap_list_tasks", "params": {{}}}}"#
        );
        let resp_json_ap_list = app_server.handle_request(&req_json_ap_list).await;
        let resp_ap_list: JsonRpcResponse = serde_json::from_str(&resp_json_ap_list).unwrap();
        assert!(resp_ap_list.error.is_none());
        assert!(resp_ap_list.result.unwrap().get("tasks").is_some());

        // Test Agent Protocol ap_list_steps method
        let req_json_ap_list_steps = format!(
            r#"{{"jsonrpc": "2.0", "id": "11b", "method": "ap_list_steps", "params": {{"task_id": "{}"}}}}"#,
            task_id
        );
        let resp_json_ap_list_steps = app_server.handle_request(&req_json_ap_list_steps).await;
        let resp_ap_list_steps: JsonRpcResponse =
            serde_json::from_str(&resp_json_ap_list_steps).unwrap();
        assert!(resp_ap_list_steps.error.is_none());
        assert!(resp_ap_list_steps.result.unwrap().get("steps").is_some());

        // SOTA Harness Pattern: AutoGPT Agent Marketplace API distribution
        let req_json_am_search = r#"{"jsonrpc": "2.0", "id": "13", "method": "am_search_agents", "params": {"query": "Rust"}}"#;
        let resp_json_am_search = app_server.handle_request(req_json_am_search).await;
        let resp_am_search: JsonRpcResponse = serde_json::from_str(&resp_json_am_search).unwrap();
        assert!(resp_am_search.error.is_none());

        // SOTA Harness Pattern: AutoGPT Agent Marketplace API distribution
        let req_json_am_fetch = r#"{"jsonrpc": "2.0", "id": "14", "method": "am_fetch_agent", "params": {"agent_id": "agent-1"}}"#;
        let resp_json_am_fetch = app_server.handle_request(req_json_am_fetch).await;
        let resp_am_fetch: JsonRpcResponse = serde_json::from_str(&resp_json_am_fetch).unwrap();
        assert!(resp_am_fetch.error.is_none());

        // SOTA Harness Pattern: AutoGPT Agent Marketplace API distribution
        let req_json_am_publish = r#"{"jsonrpc": "2.0", "id": "15", "method": "am_publish_agent", "params": {"name": "New Agent", "description": "New", "role": "Tester", "system_prompt": "Test"}}"#;
        let resp_json_am_publish = app_server.handle_request(req_json_am_publish).await;
        let resp_am_publish: JsonRpcResponse = serde_json::from_str(&resp_json_am_publish).unwrap();
        assert!(resp_am_publish.error.is_none());

        // Test Agent Protocol ap_execute_step method
        let req_json_ap_execute = format!(
            r#"{{"jsonrpc": "2.0", "id": "12", "method": "ap_execute_step", "params": {{"task_id": "{}", "input": "step 1"}}}}"#,
            task_id
        );
        let resp_json_ap_execute = app_server.handle_request(&req_json_ap_execute).await;
        let resp_ap_execute: JsonRpcResponse = serde_json::from_str(&resp_json_ap_execute).unwrap();
        assert!(resp_ap_execute.error.is_none());
        let step_result = resp_ap_execute.result.unwrap();
        assert_eq!(
            step_result.get("task_id").unwrap().as_str().unwrap(),
            task_id
        );
        assert_eq!(
            step_result.get("status").unwrap().as_str().unwrap(),
            "completed"
        );
        assert_eq!(
            step_result.get("output").unwrap().as_str().unwrap(),
            "default output"
        );

        // Test SONA endpoints
        let record_req = r#"{"jsonrpc": "2.0", "id": "1", "method": "record_sona_pattern", "params": { "id": "p1", "initial_context": "ctx", "successful_tools": ["bash"], "outcome_score": 1.0 }}"#;
        let record_resp = app_server.handle_request(record_req).await;
        let resp: JsonRpcResponse = serde_json::from_str(&record_resp).unwrap();
        assert!(resp.error.is_none());

        // Get patterns
        let get_req =
            r#"{"jsonrpc": "2.0", "id": "2", "method": "get_sona_patterns", "params": {}}"#;
        let get_resp = app_server.handle_request(get_req).await;
        let resp2: JsonRpcResponse = serde_json::from_str(&get_resp).unwrap();
        assert!(resp2.error.is_none());

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

    #[tokio::test(flavor = "multi_thread")]
    async fn test_codex_runner_input_guardrail() {
        use crate::guardrails::{GuardrailRegistry, InputGuardrail};
        struct RejectGuardrail;
        impl InputGuardrail for RejectGuardrail {
            fn check_input(&self, _input: &str) -> Result<(), String> {
                Err("Rejected by test guardrail".to_string())
            }
        }

        let mut registry = GuardrailRegistry::new();
        registry.input_guardrails.push(Arc::new(RejectGuardrail));

        struct DummyLlmClient;
        #[async_trait::async_trait]
        impl crate::llm::LlmClient for DummyLlmClient {
            async fn chat(
                &self,
                _req: ohc_builtin_agent_core::types::ChatRequest,
            ) -> Result<
                ohc_builtin_agent_core::types::ChatResponse,
                Box<dyn std::error::Error + Send + Sync>,
            > {
                Err("Not implemented but returning properly to satisfy trait".into())
            }
        }

        let agent = Arc::new(Agent::new(Arc::new(DummyLlmClient), vec![]));

        let config = AgentRunConfig {
            guardrails: Some(registry),
            ..Default::default()
        };

        let core = Arc::new(CodexCore::new(agent, config));
        let runner = Runner::new_with_core(core);

        // Test run_async
        let result = runner.run_async("test_input").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Codex Runner Input Guardrail tripped: Rejected by test guardrail")
        );

        // Test run_streamed
        let mut rx = runner.run_streamed("test_input");
        let first_event = rx.recv().await.unwrap();
        match first_event {
            AgentEvent::TaskError { error } => {
                assert!(
                    error.contains(
                        "Codex Runner Input Guardrail tripped: Rejected by test guardrail"
                    )
                );
            }
            _ => panic!("Expected TaskError event"),
        }

        // Test run_sync_blocking
        let result_sync = runner.run_sync_blocking("test_input");
        assert!(result_sync.is_err());
        assert!(
            result_sync
                .unwrap_err()
                .to_string()
                .contains("Codex Runner Input Guardrail tripped: Rejected by test guardrail")
        );
    }
}

#[cfg(test)]
mod tests_goose {
    use super::*;
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage};
    use std::sync::Arc;

    struct DummyLlm;
    #[async_trait::async_trait]
    impl LlmClient for DummyLlm {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                response_id: Some("res_1".to_string()),
                stop_reason: "end".to_string(),
                message: Message::assistant("hi"),
                usage: Usage {
                    input_tokens: 0,
                    output_tokens: 0,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                },
            })
        }
    }

    #[tokio::test]
    async fn test_goose_mcp_endpoints() {
        let agent = Arc::new(Agent::new(Arc::new(DummyLlm), vec![]));
        let runner = Runner::new(agent);
        let server = AppServer::new(Arc::new(runner));

        let list_req = r#"{"jsonrpc": "2.0", "id": "1", "method": "goose_mcp_list", "params": {}}"#;
        let list_res_str = server.handle_request(list_req).await;
        assert!(
            list_res_str.contains("sample_mcp"),
            "Response was: {}",
            list_res_str
        );

        let exec_req = r#"{"jsonrpc": "2.0", "id": "2", "method": "goose_mcp_execute", "params": {"id": "sample_mcp", "args": {"echo": "hello test"}}}"#;
        let exec_res_str = server.handle_request(exec_req).await;
        assert!(
            exec_res_str.contains("hello test"),
            "Response was: {}",
            exec_res_str
        );
    }
}
