use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;
use serde_json::json;

pub struct BookingReengagementJob {
    pub db: Arc<DB>,
}

impl BookingReengagementJob {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Run once an hour
            loop {
                interval.tick().await;

                let tenants: Vec<String> = match &db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_scalar("SELECT id FROM tenants")
                            .fetch_all(&db.pool)
                            .await
                            .unwrap_or_default()
                    },
                    crate::db::DbStore::Sqlite(_) => {
                        sqlx::query_scalar("SELECT id FROM tenants")
                            .fetch_all(&db.pool)
                            .await
                            .unwrap_or_default()
                    }
                };

                for tenant_id in tenants {
                    // Find customers who might be dormant and don't already have a pending job
                    let customers: Vec<String> = match &db.store {
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
                                    WHERE tenant_id = $1 AND job_type = 'booking_reengagement_check' AND status = 'PENDING'
                                )
                                "#
                            )
                            .bind(&tenant_id)
                            .fetch_all(&db.pool)
                            .await
                            .unwrap_or_default()
                        },
                        crate::db::DbStore::Sqlite(_) => {
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
                                    WHERE tenant_id = $1 AND job_type = 'booking_reengagement_check' AND status = 'PENDING'
                                )
                                "#
                            )
                            .bind(&tenant_id)
                            .fetch_all(&db.pool)
                            .await
                            .unwrap_or_default()
                        }
                    };

                    for customer_id in customers {
                        let payload = json!({
                            "customer_id": customer_id
                        });
                        let job_id = Uuid::new_v4().to_string();

                        match &db.store {
                            crate::db::DbStore::Postgres => {
                                let _ = sqlx::query(
                                    "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'booking_reengagement_check', $3, 'PENDING') ON CONFLICT DO NOTHING"
                                )
                                .bind(&job_id)
                                .bind(&tenant_id)
                                .bind(&payload)
                                .execute(&db.pool)
                                .await;
                            },
                            crate::db::DbStore::Sqlite(_) => {
                                let _ = sqlx::query(
                                    "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'booking_reengagement_check', ?, 'PENDING') ON CONFLICT DO NOTHING"
                                )
                                .bind(&job_id)
                                .bind(&tenant_id)
                                .bind(payload.to_string())
                                .execute(&db.pool)
                                .await;
                            }
                        }
                    }
                }
            }
        });
    }
}
