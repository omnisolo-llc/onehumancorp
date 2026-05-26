use super::server::StateSyncMcpServer;
use ::server_ohc::orchestration::McpInvokeRequest;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;

#[tokio::test]
async fn test_get_tools() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    let server = StateSyncMcpServer::new(Arc::new(pool));
    let tools = server.get_tools();
    assert_eq!(tools.len(), 2);
    assert_eq!(tools[0].id, "crdt_push");
    assert_eq!(tools[1].id, "crdt_pull");
}

#[tokio::test]
async fn test_invoke_tool() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    sqlx::query("CREATE TABLE crdt_deltas (id TEXT PRIMARY KEY, entity_id TEXT NOT NULL, data TEXT NOT NULL, updated_at TEXT NOT NULL, sync_status TEXT DEFAULT 'PENDING')").execute(&pool).await.unwrap();

    let server = StateSyncMcpServer::new(Arc::new(pool));

    // Test crdt_push
    let req = McpInvokeRequest {
        spiffe_id: "test".to_string(),
        tool_id: "crdt_push".to_string(),
        params: r#"{"entity_id": "test", "mutations": []}"#.to_string(),
        action: "invoke".to_string(),
        agent_id: "agent_1".to_string(),
    };
    let resp = server.invoke_tool(&req).await.unwrap();
    assert_eq!(resp.payload, req.params);

    // Test crdt_pull
    let req = McpInvokeRequest {
        spiffe_id: "test".to_string(),
        tool_id: "crdt_pull".to_string(),
        params: r#"{"entity_id": "test"}"#.to_string(),
        action: "invoke".to_string(),
        agent_id: "agent_1".to_string(),
    };
    let resp = server.invoke_tool(&req).await.unwrap();
    assert!(resp.payload.contains("mutations"));
    assert!(resp.payload.contains("test"));
}
