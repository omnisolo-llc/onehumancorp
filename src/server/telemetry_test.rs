#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::sync::LazyLock;
    pub static ENV_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
    use serde_json;
    use temp_env;
    use tokio;


    #[test]
    fn test_analytics_pii_redaction() {
        let mut props = std::collections::HashMap::new();
        props.insert("username".to_string(), "maya".to_string());
        props.insert("password".to_string(), "secret-123".to_string());
        props.insert("contact".to_string(), "maya@example.com".to_string());
        props.insert("safe_field".to_string(), "safe_value".to_string());
        props.insert("ip_address".to_string(), "10.0.0.1".to_string());
        props.insert("mac_address".to_string(), "FF:FF:FF:FF:FF:FF".to_string());
        props.insert("geolocation".to_string(), "0,0".to_string());
        props.insert("auth_jwt".to_string(), "eyJhbGci...".to_string());
        props.insert("authorization_bearer".to_string(), "Bearer xyz123".to_string());

        let mut sanitized_props = props;
        for (k, v) in sanitized_props.iter_mut() {
            if ::server_telemetry::is_sensitive_key(k) {
                *v = "[REDACTED]".to_string();
            } else if ::server_telemetry::is_email(v) {
                *v = "[EMAIL_REDACTED]".to_string();
            }
        }

        assert_eq!(sanitized_props.get("username").unwrap(), "[REDACTED]"); // Because username contains "name"
        assert_eq!(sanitized_props.get("password").unwrap(), "[REDACTED]"); // Because it contains "password"
        assert_eq!(sanitized_props.get("contact").unwrap(), "[EMAIL_REDACTED]");
        assert_eq!(sanitized_props.get("safe_field").unwrap(), "safe_value");
        assert_eq!(sanitized_props.get("ip_address").unwrap(), "[REDACTED]");
        assert_eq!(sanitized_props.get("mac_address").unwrap(), "[REDACTED]");
        assert_eq!(sanitized_props.get("geolocation").unwrap(), "[REDACTED]");
        assert_eq!(sanitized_props.get("auth_jwt").unwrap(), "[REDACTED]");
        assert_eq!(sanitized_props.get("authorization_bearer").unwrap(), "[REDACTED]");
    }


    use serde_json::json;
    use ::server_telemetry::{redact_interface_pii, buffer_metric};

    #[test]
    fn test_organization_id_not_redacted() {
        let input = json!({
            "organization_id": "tenant-123",
            "safe_field": "ok"
        });
        let expected = json!({
            "organization_id": "tenant-123",
            "safe_field": "ok"
        });
        assert_eq!(redact_interface_pii(input), expected);
    }

    #[test]
    fn test_redact_pii_password() {
        let input = json!({
            "username": "maya",
            "password": "secret-password-123",
            "nested": {
                "admin_key": "some-key"
            }
        });
        let expected = json!({
            "username": "[REDACTED]",
            "password": "[REDACTED]",
            "nested": {
                "admin_key": "[REDACTED]"
            }
        });
        assert_eq!(redact_interface_pii(input), expected);
    }

    #[test]
    fn test_redact_pii_email() {
        let input = json!({
            "contact": "maya@example.com",
            "other": "not-an-email"
        });
        let expected = json!({
            "contact": "[EMAIL_REDACTED]",
            "other": "not-an-email"
        });
        assert_eq!(redact_interface_pii(input), expected);
    }

    #[test]
    fn test_redact_pii_array() {
        let input = json!([
            {"token": "token1"},
            {"user": "maya"}
        ]);
        let expected = json!([
            {"token": "[REDACTED]"},
            {"user": "maya"}
        ]);
        assert_eq!(redact_interface_pii(input), expected);
    }

    #[tokio::test]
    async fn test_buffer_metric_persistence() {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let labels = json!({"user_id": "123", "secret": "shh"});
        let res: Result<(), _> = buffer_metric(&pool, "test_metric", "counter", 1.0, labels).await;
        assert!(res.is_ok());

        let row: sqlx::postgres::PgRow = sqlx::query("SELECT labels_json FROM telemetry_buffer WHERE metric_name = 'test_metric' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let _ = pool;
        let labels_json: String = row.get("labels_json");
        let redacted: serde_json::Value = serde_json::from_str(&labels_json).unwrap();

        assert_eq!(redacted["user_id"], "123");
        assert_eq!(redacted["secret"], "[REDACTED]");
    }

    #[tokio::test]
    async fn test_sqlite_metrics() {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let res: Result<(), _> = ::server_telemetry::record_sqlite_lock_contention(&pool, "test_operation").await;
        assert!(res.is_ok());

        let res: Result<(), _> = ::server_telemetry::record_sqlite_retry_exhausted(&pool, "test_operation").await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_record_token_burn_rate_predicted_24h() {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let res: Result<(), _> = ::server_telemetry::record_token_burn_rate_predicted_24h(&pool, "org_test", 15000.0).await;
        assert!(res.is_ok());

        let row: sqlx::postgres::PgRow = sqlx::query("SELECT labels_json, value FROM telemetry_buffer WHERE metric_name = 'ohc_token_burn_rate_predicted_24h' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let _ = pool;
        let value: f64 = row.get("value");
        assert_eq!(value, 15000.0);

        let labels_json: String = row.get("labels_json");
        let parsed: serde_json::Value = serde_json::from_str(&labels_json).unwrap();
        assert_eq!(parsed["organization_id"], "[REDACTED]");
    }

    #[tokio::test]
    async fn test_record_llm_call_cost() {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return,
        };

        let res: Result<(), _> = ::server_telemetry::record_llm_call_cost(&pool, "org-1", "test-model", 2.5).await;
        assert!(res.is_ok());

        let row: sqlx::postgres::PgRow = sqlx::query("SELECT labels_json, value FROM telemetry_buffer WHERE metric_name = 'ohc_llm_call_cost' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let value: f64 = row.get("value");
        assert_eq!(value, 2.5);

        let row_cents: sqlx::postgres::PgRow = sqlx::query("SELECT value FROM telemetry_buffer WHERE metric_name = 'ohc_llm_cost_total_cents' AND labels_json::jsonb->>'organization_id' = 'org-1' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();
        let value_cents: f64 = row_cents.get("value");
        assert_eq!(value_cents, 250.0);
    }

    #[tokio::test]
    async fn test_record_agent_cost() {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let res: Result<(), _> = ::server_telemetry::record_agent_cost(&pool, "agent-123", "org-1", "test-role", "test-model", "test-entity", 1.5).await;
        assert!(res.is_ok());

        let row: sqlx::postgres::PgRow = sqlx::query("SELECT labels_json, value FROM telemetry_buffer WHERE metric_name = 'ohc_agent_cost' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let _ = pool;
        let value: f64 = row.get("value");
        assert_eq!(value, 1.5);

        let labels_json: String = row.get("labels_json");
        let parsed: serde_json::Value = serde_json::from_str(&labels_json).unwrap();
        assert_eq!(parsed["agent_id"], "agent-123");
        assert_eq!(parsed["organization_id"], "[REDACTED]");
        assert_eq!(parsed["entity"], "test-entity");

        let row_cents: sqlx::postgres::PgRow = sqlx::query("SELECT value FROM telemetry_buffer WHERE metric_name = 'ohc_llm_cost_total_cents' AND labels_json::jsonb->>'agent_id' = 'agent-123' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        let value_cents: f64 = row_cents.get("value");
        assert_eq!(value_cents, 150.0);
    }

    #[tokio::test]
    async fn test_record_api_call_cost() {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let res: Result<(), _> = ::server_telemetry::record_api_call_cost(&pool, "org-2", "test-entity-2", 0.5).await;
        assert!(res.is_ok());

        let row: sqlx::postgres::PgRow = sqlx::query("SELECT labels_json, value FROM telemetry_buffer WHERE metric_name = 'ohc_api_call_cost' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let _ = pool;
        let value: f64 = row.get("value");
        assert_eq!(value, 0.5);

        let labels_json: String = row.get("labels_json");
        let parsed: serde_json::Value = serde_json::from_str(&labels_json).unwrap();
        assert_eq!(parsed["organization_id"], "[REDACTED]");
        assert_eq!(parsed["entity"], "test-entity-2");
    }

    #[tokio::test]
    async fn test_record_swarm_job_latency_by_entity() {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let res: Result<(), _> = ::server_telemetry::record_swarm_job_latency_by_entity(&pool, "cloud", "test-entity-3", 125.0).await;
        assert!(res.is_ok());

        let row: sqlx::postgres::PgRow = sqlx::query("SELECT labels_json, value FROM telemetry_buffer WHERE metric_name = 'ohc_swarm_job_latency_by_entity_seconds' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let _ = pool;
        let value: f64 = row.get("value");
        assert_eq!(value, 125.0);

        let labels_json: String = row.get("labels_json");
        let parsed: serde_json::Value = serde_json::from_str(&labels_json).unwrap();
        assert_eq!(parsed["mode"], "cloud");
        assert_eq!(parsed["entity"], "test-entity-3");
    }

    #[test]
    fn test_buffer_metric_respects_standalone() {
        let _lock = crate::tests::ENV_MUTEX.lock().unwrap();
        temp_env::with_vars(vec![("OHC_STANDALONE_MODE", Some("true"))], || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        // Ensure OHC_STANDALONE_MODE is true. Telemetry should be ignored

        let labels = json!({"user_id": "standalone_test"});
        let res: Result<(), _> = buffer_metric(&pool, "test_standalone", "counter", 1.0, labels).await;
        assert!(res.is_ok());

        let row: sqlx::postgres::PgRow = sqlx::query("SELECT COUNT(*) FROM telemetry_buffer WHERE metric_name = 'test_standalone'")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let count: i64 = row.get(0);
        assert_eq!(count, 0, "Metric should not be buffered in standalone mode");
            });
        });
    }

    #[test]
    fn test_buffer_metric_i64_respects_standalone() {
        let _lock = crate::tests::ENV_MUTEX.lock().unwrap();
        temp_env::with_vars(vec![("OHC_STANDALONE_MODE", Some("true"))], || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        // Ensure OHC_STANDALONE_MODE is true. Telemetry should be ignored
        let labels = json!({"user_id": "standalone_test_i64"});
        let res: Result<(), _> = ::server_telemetry::buffer_metric_i64(&pool, "test_standalone_i64", "counter", 1, labels).await;
        assert!(res.is_ok());

        let row: sqlx::postgres::PgRow = sqlx::query("SELECT COUNT(*) FROM telemetry_buffer WHERE metric_name = 'test_standalone_i64'")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let count: i64 = row.get(0);
        assert_eq!(count, 0, "Metric i64 should not be buffered in standalone mode");
            });
        });
    }

    #[test]
    fn test_record_rag_escalation_telemetry_respects_standalone() {
        let _lock = crate::tests::ENV_MUTEX.lock().unwrap();
        temp_env::with_vars(vec![("OHC_STANDALONE_MODE", Some("true"))], || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let res: Result<(), _> = ::server_telemetry::record_rag_escalation(&pool, "org_123", "TimeoutError").await;
        assert!(res.is_ok());

        let row: sqlx::postgres::PgRow = sqlx::query("SELECT COUNT(*) FROM telemetry_buffer WHERE metric_name = 'ohc_rag_escalation_total'")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let count: i64 = row.get(0);
        assert_eq!(count, 0, "RAG escalation should not be buffered in standalone mode");
            });
        });
    }

    #[test]
    fn test_no_pii_logging_statements() {
        let src_dir = std::path::Path::new("src/server");
        if !src_dir.exists() {
            println!("src/server not found, skipping or finding correct path.");
        }

        let mut failed_files = Vec::new();
        let pii_keywords = vec!["password", "email", "credit_card", "api_key", "token", "ssn", "dob", "cvv"];

        fn find_rs_files(dir: &std::path::Path, rs_files: &mut Vec<std::path::PathBuf>) {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        find_rs_files(&path, rs_files);
                    } else if path.extension().map_or(false, |e| e == "rs") {
                        rs_files.push(path);
                    }
                }
            }
        }

        let mut rs_files = Vec::new();
        let workspace_dir = std::env::var("BUILD_WORKSPACE_DIRECTORY").unwrap_or_else(|_| ".".to_string());
        let search_dir = std::path::Path::new(&workspace_dir).join("src/server");

        find_rs_files(&search_dir, &mut rs_files);

        if rs_files.is_empty() {
             find_rs_files(std::path::Path::new("src/server"), &mut rs_files);
        }

        for file in rs_files {
            let content = std::fs::read_to_string(&file).unwrap();
            for (line_num, line) in content.lines().enumerate() {
                if (line.contains("tracing::") || line.contains("log::")) && !line.contains("pii-safe") {
                    let lower_line = line.to_lowercase();
                    for keyword in &pii_keywords {
                        if lower_line.contains(keyword) {
                            failed_files.push(format!("{}:{}: {}", file.display(), line_num + 1, line.trim()));
                        }
                    }
                }
            }
        }

        if !failed_files.is_empty() {
            panic!("Found potential PII leakage in logging statements:\n{}", failed_files.join("\n"));
        }
    }

    #[test]
    fn test_init_telemetry_standalone_opt_out() {
        let _lock = crate::tests::ENV_MUTEX.lock().unwrap();
        temp_env::with_vars(
            [
                ("OHC_STANDALONE_MODE", Some("true")),
                ("OHC_TELEMETRY_ENABLED", Some("false")),
                ("OHC_DATABASE_URL", Some("sqlite://ohc-standalone.db")),
                ("OHC_SQLITE_KEY", Some("test-key")),
            ],
            || {
                let config = ::server_config::load().unwrap();

                // Assert that the config logic matches the policy:
                // If OHC_STANDALONE_MODE=true and OHC_TELEMETRY_ENABLED=false, telemetry should NOT run.
                let should_start_telemetry = config.telemetry_enabled;

                assert!(!(should_start_telemetry));
            },
        );
    }

    #[test]
    fn test_init_telemetry_standalone_opt_in() {
        let _lock = crate::tests::ENV_MUTEX.lock().unwrap();
        temp_env::with_vars(
            [
                ("OHC_STANDALONE_MODE", Some("true")),
                ("OHC_TELEMETRY_ENABLED", Some("true")),
                ("OHC_DATABASE_URL", Some("sqlite://ohc-standalone.db")),
                ("OHC_SQLITE_KEY", Some("test-key")),
            ],
            || {
                let config = ::server_config::load().unwrap();

                // If OHC_STANDALONE_MODE=true and OHC_TELEMETRY_ENABLED=true, telemetry SHOULD run.
                let should_start_telemetry = config.telemetry_enabled;

                assert!(should_start_telemetry);
            },
        );
    }
}

