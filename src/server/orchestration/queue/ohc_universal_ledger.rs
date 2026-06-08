use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHCLedgerEntry {
    pub id: String,
    pub tenant_id: String,
    pub event_type: String,
    pub department: String,
    pub payload: String,
    pub created_at: DateTime<Utc>,
}

pub struct OHCUniversalLedger {
    pool: Arc<PgPool>,
}

impl OHCUniversalLedger {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn append_entry(&self, tenant_id: &str, event_type: &str, department: &str, payload: &serde_json::Value) -> Result<String, String> {
        let entry_id = Uuid::new_v4().to_string();
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO ohc_universal_ledger (id, tenant_id, event_type, department, payload, created_at)
             VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)"
        )
        .bind(&entry_id)
        .bind(tenant_id)
        .bind(event_type)
        .bind(department)
        .bind(payload)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(entry_id)
    }

    pub async fn get_entries(&self, tenant_id: &str, limit: i64) -> Result<Vec<OHCLedgerEntry>, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        let rows = sqlx::query(
            "SELECT id, tenant_id, event_type, department, payload, created_at
             FROM ohc_universal_ledger
             WHERE tenant_id = $1
             ORDER BY created_at DESC
             LIMIT $2"
        )
        .bind(tenant_id)
        .bind(limit)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let mut entries = Vec::new();
        use sqlx::Row;
        for row in rows {
            let payload_val: serde_json::Value = row.try_get("payload").unwrap_or(serde_json::Value::Null);
            let redacted_payload = ::server_telemetry::redact_interface_pii(serde_json::to_value(&payload_val).unwrap_or_else(|_| serde_json::json!({})));
            let payload_str = serde_json::to_string(&redacted_payload).unwrap_or_default();
            entries.push(OHCLedgerEntry {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                event_type: row.get("event_type"),
                department: row.get("department"),
                payload: payload_str,
                created_at: row.get("created_at"),
            });
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(entries)
    }
}
