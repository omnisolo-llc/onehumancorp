#[cfg(test)]
mod tests {
    use super::*;
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
        if let Ok(db_url) = std::env::var("DATABASE_URL_NOT_SET") {
            let pool = sqlx::PgPool::connect(&db_url).await.unwrap();

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
    }
}
