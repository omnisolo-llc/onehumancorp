use super::server::EdgeCommerceMcpServer;
use ::server_ohc::orchestration::McpInvokeRequest;
use redis::AsyncCommands;

#[tokio::test]
async fn test_commerce_edge_quote_and_cache() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        println!("Database not available, skipping test_commerce_edge_quote_and_cache");
        return;
    }
    let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
    let pool = match sqlx::postgres::PgPoolOptions::new().connect(&db_url).await {
        Ok(p) => p,
        Err(_) => {
            println!("Could not connect to database, skipping test_commerce_edge_quote_and_cache");
            return;
        }
    };

    let redis_client = match redis::Client::open("redis://localhost:6379/") {
        Ok(c) => c,
        Err(_) => {
            println!("Redis client failed to open, skipping");
            return;
        }
    };

    // Check if redis is actually up before running test logic
    let conn_res = redis_client.get_multiplexed_async_connection().await;
    if conn_res.is_err() {
        // If Redis is not available, we can't run this test. So we gracefully return.
        // This makes the test robust in CI environments where Redis might not be present for all targets.
        println!("Redis not available, skipping test_commerce_edge_quote_and_cache");
        return;
    }

    // clear the cache before test
    let mut conn = conn_res.unwrap();
    let _: () = redis::cmd("DEL").arg("edge_cache:org-1:quote:prod-cache-1:2").query_async(&mut conn).await.unwrap();

    let server = EdgeCommerceMcpServer::new(redis_client.clone(), pool.clone());

    // ensure product exists
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS products (
            id TEXT,
            tenant_id TEXT,
            price_cents BIGINT,
            inventory_count BIGINT,
            PRIMARY KEY (id, tenant_id)
        )"
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO products (id, tenant_id, price_cents, inventory_count) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"
    )
    .bind("prod-cache-1")
    .bind("org-1")
    .bind(1000)
    .bind(10)
    .execute(&pool)
    .await
    .unwrap();

    let req = McpInvokeRequest {
        tool_id: "commerce_edge_quote".to_string(),
        action: "invoke".to_string(),
        params: r#"{"product_id":"prod-cache-1","quantity":2}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "spiffe://onehumancorp.io/org-1/agent-1".to_string(),
    };

    // First call: Cache miss, generates quote
    let resp1 = server.invoke_tool(&req).await.unwrap();
    let payload1: serde_json::Value = serde_json::from_str(&resp1.payload).unwrap();
    assert_eq!(payload1["status"], "success");
    let quote_id1 = payload1["quote_id"].as_str().unwrap().to_string();
    assert!(quote_id1.starts_with("quote-"));
    assert_eq!(payload1["amount"], 2000);
    assert!(payload1["checkout_url"].as_str().unwrap().starts_with("https://checkout.stripe.com/pay/cs_test_"));

    // Second call: Cache hit, should return identical payload
    let resp2 = server.invoke_tool(&req).await.unwrap();
    let payload2: serde_json::Value = serde_json::from_str(&resp2.payload).unwrap();
    assert_eq!(payload2["status"], "success");
    assert_eq!(payload2["quote_id"].as_str().unwrap(), quote_id1); // Same ID proves it came from cache
    assert_eq!(payload2["amount"], 2000);
}

#[tokio::test]
async fn test_commerce_edge_quote_different_tenant() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        println!("Database not available, skipping test_commerce_edge_quote_different_tenant");
        return;
    }
    let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
    let pool = match sqlx::postgres::PgPoolOptions::new().connect(&db_url).await {
        Ok(p) => p,
        Err(_) => {
            println!("Could not connect to database, skipping test_commerce_edge_quote_different_tenant");
            return;
        }
    };

    let redis_client = match redis::Client::open("redis://localhost:6379/") {
        Ok(c) => c,
        Err(_) => return,
    };
    let conn_res = redis_client.get_multiplexed_async_connection().await;
    if conn_res.is_err() {
        println!("Redis not available, skipping test_commerce_edge_quote_different_tenant");
        return;
    }

    // ensure product exists
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS products (
            id TEXT,
            tenant_id TEXT,
            price_cents BIGINT,
            inventory_count BIGINT,
            PRIMARY KEY (id, tenant_id)
        )"
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO products (id, tenant_id, price_cents, inventory_count) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"
    )
    .bind("prod-2")
    .bind("org-2")
    .bind(1000)
    .bind(10)
    .execute(&pool)
    .await
    .unwrap();

    let server = EdgeCommerceMcpServer::new(redis_client, pool);
    let req = McpInvokeRequest {
        tool_id: "commerce_edge_quote".to_string(),
        action: "invoke".to_string(),
        params: r#"{"product_id":"prod-2","quantity":3}"#.to_string(),
        agent_id: "agent-2".to_string(),
        spiffe_id: "spiffe://onehumancorp.io/org-2/agent-2".to_string(),
    };

    let resp = server.invoke_tool(&req).await.unwrap();
    let payload: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(payload["status"], "success");
    assert!(payload["quote_id"].as_str().unwrap().starts_with("quote-"));
    assert_eq!(payload["amount"], 3000);
    assert!(payload["checkout_url"].as_str().unwrap().starts_with("https://checkout.stripe.com/pay/cs_test_"));
}

#[tokio::test]
async fn test_commerce_edge_quote_invalid_spiffe_id() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        println!("Database not available, skipping test_commerce_edge_quote_invalid_spiffe_id");
        return;
    }
    let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
    let pool = match sqlx::postgres::PgPoolOptions::new().connect(&db_url).await {
        Ok(p) => p,
        Err(_) => return,
    };

    let redis_client = match redis::Client::open("redis://localhost:6379/") {
        Ok(c) => c,
        Err(_) => return,
    };
    let server = EdgeCommerceMcpServer::new(redis_client, pool);
    let req = McpInvokeRequest {
        tool_id: "commerce_edge_quote".to_string(),
        action: "invoke".to_string(),
        params: r#"{"product_id":"prod-3","quantity":1}"#.to_string(),
        agent_id: "agent-3".to_string(),
        spiffe_id: "invalid-spiffe-id".to_string(),
    };

    let resp = server.invoke_tool(&req).await;
    assert!(resp.is_err());
    assert_eq!(resp.unwrap_err().code(), tonic::Code::Unauthenticated);
}
