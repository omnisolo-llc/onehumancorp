use super::agent_protocol::agent_protocol_tool;
use axum::{Json, Router, response::IntoResponse, routing::post};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use tokio::net::TcpListener;

async fn mock_ap_handler(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    let method = payload.get("method").unwrap().as_str().unwrap();

    if method == "ap_list_tasks" {
        Json(json!({
            "jsonrpc": "2.0",
            "id": payload.get("id").unwrap(),
            "result": {"tasks": []}
        }))
    } else {
        Json(json!({
            "jsonrpc": "2.0",
            "id": payload.get("id").unwrap(),
            "error": {"code": -32601, "message": "Method not found"}
        }))
    }
}

#[tokio::test]
async fn test_agent_protocol_tool_executor_success() {
    temp_env::async_with_vars(
        [("MCPANY_DANGEROUS_ALLOW_LOCAL_IPS", Some("true"))],
        async {
            // Start a mock Axum server
            let app = Router::new().route("/ap", post(mock_ap_handler));
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let addr = listener.local_addr().unwrap();

            let server_task = tokio::spawn(async move {
                axum::serve(listener, app).await.unwrap();
            });

            let endpoint = format!("http://{}/ap", addr);

            let tool = agent_protocol_tool();
            let args = json!({
                "endpoint": endpoint,
                "method": "ap_list_tasks",
                "params": {}
            });
            let result: Result<String, ToolError> = tool.execute.execute(args).await;

            server_task.abort();

            assert!(result.is_ok(), "Result was: {:?}", result);
            let msg = result.unwrap();
            assert!(
                msg.contains("ap_list_tasks executed successfully. Result: {\"tasks\":[]}"),
                "Msg was: {}",
                msg
            );
        },
    )
    .await;
}

#[tokio::test]
async fn test_agent_protocol_tool_executor_missing_arg() {
    let tool = agent_protocol_tool();
    let args = json!({
        "endpoint": "http://localhost:8000/ap",
        "method": "ap_list_tasks"
    });
    let result: Result<String, ToolError> = tool.execute.execute(args).await;
    assert!(result.is_err());
    if let Err(ToolError::LlmRecoverable(msg)) = result {
        assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
    } else {
        panic!("Expected Pydantic-first validation error");
    }
}
