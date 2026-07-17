use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use chrono::Utc;
use uuid::Uuid;
use sqlx::Row;
use serde_json::json;

pub struct PredictiveSupplyChainWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl PredictiveSupplyChainWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(10),
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                if let Err(e) = Self::run_analysis(&db).await {
                    ::server_telemetry::record_error_signal("[bug] PredictiveSupplyChainWorker analysis error");
                    tracing::error!("PredictiveSupplyChainWorker analysis error: {}", e);
                }

                loop {
                    match Self::poll_po_drafts(&db).await {
                        Ok(true) => continue, // keep polling until queue is empty
                        Ok(false) => break,
                        Err(e) => {
                            ::server_telemetry::record_error_signal("[bug] PredictiveSupplyChainWorker draft error");
                            tracing::error!("PredictiveSupplyChainWorker draft error: {}", e);
                            break;
                        }
                    }
                }
            }
        });
    }

    // This simulates analyzing sales velocity and generating predictions
    pub async fn run_analysis(db: &Arc<DB>) -> Result<(), String> {
        let pool = &db.pool;

        // In a real system, this would query historical sales data to calculate velocity.
        // For simplicity, we'll find products with low inventory and generate a prediction.
        let low_stock_products = sqlx::query(
            "SELECT id, tenant_id, inventory_count FROM products WHERE inventory_count <= 5"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        for row in low_stock_products {
            let product_id: String = row.get("id");
            let tenant_id: String = row.get("tenant_id");

            // Check if prediction already exists recently to avoid spam
            let existing: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM inventory_predictions WHERE product_id = $1 AND tenant_id = $2 AND created_at > $3"
            )
            .bind(&product_id)
            .bind(&tenant_id)
            .bind(Utc::now() - chrono::Duration::hours(24))
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            if existing > 0 {
                continue;
            }

            // Predict stockout in 3 days if inventory is very low
            let predicted_date = Utc::now() + chrono::Duration::days(3);
            let prediction_id = Uuid::new_v4().to_string();

            // Insert prediction
            sqlx::query(
                "INSERT INTO inventory_predictions (id, tenant_id, product_id, predicted_stockout_date, confidence_score, suggested_reorder_quantity)
                 VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(&prediction_id)
            .bind(&tenant_id)
            .bind(&product_id)
            .bind(predicted_date)
            .bind(0.85) // High confidence
            .bind(50) // Suggest reordering 50 units
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

            // Push a job to draft a PO
            let job_id = Uuid::new_v4().to_string();
            let payload = json!({
                "product_id": product_id,
                "prediction_id": prediction_id,
                "suggested_quantity": 50
            });

            sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status)
                 VALUES ($1, $2, 'draft_purchase_order', $3, 'PENDING')"
            )
            .bind(&job_id)
            .bind(&tenant_id)
            .bind(&payload)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    // Process PO drafts
    pub async fn poll_po_drafts(db: &Arc<DB>) -> Result<bool, String> {
        let pool = &db.pool;

        // Simplified job queue fetching
        let job = sqlx::query(
            "UPDATE ohc_job_queue
             SET status = 'PROCESSING', locked_until = $1, updated_at = CURRENT_TIMESTAMP
             WHERE id = (
                 SELECT id FROM ohc_job_queue
                 WHERE status = 'PENDING' AND job_type = 'draft_purchase_order'
                 AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
                 ORDER BY created_at ASC
                 LIMIT 1
             )
             RETURNING id, tenant_id, payload"
        )
        .bind(Utc::now() + chrono::Duration::seconds(60))
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(row) = job {
            let job_id: String = row.get("id");
            let tenant_id: String = row.get("tenant_id");
            let payload: serde_json::Value = row.get("payload");

            // Support both old "product_id" payloads and new "raw_material_id" payloads
            let resource_id = payload["raw_material_id"].as_str().or_else(|| payload["product_id"].as_str()).unwrap_or_default();
            let resource_name = payload["raw_material_name"].as_str().unwrap_or("Supplies");
            let quantity = payload["suggested_quantity"].as_i64().unwrap_or(50) as f64;

            // Use Redlock pattern to prevent duplicate PO drafts (cross-agent coordination)
            // Lock key pattern: ohc:lock:{tenant_id}:{resource_type}:{resource_id}
            let lock_key = format!("ohc:lock:{}:purchase_order:{}", tenant_id, resource_id);
            let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

            // Acquire lock (simple Redis lock implementation for this worker, similar to what's defined in locks.rs)
            let mut has_lock = false;
            let mut redis_conn = None;

            if let Ok(client) = redis::Client::open(redis_url) {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let acquired: bool = redis::cmd("SET")
                        .arg(&lock_key)
                        .arg("1")
                        .arg("NX")
                        .arg("EX")
                        .arg(30)
                        .query_async(&mut conn)
                        .await
                        .unwrap_or(false);

                    if acquired {
                        has_lock = true;
                        redis_conn = Some(conn);
                    }
                }
            } else {
                // Standalone mode fallback if Redis isn't available
                has_lock = true;
            }

            if !has_lock {
                // If we can't acquire the lock, maybe another worker is drafting this PO.
                // Return to queue or fail.
                sqlx::query("UPDATE ohc_job_queue SET status = 'PENDING', locked_until = NULL WHERE id = $1")
                    .bind(&job_id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                return Ok(false);
            }

            // Generate PO
            let vendor_id = Uuid::new_v4().to_string(); // In a real scenario, map from product supplier
            let vendor_name = "Default Supplier";
            let po_id = Uuid::new_v4().to_string();
            let total_cost = quantity * 10.0; // Dummy cost

            // Insert dummy vendor if needed
            let _ = sqlx::query(
                "INSERT INTO vendors (id, tenant_id, name, contact_info) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"
            )
            .bind(&vendor_id)
            .bind(&tenant_id)
            .bind(vendor_name)
            .bind("supplier@example.com")
            .execute(pool)
            .await;

            // Generate a dummy customer profile as a "System Customer" or Vendor placeholder to satisfy the work_item customer_id requirement
            let sys_customer_id = Uuid::new_v4().to_string();
            let _ = sqlx::query(
                "INSERT INTO customer_profile (id, tenant_id, name, email) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"
            )
            .bind(&sys_customer_id)
            .bind(&tenant_id)
            .bind(vendor_name)
            .bind("vendor@example.com")
            .execute(pool)
            .await;

            sqlx::query(
                "INSERT INTO purchase_orders (id, tenant_id, vendor_id, status, total_cost)
                 VALUES ($1, $2, $3, 'DRAFT', $4)"
            )
            .bind(&po_id)
            .bind(&tenant_id)
            .bind(&vendor_id)
            .bind(total_cost)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

            // Insert into Inbox / Action Required queue
            let work_item_id = Uuid::new_v4().to_string();
            let agent_draft_id = Uuid::new_v4().to_string();

            let draft_payload = json!({
                "feature_type": "draft_purchase_order",
                "po_id": po_id,
                "vendor_name": vendor_name,
                "resource_name": resource_name,
                "suggested_quantity": quantity,
                "total_cost": total_cost,
            });

            sqlx::query(
                "INSERT INTO work_item (id, tenant_id, customer_id, source, payload, status)
                 VALUES ($1, $2, $3, $4, $5, 'DRAFT')"
            )
            .bind(&work_item_id)
            .bind(&tenant_id)
            .bind(&sys_customer_id)
            .bind("Operations Agent")
            .bind(&draft_payload)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

            let response_msg = format!("{} running low. Drafted Purchase Order for {} units.", resource_name, quantity);

            sqlx::query(
                "INSERT INTO agent_draft (id, work_item_id, response, status)
                 VALUES ($1, $2, $3, 'DRAFT')"
            )
            .bind(&agent_draft_id)
            .bind(&work_item_id)
            .bind(&response_msg)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

            // Mark job complete
            sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                .bind(&job_id)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;

            // Release lock
            if let Some(mut conn) = redis_conn {
                let _: redis::RedisResult<()> = redis::cmd("DEL").arg(&lock_key).query_async(&mut conn).await;
            }

            return Ok(true);
        }

        Ok(false)
    }
}
