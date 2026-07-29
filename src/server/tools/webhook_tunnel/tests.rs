use super::server::WebhookTunnelMcpServer;
use ::server_ohc::orchestration::McpInvokeRequest;
use std::sync::Arc;

#[tokio::test]
async fn test_webhook_forward_tool() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_millis(1))
        .connect_lazy("postgres://invalid:invalid@localhost:1/test")
        .unwrap();

    let server = WebhookTunnelMcpServer::new(Arc::new(pool));

    let tools = server.get_tools();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].id, "webhook_forward");

    let req = McpInvokeRequest {
        action: "".to_string(),
        agent_id: "agent_123".to_string(),
        tool_id: "webhook_forward".to_string(),
        params: serde_json::json!({"payload": "test_payload"}).to_string(),
        spiffe_id: "spiffe://ohc.local/org/test_org/agent/test_agent".to_string(),
    };

    // The invoke tool will try to connect to postgres which will fail.
    // That's fine, we just assert the logic returns the correct error rather than panicking.
    let res = server.invoke_tool(&req).await;
    assert!(res.is_err());
}
