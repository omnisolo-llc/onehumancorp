use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::{RawMaterial, BOMItem, Vendor, PurchaseOrder, POLineItem, DepletionLog};
use chrono::Utc;
use uuid::Uuid;
use ::server_common::auth_utils::set_org_context;

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
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;
                let result = sqlx::query_as("SELECT * FROM bom_items WHERE tenant_id = $1 AND finished_good_id = $2")
                    .bind(tenant_id)
                    .bind(product_id)
                    .fetch_all(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
                result
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
                    set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

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
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;
                let result = sqlx::query_as("SELECT * FROM raw_materials WHERE tenant_id = $1 AND current_quantity <= reorder_threshold")
                    .bind(tenant_id)
                    .fetch_all(&mut *tx)
                    .await
                    .map_err(|e| e.to_string());
                let _ = tx.commit().await;
                result
            },
            DbStore::Sqlite(pool) => {
                sqlx::query_as("SELECT * FROM raw_materials WHERE tenant_id = ? AND current_quantity <= reorder_threshold")
                    .bind(tenant_id)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())
            }
        }
    }
}
