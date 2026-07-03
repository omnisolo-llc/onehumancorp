use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::agent::AgentRunConfig;
use crate::codex_runner::Runner;

/// JSON-RPC 2.0 Request
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<serde_json::Value>,
    pub id: Option<serde_json::Value>,
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

pub struct AppState {
    pub runner: Arc<Runner>,
}

#[derive(Debug, Deserialize)]
struct RunParams {
    initial_message: String,
}

async fn handle_rpc(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(payload): Json<JsonRpcRequest>,
) -> Json<JsonRpcResponse> {
    if payload.jsonrpc != "2.0" {
        return Json(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32600,
                message: "Invalid Request: unsupported jsonrpc version".to_string(),
                data: None,
            }),
            id: payload.id,
        });
    }

    if payload.method == "goose_mcp_list" {
        let mut registry = crate::goose::GooseMcpRegistry::new();
        registry.register(std::sync::Arc::new(crate::goose::SampleExtension));
        let specs = registry.get_specs();
        return Json(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(serde_json::to_value(specs).unwrap()),
            error: None,
            id: payload.id.clone(),
        });
    }

    if payload.method == "goose_mcp_execute" {
        let mut registry = crate::goose::GooseMcpRegistry::new();
        registry.register(std::sync::Arc::new(crate::goose::SampleExtension));
        // payload.params is Option<serde_json::Value>
        let ext_id = payload
            .params
            .as_ref()
            .and_then(|p| p.get("id"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let ext_args = payload
            .params
            .as_ref()
            .and_then(|p| p.get("args"))
            .cloned()
            .unwrap_or(serde_json::json!({}));
        let result = registry.execute_extension(ext_id, ext_args).await;
        let resp = match result {
            Ok(val) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: payload.id.clone(),
                result: Some(val),
                error: None,
            },
            Err(e) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: payload.id.clone(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32603,
                    message: e,
                    data: None,
                }),
            },
        };
        return Json(resp);
    }

    if payload.method == "execute_visual_workflow" {
        let workflow_req = payload
            .params
            .clone()
            .unwrap_or_else(|| serde_json::json!({}));
        let client = reqwest::Client::new();
        let url = "http://localhost:18789/api/workflow/run".to_string();
        match client.post(&url).json(&workflow_req).send().await {
            Ok(res) => {
                let body = res.json::<serde_json::Value>().await.unwrap_or_default();
                return Json(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: payload.id.clone(),
                    result: Some(body),
                    error: None,
                });
            }
            Err(_) => {
                // Fallback to error
            }
        }
    }

    let params: RunParams = match payload.params {
        Some(ref p) => match serde_json::from_value(p.clone()) {
            Ok(params) => params,
            Err(e) => {
                return Json(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: format!("Invalid params: {}", e),
                        data: None,
                    }),
                    id: payload.id,
                });
            }
        },
        None => {
            return Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: "Invalid params: missing parameters".to_string(),
                    data: None,
                }),
                id: payload.id,
            });
        }
    };

    let _cfg = AgentRunConfig::default();

    let result = match payload.method.as_str() {
        "run_async" => state.runner.run_async(&params.initial_message).await,
        "run_sync_blocking" => {
            // Note: in a real async server you wouldn't want to actually block the tokio worker thread,
            // but the method is defined as run_sync_blocking on the Runner.
            // We'll wrap it in spawn_blocking to avoid starving the executor.
            let runner_clone = state.runner.clone();
            let initial_message = params.initial_message.clone();
            match tokio::task::spawn_blocking(move || {
                runner_clone.run_sync_blocking(&initial_message)
            })
            .await
            {
                Ok(res) => res,
                Err(e) => Err(format!("Spawn blocking failed: {}", e).into()),
            }
        }
        _method => {
            let app_server = crate::codex_runner::AppServer::new(state.runner.clone());
            let req_str = serde_json::to_string(&payload).unwrap_or_default();
            let res_str = app_server.handle_request(&req_str).await;
            if let Ok(resp) = serde_json::from_str::<JsonRpcResponse>(&res_str) {
                return Json(resp);
            }
            return Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
                id: payload.id,
            });
        }
    };

    match result {
        Ok(output) => Json(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(serde_json::Value::String(output)),
            error: None,
            id: payload.id,
        }),
        Err(e) => Json(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32000,
                message: format!("Execution Error: {}", e),
                data: None,
            }),
            id: payload.id,
        }),
    }
}

pub fn create_router(runner: Arc<Runner>) -> Router {
    let state = Arc::new(AppState { runner });
    Router::new()
        .route("/rpc", post(handle_rpc))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::Agent;
    use crate::llm::LlmClient;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage};
    use std::sync::Arc;
    use tower::ServiceExt; // for `oneshot`

    struct MockLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(
            &self,
            req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            // Check if it's the specific test message
            let last_msg = req
                .messages
                .last()
                .map(|m| m.content.clone())
                .unwrap_or_default();
            let response_text = if last_msg == "async mode test" {
                "async execution successful".to_string()
            } else if last_msg == "sync mode test" {
                "sync execution successful".to_string()
            } else {
                "default success".to_string()
            };

            Ok(ChatResponse {
                message: Message::assistant(&response_text),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    async fn build_test_app() -> Router {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        create_router(runner)
    }

    #[tokio::test]
    async fn test_rpc_run_async() {
        let app = build_test_app().await;

        let req_body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "run_async",
            "params": {
                "initial_message": "async mode test"
            },
            "id": 1
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/rpc")
                    .header("Content-Type", "application/json")
                    .body(Body::from(req_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let resp: JsonRpcResponse = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(resp.jsonrpc, "2.0");
        assert!(resp.error.is_none());
        assert_eq!(
            resp.result.unwrap().as_str().unwrap(),
            "async execution successful"
        );
    }

    #[tokio::test]
    async fn test_rpc_run_sync_blocking() {
        let app = build_test_app().await;

        let req_body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "run_sync_blocking",
            "params": {
                "initial_message": "sync mode test"
            },
            "id": 2
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/rpc")
                    .header("Content-Type", "application/json")
                    .body(Body::from(req_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let resp: JsonRpcResponse = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(resp.jsonrpc, "2.0");
        assert!(resp.error.is_none());
        assert_eq!(
            resp.result.unwrap().as_str().unwrap(),
            "sync execution successful"
        );
    }

    #[tokio::test]
    async fn test_rpc_method_not_found() {
        let app = build_test_app().await;

        let req_body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "invalid_method",
            "params": {
                "initial_message": "test"
            },
            "id": 3
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/rpc")
                    .header("Content-Type", "application/json")
                    .body(Body::from(req_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let resp: JsonRpcResponse = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(resp.jsonrpc, "2.0");
        assert!(resp.result.is_none());
        assert_eq!(resp.error.unwrap().code, -32601);
    }
}
