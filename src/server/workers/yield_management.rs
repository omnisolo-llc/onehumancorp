use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::domain::repository::models::Product;

pub struct UniversalCapacityLedger {
    db: Arc<DB>,
}

impl UniversalCapacityLedger {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    /// Finds products with inventory that are at risk of spoilage based on metadata 'expires_at'.
    pub async fn find_at_risk_inventory(&self) -> Result<Vec<Product>, String> {
        let mut results = Vec::new();

        match &self.db.store {
            DbStore::Postgres => {
                let fetch_res = sqlx::query_as::<_, Product>(
                    "SELECT id, tenant_id, type, title, name, description, price, price_cents, currency, in_stock, inventory_count, is_sold_out, metadata, created_at, updated_at
                     FROM products
                     WHERE inventory_count > 0
                     AND metadata->>'expires_at' IS NOT NULL
                     AND (metadata->>'expires_at')::timestamptz <= NOW() + INTERVAL '2 hours'
                     AND (metadata->>'expires_at')::timestamptz > NOW()"
                )
                .fetch_all(&self.db.pool)
                .await;

                if let Ok(rows) = fetch_res {
                    results = rows;
                }
            }
            DbStore::Sqlite(pool) => {
                let fetch_res = sqlx::query_as::<_, Product>(
                    "SELECT id, tenant_id, type, title, name, description, price, price_cents, currency, in_stock, inventory_count, is_sold_out, metadata, created_at, updated_at
                     FROM products
                     WHERE inventory_count > 0
                     AND json_extract(metadata, '$.expires_at') IS NOT NULL
                     AND datetime(json_extract(metadata, '$.expires_at')) <= datetime('now', '+2 hours')
                     AND datetime(json_extract(metadata, '$.expires_at')) > datetime('now')"
                )
                .fetch_all(pool)
                .await;

                if let Ok(rows) = fetch_res {
                    results = rows;
                }
            }
        }

        Ok(results)
    }
}
