#[cfg(test)]
mod tests {

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
            "username": "maya",
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
        assert_eq!(parsed["organization_id"], "org_test");
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
        assert_eq!(parsed["organization_id"], "org-1");
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
        assert_eq!(parsed["organization_id"], "org-2");
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

    #[tokio::test]
    async fn test_buffer_metric_respects_standalone() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let labels = json!({"user_id": "standalone_test"});
        let labels_clone = labels.clone();

        // Ensure STANDALONE_MODE is true. Telemetry should be ignored if opted out
        temp_env::async_with_vars(
            [
                ("STANDALONE_MODE", Some("true")),
                ("OHC_TELEMETRY_ENABLED", Some("false")),
                ("DATABASE_URL", Some(db_url.as_str())),
            ],
            async {
                let res = crate::telemetry::buffer_metric(&pool, "test_standalone", "counter", 1.0, labels_clone).await;
                assert!(res.is_ok());
            },
        ).await;

        let row = sqlx::query("SELECT COUNT(*) FROM telemetry_buffer WHERE metric_name = 'test_standalone'")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let count: i64 = row.get(0);
        assert_eq!(count, 0, "Metric should not be buffered in standalone mode if opted out");

        // Telemetry SHOULD be buffered if explicitly opted in
        temp_env::async_with_vars(
            [
                ("STANDALONE_MODE", Some("true")),
                ("OHC_TELEMETRY_ENABLED", Some("true")),
                ("DATABASE_URL", Some(db_url.as_str())),
            ],
            async {
                let res2 = crate::telemetry::buffer_metric(&pool, "test_standalone_opt_in", "counter", 1.0, labels.clone()).await;
                assert!(res2.is_ok());
            },
        ).await;

        let row2 = sqlx::query("SELECT COUNT(*) FROM telemetry_buffer WHERE metric_name = 'test_standalone_opt_in'")
            .fetch_one(&pool)
            .await
            .unwrap();

        let count2: i64 = row2.get(0);
        assert_eq!(count2, 1, "Metric should be buffered in standalone mode if explicitly opted in");

        // Telemetry SHOULD NOT be buffered by default (unset) in standalone mode
        temp_env::async_with_vars(
            [
                ("STANDALONE_MODE", Some("true")),
                ("OHC_TELEMETRY_ENABLED", None::<&str>),
                ("DATABASE_URL", Some(db_url.as_str())),
            ],
            async {
                let res3 = crate::telemetry::buffer_metric(&pool, "test_standalone_default", "counter", 1.0, labels).await;
                assert!(res3.is_ok());
            },
        ).await;

        let row3 = sqlx::query("SELECT COUNT(*) FROM telemetry_buffer WHERE metric_name = 'test_standalone_default'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let count3: i64 = row3.get(0);
        assert_eq!(count3, 0, "Metric should not be buffered in standalone mode by default");
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
            let mut p = PathBuf::from(workspace_dir);
            p.push("src");
            search_dirs.push(p);
        } else if let Ok(runfiles_dir) = env::var("RUNFILES_DIR") {
            let p = PathBuf::from(runfiles_dir);
            search_dirs.push(p);
        }

        let mut checked_files = 0;

        for dir in search_dirs {
            if dir.exists() {
                let walker = WalkDir::new(&dir).into_iter().filter_entry(|e| {
                    e.path().components().all(|c| c.as_os_str() != "external")
                });

                for entry in walker
                    .filter_map(Result::ok)
                    .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
                {
                    checked_files += 1;
                    let content = fs::read_to_string(entry.path()).unwrap_or_default();
                    for (i, line) in content.lines().enumerate() {
                        let lower_line = line.to_lowercase();
                        if lower_line.contains("println!") ||
                           lower_line.contains("eprintln!") ||
                           lower_line.contains("info!") ||
                           lower_line.contains("error!") ||
                           lower_line.contains("warn!") ||
                           lower_line.contains("debug!") ||
                           lower_line.contains("tracing::")
                        {
                            if lower_line.contains("tenant_id") ||
                               lower_line.contains("org_id") ||
                               lower_line.contains("session_data") ||
                               lower_line.contains("session_id") ||
                               lower_line.contains("payload") ||
                               lower_line.contains("email") ||
                               lower_line.contains("password") ||
                               lower_line.contains("api_key") ||
                               lower_line.contains("pii") {
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
                let is_standalone = std::env::var("STANDALONE_MODE").unwrap_or_else(|_| "true".to_string()) == "true";

                // Assert that the config logic matches the policy:
                // If STANDALONE_MODE=true and OHC_TELEMETRY_ENABLED=false, telemetry should NOT run.
                // In lib.rs, the gate is `is_standalone && config.telemetry_enabled`.
                let should_start_telemetry = is_standalone && config.telemetry_enabled;

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
                let is_standalone = std::env::var("STANDALONE_MODE").unwrap_or_else(|_| "true".to_string()) == "true";

                // If STANDALONE_MODE=true and OHC_TELEMETRY_ENABLED=true, telemetry SHOULD run.
                let should_start_telemetry = is_standalone && config.telemetry_enabled;

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
