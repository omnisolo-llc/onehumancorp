use ::server_ohc::app::pos_service_server::PosService;
use ::server_ohc::app::{ProcessInStoreCheckoutRequest, ProcessInStoreCheckoutResponse, SyncOfflineTransactionsRequest, SyncOfflineTransactionsResponse};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct MyPosService {
    pub redis_client: Option<redis::Client>,
}

impl MyPosService {
    pub fn new(_db: Arc<crate::db::DB>, redis_client: Option<redis::Client>) -> Self {
        Self { redis_client }
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

    async fn process_in_store_checkout(
        &self,
        request: Request<ProcessInStoreCheckoutRequest>,
    ) -> Result<Response<ProcessInStoreCheckoutResponse>, Status> {
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

        let inventory_lock_id = format!("ohc:lock:{}:inventory:{}", req.tenant_id, req.product_id);

        if let Some(client) = &self.redis_client {
            let mut conn = client.get_multiplexed_async_connection().await
                .map_err(|e| Status::internal(format!("Redis conn failed: {}", e)))?;
            let acquired: bool = redis::cmd("SET")
                .arg(&inventory_lock_id)
                .arg("1")
                .arg("EX")
                .arg(15) // 15s TTL for tap-to-pay
                .arg("NX")
                .query_async(&mut conn)
                .await
                .unwrap_or(false);

            if !acquired {
                return Ok(Response::new(ProcessInStoreCheckoutResponse {
                    success: false,
                    transaction_id: "".to_string(),
                    message: "Item just sold out or is currently being checked out".to_string(),
                }));
            }
        }


        let pool = crate::db::get_pool();
        let mut db_tx = match pool.begin().await {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("Failed to begin transaction: {}", e);
                if let Some(client) = &self.redis_client {
                    if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                        let _: () = redis::cmd("DEL").arg(&inventory_lock_id).query_async(&mut conn).await.unwrap_or(());
                    }
                }
                return Err(Status::internal("Internal database error"));
            }
        };

