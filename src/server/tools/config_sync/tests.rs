
use super::server::ConfigSyncServer;
use crate::ohc::orchestration::McpInvokeRequest;
use serde_json::json;

#[tokio::test]
async fn test_config_sync_unauthenticated() {
    if std::env::var("DATABASE_URL").is_err() { return; }
    let pool = crate::db::get_pool();
    let server = ConfigSyncServer::new(pool);

    let req = McpInvokeRequest {
        tool_id: "mcp_config_sync".to_string(),
        action: "".to_string(),
        agent_id: "".to_string(),
        spiffe_id: "".to_string(), // Empty means unauthenticated
        params: json!({"action": "get_hash"}).to_string(),
    };

    let res = server.invoke_tool(&req).await;
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().code(), tonic::Code::Unauthenticated);
}

#[tokio::test]
async fn test_config_sync_invalid_tool_id() {
    if std::env::var("DATABASE_URL").is_err() { return; }
    let pool = crate::db::get_pool();
    let server = ConfigSyncServer::new(pool);

    let req = McpInvokeRequest {
        tool_id: "wrong_tool_id".to_string(),
        action: "".to_string(),
        agent_id: "".to_string(),
        spiffe_id: "spiffe://test".to_string(),
        params: json!({"action": "get_hash"}).to_string(),
    };

    let res = server.invoke_tool(&req).await;
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().code(), tonic::Code::InvalidArgument);
}

#[test]
fn test_config_sync_push_too_large() {
    temp_env::with_vars(vec![("MAX_CONFIG_SIZE", Some("100"))], || {
        tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
            if std::env::var("DATABASE_URL").is_err() { return; }
            let pool = crate::db::get_pool();
            let server = ConfigSyncServer::new(pool);

            let large_payload = "x".repeat(200);

            let req = McpInvokeRequest {
                tool_id: "mcp_config_sync".to_string(),
                action: "".to_string(),
                agent_id: "".to_string(),
                spiffe_id: "spiffe://test".to_string(),
                params: json!({
                    "action": "push_config",
                    "payload": {
                        "large_data": large_payload
                    }
                }).to_string(),
            };

            let res = server.invoke_tool(&req).await;
            assert!(res.is_err());
            assert_eq!(res.unwrap_err().message(), "Config payload too large");
        });
    });
}

#[tokio::test]
async fn test_config_sync_push_and_get() {
    if std::env::var("DATABASE_URL").is_err() { return; }
    let pool = crate::db::get_pool();

    // Migrate db to have the user_configs table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_configs (
            spiffe_id VARCHAR PRIMARY KEY,
            config_json TEXT NOT NULL,
            updated_at TIMESTAMP NOT NULL,
            hash VARCHAR NOT NULL
        )"
    ).execute(&pool).await.unwrap();

    let server = ConfigSyncServer::new(pool.clone());
    let spiffe_id = "spiffe://local_test";

    // Push Config
    let push_req = McpInvokeRequest {
        tool_id: "mcp_config_sync".to_string(),
        action: "".to_string(),
        agent_id: "".to_string(),
        spiffe_id: spiffe_id.to_string(),
        params: json!({
            "action": "push_config",
            "payload": {
                "some_setting": "enabled",
                "local_proxy_password": "my_secret_password"
            }
        }).to_string(),
    };

    let push_res = server.invoke_tool(&push_req).await.unwrap();
    let push_json: serde_json::Value = serde_json::from_str(&push_res.payload).unwrap();
    assert_eq!(push_json["status"], "success");
    assert_eq!(push_json["merged"], true);

    // Get Hash
    let get_req = McpInvokeRequest {
        tool_id: "mcp_config_sync".to_string(),
        action: "".to_string(),
        agent_id: "".to_string(),
        spiffe_id: spiffe_id.to_string(),
        params: json!({
            "action": "get_hash"
        }).to_string(),
    };

    let get_res = server.invoke_tool(&get_req).await.unwrap();
    let get_json: serde_json::Value = serde_json::from_str(&get_res.payload).unwrap();
    assert_eq!(get_json["status"], "success");
    assert!(get_json.get("hash").is_some());
    assert!(get_json["hash"].as_str().unwrap().len() > 0);
}
