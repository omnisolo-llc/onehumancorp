use super::server::VectorSyncMcpServer;
use ::server_ohc::orchestration::McpInvokeRequest;

#[tokio::test]
async fn test_mcp_vector_sync_get_tools() {
    let server = VectorSyncMcpServer::new();
    let tools = server.get_tools();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].id, "mcp_vector_sync");
}

#[tokio::test]
async fn test_mcp_vector_sync_invoke_missing_params() {
    let server = VectorSyncMcpServer::new();
    let req = McpInvokeRequest {
        tool_id: "mcp_vector_sync".to_string(),
        params: "{}".to_string(),
        spiffe_id: "spiffe://ohc.network/tenant/test/agent/1".to_string(),
        action: "".to_string(),
        agent_id: "".to_string(),
    };

    let res = server.invoke_tool(&req, None).await;
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().code(), tonic::Code::InvalidArgument);
}

// Test pushing data to cloud with invalid structure safely handles error
#[tokio::test]
async fn test_mcp_vector_sync_push_cloud_invalid_structure() {
    let server = VectorSyncMcpServer::new();
    // Using an invalid JSON embedding structure
    let req = McpInvokeRequest {
        tool_id: "mcp_vector_sync".to_string(),
        params: r#"{"embeddings": [{"id": "1", "vector": [1.0, 2.0], "metadata": {}}]}"#.to_string(),
        spiffe_id: "spiffe://ohc.network/tenant/test/agent/1".to_string(),
        action: "".to_string(),
        agent_id: "".to_string(),
    };

    // We expect internal error because pool is missing
    let res = server.invoke_tool(&req, None).await;
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().code(), tonic::Code::Internal);
}
