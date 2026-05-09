use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
use ohc_builtin_agent_core::types::Message;
use crate::agent::{Agent, AgentRunConfig, AgentEvent};

/// Implementation of the "OpenAI Codex & Agents SDK 3-layer architecture"
/// 1. Codex Core (Agent code + runtime)
/// 2. App Server (Bidirectional JSON-RPC API via channel)
/// 3. Client Surfaces (using the identical harness interface)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

pub struct CodexCore {
    agent: Arc<Agent>,
    config: AgentRunConfig,
}

impl CodexCore {
    pub fn new(agent: Arc<Agent>, config: AgentRunConfig) -> Self {
        Self { agent, config }
    }

    /// Stream mode execution from the Codex mechanic
    pub fn stream_run(&self, task: String) -> mpsc::UnboundedReceiver<AgentEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        let agent = self.agent.clone();
        let cfg = self.config.clone();

        tokio::spawn(async move {
            let mut on_event = |event: AgentEvent| {
                let _ = tx.send(event);
            };
            if let Err(e) = agent.run(&cfg, &task, &mut on_event).await {
                let _ = tx.send(AgentEvent::TaskError { error: e.to_string() });
            }
        });
        rx
    }
}

pub struct AppServer {
    core: CodexCore,
}

impl AppServer {
    pub fn new(core: CodexCore) -> Self {
        Self { core }
    }

    /// Bidirectional JSON-RPC endpoint
    pub async fn handle_request(&self, req: JsonRpcRequest) -> JsonRpcResponse {
        if req.method == "run" {
            let task = req.params["task"].as_str().unwrap_or("").to_string();
            let mut rx = self.core.stream_run(task);

            let mut final_content = String::new();
            while let Some(event) = rx.recv().await {
                match event {
                    AgentEvent::TaskComplete { content } => {
                        final_content = content;
                    }
                    AgentEvent::TaskError { error } => {
                        return JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: req.id,
                            result: None,
                            error: Some(serde_json::json!({ "message": error })),
                        };
                    }
                    _ => {} // Client streaming events usually handled here via WS, simplified to block for result in RPC
                }
            }

            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(serde_json::json!({ "output": final_content })),
                error: None,
            }
        } else {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: None,
                error: Some(serde_json::json!({ "message": "Method not found" })),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage};

    struct CodexMockLlmClient;
    #[async_trait::async_trait]
    impl LlmClient for CodexMockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("Codex completed task"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id1".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_codex_architecture() {
        let agent = Arc::new(Agent::new(Arc::new(CodexMockLlmClient), vec![]));
        let config = AgentRunConfig::default();
        let core = CodexCore::new(agent, config);
        let server = AppServer::new(core);

        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "run".to_string(),
            params: serde_json::json!({ "task": "Hello Codex" }),
        };

        let res = server.handle_request(req).await;
        assert_eq!(res.jsonrpc, "2.0");
        assert_eq!(res.id, "1");
        assert!(res.error.is_none());
        assert_eq!(res.result.unwrap()["output"], "Codex completed task");
    }
}
