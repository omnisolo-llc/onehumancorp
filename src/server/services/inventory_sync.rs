use ::server_ohc::app::inventory_sync_service_server::InventorySyncService;
use ::server_ohc::app::{ReserveInventoryRequest, ReserveInventoryResponse, CommitInventoryRequest, CommitInventoryResponse};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct MyInventorySyncService {
    db: Arc<crate::db::DB>,
    redis_client: Option<redis::Client>,
}

impl MyInventorySyncService {
    pub fn new(db: Arc<crate::db::DB>, redis_client: Option<redis::Client>) -> Self {
        Self { db, redis_client }
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

        let current_stock = sqlx::query("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2 FOR UPDATE")
            .bind(&req.product_id)
            .bind(&tenant_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(row) = current_stock {
            let stock: i32 = sqlx::Row::get(&row, "inventory_count");

            if stock < req.quantity {
                let _ = tx.rollback().await;
                return Ok(Response::new(CommitInventoryResponse {
                    success: false,
                    error_message: format!("Insufficient inventory. Available: {}", stock),
                }));
            }

            let new_stock = stock - req.quantity;

            sqlx::query("UPDATE products SET inventory_count = $1 WHERE id = $2 AND tenant_id = $3")
                .bind(new_stock)
                .bind(&req.product_id)
                .bind(&tenant_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

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
            }
        } else {
            let _ = tx.rollback().await;
            return Ok(Response::new(CommitInventoryResponse {
                success: false,
                error_message: "Product not found".to_string(),
            }));
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CommitInventoryResponse {
            success: true,
            error_message: "".to_string(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::server_ohc::app::{ReserveInventoryRequest, CommitInventoryRequest};
    use tonic::Request;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_inventory_sync_service() {
        let database_url = std::env::var("OHC_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://ohc:ohc@localhost:5432/ohc".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(&database_url)
            .await
            .unwrap();

        let db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let service = MyInventorySyncService::new(db, None);

        let tenant_id = "test-tenant-123";
        let product_id = "test-product-123";

        // Setup test data
        sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
            .bind(tenant_id)
            .bind("Test Tenant")
            .bind("free")
            .execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING")
            .bind(product_id)
            .bind(tenant_id)
            .bind("Test Product")
            .bind(10)
            .execute(&pool).await.unwrap();

        // Test ReserveInventory
        let mut reserve_req = Request::new(ReserveInventoryRequest {
            product_id: product_id.to_string(),
            ttl_seconds: 60,
            tenant_id: tenant_id.to_string(),
        });
        reserve_req.metadata_mut().insert("x-spiffe-id", "spiffe://example.org/test".parse().unwrap());

        let reserve_res = service.reserve_inventory(reserve_req).await.unwrap().into_inner();
        assert!(reserve_res.success);
        let lock_id = reserve_res.lock_id;

        // Test CommitInventory
        let mut commit_req = Request::new(CommitInventoryRequest {
            product_id: product_id.to_string(),
            lock_id: lock_id.clone(),
            quantity: 2,
            tenant_id: tenant_id.to_string(),
        });
        commit_req.metadata_mut().insert("x-spiffe-id", "spiffe://example.org/test".parse().unwrap());

        let commit_res = service.commit_inventory(commit_req).await.unwrap().into_inner();
        assert!(commit_res.success);

        // Verify inventory count
        let count: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = $1")
            .bind(product_id)
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 8);

        // Test CommitInventory with insufficient stock
        let mut commit_req_fail = Request::new(CommitInventoryRequest {
            product_id: product_id.to_string(),
            lock_id: lock_id.clone(),
            quantity: 10,
            tenant_id: tenant_id.to_string(),
        });
        commit_req_fail.metadata_mut().insert("x-spiffe-id", "spiffe://example.org/test".parse().unwrap());

        let commit_res_fail = service.commit_inventory(commit_req_fail).await.unwrap().into_inner();
        assert!(!commit_res_fail.success);
        assert!(commit_res_fail.error_message.contains("Insufficient inventory"));

        // Cleanup
        sqlx::query("DELETE FROM products WHERE id = $1").bind(product_id).execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM tenants WHERE id = $1").bind(tenant_id).execute(&pool).await.unwrap();
    }
}
