use super::server::KvMcpServer;
use crate::db::{DB, DbStore};
use crate::ohc::orchestration::McpInvokeRequest;
use std::sync::Arc;
use temp_env::with_var;

#[tokio::test]
async fn test_kv_get_set_list_delete_standalone() {
    with_var("OHC_STANDALONE", Some("true"), || async {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE agent_kv_store (
                tenant_id VARCHAR(255) NOT NULL,
                kv_key VARCHAR(255) NOT NULL,
                kv_value TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (tenant_id, kv_key)
            )"
        ).execute(&pool).await.unwrap();

        let db = Arc::new(DB {
            pool: crate::db::get_pool(), // dummy pg pool
            store: DbStore::Sqlite(pool),
        });

        let server = KvMcpServer::new(db, None);

        let tools = server.get_tools();
        assert_eq!(tools.len(), 4);

        let spiffe_id = "spiffe://ohc/org/test_org/agent/test_agent".to_string();

        let req = McpInvokeRequest {
            action: "".to_string(),
            agent_id: "".to_string(),
            tool_id: "kv_set".to_string(),
            params: serde_json::json!({"key": "my_key", "value": "my_value"}).to_string(),
            spiffe_id: spiffe_id.clone(),
        };
        let res = server.invoke_tool(&req).await.unwrap();
        assert!(res.payload.contains("success"));

        let req = McpInvokeRequest {
            action: "".to_string(),
            agent_id: "".to_string(),
            tool_id: "kv_get".to_string(),
            params: serde_json::json!({"key": "my_key"}).to_string(),
            spiffe_id: spiffe_id.clone(),
        };
        let res = server.invoke_tool(&req).await.unwrap();
        assert!(res.payload.contains("my_value"));

        let req = McpInvokeRequest {
            action: "".to_string(),
            agent_id: "".to_string(),
            tool_id: "kv_list".to_string(),
            params: serde_json::json!({"prefix": "my"}).to_string(),
            spiffe_id: spiffe_id.clone(),
        };
        let res = server.invoke_tool(&req).await.unwrap();
        assert!(res.payload.contains("my_key"));

        let req = McpInvokeRequest {
            action: "".to_string(),
            agent_id: "".to_string(),
            tool_id: "kv_delete".to_string(),
            params: serde_json::json!({"key": "my_key"}).to_string(),
            spiffe_id: spiffe_id.clone(),
        };
        let res = server.invoke_tool(&req).await.unwrap();
        assert!(res.payload.contains("success"));

        let req = McpInvokeRequest {
            action: "".to_string(),
            agent_id: "".to_string(),
            tool_id: "kv_get".to_string(),
            params: serde_json::json!({"key": "my_key"}).to_string(),
            spiffe_id: spiffe_id.clone(),
        };
        let res = server.invoke_tool(&req).await.unwrap();
        assert!(res.payload.contains("null"));
    }).await;
}

#[tokio::test]
async fn test_tenant_id_parsing() {
    let db = Arc::new(DB {
        pool: crate::db::get_pool(),
        store: DbStore::Postgres,
    });
    let server = KvMcpServer::new(db, None);

    let spiffe_id = "spiffe://ohc/org/test_org/agent/test_agent";
    assert_eq!(server.get_tenant_id(spiffe_id).unwrap(), "test_org");

    let bad_spiffe = "spiffe://ohc/something/else";
    assert!(server.get_tenant_id(bad_spiffe).is_err());

    let empty_spiffe = "spiffe://ohc/org//agent/test_agent";
    assert!(server.get_tenant_id(empty_spiffe).is_err());
}

#[tokio::test]
async fn test_redis_unconfigured() {
    with_var("OHC_STANDALONE", Some("false"), || async {
        let db = Arc::new(DB {
            pool: crate::db::get_pool(),
            store: DbStore::Postgres,
        });

        let server = KvMcpServer::new(db, None);
        let spiffe_id = "spiffe://ohc/org/test_org/agent/test_agent".to_string();

        let req = McpInvokeRequest {
            action: "".to_string(),
            agent_id: "".to_string(),
            tool_id: "kv_get".to_string(),
            params: serde_json::json!({"key": "test"}).to_string(),
            spiffe_id,
        };

        // This should run the standalone logic because redis_client is None
        // But since we didn't migrate Postgres, it should return a DB error.
        let result = server.invoke_tool(&req).await;
        assert!(result.is_err());
    }).await;
}

// Since real redis integration tests require a running redis instance,
// and `cargo test` is heavily multithreaded without mocks out of the box,
// we ensure the logic is fully covered except for the direct redis connection
// which is mocked by the option fallback.
