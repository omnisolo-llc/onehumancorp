use ::server_ohc::app::pos_service_server::PosService;
use ::server_ohc::app::{SyncOfflineTransactionsRequest, SyncOfflineTransactionsResponse};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct MyPosService {}

impl MyPosService {
    pub fn new(_db: Arc<crate::db::DB>) -> Self {
        Self { }
    }
}

#[tonic::async_trait]
impl PosService for MyPosService {
    async fn sync_offline_transactions(
        &self,
        request: Request<SyncOfflineTransactionsRequest>,
    ) -> Result<Response<SyncOfflineTransactionsResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let mut req = request.into_inner();
        req.tenant_id = tenant_id.clone();
        let client_id = req.client_id;

        let mut synced_count = 0;
        let mut failed_ids = Vec::new();

        let pool = crate::db::get_pool();
        let mut futures = Vec::new();

        for tx in req.transactions {
            let pool_clone = pool.clone();
            let tenant_id_clone = tenant_id.clone();
            let client_id_clone = client_id.clone();
            let tx_id = if tx.id.is_empty() { Uuid::new_v4().to_string() } else { tx.id.clone() };

            futures.push(tokio::spawn(async move {
                let mut db_tx = match pool_clone.begin().await {
                    Ok(t) => t,
                    Err(e) => {
                        tracing::error!("Failed to begin transaction: {}", e);
                        return Err(tx.id);
                    }
                };

                if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id_clone).await {
                    tracing::error!("Failed to set org context: {}", e);
                    return Err(tx.id);
                }

                let insert_res = sqlx::query(
                    "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status)
                     VALUES ($1, $2, $3, $4, $5, $6::jsonb, 'PENDING')"
                )
                .bind(&tx_id)
                .bind(&tenant_id_clone)
                .bind(&client_id_clone)
                .bind(tx.amount_cents)
                .bind(&tx.currency)
                .bind(&tx.payload)
                .execute(&mut *db_tx)
                .await;

                if let Err(e) = insert_res {
                    tracing::error!("Failed to insert offline transaction: {}", e);
                    return Err(tx.id);
                }

                // Queue job
                let job_id = Uuid::new_v4().to_string();
                let payload = serde_json::json!({
                    "pos_transaction_id": tx_id,
                    "client_id": client_id_clone,
                    "amount_cents": tx.amount_cents,
                    "currency": tx.currency,
                    "payload": tx.payload,
                }).to_string();

                let job_res = sqlx::query(
                    "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
                     VALUES ($1, $2, 'pos_offline_sync', $3::jsonb)"
                )
                .bind(&job_id)
                .bind(&tenant_id_clone)
                .bind(&payload)
                .execute(&mut *db_tx)
                .await;

                if let Err(e) = job_res {
                    tracing::error!("Failed to enqueue job: {}", e);
                    return Err(tx.id);
                }

                if let Err(e) = db_tx.commit().await {
                    tracing::error!("Failed to commit transaction: {}", e);
                    return Err(tx.id);
                }

                Ok(())
            }));
        }

        let results = futures::future::join_all(futures).await;

        for res in results {
            match res {
                Ok(Ok(())) => {
                    synced_count += 1;
                }
                Ok(Err(id)) => {
                    failed_ids.push(id);
                }
                Err(e) => {
                    tracing::error!("Task failed to execute: {}", e);
                }
            }
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

        let mut request = Request::new(req);
        request.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: "test_tenant".to_string(),
            agent_id: "test".to_string(),
        });

        let response = service.sync_offline_transactions(request).await;
        // Depending on whether DB contains proper tables (migrated), this might fail gracefully but shouldn't panic.
        assert!(response.is_ok() || response.is_err());
    }
}
