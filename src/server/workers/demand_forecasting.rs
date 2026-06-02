use std::sync::Arc;
use crate::db::DB;
use crate::db::DbStore;
use std::time::Duration;
use chrono::Utc;
use uuid::Uuid;
use sqlx::Row;

const POLL_INTERVAL: Duration = Duration::from_secs(60 * 60); // Run every hour

pub struct DemandForecastingWorker {
    pub db: Arc<DB>,
}

impl DemandForecastingWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(POLL_INTERVAL);
            loop {
                interval.tick().await;
                if let Err(e) = Self::run_forecast(&db).await {
                    tracing::error!("DemandForecastingWorker error: {}", e);
                }
            }
        });
    }

    pub async fn run_forecast(db: &Arc<DB>) -> Result<(), String> {
        // Find raw materials to forecast
        let materials = match &db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    SELECT id, tenant_id, current_quantity, reorder_threshold
                    FROM raw_materials
                    "#
                )
                .fetch_all(&db.pool)
                .await
                .map_err(|e| e.to_string())?
            },
            DbStore::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    SELECT id, tenant_id, current_quantity, reorder_threshold
                    FROM raw_materials
                    "#
                )
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?
            }
        };

        for row in materials {
            let id: String = row.get("id");
            let tenant_id: String = row.get("tenant_id");
            let current_quantity: i32 = row.try_get("current_quantity").unwrap_or(0);

            // Calculate velocity over the last 30 days
            let thirty_days_ago = Utc::now() - chrono::Duration::days(30);

            let recent_depletions: i64 = match &db.store {
                DbStore::Postgres => {
                    sqlx::query_scalar(
                        "SELECT COALESCE(SUM(quantity_deducted), 0) FROM depletion_logs WHERE raw_material_id = $1 AND tenant_id = $2 AND created_at >= $3"
                    )
                    .bind(&id)
                    .bind(&tenant_id)
                    .bind(thirty_days_ago)
                    .fetch_one(&db.pool)
                    .await
                    .unwrap_or(0)
                },
                DbStore::Sqlite(pool) => {
                    sqlx::query_scalar(
                        "SELECT COALESCE(SUM(quantity_deducted), 0) FROM depletion_logs WHERE raw_material_id = ? AND tenant_id = ? AND created_at >= ?"
                    )
                    .bind(&id)
                    .bind(&tenant_id)
                    .bind(thirty_days_ago.format("%Y-%m-%d %H:%M:%S").to_string())
                    .fetch_one(pool)
                    .await
                    .unwrap_or(0)
                }
            };

            let daily_velocity = (recent_depletions as f64) / 30.0;

            if daily_velocity > 0.0 {
                let days_until_empty = (current_quantity as f64) / daily_velocity;
                let predicted_stockout_date = Utc::now() + chrono::Duration::days(days_until_empty as i64);

                // Check if threshold logic dictates a prediction
                // E.g. we will run out in less than 14 days
                if days_until_empty < 14.0 {
                    // Generate prediction record
                    let prediction_id = Uuid::new_v4().to_string();
                    let now = Utc::now();

                    match &db.store {
                        DbStore::Postgres => {
                            let _ = sqlx::query(
                                r#"
                                INSERT INTO inventory_predictions (id, tenant_id, raw_material_id, predicted_stockout_date, predicted_daily_velocity, current_inventory, status, created_at, updated_at)
                                VALUES ($1, $2, $3, $4, $5, $6, 'PENDING', $7, $7)
                                "#
                            )
                            .bind(&prediction_id)
                            .bind(&tenant_id)
                            .bind(&id)
                            .bind(predicted_stockout_date)
                            .bind(daily_velocity)
                            .bind(current_quantity)
                            .bind(now)
                            .execute(&db.pool)
                            .await;

                            // Also emit event to OperationsWorker
                            let task_id = Uuid::new_v4().to_string();
                            let payload = serde_json::json!({
                                "prediction_id": prediction_id,
                                "raw_material_id": id,
                                "predicted_stockout_date": predicted_stockout_date.to_rfc3339()
                            });

                            let _ = sqlx::query(
                                r#"
                                INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status, created_at)
                                VALUES ($1, $2, 'operations', 'PredictiveRestock', $3, 'PENDING', $4)
                                "#
                            )
                            .bind(&task_id)
                            .bind(&tenant_id)
                            .bind(payload)
                            .bind(now)
                            .execute(&db.pool)
                            .await;
                        },
                        DbStore::Sqlite(pool) => {
                            let _ = sqlx::query(
                                r#"
                                INSERT INTO inventory_predictions (id, tenant_id, raw_material_id, predicted_stockout_date, predicted_daily_velocity, current_inventory, status, created_at, updated_at)
                                VALUES (?, ?, ?, ?, ?, ?, 'PENDING', ?, ?)
                                "#
                            )
                            .bind(&prediction_id)
                            .bind(&tenant_id)
                            .bind(&id)
                            .bind(predicted_stockout_date)
                            .bind(daily_velocity)
                            .bind(current_quantity)
                            .bind(now)
                            .bind(now)
                            .execute(pool)
                            .await;

                            let task_id = Uuid::new_v4().to_string();
                            let payload = serde_json::json!({
                                "prediction_id": prediction_id,
                                "raw_material_id": id,
                                "predicted_stockout_date": predicted_stockout_date.to_rfc3339()
                            });

                            let _ = sqlx::query(
                                r#"
                                INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status, created_at)
                                VALUES (?, ?, 'operations', 'PredictiveRestock', ?, 'PENDING', ?)
                                "#
                            )
                            .bind(&task_id)
                            .bind(&tenant_id)
                            .bind(payload)
                            .bind(now)
                            .execute(pool)
                            .await;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
