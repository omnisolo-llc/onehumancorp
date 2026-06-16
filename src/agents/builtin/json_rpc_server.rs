use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::codex_runner::Runner;

/// JSON-RPC 2.0 Request
#[derive(Debug, Deserialize)]
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

    // We need to map `json_rpc_server::JsonRpcRequest` to `codex_runner::JsonRpcRequest` structure,
    // which codex_runner::AppServer parses from a JSON string.
    let codex_req = crate::codex_runner::JsonRpcRequest {
        jsonrpc: payload.jsonrpc.clone(),
        id: payload.id.clone(),
        method: payload.method.clone(),
        params: payload.params.clone().unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new())),
    };

    let req_str = serde_json::to_string(&codex_req).unwrap_or_default();

    // Route the raw JSON string to codex_runner::AppServer
    let app_server = crate::codex_runner::AppServer::new(state.runner.clone());
    let resp_str = app_server.handle_request(&req_str).await;

    let resp: JsonRpcResponse = serde_json::from_str(&resp_str).unwrap_or_else(|_| JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: None,
        error: Some(JsonRpcError {
            code: -32603,
            message: "Internal error parsing app server response".to_string(),
            data: None,
        }),
        id: payload.id.clone(),
    });

    Json(resp)
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
