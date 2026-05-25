use super::server::CrdtSyncMcpServer;
use ::server_ohc::orchestration::McpInvokeRequest;

#[tokio::test]
async fn test_crdt_sync_mcp_pull() {
    let server = CrdtSyncMcpServer::new();

    let req = McpInvokeRequest {
        tool_id: "crdt_pull".to_string(),
        action: "invoke".to_string(),
        params: r#"{"entity_id":"task_123"}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "spiffe-1".to_string(),
    };

    let resp = server.invoke_tool(&req, None, None).await.unwrap();
    let payload: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(payload["status"], "success");
}

#[tokio::test]
async fn test_crdt_sync_mcp_push() {
    let server = CrdtSyncMcpServer::new();

    let req = McpInvokeRequest {
        tool_id: "crdt_push".to_string(),
        action: "invoke".to_string(),
        params: r#"{"deltas":[{"id":"1","entity_id":"task_1","data":"{}","updated_at":"2026-05-25T12:00:00Z"}]}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "spiffe-1".to_string(),
    };

    // Since we don't pass a PG pool, it should fail with pool required in cloud mode
    // We avoid unsafe remove_var by just passing None and seeing the failure naturally
    let err = server.invoke_tool(&req, None, None).await.unwrap_err();
    assert_eq!(err.message(), "sqlite pool required for standalone push"); // Assuming tests default to standalone or we get an error
}
