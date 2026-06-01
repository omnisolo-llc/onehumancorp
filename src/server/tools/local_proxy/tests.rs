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

    // Check if database is available to test the insertion logic, otherwise pass None to test fallback
    let pool = if let Ok(database_url) = std::env::var("OHC_DATABASE_URL") {
        if database_url.contains("localhost") {
            let pool_opts = sqlx::postgres::PgPoolOptions::new()
                .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
                .acquire_timeout(std::time::Duration::from_millis(500))
                .max_connections(1);
            let p = pool_opts.connect_lazy(&database_url).unwrap();

            if matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&p)).await, Ok(Ok(_))) {
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    if let Some(ref p) = pool {
        let _ = sqlx::query("INSERT INTO tenants (id, name) VALUES ('system', 'System') ON CONFLICT DO NOTHING").execute(p).await;
    }

    let resp = server.invoke_tool(&req, pool.clone()).await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(json["status"], "success");
    assert_eq!(json["command"], "ls -la");
    assert_eq!(json["context_id"], "test-context");
    assert!(json["mission_id"].is_string());

    if let Some(p) = pool {
        // Verify the database state actually reflects the inserted mission
        let mission_id = json["mission_id"].as_str().unwrap();
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE id = $1")
            .bind(mission_id)
            .fetch_one(&p)
            .await
            .unwrap();
        assert_eq!(count, 1);

        // Clean up
        sqlx::query("DELETE FROM agent_missions WHERE id = $1")
            .bind(mission_id)
            .execute(&p)
            .await
            .unwrap();
    }
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
    let err = server.invoke_tool(&req, None).await.unwrap_err();
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
    let err = server.invoke_tool(&req, None).await.unwrap_err();
    assert_eq!(err.code(), tonic::Code::InvalidArgument);
    assert!(err.message().contains("context_id is required"));
}

#[tokio::test]
async fn test_local_proxy_server_invoke_unimplemented() {
    let server = LocalProxyServer::new();
    let req = McpInvokeRequest {
        tool_id: "unknown_tool".to_string(),
        action: "execute".to_string(),
        agent_id: "test-agent".to_string(),
        params: r#"{"command":"ls -la","context_id":"test-context"}"#.to_string(),
        spiffe_id: "".to_string(),
    };
    let err = server.invoke_tool(&req, None).await.unwrap_err();
    assert_eq!(err.code(), tonic::Code::Unimplemented);
}