#[tokio::test]
async fn test_queue_length_gauge_initialization() {
    let gauge = ::server_telemetry::get_queue_length_gauge();
    gauge.add(1, &[]);
}

#[tokio::test]
async fn test_record_queue_length_with_deployment_mode() {
    let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
    let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
        Ok(Ok(p)) => p,
        _ => return, // Gracefully exit if DB is not available in sandbox or times out
    };

    let res: Result<(), _> = ::server_telemetry::record_queue_length(&pool, 5).await;
    assert!(res.is_ok());

    let row: sqlx::postgres::PgRow = sqlx::query("SELECT labels_json, value FROM telemetry_buffer WHERE metric_name = 'ohc_sub_agent_queue_length' ORDER BY timestamp DESC LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    use sqlx::Row;
        let _ = pool;
    let labels_json: String = row.get("labels_json");
    let parsed: serde_json::Value = serde_json::from_str(&labels_json).unwrap();
    assert!(parsed.get("deployment_mode").is_some());
}
#[test]
fn test_standalone_wrapper_audit() {
    let mut script_path = std::path::PathBuf::from("deploy/scripts/ohc-standalone.sh");
    if let Ok(workspace_dir) = std::env::var("BUILD_WORKSPACE_DIRECTORY") {
        script_path = std::path::PathBuf::from(workspace_dir).join("deploy/scripts/ohc-standalone.sh");
    } else if let Ok(runfiles_dir) = std::env::var("RUNFILES_DIR") {
        script_path = std::path::PathBuf::from(runfiles_dir).join("ohc/deploy/scripts/ohc-standalone.sh");
    }
    if !script_path.exists() {
        script_path = std::path::PathBuf::from("deploy/scripts/ohc-standalone.sh");
    }
    let content = std::fs::read_to_string(script_path).expect("Failed to read ohc-standalone.sh script");

    let expected_telemetry_check = r#"if [ "$OHC_TELEMETRY_ENABLED" != "true" ]; then
  export OHC_TELEMETRY_ENABLED=false
  export DISABLE_TELEMETRY=true
else
  export OHC_TELEMETRY_ENABLED=true
  unset DISABLE_TELEMETRY
fi"#;

    assert!(
        content.contains(expected_telemetry_check),
        "Local Sovereignty violation: ohc-standalone.sh does not properly strictly enforce OHC_TELEMETRY_ENABLED opt-in boundary."
    );
}

