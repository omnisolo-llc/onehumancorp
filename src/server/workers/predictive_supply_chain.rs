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

                if let Err(e) = Self::check_subscription_runouts(&db).await {
                    tracing::error!("PredictiveSupplyChainWorker subscription runout error: {}", e);
                }

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

    pub async fn check_subscription_runouts(db: &Arc<DB>) -> Result<(), String> {
        let pool = &db.pool;

        let runout_query = r#"
            SELECT rm.id, rm.tenant_id, rm.name, rm.current_quantity, SUM(bi.quantity_required) as needed
            FROM subscriptions s
            JOIN products p ON s.plan_id = p.id
            JOIN bom_items bi ON p.id = bi.finished_good_id
            JOIN raw_materials rm ON bi.raw_material_id = rm.id
            WHERE s.status = 'active'
            AND s.current_period_end <= NOW() + INTERVAL '7 days'
            GROUP BY rm.id, rm.tenant_id, rm.name, rm.current_quantity
            HAVING rm.current_quantity < SUM(bi.quantity_required)
        "#;

        let sqlite_runout_query = r#"
            SELECT rm.id, rm.tenant_id, rm.name, rm.current_quantity, SUM(bi.quantity_required) as needed
            FROM subscriptions s
            JOIN products p ON s.plan_id = p.id
            JOIN bom_items bi ON p.id = bi.finished_good_id
            JOIN raw_materials rm ON bi.raw_material_id = rm.id
            WHERE s.status = 'active'
            AND s.current_period_end <= datetime('now', '+7 days')
            GROUP BY rm.id, rm.tenant_id, rm.name, rm.current_quantity
            HAVING rm.current_quantity < SUM(bi.quantity_required)
        "#;

        let runouts = match &db.store {
            crate::db::DbStore::Postgres => sqlx::query(runout_query).fetch_all(pool).await.map_err(|e| e.to_string())?,
            crate::db::DbStore::Sqlite(sqlite_pool) => sqlx::query(sqlite_runout_query).fetch_all(sqlite_pool).await.map_err(|e| e.to_string())?,
        };

        for row in runouts {
            use sqlx::Row;
            let material_id: String = row.get("id");
            let tenant_id: String = row.get("tenant_id");
            let material_name: String = row.get("name");
            let current_qty: i32 = row.try_get("current_quantity").unwrap_or(0);

            // Needed could be stored as i64 in sqlite SUM, handle safely
            let needed: i32 = match row.try_get::<i32, _>("needed") {
                Ok(v) => v,
                Err(_) => row.try_get::<i64, _>("needed").unwrap_or(0) as i32
            };

            let reorder_quantity = needed * 2; // Simple buffer logic

            let existing: i64 = match &db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query_scalar("SELECT COUNT(*) FROM agent_reorder_intents WHERE raw_material_id = $1 AND tenant_id = $2 AND status = 'DRAFT'")
                        .bind(&material_id)
                        .bind(&tenant_id)
                        .fetch_one(pool)
                        .await
                        .unwrap_or(0)
                }
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query_scalar("SELECT COUNT(*) FROM agent_reorder_intents WHERE raw_material_id = ? AND tenant_id = ? AND status = 'DRAFT'")
                        .bind(&material_id)
                        .bind(&tenant_id)
                        .fetch_one(sqlite_pool)
                        .await
                        .unwrap_or(0)
                }
            };

            if existing > 0 {
                continue;
            }

            let intent_id = Uuid::new_v4().to_string();

            match &db.store {
                crate::db::DbStore::Postgres => {
                    let _ = sqlx::query("INSERT INTO agent_reorder_intents (id, tenant_id, raw_material_id, suggested_quantity, status) VALUES ($1, $2, $3, $4, 'DRAFT')")
                        .bind(&intent_id)
                        .bind(&tenant_id)
                        .bind(&material_id)
                        .bind(reorder_quantity)
                        .execute(pool)
                        .await;
                }
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    let _ = sqlx::query("INSERT INTO agent_reorder_intents (id, tenant_id, raw_material_id, suggested_quantity, status) VALUES (?, ?, ?, ?, 'DRAFT')")
                        .bind(&intent_id)
                        .bind(&tenant_id)
                        .bind(&material_id)
                        .bind(reorder_quantity)
                        .execute(sqlite_pool)
                        .await;
                }
            }

            let payload = json!({
                "feature_type": "supply_order",
                "product_id": material_id,
                "product_name": material_name,
                "remaining_stock": current_qty,
                "est_runout_days": 7, // Due within 7 days
                "suggested_reorder_quantity": reorder_quantity,
                "vendor_name": "Default Supplier",
                "intent_id": intent_id,
                "draft_message": format!("Please restock {} units of {} for upcoming subscriptions.", reorder_quantity, material_name),
                "message": format!("Upcoming subscriptions require {} {}, but only {} are in stock.", needed, material_name, current_qty)
            });

            let job_id = Uuid::new_v4().to_string();
            match &db.store {
                crate::db::DbStore::Postgres => {
                    let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, action_type, status, product_id, payload, source, agent_type, created_at, updated_at) VALUES ($1, $2, 'Reorder', 'Pending', $3, $4::jsonb, 'operations_agent', 'operations', NOW(), NOW())")
                        .bind(&job_id)
                        .bind(&tenant_id)
                        .bind(&material_id)
                        .bind(&payload)
                        .execute(pool)
                        .await;
                }
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, action_type, status, product_id, payload, source, agent_type, created_at, updated_at) VALUES (?, ?, 'Reorder', 'Pending', ?, ?, 'operations_agent', 'operations', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                        .bind(&job_id)
                        .bind(&tenant_id)
                        .bind(&material_id)
                        .bind(payload.to_string())
                        .execute(sqlite_pool)
                        .await;
                }
            }
        }

        Ok(())
    }

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

            // Use Redlock pattern to prevent duplicate PO drafts (cross-agent coordination)
            // Lock key pattern: ohc:lock:{tenant_id}:{resource_type}:{resource_id}
            let product_id = payload["product_id"].as_str().unwrap_or_default();
            let quantity = payload["suggested_quantity"].as_i64().unwrap_or(50) as f64;

            // We lock on the product to avoid multiple agents drafting POs for the same product at the exact same time
            let lock_key = format!("ohc:lock:{}:purchase_order:{}", tenant_id, product_id);
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
            let po_id = Uuid::new_v4().to_string();

            // Insert dummy vendor if needed
            let _ = sqlx::query(
                "INSERT INTO vendors (id, tenant_id, name, contact_info) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"
            )
            .bind(&vendor_id)
            .bind(&tenant_id)
            .bind("Default Supplier")
            .bind("supplier@example.com")
            .execute(pool)
            .await;

            sqlx::query(
                "INSERT INTO purchase_orders (id, tenant_id, vendor_id, status, total_cost)
                 VALUES ($1, $2, $3, 'DRAFT', $4)"
            )
            .bind(&po_id)
            .bind(&tenant_id)
            .bind(&vendor_id)
            .bind(quantity * 10.0) // Dummy cost
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
