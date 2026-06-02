use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::{RawMaterial, BOMItem, Vendor, PurchaseOrder, POLineItem, DepletionLog};
use chrono::Utc;
use uuid::Uuid;

pub struct SupplyChainRepo {
    db: Arc<DB>,
}

impl SupplyChainRepo {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn process_depletion(&self, tenant_id: &str, product_id: &str, quantity_sold: i32, sales_event_id: &str) -> Result<(), String> {
        let now = Utc::now();

        // 1. Get BOM items for the finished good
        let bom_items: Vec<BOMItem> = match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as("SELECT * FROM bom_items WHERE tenant_id = $1 AND finished_good_id = $2")
                    .bind(tenant_id)
                    .bind(product_id)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?
            },
            DbStore::Sqlite(pool) => {
                sqlx::query_as("SELECT * FROM bom_items WHERE tenant_id = ? AND finished_good_id = ?")
                    .bind(tenant_id)
                    .bind(product_id)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?
            }
        };

        // 2. Deduct raw materials and create depletion logs
        for bom in bom_items {
            let qty_to_deduct = bom.quantity_required.unwrap_or(1) * quantity_sold;
            let log_id = Uuid::new_v4().to_string();

            match &self.db.store {
                DbStore::Postgres => {
                    let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                    sqlx::query("UPDATE raw_materials SET current_quantity = current_quantity - $1, updated_at = $2 WHERE id = $3 AND tenant_id = $4")
                        .bind(qty_to_deduct)
                        .bind(now)
                        .bind(&bom.raw_material_id)
                        .bind(tenant_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    sqlx::query("INSERT INTO depletion_logs (id, tenant_id, raw_material_id, sales_event_id, quantity_deducted, created_at) VALUES ($1, $2, $3, $4, $5, $6)")
                        .bind(&log_id)
                        .bind(tenant_id)
                        .bind(&bom.raw_material_id)
                        .bind(sales_event_id)
                        .bind(qty_to_deduct)
                        .bind(now)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                },
                DbStore::Sqlite(pool) => {
                    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

                    sqlx::query("UPDATE raw_materials SET current_quantity = current_quantity - ?, updated_at = ? WHERE id = ? AND tenant_id = ?")
                        .bind(qty_to_deduct)
                        .bind(now)
                        .bind(&bom.raw_material_id)
                        .bind(tenant_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    sqlx::query("INSERT INTO depletion_logs (id, tenant_id, raw_material_id, sales_event_id, quantity_deducted, created_at) VALUES (?, ?, ?, ?, ?, ?)")
                        .bind(&log_id)
                        .bind(tenant_id)
                        .bind(&bom.raw_material_id)
                        .bind(sales_event_id)
                        .bind(qty_to_deduct)
                        .bind(now)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                }
            }
        }

        Ok(())
    }

    pub async fn get_low_stock_materials(&self, tenant_id: &str) -> Result<Vec<RawMaterial>, String> {
        let thirty_days_ago = Utc::now() - chrono::Duration::days(30);
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as(
                    r#"
                    WITH velocity AS (
                        SELECT
                            dl.raw_material_id,
                            COALESCE(SUM(dl.quantity_deducted), 0) / 30.0 as daily_velocity
                        FROM depletion_logs dl
                        WHERE dl.tenant_id = $1 AND dl.created_at >= $2
                        GROUP BY dl.raw_material_id
                    )
                    SELECT rm.*, v.daily_velocity
                    FROM raw_materials rm
                    LEFT JOIN velocity v ON rm.id = v.raw_material_id
                    WHERE rm.tenant_id = $1
                      AND (
                          rm.current_quantity <= rm.reorder_threshold
                          OR (v.daily_velocity > 0 AND (rm.current_quantity / v.daily_velocity) <= COALESCE(rm.lead_time_days, 7))
                      )
                    "#
                )
                .bind(tenant_id)
                .bind(thirty_days_ago)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| e.to_string())
            },
            DbStore::Sqlite(pool) => {
                sqlx::query_as(
                    r#"
                    WITH velocity AS (
                        SELECT
                            dl.raw_material_id,
                            COALESCE(SUM(dl.quantity_deducted), 0) / 30.0 as daily_velocity
                        FROM depletion_logs dl
                        WHERE dl.tenant_id = ? AND dl.created_at >= ?
                        GROUP BY dl.raw_material_id
                    )
                    SELECT rm.*, v.daily_velocity
                    FROM raw_materials rm
                    LEFT JOIN velocity v ON rm.id = v.raw_material_id
                    WHERE rm.tenant_id = ?
                      AND (
                          rm.current_quantity <= rm.reorder_threshold
                          OR (v.daily_velocity > 0 AND (rm.current_quantity / v.daily_velocity) <= COALESCE(rm.lead_time_days, 7))
                      )
                    "#
                )
                .bind(tenant_id)
                .bind(thirty_days_ago.format("%Y-%m-%d %H:%M:%S").to_string())
                .bind(tenant_id)
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::time::Duration;
    use serde_json::json;

    async fn setup_test_db() -> Arc<DB> {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create memory db");

        sqlx::query(
            "CREATE TABLE tenants (id TEXT PRIMARY KEY, business_name TEXT, plan_tier TEXT)"
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE products (id TEXT PRIMARY KEY, tenant_id TEXT, title TEXT, type TEXT)"
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE vendors (id TEXT PRIMARY KEY, tenant_id TEXT, name TEXT, contact_info TEXT)"
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE raw_materials (id TEXT PRIMARY KEY, tenant_id TEXT, name TEXT, current_quantity INT, reorder_threshold INT, lead_time_days INT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)"
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE bom_items (id TEXT PRIMARY KEY, tenant_id TEXT, finished_good_id TEXT, raw_material_id TEXT, quantity_required INT)"
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE depletion_logs (id TEXT PRIMARY KEY, tenant_id TEXT, raw_material_id TEXT, sales_event_id TEXT, quantity_deducted INT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)"
        )
        .execute(&pool)
        .await
        .unwrap();

        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();

        Arc::new(DB {
            pool: dummy_pg_pool,
            store: DbStore::Sqlite(pool),
        })
    }

    #[tokio::test]
    async fn test_process_depletion() {
        let db = setup_test_db().await;
        let repo = SupplyChainRepo::new(db.clone());

        if let DbStore::Sqlite(pool) = &db.store {
            sqlx::query("INSERT INTO tenants (id, business_name, plan_tier) VALUES ('tenant1', 'Test Business', 'free')")
                .execute(pool).await.unwrap();

            sqlx::query("INSERT INTO raw_materials (id, tenant_id, name, current_quantity, reorder_threshold) VALUES ('mat1', 'tenant1', 'Cocoa', 100, 10)")
                .execute(pool).await.unwrap();

            sqlx::query("INSERT INTO products (id, tenant_id, title, type) VALUES ('prod1', 'tenant1', 'Cake', 'physical')")
                .execute(pool).await.unwrap();

            sqlx::query("INSERT INTO bom_items (id, tenant_id, finished_good_id, raw_material_id, quantity_required) VALUES ('bom1', 'tenant1', 'prod1', 'mat1', 2)")
                .execute(pool).await.unwrap();
        }

        repo.process_depletion("tenant1", "prod1", 5, "event1").await.unwrap();

        if let DbStore::Sqlite(pool) = &db.store {
            let row = sqlx::query("SELECT current_quantity FROM raw_materials WHERE id = 'mat1'")
                .fetch_one(pool).await.unwrap();
            use sqlx::Row;
            let qty: i32 = row.get("current_quantity");
            assert_eq!(qty, 90); // 100 - (2 * 5)
        }
    }

    #[tokio::test]
    async fn test_get_low_stock_materials_predictive() {
        let db = setup_test_db().await;
        let repo = SupplyChainRepo::new(db.clone());

        if let DbStore::Sqlite(pool) = &db.store {
            sqlx::query("INSERT INTO tenants (id, business_name, plan_tier) VALUES ('tenant1', 'Test Business', 'free')")
                .execute(pool).await.unwrap();

            // Product with normal inventory, but high velocity => low stock
            sqlx::query("INSERT INTO raw_materials (id, tenant_id, name, current_quantity, reorder_threshold, lead_time_days) VALUES ('mat1', 'tenant1', 'Cocoa', 50, 5, 7)")
                .execute(pool).await.unwrap();

            // Insert 300 unit depletion over last 30 days => velocity is 10/day
            // current_qty (50) / 10 = 5 days until stockout. 5 <= 7 lead_time_days => TRUE.
            let thirty_days_ago = Utc::now() - chrono::Duration::days(15);
            sqlx::query("INSERT INTO depletion_logs (id, tenant_id, raw_material_id, sales_event_id, quantity_deducted, created_at) VALUES ('log1', 'tenant1', 'mat1', 'event1', 300, ?)")
                .bind(thirty_days_ago.format("%Y-%m-%d %H:%M:%S").to_string())
                .execute(pool).await.unwrap();

            // Product with low absolute inventory => low stock
            sqlx::query("INSERT INTO raw_materials (id, tenant_id, name, current_quantity, reorder_threshold, lead_time_days) VALUES ('mat2', 'tenant1', 'Flour', 3, 5, 7)")
                .execute(pool).await.unwrap();

            // Product with high inventory and low velocity => NOT low stock
            sqlx::query("INSERT INTO raw_materials (id, tenant_id, name, current_quantity, reorder_threshold, lead_time_days) VALUES ('mat3', 'tenant1', 'Sugar', 500, 5, 7)")
                .execute(pool).await.unwrap();
            sqlx::query("INSERT INTO depletion_logs (id, tenant_id, raw_material_id, sales_event_id, quantity_deducted, created_at) VALUES ('log2', 'tenant1', 'mat3', 'event2', 30, ?)")
                .bind(thirty_days_ago.format("%Y-%m-%d %H:%M:%S").to_string())
                .execute(pool).await.unwrap(); // velocity 1/day, 500 days until empty
        }

        let low_stock = repo.get_low_stock_materials("tenant1").await.unwrap();
        assert_eq!(low_stock.len(), 2);

        let ids: Vec<_> = low_stock.iter().map(|m| m.id.clone()).collect();
        assert!(ids.contains(&"mat1".to_string()));
        assert!(ids.contains(&"mat2".to_string()));
    }
}
