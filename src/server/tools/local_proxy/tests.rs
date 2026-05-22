use super::server::LocalProxyServer;
use ::server_ohc::orchestration::McpInvokeRequest;

#[tokio::test]
async fn test_local_proxy_server_tools() {
    let server = LocalProxyServer::new();
    let tools = server.get_tools();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].id, "local_stateful_proxy");
}

#[tokio::test]
async fn test_local_proxy_server_invoke() {
    let server = LocalProxyServer::new();
    let req = McpInvokeRequest {
        tool_id: "local_stateful_proxy".to_string(),
        action: "execute".to_string(),
        agent_id: "test-agent".to_string(),
        params: r#"{"command":"ls -la","context_id":"test-context"}"#.to_string(),
        spiffe_id: "".to_string(),
    };
    let resp = server.invoke_tool(&req).await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(json["status"], "success");
    assert_eq!(json["command"], "ls -la");
    assert_eq!(json["context_id"], "test-context");
}

#[tokio::test]
async fn test_local_proxy_server_invoke_missing_command() {
    let server = LocalProxyServer::new();
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
}

#[tokio::test]
async fn test_local_proxy_server_invoke_missing_context_id() {
    let server = LocalProxyServer::new();
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
}

#[tokio::test]
async fn test_local_proxy_server_invoke_unknown() {
    let server = LocalProxyServer::new();
    let req = McpInvokeRequest {
        tool_id: "unknown_tool".to_string(),
        action: "execute".to_string(),
        agent_id: "test-agent".to_string(),
        params: r#"{"command":"ls -la","context_id":"test-context"}"#.to_string(),
        spiffe_id: "".to_string(),
    };
    let resp = server.invoke_tool(&req).await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(json["status"], "error");
}
pub fn pad_test() {}
