use opentelemetry::global;
use opentelemetry::metrics::Counter;
use sqlx::PgPool;
use serde_json::Value;
use uuid::Uuid;
use opentelemetry::KeyValue;

pub struct ViolationStore {
    pool: Option<PgPool>,
    violation_counter: Counter<u64>,
    pub token_usage_counter: Counter<u64>,
    pub llm_cost_counter: Counter<u64>,
    pub storage_bytes_counter: Counter<u64>,
    pub rate_limit_checks_total: Counter<u64>,
    pub rate_limit_exceeded_total: Counter<u64>,
}

impl ViolationStore {
    pub fn new(pool: Option<PgPool>) -> Self {
        let meter = global::meter("ohc.harness.telemetry");
        let violation_counter = meter.u64_counter("ohc_agent_violations_total").build();
        let token_usage_counter = meter.u64_counter("ohc_tenant_token_usage_total").build();
        let llm_cost_counter = meter.u64_counter("ohc_tenant_llm_cost_cents").build();
        let storage_bytes_counter = meter.u64_counter("ohc_storage_bytes_total").build();
        let rate_limit_checks_total = meter.u64_counter("ohc_rate_limit_checks_total").build();
        let rate_limit_exceeded_total = meter.u64_counter("ohc_rate_limit_exceeded_total").build();

        Self {
            pool,
            violation_counter,
            token_usage_counter,
            llm_cost_counter,
            storage_bytes_counter,
            rate_limit_checks_total,
            rate_limit_exceeded_total,
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
            // To be compatible with both PostgreSQL and SQLite logic:
            // For PostgreSQL, details needs to be explicitly casted or we use sqlx::types::Json.
            // Since this is a simple module, we use the string bind and explicit cast on PG, or rely on the string on SQLite.
            // Wait, our migration script is specific to postgres. For SQLite we store as text.
            // A safer approach that works for both is bind as string, but for PostgreSQL the migration script might need to ensure it's compatible.
            // Actually, in `sqlx`, to insert into a `JSONB` column without casting, one can use `sqlx::types::Json` or just cast it in the query `CAST($6 as JSONB)`. However, since SQLite doesn't have `JSONB`, casting breaks on SQLite.
            // Better to use `sqlx::types::Json`.

            let redacted_details = ::server_telemetry::redact_interface_pii(details);
            let json_value: sqlx::types::Json<Value> = sqlx::types::Json(redacted_details);

            // Execute in an explicit transaction to set RLS correctly
            let mut tx = pool.begin().await?;

            // Since PgPool is specifically a PostgreSQL pool, it can never be SQLite.
            // We can safely apply the SET LOCAL for PostgreSQL.
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
            .bind(json_value)
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
        let store = ViolationStore::new(None);
        let result = store.record_violation(
            "tenant-123",
            "agent-123",
            "session-456",
            "file_access",
            json!({"path": "/etc/shadow"}),
        ).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_record_violation_with_pool() {
        // To accurately test DB logic locally, try connecting to Postgres.
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };

        // Create table in test db if it doesn't exist
        let _ = sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS agent_violations (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT NOT NULL,
                session_id TEXT NOT NULL,
                violation_type TEXT NOT NULL,
                details JSONB NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            ALTER TABLE agent_violations ENABLE ROW LEVEL SECURITY;
            "#
        ).execute(&pool).await;

        let store = ViolationStore::new(Some(pool.clone()));

        let details = json!({"path": "/etc/passwd"});
        let result = store.record_violation(
            "test-tenant",
            "test-agent",
            "test-session",
            "network_access",
            details.clone(),
        ).await;

        assert!(result.is_ok());

        // Verify the record was inserted correctly
        let row = sqlx::query("SELECT tenant_id, agent_id, violation_type FROM agent_violations WHERE session_id = 'test-session' LIMIT 1")
            .fetch_one(&pool)
            .await;

        assert!(row.is_ok());
        use sqlx::Row;
        let record = row.unwrap();

        let fetched_tenant_id: String = record.get("tenant_id");
        let fetched_agent_id: String = record.get("agent_id");
        let fetched_violation_type: String = record.get("violation_type");

        assert_eq!(fetched_tenant_id, "test-tenant");
        assert_eq!(fetched_agent_id, "test-agent");
        assert_eq!(fetched_violation_type, "network_access");

        // Clean up
        let _ = sqlx::query("DELETE FROM agent_violations WHERE session_id = 'test-session'").execute(&pool).await;
    }

}
