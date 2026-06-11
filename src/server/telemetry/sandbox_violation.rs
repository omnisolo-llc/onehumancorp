use opentelemetry::global;
use opentelemetry::metrics::Counter;
use sqlx::PgPool;
use serde_json::Value;
use uuid::Uuid;
use opentelemetry::KeyValue;

pub struct SandboxViolationStore {
    pool: Option<PgPool>,
    violation_counter: Counter<u64>,
}

impl SandboxViolationStore {
    pub fn new(pool: Option<PgPool>) -> Self {
        let meter = global::meter("ohc.telemetry");
        let violation_counter = meter.u64_counter("ohc_sandbox_violation_total").build();

        Self {
            pool,
            violation_counter,
        }
    }

    pub async fn record_violation(
        &self,
        tenant_id: &str,
        agent_id: &str,
        session_id: &str,
        violation_type: &str,
        details: Value,
    ) -> Result<(), sqlx::Error> {
        // Emit OpenTelemetry metric
        self.violation_counter.add(
            1,
            &[KeyValue::new("type", violation_type.to_string())],
        );

        // Save to DB if pool is available
        if let Some(pool) = &self.pool {
            let id = Uuid::new_v4().to_string();

            // Explicitly map details to Json to be stored natively as JSONB (in PG) or TEXT (in SQLite)
            // if needed, or stringify it based on what `agent_violations` schema actually is.
            // In 002_missing_tables.sql it is `details TEXT NOT NULL`
            // Wait, the harness store uses `sqlx::types::Json`, but `002_missing_tables.sql` has `details TEXT NOT NULL`.
            // Let's just stringify details to be safe and compatible with TEXT.
            let redacted_details = crate::redact_interface_pii(details);
            let details_str = redacted_details.to_string();

            // Execute in an explicit transaction
            let mut tx = pool.begin().await?;

            sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                .bind(tenant_id)
                .execute(&mut *tx)
                .await?;

            sqlx::query(
                r#"
                INSERT INTO agent_violations (id, tenant_id, agent_id, session_id, violation_type, details)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#
            )
            .bind(id)
            .bind(tenant_id)
            .bind(agent_id)
            .bind(session_id)
            .bind(violation_type)
            .bind(details_str)
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_record_violation_no_pool() {
        let store = SandboxViolationStore::new(None);
        let result = store.record_violation(
            "tenant-123",
            "agent-123",
            "session-456",
            "capability_breach",
            json!({"action": "network_request"}),
        ).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_record_violation_with_pool() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available
        };

        // Ensure table exists for testing
        let _ = sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS agent_violations (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT NOT NULL,
                session_id TEXT NOT NULL,
                violation_type TEXT NOT NULL,
                details TEXT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                _sync_status TEXT DEFAULT 'pending',
                version INTEGER DEFAULT 1
            );
            "#
        ).execute(&pool).await;

        let store = SandboxViolationStore::new(Some(pool.clone()));

        let result = store.record_violation(
            "test-tenant",
            "test-agent",
            "test-session",
            "sandbox_escape",
            json!({
                "path": "/etc/shadow",
                "password": "supersecretpassword123!"
            }),
        ).await;

        assert!(result.is_ok());

        use sqlx::Row;
        let row = sqlx::query("SELECT tenant_id, agent_id, violation_type, details FROM agent_violations WHERE session_id = 'test-session' LIMIT 1")
            .fetch_one(&pool)
            .await;

        assert!(row.is_ok());
        let record = row.unwrap();

        let fetched_tenant_id: String = record.get("tenant_id");
        let fetched_agent_id: String = record.get("agent_id");
        let fetched_violation_type: String = record.get("violation_type");

        let fetched_details: String = record.get("details");

        assert_eq!(fetched_tenant_id, "test-tenant");
        assert_eq!(fetched_agent_id, "test-agent");
        assert_eq!(fetched_violation_type, "sandbox_escape");

        let details_json: Value = serde_json::from_str(&fetched_details).unwrap();
        assert_eq!(details_json["path"], "/etc/shadow");
        assert_eq!(details_json["password"], "[REDACTED]");

        // Clean up
        let _ = sqlx::query("DELETE FROM agent_violations WHERE session_id = 'test-session'").execute(&pool).await;
    }
}
