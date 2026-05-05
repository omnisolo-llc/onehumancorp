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
    async fn test_record_token_burn_rate_predicted_24h() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let res = crate::telemetry::record_token_burn_rate_predicted_24h(&pool, "tenant_a", "cloud", 50.5).await;
        assert!(res.is_ok());

        let row = sqlx::query("SELECT labels_json, value FROM telemetry_buffer WHERE metric_name = 'ohc_token_burn_rate_predicted_24h' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let value: f32 = row.get("value");
        assert_eq!(value, 50.5);

        let labels_json: String = row.get("labels_json");
        let parsed: Value = serde_json::from_str(&labels_json).unwrap();
        assert_eq!(parsed["tenant_id"], "tenant_a");
        assert_eq!(parsed["deployment_mode"], "cloud");
    }

    #[tokio::test]
    async fn test_record_token_budget_alert_total() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let res = crate::telemetry::record_token_budget_alert_total(&pool, "tenant_b", "standalone", 1.0).await;
        assert!(res.is_ok());

        let row = sqlx::query("SELECT labels_json, value FROM telemetry_buffer WHERE metric_name = 'ohc_token_budget_alert_total' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let value: f32 = row.get("value");
        assert_eq!(value, 1.0);

        let labels_json: String = row.get("labels_json");
        let parsed: Value = serde_json::from_str(&labels_json).unwrap();
        assert_eq!(parsed["tenant_id"], "tenant_b");
        assert_eq!(parsed["deployment_mode"], "standalone");
    }

    #[tokio::test]
    async fn test_record_mission_dead_letter_total() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        let res = crate::telemetry::record_mission_dead_letter_total(&pool, "tenant_c", "cloud", 3.0).await;
        assert!(res.is_ok());

        let row = sqlx::query("SELECT labels_json, value FROM telemetry_buffer WHERE metric_name = 'ohc_mission_dead_letter_total' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        use sqlx::Row;
        let value: f32 = row.get("value");
        assert_eq!(value, 3.0);

        let labels_json: String = row.get("labels_json");
        let parsed: Value = serde_json::from_str(&labels_json).unwrap();
        assert_eq!(parsed["tenant_id"], "tenant_c");
        assert_eq!(parsed["deployment_mode"], "cloud");
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
    async fn test_buffer_metric_respects_standalone() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        // Ensure STANDALONE_MODE is true. Telemetry should be ignored
        unsafe { std::env::set_var("STANDALONE_MODE", "true"); }
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
            p.push("src/server");
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
                               lower_line.contains("payload") {
                                violations.push(format!("{}:{}: {}", entry.path().display(), i + 1, line.trim()));
                            }
                        }
                    }
                }

                if checked_files > 10 {
                    break;
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
}
