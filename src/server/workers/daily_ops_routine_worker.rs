use crate::db::DB;
use std::sync::Arc;

use serde_json::json;
use uuid::Uuid;

pub struct DailyOpsRoutineWorker {
    pub db: Arc<DB>,
}

impl DailyOpsRoutineWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        tracing::info!(
            event = "operations.scheduled_checklists_unavailable",
            "No configured checklist schedule is stored; automatic checklist creation is unavailable"
        );
        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60)); // Check every minute in dev (or configurable)
            loop {
                interval.tick().await;

                // 1. Get all active tenants
                let tenants: Vec<String> = match &db.store {
                    crate::db::DbStore::Postgres => sqlx::query_scalar("SELECT id FROM tenants")
                        .fetch_all(&db.pool)
                        .await
                        .unwrap_or_default(),
                    crate::db::DbStore::Sqlite(pool) => {
                        sqlx::query_scalar("SELECT id FROM tenants")
                            .fetch_all(pool)
                            .await
                            .unwrap_or_default()
                    }
                };

                for tenant_id in tenants {
                    // Find dormant customers and insert into ohc_job_queue
                    let dormant_customers: Vec<String> = match &db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query_scalar(
                                r#"
                                WITH customer_stats AS (
                                    SELECT customer_id, COUNT(*) as total_bookings, MAX(start_time) as last_booking
                                    FROM bookings
                                    WHERE tenant_id = $1
                                    GROUP BY customer_id
                                )
                                SELECT customer_id
                                FROM customer_stats
                                WHERE total_bookings > 1 AND last_booking < CURRENT_TIMESTAMP - INTERVAL '14 days'
                                AND customer_id NOT IN (
                                    SELECT payload->>'customer_id'
                                    FROM ohc_job_queue
                                    WHERE tenant_id = $1 AND job_type = 'booking_reengagement_check' AND status IN ('PENDING', 'PROCESSING', 'COMPLETED') AND created_at > CURRENT_TIMESTAMP - INTERVAL '14 days'
                                )
                                "#
                            )
                            .bind(&tenant_id)
                            .fetch_all(&db.pool).await.unwrap_or_default()
                        },
                        crate::db::DbStore::Sqlite(pool) => {
                            sqlx::query_scalar(
                                r#"
                                WITH customer_stats AS (
                                    SELECT customer_id, COUNT(*) as total_bookings, MAX(start_time) as last_booking
                                    FROM bookings
                                    WHERE tenant_id = $1
                                    GROUP BY customer_id
                                )
                                SELECT customer_id
                                FROM customer_stats
                                WHERE total_bookings > 1 AND last_booking < datetime('now', '-14 days')
                                AND customer_id NOT IN (
                                    SELECT json_extract(payload, '$.customer_id')
                                    FROM ohc_job_queue
                                    WHERE tenant_id = $1 AND job_type = 'booking_reengagement_check' AND status IN ('PENDING', 'PROCESSING', 'COMPLETED') AND created_at > datetime('now', '-14 days')
                                )
                                "#
                            )
                            .bind(&tenant_id)
                            .fetch_all(pool).await.unwrap_or_default()
                        }
                    };

                    for dormant_customer in dormant_customers {
                        let payload = json!({"customer_id": dormant_customer});
                        match &db.store {
                            crate::db::DbStore::Postgres => {
                                let _ = sqlx::query(
                                    "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES ($1, $2, 'booking_reengagement_check', $3, 'PENDING', CURRENT_TIMESTAMP)"
                                )
                                .bind(Uuid::new_v4().to_string())
                                .bind(&tenant_id)
                                .bind(sqlx::types::Json(payload))
                                .execute(&db.pool).await;
                            }
                            crate::db::DbStore::Sqlite(pool) => {
                                let payload_str = payload.to_string();
                                let _ = sqlx::query(
                                    "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES (?, ?, 'booking_reengagement_check', ?, 'PENDING', CURRENT_TIMESTAMP)"
                                )
                                .bind(Uuid::new_v4().to_string())
                                .bind(&tenant_id)
                                .bind(payload_str)
                                .execute(pool).await;
                            }
                        }
                    }
                }
            }
        });
    }
}
