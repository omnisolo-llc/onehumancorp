#[cfg(test)]
mod tests {
    use crate::tools::edge_caching::server::EdgeCachingMcpServer;
    #[allow(unused_imports)]
    use crate::ohc::orchestration::McpInvokeRequest;

    #[tokio::test]
    async fn test_mcp_seo_generator() {
        let server = EdgeCachingMcpServer::new();
        let req = McpInvokeRequest {
            tool_id: "mcp_seo_generator".to_string(),
            params: serde_json::json!({
                "tenant_id": "tenant_123",
                "product_data": {
                    "name": "Vegan Chocolate Cake",
                    "description": "Delicious vegan chocolate cake."
                }
            }).to_string(),
            spiffe_id: "spiffe://onehumancorp.io/org/tenant_123/agent/agent_1".to_string(),
            action: "".to_string(),
            agent_id: "".to_string(),
        };

        let res = server.invoke_tool(&req).await.unwrap();
        let payload: serde_json::Value = serde_json::from_str(&res.payload).unwrap();

        assert_eq!(payload["status"], "success");
        assert_eq!(payload["tenant_id"], "tenant_123");
        assert!(payload["seo_metadata"]["json_ld"]["name"].as_str().unwrap() == "Vegan Chocolate Cake");
    }

    #[tokio::test]
    async fn test_mcp_edge_kv_sync() {
        let server = EdgeCachingMcpServer::new();
        let req = McpInvokeRequest {
            tool_id: "mcp_edge_kv_sync".to_string(),
            params: serde_json::json!({
                "tenant_id": "tenant_123",
                "product_id": "prod_456",
                "inventory_count": 10
            }).to_string(),
            spiffe_id: "spiffe://onehumancorp.io/org/tenant_123/agent/agent_1".to_string(),
            action: "".to_string(),
            agent_id: "".to_string(),
        };

        let res = server.invoke_tool(&req).await.unwrap();
        let payload: serde_json::Value = serde_json::from_str(&res.payload).unwrap();

        assert_eq!(payload["status"], "success");
        assert_eq!(payload["synced_key"], "tenant:tenant_123:product:prod_456:inventory");
        assert_eq!(payload["inventory_count"], 10);
    }
}
