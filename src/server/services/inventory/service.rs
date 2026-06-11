
use uuid::Uuid;

pub struct InventoryService {

    redis_client: Option<redis::Client>,
}

#[derive(Debug)]
pub struct ReserveResult {
    pub success: bool,
    pub lock_id: String,
    pub error_message: String,
}

#[derive(Debug)]
pub struct ReleaseResult {
    pub success: bool,
    pub error_message: String,
}

#[derive(Debug)]
pub struct CommitResult {
    pub success: bool,
    pub error_message: String,
}

impl InventoryService {
    pub fn new( redis_client: Option<redis::Client>) -> Self {
        Self { redis_client }
    }

    pub async fn reserve_inventory(
        &self,
        tenant_id: &str,
        product_id: &str,
        quantity: i32,
        ttl_seconds: i32,
    ) -> Result<ReserveResult, String> {
        let lock_id = Uuid::new_v4().to_string();
        let lock_key = format!("ohc:lock:{}:inventory:{}", tenant_id, product_id);

        if let Some(client) = &self.redis_client {
            let mut conn = client.get_multiplexed_async_connection().await
                .map_err(|e| format!("Redis conn failed: {}", e))?;

            let ttl = if ttl_seconds > 0 { ttl_seconds } else { 15 };

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
                let pool = crate::db::get_pool();
                let action_request_id = Uuid::new_v4().to_string();
                let payload = serde_json::json!({
                    "product_id": product_id,
                    "suggested_action": "Restock Item",
                    "reason": "Lock contention on limited item"
                }).to_string();

                let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, action_type, status, confidence_score, product_id, payload, created_at, updated_at) VALUES ($1, $2, 'Reorder', 'Pending', 0.95, $3, $4::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                    .bind(&action_request_id)
                    .bind(tenant_id)
                    .bind(product_id)
                    .bind(&payload)
                    .execute(&pool)
                    .await;

                return Ok(ReserveResult {
                    success: false,
                    lock_id: "".to_string(),
                    error_message: "Item is currently being checked out by another customer".to_string(),
                });
            }

            let pool = crate::db::get_pool();
            if let Ok(mut tx) = pool.begin().await {
                if let Ok(_) = crate::common::auth_utils::set_org_context(&mut *tx, tenant_id).await {
                    let current_stock: Option<i32> = sqlx::query_scalar("SELECT available_quantity FROM products WHERE id = $1 AND tenant_id = $2")
                        .bind(product_id)
                        .bind(tenant_id)
                        .fetch_optional(&mut *tx)
                        .await
                        .unwrap_or(None);

                    if let Some(stock) = current_stock {
                        if stock < quantity {
                            let _ = tx.rollback().await;
                            let _: () = redis::cmd("DEL").arg(&lock_key).query_async(&mut conn).await.unwrap_or(());
                            return Ok(ReserveResult {
                                success: false,
                                lock_id: "".to_string(),
                                error_message: format!("Insufficient inventory. Available: {}", stock)
                            });
                        } else {
                            let _ = sqlx::query("UPDATE products SET locked_quantity = locked_quantity + $1, available_quantity = available_quantity - $1 WHERE id = $2 AND tenant_id = $3")
                                .bind(quantity)
                                .bind(product_id)
                                .bind(tenant_id)
                                .execute(&mut *tx)
                                .await;
                        }
                    } else {
                        let fallback_stock: Option<i32> = sqlx::query_scalar("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2")
                            .bind(product_id)
                            .bind(tenant_id)
                            .fetch_optional(&mut *tx)
                            .await
                            .unwrap_or(None);

                        if let Some(f_stock) = fallback_stock {
                            if f_stock < quantity {
                                let _ = tx.rollback().await;
                                let _: () = redis::cmd("DEL").arg(&lock_key).query_async(&mut conn).await.unwrap_or(());
                                return Ok(ReserveResult {
                                    success: false,
                                    lock_id: "".to_string(),
                                    error_message: format!("Insufficient inventory. Available: {}", f_stock)
                                });
                            } else {
                                let _ = sqlx::query("UPDATE products SET locked_quantity = $1, available_quantity = inventory_count - $1 WHERE id = $2 AND tenant_id = $3")
                                    .bind(quantity)
                                    .bind(product_id)
                                    .bind(tenant_id)
                                    .execute(&mut *tx)
                                    .await;
                            }
                        } else {
                            let _ = tx.rollback().await;
                            let _: () = redis::cmd("DEL").arg(&lock_key).query_async(&mut conn).await.unwrap_or(());
                            return Ok(ReserveResult {
                                success: false,
                                lock_id: "".to_string(),
                                error_message: "Product not found".to_string()
                            });
                        }
                    }
                    let _ = tx.commit().await;
                } else {
                    let _: () = redis::cmd("DEL").arg(&lock_key).query_async(&mut conn).await.unwrap_or(());
                }
            } else {
                let _: () = redis::cmd("DEL").arg(&lock_key).query_async(&mut conn).await.unwrap_or(());
            }
        }