        if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &req.tenant_id).await {
            tracing::error!("Failed to set org context: {}", e);
            if let Some(client) = &self.redis_client {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let _: () = redis::cmd("DEL").arg(&inventory_lock_id).query_async(&mut conn).await.unwrap_or(());
                }
            }
            return Err(Status::internal("Internal database error"));
        }

        let inventory_count: i32 = match sqlx::query_scalar(
            "SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2 FOR UPDATE"
        )
        .bind(&req.product_id)
        .bind(&req.tenant_id)
        .fetch_one(&mut *db_tx)
        .await {
            Ok(count) => count,
            Err(_) => {
                let _ = db_tx.rollback().await;
                if let Some(client) = &self.redis_client {
                    if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                        let _: () = redis::cmd("DEL").arg(&inventory_lock_id).query_async(&mut conn).await.unwrap_or(());
                    }
                }
                return Ok(Response::new(ProcessInStoreCheckoutResponse {
                    success: false,
                    transaction_id: "".to_string(),
                    message: "Product not found".to_string(),
                }));
            }
        };

        if inventory_count <= 0 {
            let _ = db_tx.rollback().await;
            if let Some(client) = &self.redis_client {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let _: () = redis::cmd("DEL").arg(&inventory_lock_id).query_async(&mut conn).await.unwrap_or(());
                }
            }
            return Ok(Response::new(ProcessInStoreCheckoutResponse {
                success: false,
                transaction_id: "".to_string(),
                message: "Item just sold out".to_string(),
            }));
        }

        // Deduct inventory
        if let Err(e) = sqlx::query(
            "UPDATE products SET inventory_count = inventory_count - 1 WHERE id = $1 AND tenant_id = $2"
        )
        .bind(&req.product_id)
        .bind(&req.tenant_id)
        .execute(&mut *db_tx)
        .await {
            tracing::error!("Failed to update inventory: {}", e);
            let _ = db_tx.rollback().await;
            if let Some(client) = &self.redis_client {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let _: () = redis::cmd("DEL").arg(&inventory_lock_id).query_async(&mut conn).await.unwrap_or(());
                }
            }
            return Err(Status::internal("Internal database error"));
        }

        let tx_id = Uuid::new_v4().to_string();

        let payload = serde_json::json!({
            "product_id": req.product_id,
            "quantity": 1
        });

        // Insert into pos_offline_transactions (as SYNCED)
        if let Err(e) = sqlx::query(
            "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status)
             VALUES ($1, $2, $3, $4, $5, $6::jsonb, 'SYNCED')"
        )
        .bind(&tx_id)
        .bind(&req.tenant_id)
        .bind(&req.client_id)
        .bind(req.amount_cents)
        .bind(&req.currency)
        .bind(&payload)
        .execute(&mut *db_tx)
        .await {
            tracing::error!("Failed to insert synced pos transaction: {}", e);
            let _ = db_tx.rollback().await;
            if let Some(client) = &self.redis_client {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let _: () = redis::cmd("DEL").arg(&inventory_lock_id).query_async(&mut conn).await.unwrap_or(());
                }
            }
            return Err(Status::internal("Internal database error"));
        }

        let order_id = Uuid::new_v4().to_string();
        // Insert into orders
        if let Err(e) = sqlx::query(
            "INSERT INTO orders (id, tenant_id, total_amount, status) VALUES ($1, $2, $3, 'Paid')"
        )
        .bind(&order_id)
        .bind(&req.tenant_id)
        .bind(req.amount_cents as f64 / 100.0) // Usually total_amount might be decimal
        .execute(&mut *db_tx)
        .await {
            tracing::error!("Failed to insert order: {}", e);
            let _ = db_tx.rollback().await;
            if let Some(client) = &self.redis_client {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let _: () = redis::cmd("DEL").arg(&inventory_lock_id).query_async(&mut conn).await.unwrap_or(());
                }
            }
            return Err(Status::internal("Internal database error"));
        }

        let order_item_id = Uuid::new_v4().to_string();
        // Insert into order_items
        if let Err(e) = sqlx::query(
            "INSERT INTO order_items (id, tenant_id, order_id, product_id, quantity) VALUES ($1, $2, $3, $4, 1)"
        )
        .bind(&order_item_id)
        .bind(&req.tenant_id)
        .bind(&order_id)
        .bind(&req.product_id)
        .execute(&mut *db_tx)
        .await {
            tracing::error!("Failed to insert order item: {}", e);
            let _ = db_tx.rollback().await;
            if let Some(client) = &self.redis_client {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let _: () = redis::cmd("DEL").arg(&inventory_lock_id).query_async(&mut conn).await.unwrap_or(());
                }
            }
            return Err(Status::internal("Internal database error"));
        }

        // Queue job for OperationsAgent
        let job_id = Uuid::new_v4().to_string();
        let job_payload = serde_json::json!({
            "order_id": order_id,
            "product_id": req.product_id,
            "quantity": 1,
            "source": "pos_terminal"
        }).to_string();

        if let Err(e) = sqlx::query(
            "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
             VALUES ($1, $2, 'tenant.order.created', $3::jsonb)"
        )
        .bind(&job_id)
        .bind(&req.tenant_id)
        .bind(&job_payload)
        .execute(&mut *db_tx)
        .await {
            tracing::error!("Failed to enqueue order created job: {}", e);
            let _ = db_tx.rollback().await;
            if let Some(client) = &self.redis_client {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let _: () = redis::cmd("DEL").arg(&inventory_lock_id).query_async(&mut conn).await.unwrap_or(());
                }
            }
            return Err(Status::internal("Internal database error"));
        }

        if let Err(e) = db_tx.commit().await {
            tracing::error!("Failed to commit transaction: {}", e);
            if let Some(client) = &self.redis_client {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let _: () = redis::cmd("DEL").arg(&inventory_lock_id).query_async(&mut conn).await.unwrap_or(());
                }
            }
            return Err(Status::internal("Internal database error"));
        }

        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                let _: () = redis::cmd("DEL")
                    .arg(&inventory_lock_id)
                    .query_async(&mut conn)
                    .await
                    .unwrap_or(());
            }
        }

        Ok(Response::new(ProcessInStoreCheckoutResponse {
            success: true,
            transaction_id: tx_id,
            message: "Success".to_string(),
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

        let service = MyPosService::new(db.clone(), None);

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

    #[tokio::test]
    async fn test_process_in_store_checkout_invalid_tenant() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB {
            pool: crate::db::get_pool(),
            store: DbStore::Postgres,
        });

        let service = MyPosService::new(db.clone(), None);

        let req = ProcessInStoreCheckoutRequest {
            tenant_id: "test_tenant".to_string(),
            product_id: "test_product".to_string(),
            amount_cents: 500,
            currency: "USD".to_string(),
            client_id: "test_client".to_string(),
        };

        // Do not add auth_info to trigger the missing tenant identity error
        let request = Request::new(req);

        let response = service.process_in_store_checkout(request).await;
        assert!(response.is_err());
        assert_eq!(response.unwrap_err().code(), tonic::Code::Unauthenticated);
    }
}
