use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;
use redis::{AsyncCommands, aio::MultiplexedConnection};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct InventoryLedgerService {
    pub db: PgPool,
    pub redis: Option<MultiplexedConnection>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InventoryUpdateJob {
    pub tenant_id: String,
    pub product_id: String,
    pub variant_id: Option<String>,
    pub delta: i32,
}

impl InventoryLedgerService {
    pub fn new(db: PgPool, redis: Option<MultiplexedConnection>) -> Self {
        Self { db, redis }
    }

    pub async fn get_inventory(&self, tenant_id: &str, product_id: &str) -> Result<i32, String> {
        // Try redis cache first
        let cache_key = format!("ohc:inventory:{}:{}", tenant_id, product_id);
        if let Some(mut redis_conn) = self.redis.clone() {
            let cached: redis::RedisResult<i32> = redis_conn.get(&cache_key).await;
            if let Ok(val) = cached {
                return Ok(val);
            }
        }

        // Fallback to db
        let row = sqlx::query("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2")
            .bind(product_id)
            .bind(tenant_id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        let count: i32 = match row {
            Some(r) => r.try_get("inventory_count").unwrap_or(0),
            None => 0,
        };

        // Populate cache
        if let Some(mut redis_conn) = self.redis.clone() {
            let _: redis::RedisResult<()> = redis_conn.set(&cache_key, count).await;
        }

        Ok(count)
    }

    pub async fn apply_optimistic_update(&self, tenant_id: &str, product_id: &str, variant_id: Option<String>, delta: i32) -> Result<i32, String> {
        let cache_key = format!("ohc:inventory:{}:{}", tenant_id, product_id);

        // Optimistically update Redis
        let mut new_val = 0;
        if let Some(mut redis_conn) = self.redis.clone() {
            let curr: redis::RedisResult<i32> = redis_conn.get(&cache_key).await;
            let mut val = curr.unwrap_or_else(|_| {
                // Not in cache, try DB (synchronous fallback for cache warming before optimistic increment)
                0
            });

            // Need to actually fetch if not in cache so delta applies correctly
            if val == 0 {
                 let row = sqlx::query("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2")
                    .bind(product_id)
                    .bind(tenant_id)
                    .fetch_optional(&self.db)
                    .await
                    .map_err(|e| e.to_string())?;
                 val = match row {
                    Some(r) => r.try_get("inventory_count").unwrap_or(0),
                    None => 0,
                 };
            }

            val += delta;
            if val < 0 {
                val = 0; // prevent negative inventory at edge
            }

            let _: redis::RedisResult<()> = redis_conn.set(&cache_key, val).await;
            new_val = val;
        }

        // Enqueue Job for processing
        let job = InventoryUpdateJob {
            tenant_id: tenant_id.to_string(),
            product_id: product_id.to_string(),
            variant_id,
            delta,
        };

        sqlx::query(
            "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'inventory_sync', $3, 'PENDING')"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(tenant_id)
        .bind(serde_json::to_value(&job).unwrap())
        .execute(&self.db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(new_val)
    }
}
