#[cfg(test)]
mod tests {

    #[test]
    fn test_analytics_pii_redaction() {
        let mut props = std::collections::HashMap::new();
        props.insert("username".to_string(), "maya".to_string());
        props.insert("password".to_string(), "secret-123".to_string());
        props.insert("contact".to_string(), "maya@example.com".to_string());
        props.insert("safe_field".to_string(), "safe_value".to_string());

        let mut sanitized_props = props;
        for (k, v) in sanitized_props.iter_mut() {
            if crate::telemetry::is_sensitive_key(k) {
                *v = "[REDACTED]".to_string();
            } else if crate::telemetry::is_email(v) {
                *v = "[EMAIL_REDACTED]".to_string();
            }
        }

        assert_eq!(sanitized_props.get("username").unwrap(), "[REDACTED]"); // Because username contains "name"
        assert_eq!(sanitized_props.get("password").unwrap(), "[REDACTED]"); // Because it contains "password"
        assert_eq!(sanitized_props.get("contact").unwrap(), "[EMAIL_REDACTED]");
        assert_eq!(sanitized_props.get("safe_field").unwrap(), "safe_value");
    }


    use serde_json::{json, Value};
    use crate::telemetry::{redact_interface_pii, buffer_metric};

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
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let labels = json!({"user_id": "123", "secret": "shh"});
        let res = buffer_metric(&pool, "test_metric", "counter", 1.0, labels).await;
        assert!(res.is_ok());

        let row = sqlx::query("SELECT labels_json FROM telemetry_buffer WHERE metric_name = 'test_metric' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let labels_json: String = row.get("labels_json");
        let redacted: Value = serde_json::from_str(&labels_json).unwrap();

        assert_eq!(redacted["user_id"], "123");
        assert_eq!(redacted["secret"], "[REDACTED]");
    }

    #[tokio::test]
    async fn test_sqlite_metrics() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let res = crate::telemetry::record_sqlite_lock_contention(&pool, "test_operation").await;
        assert!(res.is_ok());

        let res = crate::telemetry::record_sqlite_retry_exhausted(&pool, "test_operation").await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_record_token_usage_forecast() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let res = crate::telemetry::record_token_usage_forecast(&pool, "org_test", 15000.0).await;
        assert!(res.is_ok());

        let row = sqlx::query("SELECT labels_json, value FROM telemetry_buffer WHERE metric_name = 'ohc_token_usage_forecast' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let value: f32 = row.get("value");
        assert_eq!(value, 15000.0);

        let labels_json: String = row.get("labels_json");
        let parsed: Value = serde_json::from_str(&labels_json).unwrap();
        assert_eq!(parsed["organization_id"], "[REDACTED]");
    }

    #[tokio::test]
    async fn test_record_agent_cost() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let res = crate::telemetry::record_agent_cost(&pool, "agent-123", "org-1", "test-role", "test-model", "test-entity", 1.5).await;
        assert!(res.is_ok());

        let row = sqlx::query("SELECT labels_json, value FROM telemetry_buffer WHERE metric_name = 'ohc_agent_cost' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let value: f32 = row.get("value");
        assert_eq!(value, 1.5);

        let labels_json: String = row.get("labels_json");
        let parsed: Value = serde_json::from_str(&labels_json).unwrap();
        assert_eq!(parsed["agent_id"], "agent-123");
        assert_eq!(parsed["organization_id"], "[REDACTED]");
        assert_eq!(parsed["entity"], "test-entity");
    }

    #[tokio::test]
    async fn test_record_api_call_cost() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let res = crate::telemetry::record_api_call_cost(&pool, "org-2", "test-entity-2", 0.5).await;
        assert!(res.is_ok());

        let row = sqlx::query("SELECT labels_json, value FROM telemetry_buffer WHERE metric_name = 'ohc_api_call_cost' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let value: f32 = row.get("value");
        assert_eq!(value, 0.5);

        let labels_json: String = row.get("labels_json");
        let parsed: Value = serde_json::from_str(&labels_json).unwrap();
        assert_eq!(parsed["organization_id"], "[REDACTED]");
        assert_eq!(parsed["entity"], "test-entity-2");
    }

    #[tokio::test]
    async fn test_record_swarm_job_latency_by_entity() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let res = crate::telemetry::record_swarm_job_latency_by_entity(&pool, "cloud", "test-entity-3", 125.0).await;
        assert!(res.is_ok());

        let row = sqlx::query("SELECT labels_json, value FROM telemetry_buffer WHERE metric_name = 'ohc_swarm_job_latency_by_entity_seconds' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let value: f32 = row.get("value");
        assert_eq!(value, 125.0);

        let labels_json: String = row.get("labels_json");
        let parsed: Value = serde_json::from_str(&labels_json).unwrap();
        assert_eq!(parsed["mode"], "cloud");
        assert_eq!(parsed["entity"], "test-entity-3");
    }

