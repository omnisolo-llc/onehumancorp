use std::sync::Arc;
use uuid::Uuid;


use dashmap::DashMap;
use std::time::{Instant, Duration};



#[async_trait::async_trait]
pub trait InventoryLocker: Send + Sync {
    async fn acquire(&self, lock_key: &str, lock_id: &str, ttl: i32) -> bool;
    async fn release(&self, lock_key: &str, expected_lock_id: &str) -> bool;
    async fn get_lock_id(&self, lock_key: &str) -> Option<String>;
    async fn clear(&self, lock_key: &str);
}

pub struct MemoryLocker {
    locks: Arc<DashMap<String, (String, Instant)>>,
}

impl MemoryLocker {
    pub fn new() -> Self {
        Self {
            locks: Arc::new(DashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl InventoryLocker for MemoryLocker {
    async fn acquire(&self, lock_key: &str, lock_id: &str, ttl: i32) -> bool {
        let now = Instant::now();
        self.locks.retain(|_, (_, expires_at)| *expires_at > now);
        if !self.locks.contains_key(lock_key) {
            self.locks.insert(lock_key.to_string(), (lock_id.to_string(), now + Duration::from_secs(ttl as u64)));
            true
        } else {
            false
        }
    }

    async fn release(&self, lock_key: &str, expected_lock_id: &str) -> bool {
        let now = Instant::now();
        self.locks.retain(|_, (_, expires_at)| *expires_at > now);
        if let Some(v) = self.locks.get(lock_key) {
            if v.0 == expected_lock_id {
                self.locks.remove(lock_key);
                return true;
            }
        }
        false
    }

    async fn get_lock_id(&self, lock_key: &str) -> Option<String> {
        let now = Instant::now();
        self.locks.retain(|_, (_, expires_at)| *expires_at > now);
        self.locks.get(lock_key).map(|v| v.0.clone())
    }

    async fn clear(&self, lock_key: &str) {
        self.locks.remove(lock_key);
    }
}

pub struct RedisLocker {
    client: redis::Client,
}

impl RedisLocker {
    pub fn new(client: redis::Client) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl InventoryLocker for RedisLocker {
    async fn acquire(&self, lock_key: &str, lock_id: &str, ttl: i32) -> bool {
        if let Ok(mut conn) = self.client.get_multiplexed_async_connection().await {
            redis::cmd("SET")
                .arg(lock_key)
                .arg(lock_id)
                .arg("EX")
                .arg(ttl)
                .arg("NX")
                .query_async(&mut conn)
                .await
                .unwrap_or(false)
        } else {
            false
        }
    }

    async fn release(&self, lock_key: &str, expected_lock_id: &str) -> bool {
        if let Ok(mut conn) = self.client.get_multiplexed_async_connection().await {
            let script = redis::Script::new(
                r#"
                if redis.call("get", KEYS[1]) == ARGV[1] then
                    return redis.call("del", KEYS[1])
                else
                    return 0
                end
                "#,
            );
            script.key(lock_key).arg(expected_lock_id).invoke_async(&mut conn).await.unwrap_or(false)
        } else {
            false
        }
    }

    async fn get_lock_id(&self, lock_key: &str) -> Option<String> {
        if let Ok(mut conn) = self.client.get_multiplexed_async_connection().await {
            redis::cmd("GET").arg(lock_key).query_async(&mut conn).await.ok()
        } else {
            None
        }
    }

    async fn clear(&self, lock_key: &str) {
        if let Ok(mut conn) = self.client.get_multiplexed_async_connection().await {
            let _: () = redis::cmd("DEL").arg(lock_key).query_async(&mut conn).await.unwrap_or(());
        }
    }
}

pub struct InventoryService {
    locker: Box<dyn InventoryLocker>,
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
    pub fn new(redis_client: Option<redis::Client>) -> Self {
        let locker: Box<dyn InventoryLocker> = if let Some(ref client) = redis_client {
            Box::new(RedisLocker::new(client.clone()))
        } else {
            Box::new(MemoryLocker::new())
        };
        Self { locker, redis_client }
    }

    // Redis Redlock pattern for distributed lock
    fn get_lock_key(tenant_id: &str, product_id: &str) -> String {
        format!("ohc:lock:{}:inventory:{}", tenant_id, product_id)
    }

    pub async fn reserve_inventory(
        &self,
        tenant_id: &str,
        product_id: &str,
        quantity: i32,
        ttl_seconds: i32,
    ) -> Result<ReserveResult, String> {
        let lock_id = Uuid::new_v4().to_string();
        let lock_key = Self::get_lock_key(tenant_id, product_id);

        let ttl = if ttl_seconds > 0 { ttl_seconds } else { 15 }; // Distributed lock TTL

        let acquired = self.locker.acquire(&lock_key, &lock_id, ttl).await;

        if !acquired {
            let pool = crate::db::get_pool();
                let action_request_id = Uuid::new_v4().to_string();
                let payload = serde_json::json!({
                    "product_id": product_id,
                    "suggested_action": "Restock Item",
                    "reason": "Lock contention on limited item"
                }).to_string();

                let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, action_type, status, confidence_score, product_id, payload, source, agent_type, created_at, updated_at) VALUES ($1, $2, 'Reorder', 'Pending', 0.95, $3, $4::jsonb, 'inventory_service', 'operations', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                    .bind(&action_request_id)
                    .bind(tenant_id)
                    .bind(product_id)
                    .bind(&payload)
                    .execute(&pool)
                    .await;

                let cs_action_request_id = Uuid::new_v4().to_string();
                let cs_payload = serde_json::json!({
                    "product_id": product_id,
                    "suggested_action": "Notify Customer of Out of Stock",
                    "reason": "Lock contention on limited item during checkout"
                }).to_string();

                let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, action_type, status, confidence_score, product_id, payload, source, agent_type, created_at, updated_at) VALUES ($1, $2, 'NotifyCustomer', 'Pending', 0.99, $3, $4::jsonb, 'inventory_service', 'customer_success', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                    .bind(&cs_action_request_id)
                    .bind(tenant_id)
                    .bind(product_id)
                    .bind(&cs_payload)
                    .execute(&pool)
                    .await;

                let product_title: String = sqlx::query_scalar("SELECT title FROM products WHERE id = $1 AND tenant_id = $2")
                    .bind(product_id)
                    .bind(tenant_id)
                    .fetch_optional(&pool)
                    .await
                    .unwrap_or(Some(product_id.to_string()))
                    .unwrap_or_else(|| product_id.to_string());

                // Operations Agent: trigger push notification for out-of-stock/lock failure
                let job_id = Uuid::new_v4().to_string();
                let message = format!("{} sold out. Would you like to draft a restock order?", product_title);
                let job_payload = serde_json::json!({
                    "product_id": product_id,
                    "product_title": product_title,
                    "remaining_stock": 0,
                    "threshold": 5,
                    "message": message
                }).to_string();

                let _ = sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status) VALUES ($1, $2, 'operations', 'LowStockAlert', $3::jsonb, 'PENDING')")
                    .bind(job_id)
                    .bind(tenant_id)
                    .bind(&job_payload)
                    .execute(&pool)
                    .await;

                return Ok(ReserveResult {
                    success: false,
                    lock_id: "".to_string(),
                    error_message: "Item is currently being checked out by another customer.".to_string(),
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
                            self.locker.clear(&lock_key).await;
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
                        let fallback_stock: Option<i32> = sqlx::query_scalar("SELECT available_quantity FROM products WHERE id = $1 AND tenant_id = $2")
                            .bind(product_id)
                            .bind(tenant_id)
                            .fetch_optional(&mut *tx)
                            .await
                            .unwrap_or(None);

                        if let Some(f_stock) = fallback_stock {
                            if f_stock < quantity {
                                let _ = tx.rollback().await;
                                self.locker.clear(&lock_key).await;
                                return Ok(ReserveResult {
                                    success: false,
                                    lock_id: "".to_string(),
                                    error_message: format!("Insufficient inventory. Available: {}", f_stock)
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
                            let _ = tx.rollback().await;
                            self.locker.clear(&lock_key).await;
                            return Ok(ReserveResult {
                                success: false,
                                lock_id: "".to_string(),
                                error_message: "Product not found".to_string()
                            });
                        }
                    }
                    let _ = tx.commit().await;                    // Publish to Redis Pub/Sub for Real-Time Sync
                    if let Some(client) = &self.redis_client {
                        if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                            let invalidation_topic = "cache_invalidation_events";
                            let invalidation_payload = serde_json::json!({
                                "event": "inventory.updated",
                                "tags": [
                                    format!("tenant-id:{}", tenant_id),
                                    format!("entity:product:{}", product_id)
                                ]
                            }).to_string();
                            let _: Result<(), _> = redis::cmd("PUBLISH").arg(invalidation_topic).arg(invalidation_payload).query_async(&mut conn).await;
                        }
                    }
                } else {
            self.locker.clear(&lock_key).await;
                    return Ok(ReserveResult {
                        success: false,
                        lock_id: "".to_string(),
                        error_message: "Failed to set org context".to_string()
                    });
                }
            } else {
                self.locker.clear(&lock_key).await;
                return Ok(ReserveResult {
                    success: false,
                    lock_id: "".to_string(),
                    error_message: "Database error".to_string()
                });
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
        let lock_key = Self::get_lock_key(tenant_id, product_id);

        let current_lock_id: Option<String> = self.locker.get_lock_id(&lock_key).await;

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

        self.locker.clear(&lock_key).await;

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
        let mut all_match = true;
        let mut valid_indices = Vec::new();
        let lock_id_base;

        let parts: Vec<&str> = lock_id.split(':').collect();
        if parts.len() == 2 {
            lock_id_base = parts[0];
            let indices: Vec<&str> = parts[1].split(',').collect();

            let mut get_lock_futures = Vec::new();
            for idx in &indices {
                let lock_key = format!("{}:{}", Self::get_lock_key(tenant_id, product_id), idx);
                get_lock_futures.push(async move {
                    self.locker.get_lock_id(&lock_key).await
                });
            }

            let current_lock_ids = futures::future::join_all(get_lock_futures).await;

            for (idx, current_lock_id) in indices.iter().zip(current_lock_ids.iter()) {
                if current_lock_id.as_deref() != Some(lock_id_base) {
                    all_match = false;
                    break;
                }
                valid_indices.push(*idx);
            }

            if !all_match {
                return Ok(CommitResult {
                    success: false,
                    error_message: "Lock ID mismatch. Reservation may have expired.".to_string(),
                });
            }

            let mut clear_lock_futures = Vec::new();
            for idx in &valid_indices {
                let lock_key = format!("{}:{}", Self::get_lock_key(tenant_id, product_id), idx);
                clear_lock_futures.push(async move {
                    self.locker.clear(&lock_key).await;
                });
            }
            futures::future::join_all(clear_lock_futures).await;
        } else {
            let lock_key = Self::get_lock_key(tenant_id, product_id);
            let current_lock_id = self.locker.get_lock_id(&lock_key).await;
            if current_lock_id != Some(lock_id.to_string()) && !lock_id.is_empty() {
                return Ok(CommitResult {
                    success: false,
                    error_message: "Lock ID mismatch. Reservation may have expired.".to_string(),
                });
            }
            self.locker.clear(&lock_key).await;
        }

        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        crate::common::auth_utils::set_org_context(&mut *tx, tenant_id)
            .await
            .map_err(|e| e.to_string())?;

        // Enforce row-level locking for final commit
        let _ = sqlx::query("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2 FOR UPDATE")
            .bind(product_id)
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let update_result: Option<i32> = sqlx::query_scalar("UPDATE products SET inventory_count = inventory_count - $1, locked_quantity = locked_quantity - $1 WHERE id = $2 AND tenant_id = $3 AND inventory_count >= $1 AND locked_quantity >= $1 RETURNING inventory_count")
            .bind(quantity)
            .bind(product_id)
            .bind(tenant_id)
            .fetch_optional(&mut *tx)
            .await
            .unwrap_or(None);

        let update_result = if update_result.is_none() {
            // Fallback: If not enough locked quantity, deduct from available quantity directly (e.g. offline POS sync or direct commit without reserve)
            sqlx::query_scalar("UPDATE products SET inventory_count = inventory_count - $1, available_quantity = available_quantity - $1 WHERE id = $2 AND tenant_id = $3 AND inventory_count >= $1 AND available_quantity >= $1 RETURNING inventory_count")
            .bind(quantity)
            .bind(product_id)
            .bind(tenant_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?
        } else {
            update_result
        };

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
                let product_title: String = sqlx::query_scalar("SELECT title FROM products WHERE id = $1 AND tenant_id = $2")
                    .bind(product_id)
                    .bind(tenant_id)
                    .fetch_optional(&mut *tx)
                    .await
                    .unwrap_or(Some(product_id.to_string()))
                    .unwrap_or_else(|| product_id.to_string());


                let job_id = Uuid::new_v4().to_string();

                let message = if new_stock == 0 {
                    format!("{} sold out. Would you like to draft a restock order?", product_title)
                } else {
                    format!("Stock for {} has dropped to {}.", product_title, new_stock)
                };

                let job_payload = serde_json::json!({
                    "product_id": product_id,
                    "product_title": product_title,
                    "remaining_stock": new_stock,
                    "threshold": 5,
                    "message": message
                }).to_string();

                let _ = sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status) VALUES ($1, $2, 'operations', 'LowStockAlert', $3::jsonb, 'PENDING')")
                    .bind(job_id)
                    .bind(tenant_id)
                    .bind(&job_payload)
                    .execute(&mut *tx)
                    .await;

                // Directly notify Operations Agent for real-time monitoring as per Step 3
                tracing::info!("Operations Agent Integration: stock level monitored: {} drops below threshold. Triggered LowStockAlert for Operations Agent.", product_id);

                let action_request_id = Uuid::new_v4().to_string();
                let action_payload = serde_json::json!({
                    "product_id": product_id,
                    "remaining_stock": new_stock,
                    "suggested_action": "Restock Item"
                }).to_string();
                let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, action_type, status, confidence_score, product_id, payload, source, agent_type, created_at, updated_at) VALUES ($1, $2, 'Reorder', 'Pending', 0.95, $3, $4::jsonb, 'inventory_service', 'operations', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                    .bind(&action_request_id)
                    .bind(tenant_id)
                    .bind(product_id)
                    .bind(&action_payload)
                    .execute(&mut *tx)
                    .await;

                let feed_id = Uuid::new_v4().to_string();
                let feed_payload = serde_json::json!({
                    "product_id": product_id,
                    "remaining_stock": new_stock,
                    "message": message,
                });
                let proposed_action = serde_json::json!({
                    "action": "Review and approve restock order"
                });
                let _ = sqlx::query(
                    "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state) VALUES ($1, $2, 'operations', $3::jsonb, $4::jsonb, 'PENDING_APPROVAL')"
                )
                .bind(&feed_id)
                .bind(tenant_id)
                .bind(&feed_payload)
                .bind(&proposed_action)
                .execute(&mut *tx)
                .await;
            }
        } else {
            let current_stock: Option<i32> = sqlx::query_scalar("SELECT available_quantity FROM products WHERE id = $1 AND tenant_id = $2")
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

        tx.commit().await.map_err(|e| e.to_string())?;        // Publish to Redis Pub/Sub for Real-Time Sync
        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                let invalidation_topic = "cache_invalidation_events";
                let invalidation_payload = serde_json::json!({
                    "event": "inventory.updated",
                    "tags": [
                        format!("tenant-id:{}", tenant_id),
                        format!("entity:product:{}", product_id)
                    ]
                }).to_string();
                let _: Result<(), _> = redis::cmd("PUBLISH").arg(invalidation_topic).arg(invalidation_payload).query_async(&mut conn).await;
            }
        }

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
    use std::sync::Arc;

    #[tokio::test]
    async fn test_commit_inventory_low_stock() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let pool = crate::db::get_pool();
        let _db = Arc::new(crate::db::DB {
            pool: pool.clone(),
            store: DbStore::Postgres,
        });

        let tenant_id = "test_inventory_tenant";
        let product_id = "test_product_low_stock";

        let _ = sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'Test Tenant') ON CONFLICT DO NOTHING")
            .bind(tenant_id)
            .execute(&pool)
            .await;

        let _ = sqlx::query("INSERT INTO products (id, tenant_id, name, inventory_count, available_quantity) VALUES ($1, $2, 'Test Product', 6, 6) ON CONFLICT DO NOTHING")
            .bind(product_id)
            .bind(tenant_id)
            .execute(&pool)
            .await;

        let _ = sqlx::query("UPDATE products SET inventory_count = 6, available_quantity = 6, locked_quantity = 0 WHERE id = $1 AND tenant_id = $2")
            .bind(product_id)
            .bind(tenant_id)
            .execute(&pool)
            .await;

        let redis_client_opt = None;
        let service = Arc::new(InventoryService::new(redis_client_opt));

        let res = service.commit_inventory(tenant_id, product_id, 2, "").await.unwrap();
        assert!(res.success);

        let feed_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = $1 AND context_payload->>'product_id' = $2")
            .bind(tenant_id)
            .bind(product_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert!(feed_count.0 > 0);
    }

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

        let _ = sqlx::query("INSERT INTO products (id, tenant_id, name, inventory_count, available_quantity) VALUES ($1, $2, 'Test Product', 1, 1) ON CONFLICT DO NOTHING")
            .bind(product_id)
            .bind(tenant_id)
            .execute(&pool)
            .await;

        let _ = sqlx::query("UPDATE products SET inventory_count = 1, available_quantity = 1, locked_quantity = 0 WHERE id = $1 AND tenant_id = $2")
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
            assert_eq!(failed_res.error_message, "Item is currently being checked out by another customer.");
        }
    }
}
