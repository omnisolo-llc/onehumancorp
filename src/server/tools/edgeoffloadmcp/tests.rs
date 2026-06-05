use super::server::EdgeOffloadMcpServer;
use ::server_ohc::orchestration::McpInvokeRequest;

#[tokio::test]
async fn test_edge_offload_mcp_server_local_route_sensitive() {
    let server = EdgeOffloadMcpServer::new();

    let req = McpInvokeRequest {
        tool_id: "mcp_inference_router".to_string(),
        action: "invoke".to_string(),
        params: r#"{"prompt":"hello","is_sensitive":true,"complexity":"high"}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "spiffe://onehumancorp.io/org-1/agent-1".to_string(),
    };

    let resp = server.invoke_tool(&req).await.unwrap();
    let payload: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(payload["status"], "success");
    assert_eq!(payload["route"], "local");
    assert_eq!(payload["response"], "Local Response to: hello");
}

#[tokio::test]
async fn test_edge_offload_mcp_server_unknown_tool() {
    let server = EdgeOffloadMcpServer::new();

    let req = McpInvokeRequest {
        tool_id: "unknown_tool_123".to_string(),
        action: "invoke".to_string(),
        params: r#"{}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "spiffe://onehumancorp.io/org-1/agent-1".to_string(),
    };

    let err = server.invoke_tool(&req).await.unwrap_err();
    assert_eq!(err.code(), tonic::Code::NotFound);
    assert_eq!(err.message(), "tool unknown_tool_123 not found");
}

#[tokio::test]
async fn test_edge_offload_mcp_server_local_route_low_complexity() {
    let server = EdgeOffloadMcpServer::new();

    let req = McpInvokeRequest {
        tool_id: "mcp_inference_router".to_string(),
        action: "invoke".to_string(),
        params: r#"{"prompt":"hello","is_sensitive":false,"complexity":"low"}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "spiffe://onehumancorp.io/org-1/agent-1".to_string(),
    };

    let resp = server.invoke_tool(&req).await.unwrap();
    let payload: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(payload["status"], "success");
    assert_eq!(payload["route"], "local");
    assert_eq!(payload["response"], "Local Response to: hello");
}

#[tokio::test]
async fn test_edge_offload_mcp_server_cloud_route() {
    let server = EdgeOffloadMcpServer::new();

    let req = McpInvokeRequest {
        tool_id: "mcp_inference_router".to_string(),
        action: "invoke".to_string(),
        params: r#"{"prompt":"hello","is_sensitive":false,"complexity":"high"}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "spiffe://onehumancorp.io/org-1/agent-1".to_string(),
    };

    let resp = server.invoke_tool(&req).await.unwrap();
    let payload: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(payload["status"], "success");
    assert_eq!(payload["route"], "cloud");
    assert_eq!(payload["response"], "Cloud Assisted Response to: hello");
}

#[tokio::test]
async fn test_edge_offload_mcp_server_cloud_route_auth_fallback() {
    let server = EdgeOffloadMcpServer::new();

    let req = McpInvokeRequest {
        tool_id: "mcp_inference_router".to_string(),
        action: "invoke".to_string(),
        params: r#"{"prompt":"hello","is_sensitive":false,"complexity":"high"}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "".to_string(), // Invalid auth
    };

    let resp = server.invoke_tool(&req).await.unwrap();
    let payload: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(payload["status"], "success");
    assert_eq!(payload["route"], "local");
    assert_eq!(payload["response"], "Local Response to: hello");
}

#[tokio::test]
async fn test_edge_offload_mcp_server_cloud_route_force_fallback() {
    let server = EdgeOffloadMcpServer::new();

    let req = McpInvokeRequest {
        tool_id: "mcp_inference_router".to_string(),
        action: "invoke".to_string(),
        params: r#"{"prompt":"hello","is_sensitive":false,"complexity":"high","force_fallback":true}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "spiffe://onehumancorp.io/org-1/agent-1".to_string(),
    };

    let resp = server.invoke_tool(&req).await.unwrap();
    let payload: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(payload["status"], "success");
    assert_eq!(payload["route"], "local");
    assert_eq!(payload["response"], "Local Response to: hello");
}