    #[test]
    fn test_buffer_metric_respects_standalone() {
        temp_env::with_vars(vec![("STANDALONE_MODE", Some("true"))], || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        // Ensure STANDALONE_MODE is true. Telemetry should be ignored

        let labels = json!({"user_id": "standalone_test"});
        let res = buffer_metric(&pool, "test_standalone", "counter", 1.0, labels).await;
        assert!(res.is_ok());

        let row = sqlx::query("SELECT COUNT(*) FROM telemetry_buffer WHERE metric_name = 'test_standalone'")
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
    fn test_no_pii_logging_statements() {
        use walkdir::WalkDir;
        use std::fs;
        use std::env;
        use std::path::PathBuf;

        let mut violations = Vec::new();

        let mut search_dirs = vec![PathBuf::from(".")];
        if let Ok(workspace_dir) = env::var("BUILD_WORKSPACE_DIRECTORY") {
            let mut p = PathBuf::from(&workspace_dir);
            p.push("src");
            search_dirs.push(p.clone());
            let mut p2 = PathBuf::from(&workspace_dir);
            p2.push("srcs");
            search_dirs.push(p2);
        } else if let Ok(runfiles_dir) = env::var("RUNFILES_DIR") {
            let p = PathBuf::from(runfiles_dir.clone());
            search_dirs.push(p);
            let mut p2 = PathBuf::from(runfiles_dir);
            p2.push("srcs");
            search_dirs.push(p2);
        }

        let mut checked_files = 0;

        for dir in search_dirs {
            if dir.exists() {
                let walker = WalkDir::new(&dir).into_iter().filter_entry(|e| {
                    e.path().components().all(|c| c.as_os_str() != "external")
                });

                for entry in walker
                    .filter_map(Result::ok)
                    .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs" || ext == "go"))
                {
                    checked_files += 1;
                    let content = fs::read_to_string(entry.path()).unwrap_or_default();
                    for (i, line) in content.lines().enumerate() {
                        let lower_line = line.to_lowercase();
                        if lower_line.contains("tracing::info!") ||
                           lower_line.contains("etracing::info!") ||
                           lower_line.contains("info!") ||
                           lower_line.contains("error!") ||
                           lower_line.contains("warn!") ||
                           lower_line.contains("debug!") ||
                           lower_line.contains("println!") ||
                           lower_line.contains("eprintln!") ||
                           lower_line.contains("fmt.print") ||
                           lower_line.contains("log.print") ||
                           lower_line.contains("tracing::")
                        {
                            if lower_line.contains("tenant_id") ||
                               lower_line.contains("organization_id") ||
                               lower_line.contains("org_id") ||
                               lower_line.contains("session_data") ||
                               lower_line.contains("session_id") ||
                               lower_line.contains("payload") ||
                               lower_line.contains("email") ||
                               lower_line.contains("password") ||
                               lower_line.contains("pii") ||
                               lower_line.contains("api_key") ||
                               lower_line.contains("secret_key") ||
                               lower_line.contains("credit") ||
                               lower_line.contains("card") ||
                               lower_line.contains("cvv") ||
                               lower_line.contains("dob") ||
                               lower_line.contains("birth") ||
                               lower_line.contains("passport") ||
                               lower_line.contains("bank") ||
                               lower_line.contains("account") ||
                               lower_line.contains("stripe") ||
                               lower_line.contains("billing") {
                                violations.push(format!("{}:{}: {}", entry.path().display(), i + 1, line.trim()));
                            }
                        }
                    }
                }
            }
        }

        assert!(checked_files > 10, "Could not find enough .rs files to run PII leakage test. Checked: {}", checked_files);
        assert!(
            violations.is_empty(),
            "Found PII logging violations in the following lines:\n{:#?}",
            violations
        );
    }

    #[test]
    fn test_init_telemetry_standalone_opt_out() {
        temp_env::with_vars(
            [
                ("OHC_STANDALONE", Some("true")),
                ("STANDALONE_MODE", Some("true")),
                ("OHC_TELEMETRY_ENABLED", Some("false")),
                ("DATABASE_URL", Some("sqlite://ohc-standalone.db")),
            ],
            || {
                let config = crate::config::load().unwrap();

                // Assert that the config logic matches the policy:
                // If STANDALONE_MODE=true and OHC_TELEMETRY_ENABLED=false, telemetry should NOT run.
                let should_start_telemetry = config.telemetry_enabled;

                assert_eq!(should_start_telemetry, false);
            },
        );
    }

    #[test]
    fn test_init_telemetry_standalone_opt_in() {
        temp_env::with_vars(
            [
                ("OHC_STANDALONE", Some("true")),
                ("STANDALONE_MODE", Some("true")),
                ("OHC_TELEMETRY_ENABLED", Some("true")),
                ("DATABASE_URL", Some("sqlite://ohc-standalone.db")),
            ],
            || {
                let config = crate::config::load().unwrap();

                // If STANDALONE_MODE=true and OHC_TELEMETRY_ENABLED=true, telemetry SHOULD run.
                let should_start_telemetry = config.telemetry_enabled;

                assert_eq!(should_start_telemetry, true);
            },
        );
    }
}

#[tokio::test]
async fn test_queue_length_gauge_initialization() {
    let gauge = crate::telemetry::get_queue_length_gauge();
    gauge.add(1, &[]);
}

#[tokio::test]
async fn test_record_queue_length_with_deployment_mode() {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
    let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
        Ok(Ok(p)) => p,
        _ => return, // Gracefully exit if DB is not available in sandbox or times out
    };

    let res = crate::telemetry::record_queue_length(&pool, 5).await;
    assert!(res.is_ok());

    let row = sqlx::query("SELECT labels_json, value FROM telemetry_buffer WHERE metric_name = 'ohc_sub_agent_queue_length' ORDER BY timestamp DESC LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    use sqlx::Row;
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

    let redacted = crate::telemetry::redact_interface_pii(payload);

    // Verify root level safe fields
    assert_eq!(redacted["safe_field"], "This should not be redacted");
    assert_eq!(redacted["another_safe"], 123);

    // Because the key is "payload", the entire object gets redacted to "[REDACTED]"
    assert_eq!(redacted["payload"], "[REDACTED]");

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
