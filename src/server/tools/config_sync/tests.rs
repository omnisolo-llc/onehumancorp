
use super::server::ConfigSyncServer;
use crate::ohc::orchestration::McpInvokeRequest;
use serde_json::json;

#[tokio::test]
async fn test_config_sync_unauthenticated() {
    if std::env::var("OHC_DATABASE_URL").is_err() { return; }
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
    if std::env::var("OHC_DATABASE_URL").is_err() { return; }
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
            if std::env::var("OHC_DATABASE_URL").is_err() { return; }
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
    if std::env::var("OHC_DATABASE_URL").is_err() { return; }
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

    // Push Config without allow_sensitive_upload, testing recursive scrub
    let push_req = McpInvokeRequest {
        tool_id: "mcp_config_sync".to_string(),
        action: "".to_string(),
        agent_id: "".to_string(),
        spiffe_id: spiffe_id.to_string(),
        params: json!({
            "action": "push_config",
            "payload": {
                "some_setting": "enabled",
                "nested_obj": {
                    "local_proxy_password": "my_secret_password",
                    "safe_value": "hello"
                }
            }
        }).to_string(),
    };

    let push_res = server.invoke_tool(&push_req).await.unwrap();
    let push_json: serde_json::Value = serde_json::from_str(&push_res.payload).unwrap();
    assert_eq!(push_json["status"], "success");
    assert_eq!(push_json["merged"], true);

    // Get Config
    let get_req = McpInvokeRequest {
        tool_id: "mcp_config_sync".to_string(),
        action: "".to_string(),
        agent_id: "".to_string(),
        spiffe_id: spiffe_id.to_string(),
        params: json!({
            "action": "get_config"
        }).to_string(),
    };

    let get_res = server.invoke_tool(&get_req).await.unwrap();
    let get_json: serde_json::Value = serde_json::from_str(&get_res.payload).unwrap();
    assert_eq!(get_json["status"], "success");
    assert_eq!(get_json["config"]["some_setting"].as_str().unwrap(), "enabled");
    assert!(get_json["config"]["nested_obj"].get("local_proxy_password").is_none(), "Sensitive password should have been stripped recursively");
    assert_eq!(get_json["config"]["nested_obj"]["safe_value"].as_str().unwrap(), "hello");
    let server_updated_at = get_json["updated_at"].as_i64().unwrap();

    // Push Config with allow_sensitive_upload
    let push_req_sensitive = McpInvokeRequest {
        tool_id: "mcp_config_sync".to_string(),
        action: "".to_string(),
        agent_id: "".to_string(),
        spiffe_id: spiffe_id.to_string(),
        params: json!({
            "action": "push_config",
            "allow_sensitive_upload": true,
            "payload": {
                "some_setting": "enabled",
                "nested_obj": {
                    "local_proxy_password": "my_secret_password"
                }
            }
        }).to_string(),
    };

    let push_res_sensitive = server.invoke_tool(&push_req_sensitive).await.unwrap();
    let push_json_sensitive: serde_json::Value = serde_json::from_str(&push_res_sensitive.payload).unwrap();
    assert_eq!(push_json_sensitive["status"], "success");
    assert_eq!(push_json_sensitive["merged"], true);

    // Get Config again, sensitive should be available and decrypted recursively
    let get_res_sensitive = server.invoke_tool(&get_req).await.unwrap();
    let get_json_sensitive: serde_json::Value = serde_json::from_str(&get_res_sensitive.payload).unwrap();
    assert_eq!(get_json_sensitive["status"], "success");
    assert_eq!(get_json_sensitive["config"]["nested_obj"]["local_proxy_password"].as_str().unwrap(), "my_secret_password");

    // Get Hash
    let get_hash_req = McpInvokeRequest {
        tool_id: "mcp_config_sync".to_string(),
        action: "".to_string(),
        agent_id: "".to_string(),
        spiffe_id: spiffe_id.to_string(),
        params: json!({
            "action": "get_hash"
        }).to_string(),
    };

    let get_hash_res = server.invoke_tool(&get_hash_req).await.unwrap();
    let get_hash_json: serde_json::Value = serde_json::from_str(&get_hash_res.payload).unwrap();
    assert_eq!(get_hash_json["status"], "success");
    assert!(get_hash_json.get("hash").is_some());
    assert!(get_hash_json["hash"].as_str().unwrap().len() > 0);

    // Test missing configuration
    let get_missing_req = McpInvokeRequest {
        tool_id: "mcp_config_sync".to_string(),
        action: "".to_string(),
        agent_id: "".to_string(),
        spiffe_id: "spiffe://missing_user".to_string(),
        params: json!({
            "action": "get_config"
        }).to_string(),
    };
    let get_missing_res = server.invoke_tool(&get_missing_req).await.unwrap();
    let get_missing_json: serde_json::Value = serde_json::from_str(&get_missing_res.payload).unwrap();
    assert_eq!(get_missing_json["status"], "not_found");

    // Test timestamp resolution conflict
    let conflict_push_req = McpInvokeRequest {
        tool_id: "mcp_config_sync".to_string(),
        action: "".to_string(),
        agent_id: "".to_string(),
        spiffe_id: spiffe_id.to_string(),
        params: json!({
            "action": "push_config",
            "client_updated_at": server_updated_at - 1000,
            "payload": {
                "some_setting": "conflict"
            }
        }).to_string(),
    };
    let conflict_res = server.invoke_tool(&conflict_push_req).await.unwrap();
    let conflict_json: serde_json::Value = serde_json::from_str(&conflict_res.payload).unwrap();
    assert_eq!(conflict_json["status"], "conflict");
    assert_eq!(conflict_json["message"], "Server configuration is newer than client configuration");

    // Test recursive depth limit
    let mut deep_payload = json!({"level": 0});
    let mut current = &mut deep_payload;
    for i in 1..15 {
        current["child"] = json!({"level": i});
        current = &mut current["child"];
    }

    let depth_req = McpInvokeRequest {
        tool_id: "mcp_config_sync".to_string(),
        action: "".to_string(),
        agent_id: "".to_string(),
        spiffe_id: spiffe_id.to_string(),
        params: json!({
            "action": "push_config",
            "payload": deep_payload
        }).to_string(),
    };
    let depth_res = server.invoke_tool(&depth_req).await;
    assert!(depth_res.is_err());
    assert_eq!(depth_res.unwrap_err().message(), "JSON payload exceeds maximum depth of 10");
}
