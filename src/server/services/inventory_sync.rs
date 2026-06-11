use ::server_ohc::app::inventory_sync_service_server::InventorySyncService;
use ::server_ohc::app::{ReserveInventoryRequest, ReserveInventoryResponse, CommitInventoryRequest, CommitInventoryResponse};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct MyInventorySyncService {
    _db: Arc<crate::db::DB>,
    redis_client: Option<redis::Client>,
}

impl MyInventorySyncService {
    pub fn new(db: Arc<crate::db::DB>, redis_client: Option<redis::Client>) -> Self {
        Self { _db: db, redis_client }
    }
}

#[tonic::async_trait]
impl InventorySyncService for MyInventorySyncService {
    async fn reserve_inventory(
        &self,
        request: Request<ReserveInventoryRequest>,
    ) -> Result<Response<ReserveInventoryResponse>, Status> {
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

        let lock_id = Uuid::new_v4().to_string();
        let lock_key = format!("ohc:lock:{}:inventory:{}", req.tenant_id, req.product_id);

        if let Some(client) = &self.redis_client {
            let mut conn = client.get_multiplexed_async_connection().await
                .map_err(|e| Status::internal(format!("Redis conn failed: {}", e)))?;

            let ttl = if req.ttl_seconds > 0 { req.ttl_seconds } else { 15 };

            let acquired: bool = redis::cmd("SET")
                .arg(&lock_key)
                .arg(&lock_id)
                .arg("EX")
                .arg(ttl)
                .arg("NX")
                .query_async(&mut conn)
                .await
                .unwrap_or(false);

            if !acquired {
                return Ok(Response::new(ReserveInventoryResponse {
                    success: false,
                    lock_id: "".to_string(),
                    error_message: "Item is currently being checked out by another customer".to_string(),
                }));
            }

            // Verify capacity AFTER acquiring the lock within a transaction
            let pool = crate::db::get_pool();
            if let Ok(mut tx) = pool.begin().await {
                if let Ok(_) = ::server_common::auth_utils::set_org_context(&mut *tx, &req.tenant_id).await {
                    let current_stock: Option<i32> = sqlx::query_scalar("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2")
                        .bind(&req.product_id)
                        .bind(&req.tenant_id)
                        .fetch_optional(&mut *tx)
                        .await
                        .unwrap_or(None);

                    if let Some(stock) = current_stock {
                        if stock < req.quantity {
                            let _ = tx.rollback().await;
                            let _: () = redis::cmd("DEL").arg(&lock_key).query_async(&mut conn).await.unwrap_or(());
                            return Ok(Response::new(ReserveInventoryResponse {
                                success: false,
                                lock_id: "".to_string(),
                                error_message: format!("Insufficient inventory. Available: {}", stock),
                            }));
                        }
                    } else {
                        let _ = tx.rollback().await;
                        let _: () = redis::cmd("DEL").arg(&lock_key).query_async(&mut conn).await.unwrap_or(());
                        return Ok(Response::new(ReserveInventoryResponse {
                            success: false,
                            lock_id: "".to_string(),
                            error_message: "Product not found".to_string(),
                        }));
                    }
                }
                let _ = tx.commit().await;
            }
        }

        Ok(Response::new(ReserveInventoryResponse {
            success: true,
            lock_id,
            error_message: "".to_string(),
        }))
    }

    async fn commit_inventory(
        &self,
        request: Request<CommitInventoryRequest>,
    ) -> Result<Response<CommitInventoryResponse>, Status> {
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

        let lock_key = format!("ohc:lock:{}:inventory:{}", req.tenant_id, req.product_id);

        if let Some(client) = &self.redis_client {
            let mut conn = client.get_multiplexed_async_connection().await
                .map_err(|e| Status::internal(format!("Redis conn failed: {}", e)))?;

            let current_lock_id: Option<String> = redis::cmd("GET")
                .arg(&lock_key)
                .query_async(&mut conn)
                .await
                .unwrap_or(None);

            if let Some(cid) = current_lock_id {
                if cid != req.lock_id && !req.lock_id.is_empty() {
                    return Ok(Response::new(CommitInventoryResponse {
                        success: false,
                        error_message: "Lock ID mismatch. Reservation may have expired.".to_string(),
                    }));
                }
            }

            let _: () = redis::cmd("DEL")
                .arg(&lock_key)
                .query_async(&mut conn)
                .await
                .unwrap_or(());
        }

        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let update_result: Option<i32> = sqlx::query_scalar("UPDATE products SET inventory_count = inventory_count - $1 WHERE id = $2 AND tenant_id = $3 AND inventory_count >= $1 RETURNING inventory_count")
            .bind(req.quantity)
            .bind(&req.product_id)
            .bind(&tenant_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(new_stock) = update_result {

            let event_id = Uuid::new_v4().to_string();
            let event_payload = serde_json::json!({
                "product_id": req.product_id,
                "quantity_deducted": req.quantity,
                "remaining_stock": new_stock
            }).to_string();

            let _ = sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status) VALUES ($1, $2, 'operations', 'InventoryUpdated', $3::jsonb, 'PENDING')")
                .bind(event_id)
                .bind(&tenant_id)
                .bind(&event_payload)
                .execute(&mut *tx)
                .await;

            let payload_str = serde_json::json!({
                "product_id": req.product_id,
                "quantity_deducted": req.quantity,
                "remaining_stock": new_stock,
                "lock_id": req.lock_id,
            }).to_string();

            sqlx::query("INSERT INTO ohc_universal_ledger (id, tenant_id, department, action_type, state_change) VALUES ($1, $2, 'Operations', 'INVENTORY_DEDUCTION', $3::jsonb)")
                .bind(Uuid::new_v4().to_string())
                .bind(&tenant_id)
                .bind(&payload_str)
                .execute(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            if new_stock <= 5 {
                let job_id = Uuid::new_v4().to_string();
                let job_payload = serde_json::json!({
                    "product_id": req.product_id,
                    "remaining_stock": new_stock,
                    "threshold": 5,
                    "message": format!("Stock for product {} has dropped to {}.", req.product_id, new_stock)
                }).to_string();

                sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status) VALUES ($1, $2, 'operations', 'LowStockAlert', $3::jsonb, 'PENDING')")
                    .bind(job_id)
                    .bind(&tenant_id)
                    .bind(&job_payload)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;

                let action_request_id = Uuid::new_v4().to_string();
                let action_payload = serde_json::json!({
                    "product_id": req.product_id,
                    "remaining_stock": new_stock,
                    "suggested_action": "Restock Item"
                }).to_string();
                sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, action_type, status, confidence_score, product_id, payload, created_at, updated_at) VALUES ($1, $2, 'Reorder', 'Pending', 0.95, $3, $4::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                    .bind(&action_request_id)
                    .bind(&tenant_id)
                    .bind(&req.product_id)
                    .bind(&action_payload)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
            }
        } else {
            let current_stock: Option<i32> = sqlx::query_scalar("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2")
                .bind(&req.product_id)
                .bind(&tenant_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            let _ = tx.rollback().await;

            if let Some(stock) = current_stock {
                return Ok(Response::new(CommitInventoryResponse {
                    success: false,
                    error_message: format!("Insufficient inventory. Available: {}", stock),
                }));
            } else {
                return Ok(Response::new(CommitInventoryResponse {
                    success: false,
                    error_message: "Product not found".to_string(),
                }));
            }
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CommitInventoryResponse {
            success: true,
            error_message: "".to_string(),
        }))
    }
}