#[test]
fn test_redact_interface_pii_malicious_payloads() {
    let payload = serde_json::json!({
        "payload": {
            "credit_card": "4111-1111-1111-1111",
            "cvv": "123",
            "dob": "1990-01-01",
            "passport_number": "A1234567",
            "bank_account": "123456789",
            "stripe_token": "tok_123456789",
            "billing_address": "123 Main St, Anytown USA",
            "ssn": "123-45-6789",
            "phone_number": "555-123-4567",
            "email_address": "malicious@example.com",
            "tenant_id": "tenant-123",
            "organization_id": "org-456",
            "session_id": "session-789",
            "ip_address": "192.168.1.1",
            "mac_address": "00:1B:44:11:3A:B7",
            "geolocation": "37.7749,-122.4194",
        },
        "nested": {
            "deep": {
                "secret_key": "sk-1234567890",
                "api_key": "ak-0987654321",
                "auth_token": "Bearer token",
                "password_hash": "hash",
                "cookie_session": "cookie",
                "credential_id": "cred-1",
            }
        },
        "array_of_evil": [
            { "name": "John Doe", "email": "john@doe.com" },
            { "address": "456 Elm St", "phone": "555-987-6543" }
        ],
        "safe_field": "This should not be redacted",
        "another_safe": 123
    });

    let redacted = ::server_telemetry::redact_interface_pii(payload);

    // Verify root level safe fields
    assert_eq!(redacted["safe_field"], "This should not be redacted");
    assert_eq!(redacted["another_safe"], 123);

    // Because the key is "payload", the entire object gets redacted to "[REDACTED]"
    assert_eq!(redacted["payload"], "[REDACTED]");
    // Added explicitly nested checks are hidden by payload redaction, but if we moved them, they would be redacted.

    // Verify deeply nested secret redactions
    assert_eq!(redacted["nested"]["deep"]["secret_key"], "[REDACTED]");
    assert_eq!(redacted["nested"]["deep"]["api_key"], "[REDACTED]");
    assert_eq!(redacted["nested"]["deep"]["auth_token"], "[REDACTED]");
    assert_eq!(redacted["nested"]["deep"]["password_hash"], "[REDACTED]");
    assert_eq!(redacted["nested"]["deep"]["cookie_session"], "[REDACTED]");
    assert_eq!(redacted["nested"]["deep"]["credential_id"], "[REDACTED]");

    // Verify array redactions
    assert_eq!(redacted["array_of_evil"][0]["name"], "[REDACTED]");
    assert_eq!(redacted["array_of_evil"][0]["email"], "[REDACTED]");
    assert_eq!(redacted["array_of_evil"][1]["address"], "[REDACTED]");
    assert_eq!(redacted["array_of_evil"][1]["phone"], "[REDACTED]");
}

