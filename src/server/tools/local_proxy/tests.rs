use super::server::LocalProxyServer;
use ::server_ohc::orchestration::McpInvokeRequest;
use sqlx::sqlite::SqlitePoolOptions;
use uuid::Uuid;

async fn get_test_server() -> (LocalProxyServer, String) {
    let db_id = Uuid::new_v4();
    let uri = format!("sqlite:file:{}?mode=memory&cache=shared", db_id);
    let pool = SqlitePoolOptions::new().connect(&uri).await.unwrap();
    let dir = format!("/tmp/test_session_{}", db_id);
    (LocalProxyServer::new(pool, dir.clone()), dir)
}

#[tokio::test]
async fn test_local_proxy_server_tools() {
    let (server, dir) = get_test_server().await;
    let tools = server.get_tools();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].id, "local_stateful_proxy");
    let _ = tokio::fs::remove_dir_all(&dir).await;
}

#[tokio::test]
async fn test_local_proxy_server_invoke() {
    let (server, dir) = get_test_server().await;
    let req = McpInvokeRequest {
        tool_id: "local_stateful_proxy".to_string(),
        action: "execute".to_string(),
        agent_id: "test-agent".to_string(),
        params: r#"{"command":"echo 'hello_proxy_test'","context_id":"test-context"}"#.to_string(),
        spiffe_id: "".to_string(),
    };
    let resp = server.invoke_tool(&req).await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(json["status"], "success");
    assert_eq!(json["command"], "echo 'hello_proxy_test'");
    assert_eq!(json["context_id"], "test-context");
    assert!(json["output"].as_str().unwrap().contains("hello_proxy_test"));
    let _ = tokio::fs::remove_dir_all(&dir).await;
}

#[tokio::test]
async fn test_local_proxy_server_invoke_missing_command() {
    let (server, dir) = get_test_server().await;
    let req = McpInvokeRequest {
        tool_id: "local_stateful_proxy".to_string(),
        action: "execute".to_string(),
        agent_id: "test-agent".to_string(),
        params: r#"{"context_id":"test-context"}"#.to_string(),
        spiffe_id: "".to_string(),
    };
    let err = server.invoke_tool(&req).await.unwrap_err();
    assert_eq!(err.code(), tonic::Code::InvalidArgument);
    assert!(err.message().contains("command is required"));
    let _ = tokio::fs::remove_dir_all(&dir).await;
}

#[tokio::test]
async fn test_local_proxy_server_invoke_missing_context_id() {
    let (server, dir) = get_test_server().await;
    let req = McpInvokeRequest {
        tool_id: "local_stateful_proxy".to_string(),
        action: "execute".to_string(),
        agent_id: "test-agent".to_string(),
        params: r#"{"command":"ls -la"}"#.to_string(),
        spiffe_id: "".to_string(),
    };
    let err = server.invoke_tool(&req).await.unwrap_err();
    assert_eq!(err.code(), tonic::Code::InvalidArgument);
    assert!(err.message().contains("context_id is required"));
    let _ = tokio::fs::remove_dir_all(&dir).await;
}

#[tokio::test]
async fn test_local_proxy_server_invoke_unimplemented() {
    let (server, dir) = get_test_server().await;
    let req = McpInvokeRequest {
        tool_id: "unknown_tool".to_string(),
        action: "execute".to_string(),
        agent_id: "test-agent".to_string(),
        params: r#"{"command":"ls -la","context_id":"test-context"}"#.to_string(),
        spiffe_id: "".to_string(),
    };
    let err = server.invoke_tool(&req).await.unwrap_err();
    assert_eq!(err.code(), tonic::Code::Unimplemented);
    let _ = tokio::fs::remove_dir_all(&dir).await;
}
pub fn pad_test() {}
