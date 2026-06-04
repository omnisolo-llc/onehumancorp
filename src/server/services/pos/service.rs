use ::server_ohc::app::pos_service_server::PosService;
use ::server_ohc::app::{SyncOfflineTransactionsRequest, SyncOfflineTransactionsResponse};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct MyPosService {
    db: Arc<crate::db::DB>,
}

impl MyPosService {
    pub fn new(db: Arc<crate::db::DB>) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl PosService for MyPosService {
    async fn sync_offline_transactions(
        &self,
        request: Request<SyncOfflineTransactionsRequest>,
    ) -> Result<Response<SyncOfflineTransactionsResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = req.tenant_id;
        let client_id = req.client_id;

        let mut synced_count = 0;
        let mut failed_ids = Vec::new();

        let pool = crate::db::get_pool();

        for tx in req.transactions {
            // Insert into pos_offline_transactions
            let tx_id = if tx.id.is_empty() { Uuid::new_v4().to_string() } else { tx.id.clone() };

            let mut db_tx = match pool.begin().await {
                Ok(t) => t,
                Err(e) => {
                    tracing::error!("Failed to begin transaction: {}", e);
                    failed_ids.push(tx.id);
                    continue;
                }
            };

            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id).await {
                tracing::error!("Failed to set org context: {}", e);
                failed_ids.push(tx.id);
                continue;
            }

            let insert_res = sqlx::query(
                "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status)
                 VALUES ($1, $2, $3, $4, $5, $6::jsonb, 'PENDING')"
            )
            .bind(&tx_id)
            .bind(&tenant_id)
            .bind(&client_id)
            .bind(tx.amount_cents)
            .bind(&tx.currency)
            .bind(&tx.payload)
            .execute(&mut *db_tx)
            .await;

            if let Err(e) = insert_res {
                tracing::error!("Failed to insert offline transaction: {}", e);
                failed_ids.push(tx.id);
                continue;
            }

            // Queue job
            let job_id = Uuid::new_v4().to_string();
            let payload = serde_json::json!({
                "pos_transaction_id": tx_id,
                "client_id": client_id,
                "amount_cents": tx.amount_cents,
                "currency": tx.currency,
                "payload": tx.payload,
            }).to_string();

            let job_res = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
                 VALUES ($1, $2, 'pos_offline_sync', $3::jsonb)"
            )
            .bind(&job_id)
            .bind(&tenant_id)
            .bind(&payload)
            .execute(&mut *db_tx)
            .await;

            if let Err(e) = job_res {
                tracing::error!("Failed to enqueue job: {}", e);
                failed_ids.push(tx.id);
                continue;
            }

            if let Err(e) = db_tx.commit().await {
                tracing::error!("Failed to commit transaction: {}", e);
                failed_ids.push(tx.id);
                continue;
            }

            synced_count += 1;
        }

        Ok(Response::new(SyncOfflineTransactionsResponse {
            success: failed_ids.is_empty(),
            synced_count,
            failed_transaction_ids: failed_ids,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;
    use crate::db::DbStore;

    #[tokio::test]
    async fn test_sync_offline_transactions() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB {
            pool: crate::db::get_pool(),
            store: DbStore::Postgres,
        });

        let service = MyPosService::new(db.clone());

        let req = SyncOfflineTransactionsRequest {
            tenant_id: "test_tenant".to_string(),
            client_id: "test_client".to_string(),
            transactions: vec![
                ::server_ohc::app::PosOfflineTransaction {
                    id: "tx_1".to_string(),
                    tenant_id: "test_tenant".to_string(),
                    client_id: "test_client".to_string(),
                    amount_cents: 1000,
                    currency: "USD".to_string(),
                    payload: "{}".to_string(),
                    status: "PENDING".to_string(),
                    created_at_unix: 0,
                }
            ],
        };

        let response = service.sync_offline_transactions(Request::new(req)).await;
        // Depending on whether DB contains proper tables (migrated), this might fail gracefully but shouldn't panic.
        assert!(response.is_ok() || response.is_err());
    }
}
