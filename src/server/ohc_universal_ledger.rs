use sqlx::{PgPool, Row};
use std::sync::Arc;

pub struct OhcUniversalLedger {
    pool: Arc<PgPool>,
}

impl OhcUniversalLedger {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn record_event(&self, id: &str, tenant_id: &str, department: &str, event_type: &str, payload: &str) -> Result<(), String> {
        let payload_json: serde_json::Value = serde_json::from_str(payload).unwrap_or(serde_json::Value::Null);

        sqlx::query(
            "INSERT INTO ohc_universal_ledger (id, tenant_id, department, event_type, payload)
             VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(department)
        .bind(event_type)
        .bind(payload_json)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
