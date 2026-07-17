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

            // 3. Check if material went below threshold and queue draft_purchase_order if needed
            let current_mat: Option<RawMaterial> = match &self.db.store {
                DbStore::Postgres => {
                    sqlx::query_as("SELECT * FROM raw_materials WHERE id = $1 AND tenant_id = $2")
                        .bind(&bom.raw_material_id)
                        .bind(tenant_id)
                        .fetch_optional(&self.db.pool)
                        .await
                        .map_err(|e| e.to_string())?
                },
                DbStore::Sqlite(pool) => {
                    sqlx::query_as("SELECT * FROM raw_materials WHERE id = ? AND tenant_id = ?")
                        .bind(&bom.raw_material_id)
                        .bind(tenant_id)
                        .fetch_optional(pool)
                        .await
                        .map_err(|e| e.to_string())?
                }
            };

            if let Some(mat) = current_mat {
                if let (Some(current_qty), Some(threshold)) = (mat.current_quantity, mat.reorder_threshold) {
                    if current_qty <= threshold {
                        // Queue job if not recently queued
                        let job_id = Uuid::new_v4().to_string();
                        let suggested_quantity = threshold * 2; // e.g. order double the threshold

                        let payload = serde_json::json!({
                            "raw_material_id": mat.id,
                            "raw_material_name": mat.name,
                            "suggested_quantity": suggested_quantity
                        });

                        // We do a simple insert. In a real system, you might want to avoid duplicate pending jobs
                        match &self.db.store {
                            DbStore::Postgres => {
                                let _ = sqlx::query(
                                    "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status)
                                     SELECT $1, $2, 'draft_purchase_order', $3, 'PENDING'
                                     WHERE NOT EXISTS (
                                         SELECT 1 FROM ohc_job_queue
                                         WHERE tenant_id = $2 AND job_type = 'draft_purchase_order'
                                         AND status = 'PENDING' AND payload->>'raw_material_id' = $4
                                     )"
                                )
                                .bind(&job_id)
                                .bind(tenant_id)
                                .bind(&payload)
                                .bind(&mat.id)
                                .execute(&self.db.pool)
                                .await;
                            },
                            DbStore::Sqlite(pool) => {
                                let _ = sqlx::query(
                                    "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status)
                                     SELECT ?, ?, 'draft_purchase_order', ?, 'PENDING'
                                     WHERE NOT EXISTS (
                                         SELECT 1 FROM ohc_job_queue
                                         WHERE tenant_id = ? AND job_type = 'draft_purchase_order'
                                         AND status = 'PENDING' AND json_extract(payload, '$.raw_material_id') = ?
                                     )"
                                )
                                .bind(&job_id)
                                .bind(tenant_id)
                                .bind(&payload)
                                .bind(tenant_id)
                                .bind(&mat.id)
                                .execute(pool)
                                .await;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn get_low_stock_materials(&self, tenant_id: &str) -> Result<Vec<RawMaterial>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as("SELECT * FROM raw_materials WHERE tenant_id = $1 AND current_quantity <= reorder_threshold")
                    .bind(tenant_id)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())
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