#[test]
    fn test_multi_tenant_pii_leakage_guardrail() {
        let payload = serde_json::json!({
            "tenant_id": "tenant-xyz",
            "user_email": "user@example.com",
            "api_key": "secret_123",
            "data": {
                "credit_card": "1234-5678",
                "safe_metric": 42
            }
        });

        let redacted = ::server_telemetry::redact_interface_pii(payload);

        assert_eq!(redacted["tenant_id"], "tenant-xyz", "tenant_id should be kept for multi-tenant analytics routing");
        assert_eq!(redacted["user_email"], "[REDACTED]", "user_email must be redacted");
        assert_eq!(redacted["api_key"], "[REDACTED]", "api_key must be redacted");
        assert_eq!(redacted["data"]["credit_card"], "[REDACTED]", "nested PII must be redacted");
        assert_eq!(redacted["data"]["safe_metric"], 42, "safe metrics should remain intact");
}

#[test]
fn test_redact_interface_pii_edge_cases() {
    let payload_mixed_array = serde_json::json!({
        "mixed_array": [
            "safe_string",
            123,
            { "email": "should_be_redacted@test.com", "safe_field": "ok" },
            ["another_safe", { "password": "super_secret" }],
            [
                { "nested_array_obj_ip_address": "127.0.0.1" },
                { "mac_address": "00:00:00:00:00:00" },
                "test@test.com" // string pattern match for email
            ]
        ],
        "non_sensitive_parent": {
            "userEmail": "camelCase@test.com",
            "CREDIT_CARD": "1234",
            "secret_token_123": "token",
            "pAsSwOrD": "camelCasePassword",
            "S S N": "123-45-6789",
            "phONe": "555-1234",
            "medical_history": "hypertension",
            "social_security_num": "987-65-4321",
            "tax_ID_number": "123456789"
        }
    });

    let redacted_mixed = ::server_telemetry::redact_interface_pii(payload_mixed_array);
    assert_eq!(redacted_mixed["mixed_array"][0], "safe_string");
    assert_eq!(redacted_mixed["mixed_array"][1], 123);
    assert_eq!(redacted_mixed["mixed_array"][2]["email"], "[REDACTED]");
    assert_eq!(redacted_mixed["mixed_array"][2]["safe_field"], "ok");
    assert_eq!(redacted_mixed["mixed_array"][3][0], "another_safe");
    assert_eq!(redacted_mixed["mixed_array"][3][1]["password"], "[REDACTED]");
    assert_eq!(redacted_mixed["mixed_array"][4][0]["nested_array_obj_ip_address"], "[REDACTED]");
    assert_eq!(redacted_mixed["mixed_array"][4][1]["mac_address"], "[REDACTED]");
    assert_eq!(redacted_mixed["mixed_array"][4][2], "[EMAIL_REDACTED]");

    assert_eq!(redacted_mixed["non_sensitive_parent"]["userEmail"], "[REDACTED]");
    assert_eq!(redacted_mixed["non_sensitive_parent"]["CREDIT_CARD"], "[REDACTED]");
    assert_eq!(redacted_mixed["non_sensitive_parent"]["secret_token_123"], "[REDACTED]");
    assert_eq!(redacted_mixed["non_sensitive_parent"]["pAsSwOrD"], "[REDACTED]");
    assert_eq!(redacted_mixed["non_sensitive_parent"]["S S N"], "[REDACTED]");
    assert_eq!(redacted_mixed["non_sensitive_parent"]["phONe"], "[REDACTED]");
    assert_eq!(redacted_mixed["non_sensitive_parent"]["medical_history"], "[REDACTED]");
    assert_eq!(redacted_mixed["non_sensitive_parent"]["social_security_num"], "[REDACTED]");
    assert_eq!(redacted_mixed["non_sensitive_parent"]["tax_ID_number"], "[REDACTED]");
}

