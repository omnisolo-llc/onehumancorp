use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LedgerEntry {
    pub id: Uuid,
    pub tenant_id: String,
    pub department: String,
    pub event_type: String,
    pub payload: sqlx::types::Json<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

pub struct Ledger {
    pool: PgPool,
}

impl Ledger {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn append_event(
        &self,
        tenant_id: &str,
        department: &str,
        event_type: &str,
        payload: serde_json::Value,
    ) -> Result<LedgerEntry, sqlx::Error> {
        let entry: LedgerEntry = sqlx::query_as!(
            LedgerEntry,
            r#"
            INSERT INTO ohc_universal_ledger (tenant_id, department, event_type, payload)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, department, event_type, payload as "payload: sqlx::types::Json<serde_json::Value>", created_at
            "#,
            tenant_id,
            department,
            event_type,
            sqlx::types::Json(payload) as _,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(entry)
    }
}
