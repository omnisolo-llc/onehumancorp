use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OfflinePosTransaction {
    pub id: String,
    pub amount_cents: i64,
    pub currency: String,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOfflinePosRequest {
    pub tenant_id: String,
    pub client_id: String,
    pub transactions: Vec<OfflinePosTransaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOfflinePosResponse {
    pub success: bool,
    pub synced_count: i32,
    pub failed_ids: Vec<String>,
}

pub struct OfflinePosHandler {
    pool: PgPool,
}

impl OfflinePosHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn process_batch(
        &self,
        request: BatchOfflinePosRequest,
    ) -> Result<BatchOfflinePosResponse, String> {
        let mut synced_count = 0;
        let mut failed_ids = Vec::new();

        for tx in request.transactions {
            match self.process_single_transaction(&request.tenant_id, &request.client_id, tx.clone()).await {
                Ok(_) => synced_count += 1,
                Err(e) => {
                    tracing::error!("Failed to process offline POS transaction {}: {}", tx.id, e);
                    failed_ids.push(tx.id);
                }
            }
        }

        Ok(BatchOfflinePosResponse {
            success: failed_ids.is_empty(),
            synced_count,
            failed_ids,
        })
    }

    async fn process_single_transaction(
        &self,
        tenant_id: &str,
        client_id: &str,
        tx: OfflinePosTransaction,
    ) -> Result<(), String> {
        let mut db_tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        // Enforce RLS/Tenant isolation
        ::server_common::auth_utils::set_org_context(&mut *db_tx, tenant_id)
            .await
            .map_err(|e| e.to_string())?;

        // 1. Record the delta for CRDT-based sync
        let delta_id = format!("pos_{}", tx.id);
        sqlx::query(
            "INSERT INTO crdt_deltas (tenant_id, id, entity_id, data, updated_at)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (tenant_id, id) DO UPDATE SET
             data = excluded.data, updated_at = excluded.updated_at
             WHERE crdt_deltas.updated_at < excluded.updated_at"
        )
        .bind(tenant_id)
        .bind(&delta_id)
        .bind(&tx.id)
        .bind(serde_json::to_value(&tx).unwrap())
        .bind(tx.timestamp)
        .execute(&mut *db_tx)
        .await
        .map_err(|e| e.to_string())?;

        // 2. Insert into offline transactions table
        sqlx::query(
            "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status)
             VALUES ($1, $2, $3, $4, $5, $6, 'PENDING')
             ON CONFLICT (id) DO NOTHING"
        )
        .bind(&tx.id)
        .bind(tenant_id)
        .bind(client_id)
        .bind(tx.amount_cents)
        .bind(&tx.currency)
        .bind(&tx.payload)
        .execute(&mut *db_tx)
        .await
        .map_err(|e| e.to_string())?;

        // 3. Queue job for processing
        let job_id = Uuid::new_v4().to_string();
        let job_payload = serde_json::json!({
            "transaction_id": tx.id,
            "amount_cents": tx.amount_cents,
            "currency": tx.currency,
            "payload": tx.payload,
        });

        sqlx::query(
            "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
             VALUES ($1, $2, 'offline_pos_sync', $3)"
        )
        .bind(&job_id)
        .bind(tenant_id)
        .bind(job_payload)
        .execute(&mut *db_tx)
        .await
        .map_err(|e| e.to_string())?;

        db_tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}