#[test]
fn test_redact_interface_pii_highly_nested() {
    let payload = serde_json::json!({
        "level1": {
            "level2": {
                "level3": {
                    "level4": {
                        "level5": {
                            "level6": {
                                "secret_token": "token123",
                                "safe_value": 42,
                                "level7": {
                                    "user_dob": "01-01-2000",
                                    "level8": {
                                        "level9": [
                                            { "health_condition": "stable" },
                                            { "safe_array_val": true },
                                            "123-45-6789", // pattern match test inside array
                                            { "nested_again": { "passport": "AB12345" } }
                                        ]
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    let redacted = ::server_telemetry::redact_interface_pii(payload);
    assert_eq!(redacted["level1"]["level2"]["level3"]["level4"]["level5"]["level6"]["secret_token"], "[REDACTED]");
    assert_eq!(redacted["level1"]["level2"]["level3"]["level4"]["level5"]["level6"]["safe_value"], 42);
    assert_eq!(redacted["level1"]["level2"]["level3"]["level4"]["level5"]["level6"]["level7"]["user_dob"], "[REDACTED]");
    assert_eq!(redacted["level1"]["level2"]["level3"]["level4"]["level5"]["level6"]["level7"]["level8"]["level9"][0]["health_condition"], "[REDACTED]");
    assert_eq!(redacted["level1"]["level2"]["level3"]["level4"]["level5"]["level6"]["level7"]["level8"]["level9"][1]["safe_array_val"], true);
    assert_eq!(redacted["level1"]["level2"]["level3"]["level4"]["level5"]["level6"]["level7"]["level8"]["level9"][2], "[REDACTED]");
    assert_eq!(redacted["level1"]["level2"]["level3"]["level4"]["level5"]["level6"]["level7"]["level8"]["level9"][3]["nested_again"]["passport"], "[REDACTED]");
}

#[test]
fn test_harness_telemetry_recording() {
    // This test ensures the metric recording logic runs without panicking.
    // It calls the `record_harness_init_latency` and `record_harness_db_io_latency` functions.
    // In a real environment, opentelemetry global meter would capture these.

    ::server_telemetry::record_harness_init_latency(1.23);
    ::server_telemetry::record_harness_db_io_latency("fs_read", 0.45);
    ::server_telemetry::record_harness_db_io_latency("fs_write", 0.67);
}

#[test]
fn test_record_postgres_lock_contention() {
    // This test verifies that the metric recording logic for postgres lock contention runs without panicking.
    ::server_telemetry::record_postgres_lock_contention("upsert_mission");
}

#[test]
fn test_record_llm_network_latency() {
    // This test verifies that the metric recording logic for llm network latency runs without panicking.
    ::server_telemetry::record_llm_network_latency("gpt-4-turbo", 1.45);
}


#[test]
fn test_value_based_pii_redaction() {
    let payload = serde_json::json!({
        "safe_field_1": "123-45-6789", // SSN pattern
        "safe_field_2": "4111-1111-1111-1111", // CC pattern
        "safe_field_3": "sk-1234567890abcdefg", // API key pattern
        "safe_field_4": "+1 (555) 123-4567", // Phone pattern
        "safe_field_5": "just a normal string",
        "safe_field_7": "hello@world.com", // Email pattern
        "safe_field_8": "John Doe", // Name pattern
        "safe_field_9": 123,
        "safe_field_10": true,
        "safe_field_11": null,
        "nested": {
            "safe_field_6": "ak-abcdefghijklmnopqrstuvwxyz"
        }
    });

    let redacted = ::server_telemetry::redact_interface_pii(payload);

    assert_eq!(redacted["safe_field_1"], "[REDACTED]");
    assert_eq!(redacted["safe_field_2"], "[REDACTED]");
    assert_eq!(redacted["safe_field_3"], "[REDACTED]");
    assert_eq!(redacted["safe_field_4"], "[REDACTED]");
    assert_eq!(redacted["safe_field_5"], "just a normal string");
    assert_eq!(redacted["safe_field_7"], "[EMAIL_REDACTED]");
    assert_eq!(redacted["safe_field_8"], "John Doe");
    assert_eq!(redacted["safe_field_9"], 123);
    assert_eq!(redacted["safe_field_10"], true);
    assert!(redacted["safe_field_11"].is_null());
    assert_eq!(redacted["nested"]["safe_field_6"], "[REDACTED]");
}

#[test]
fn test_redact_interface_pii_with_empty_objects() {
    let payload = serde_json::json!({
        "empty_obj": {},
        "empty_arr": [],
        "nested": {
            "empty": {},
            "secret": "password"
        }
    });

    let redacted = ::server_telemetry::redact_interface_pii(payload);
    assert_eq!(redacted["empty_obj"], serde_json::json!({}));
    assert_eq!(redacted["empty_arr"], serde_json::json!([]));
    assert_eq!(redacted["nested"]["empty"], serde_json::json!({}));
    assert_eq!(redacted["nested"]["secret"], "[REDACTED]");
}
#[test]
fn test_categorize_error_signal() {
    assert_eq!(::server_telemetry::categorize_error_signal("this is a panic!"), "bug");
    assert_eq!(::server_telemetry::categorize_error_signal("segfault occurred"), "bug");
    assert_eq!(::server_telemetry::categorize_error_signal("fatal error"), "bug");

    assert_eq!(::server_telemetry::categorize_error_signal("missing feature flag"), "feature");

    assert_eq!(::server_telemetry::categorize_error_signal("this api is deprecated"), "refactor");

    assert_eq!(::server_telemetry::categorize_error_signal("memory leak detected"), "cleanup");
    assert_eq!(::server_telemetry::categorize_error_signal("needs cleanup"), "cleanup");

    assert_eq!(::server_telemetry::categorize_error_signal("update the readme"), "docs");

    assert_eq!(::server_telemetry::categorize_error_signal("cve-2023-1234"), "security");
    assert_eq!(::server_telemetry::categorize_error_signal("sql injection detected"), "security");
    assert_eq!(::server_telemetry::categorize_error_signal("permission denied"), "security");

    assert_eq!(::server_telemetry::categorize_error_signal("something else random"), "bug");
}

#[test]
fn test_record_error_signal() {
    // It's hard to test the opentelemetry meter without mocking the provider,
    // but we can at least test that `record_error_signal` executes without panicking
    // and categorize correctly behind the scenes.
    ::server_telemetry::record_error_signal("[bug] panic: test");
}

    #[test]
    fn test_telemetry_standalone_strict_override() {
        // Enforce Local Sovereignty
        // Ensures that OHC_STANDALONE_MODE properly overrides any implicit telemetry activation.
        // It must default to false unless OHC_TELEMETRY_ENABLED is explicitly "true".
        let _lock = crate::tests::ENV_MUTEX.lock().unwrap();
        temp_env::with_vars(
            [
                ("OHC_STANDALONE_MODE", Some("true")),
                ("OHC_TELEMETRY_ENABLED", None::<&str>), // No explicit opt-in
                ("OHC_DATABASE_URL", Some("sqlite://ohc-standalone.db")),
                ("OHC_SQLITE_KEY", Some("test-key")),
            ],
            || {
                let config = ::server_config::load().unwrap();
                assert!(!config.telemetry_enabled, "Local Sovereignty violation: Telemetry must default to false in standalone mode without explicit user opt-in.");
            },
        );
}
#[test]
fn test_categorize_stuck_error_signal() {
    assert_eq!(::server_telemetry::categorize_error_signal("stuck item found"), "cleanup");
    assert_eq!(::server_telemetry::categorize_error_signal("stagnant backlog item"), "cleanup");
}
#[test]
fn test_record_harness_init_latency_respects_standalone() {
    let _lock = crate::tests::ENV_MUTEX.lock().unwrap();
    temp_env::with_vars(vec![("OHC_STANDALONE_MODE", Some("true")), ("OHC_TELEMETRY_ENABLED", None::<&str>)], || {
        // Without opt-in, telemetry is disabled in standalone mode.
        // It shouldn't panic, and logic inside should early return.
        ::server_telemetry::record_harness_init_latency(1.23);

        let config = ::server_config::load().unwrap();
        assert!(!config.telemetry_enabled, "telemetry should be disabled in standalone mode");
    });
}

#[test]
fn test_telemetry_network_disk_usage() {
    let _lock = crate::tests::ENV_MUTEX.lock().unwrap();

    // With telemetry disabled, running sync_metrics should just return Ok(()) instead of trying to hit the dummy network
    temp_env::with_vars(
        [
            ("OHC_STANDALONE_MODE", Some("true")),
            ("OHC_TELEMETRY_ENABLED", Some("false")),
        ],
        || {
            std::thread::spawn(|| {
                let pool = tokio::runtime::Runtime::new().unwrap().block_on(sqlx::sqlite::SqlitePoolOptions::new()
                    .connect("sqlite::memory:")).unwrap();
                let pg_pool = tokio::runtime::Runtime::new().unwrap().block_on(async { sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy/dummy") }).unwrap();

                let config = ::server_config::load().unwrap();
                assert!(!config.telemetry_enabled, "telemetry should be disabled");

                let worker = ::server_telemetry::mcp_sync_worker::McpSyncWorker::new(pool.clone(), pg_pool.clone());
                let result = tokio::runtime::Runtime::new().unwrap().block_on(worker.sync_metrics());
                assert!(result.is_ok(), "sync_metrics should return early when telemetry is disabled");
            }).join().unwrap();
        }
    );
}

#[test]
fn test_telemetry_batch_pii_redaction() {
    let payload = serde_json::json!({
        "labels": {
            "user_email": "test@example.com",
            "api_key": "sk-1234567890abcdef",
            "credit_card": "4111-1111-1111-1111",
            "safe_metric": 42
        }
    });

    let redacted = ::server_telemetry::redact_interface_pii(payload);

    assert_eq!(redacted["labels"]["user_email"], "[REDACTED]", "user_email must be redacted");
    assert_eq!(redacted["labels"]["api_key"], "[REDACTED]", "api_key must be redacted");
    assert_eq!(redacted["labels"]["credit_card"], "[REDACTED]", "credit_card must be redacted");
    assert_eq!(redacted["labels"]["safe_metric"], 42, "safe metrics should remain intact");
}