        Ok(ReserveResult {
            success: true,
            lock_id,
            error_message: "".to_string(),
        })
    }

    pub async fn release_inventory(
        &self,
        tenant_id: &str,
        product_id: &str,
        quantity: i32,
        lock_id: &str,
    ) -> Result<ReleaseResult, String> {
        let lock_key = format!("ohc:lock:{}:inventory:{}", tenant_id, product_id);

        if let Some(client) = &self.redis_client {
            let mut conn = client.get_multiplexed_async_connection().await
                .map_err(|e| format!("Redis conn failed: {}", e))?;

            let current_lock_id: Option<String> = redis::cmd("GET")
                .arg(&lock_key)
                .query_async(&mut conn)
                .await
                .unwrap_or(None);

            if let Some(cid) = current_lock_id {
                if cid != lock_id && !lock_id.is_empty() {
                    return Ok(ReleaseResult {
                        success: false,
                        error_message: "Lock ID mismatch. Reservation may have expired.".to_string(),
                    });
                }
            }

            let pool = crate::db::get_pool();
            if let Ok(mut tx) = pool.begin().await {
                if let Ok(_) = crate::common::auth_utils::set_org_context(&mut *tx, tenant_id).await {
                    let _ = sqlx::query("UPDATE products SET locked_quantity = locked_quantity - $1, available_quantity = available_quantity + $1 WHERE id = $2 AND tenant_id = $3")
                        .bind(quantity)
                        .bind(product_id)
                        .bind(tenant_id)
                        .execute(&mut *tx)
                        .await;
                    let _ = tx.commit().await;
                }
            }

            let _: () = redis::cmd("DEL")
                .arg(&lock_key)
                .query_async(&mut conn)
                .await
                .unwrap_or(());
        }

        Ok(ReleaseResult {
            success: true,
            error_message: "".to_string(),
        })
    }

    pub async fn commit_inventory(
        &self,
        tenant_id: &str,
        product_id: &str,
        quantity: i32,
        lock_id: &str,
    ) -> Result<CommitResult, String> {
        let lock_key = format!("ohc:lock:{}:inventory:{}", tenant_id, product_id);

        if let Some(client) = &self.redis_client {
            let mut conn = client.get_multiplexed_async_connection().await
                .map_err(|e| format!("Redis conn failed: {}", e))?;

            let current_lock_id: Option<String> = redis::cmd("GET")
                .arg(&lock_key)
                .query_async(&mut conn)
                .await
                .unwrap_or(None);

            if let Some(cid) = current_lock_id {
                if cid != lock_id && !lock_id.is_empty() {
                    return Ok(CommitResult {
                        success: false,
                        error_message: "Lock ID mismatch. Reservation may have expired.".to_string(),
                    });
                }
            }

            let _: () = redis::cmd("DEL")
                .arg(&lock_key)
                .query_async(&mut conn)
                .await
                .unwrap_or(());
        }

        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        crate::common::auth_utils::set_org_context(&mut *tx, tenant_id)
            .await
            .map_err(|e| e.to_string())?;

        let update_result: Option<i32> = sqlx::query_scalar("UPDATE products SET inventory_count = inventory_count - $1, locked_quantity = locked_quantity - $1 WHERE id = $2 AND tenant_id = $3 AND inventory_count >= $1 RETURNING inventory_count")
            .bind(quantity)
            .bind(product_id)
            .bind(tenant_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(new_stock) = update_result {
            let event_id = Uuid::new_v4().to_string();
            let event_payload = serde_json::json!({
                "product_id": product_id,
                "quantity_deducted": quantity,
                "remaining_stock": new_stock
            }).to_string();

            let _ = sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status) VALUES ($1, $2, 'operations', 'InventoryUpdated', $3::jsonb, 'PENDING')")
                .bind(event_id)
                .bind(tenant_id)
                .bind(&event_payload)
                .execute(&mut *tx)
                .await;

            let payload_str = serde_json::json!({
                "product_id": product_id,
                "quantity_deducted": quantity,
                "remaining_stock": new_stock,
                "lock_id": lock_id,
            }).to_string();

            let _ = sqlx::query("INSERT INTO ohc_universal_ledger (id, tenant_id, department, action_type, state_change) VALUES ($1, $2, 'Operations', 'INVENTORY_DEDUCTION', $3::jsonb)")
                .bind(Uuid::new_v4().to_string())
                .bind(tenant_id)
                .bind(&payload_str)
                .execute(&mut *tx)
                .await;

            if new_stock <= 5 {
                let job_id = Uuid::new_v4().to_string();
                let job_payload = serde_json::json!({
                    "product_id": product_id,
                    "remaining_stock": new_stock,
                    "threshold": 5,
                    "message": format!("Stock for product {} has dropped to {}.", product_id, new_stock)
                }).to_string();

                let _ = sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status) VALUES ($1, $2, 'operations', 'LowStockAlert', $3::jsonb, 'PENDING')")
                    .bind(job_id)
                    .bind(tenant_id)
                    .bind(&job_payload)
                    .execute(&mut *tx)
                    .await;

                let action_request_id = Uuid::new_v4().to_string();
                let action_payload = serde_json::json!({
                    "product_id": product_id,
                    "remaining_stock": new_stock,
                    "suggested_action": "Restock Item"
                }).to_string();
                let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, action_type, status, confidence_score, product_id, payload, created_at, updated_at) VALUES ($1, $2, 'Reorder', 'Pending', 0.95, $3, $4::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                    .bind(&action_request_id)
                    .bind(tenant_id)
                    .bind(product_id)
                    .bind(&action_payload)
                    .execute(&mut *tx)
                    .await;
            }
        } else {
            let current_stock: Option<i32> = sqlx::query_scalar("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2")
                .bind(product_id)
                .bind(tenant_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

            let _ = tx.rollback().await;

            if let Some(stock) = current_stock {
                return Ok(CommitResult {
                    success: false,
                    error_message: format!("Insufficient inventory. Available: {}", stock),
                });
            } else {
                return Ok(CommitResult {
                    success: false,
                    error_message: "Product not found".to_string(),
                });
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(CommitResult {
            success: true,
            error_message: "".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbStore;

    #[tokio::test]
    async fn test_reserve_inventory_concurrent_redlock() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let pool = crate::db::get_pool();
        let _db = Arc::new(crate::db::DB {
            pool: pool.clone(),
            store: DbStore::Postgres,
        });

        let tenant_id = "test_inventory_tenant";
        let product_id = "test_product_concurrent";

        let _ = sqlx::query("INSERT INTO products (id, tenant_id, name, inventory_count, available_quantity) VALUES ($1, $2, 'Test Product', 10, 10) ON CONFLICT DO NOTHING")
            .bind(product_id)
            .bind(tenant_id)
            .execute(&pool)
            .await;

        let _ = sqlx::query("UPDATE products SET inventory_count = 10, available_quantity = 10, locked_quantity = 0 WHERE id = $1 AND tenant_id = $2")
            .bind(product_id)
            .bind(tenant_id)
            .execute(&pool)
            .await;

        let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        let redis_client_opt = redis::Client::open(redis_url).ok();

        let service = Arc::new(InventoryService::new( redis_client_opt));

        let svc1 = service.clone();
        let svc2 = service.clone();

        let handle1 = tokio::spawn(async move {
            svc1.reserve_inventory(tenant_id, product_id, 1, 5).await
        });

        let handle2 = tokio::spawn(async move {
            svc2.reserve_inventory(tenant_id, product_id, 1, 5).await
        });

        let res1 = handle1.await.unwrap().unwrap();
        let res2 = handle2.await.unwrap().unwrap();

        let success_count = (if res1.success { 1 } else { 0 }) + (if res2.success { 1 } else { 0 });

        if std::env::var("OHC_REDIS_URL").is_ok() {
            assert_eq!(success_count, 1, "Only one concurrent request should acquire the lock");
            let failed_res = if res1.success { res2 } else { res1 };
            assert_eq!(failed_res.error_message, "Item is currently being checked out by another customer");
        }
    }
}
