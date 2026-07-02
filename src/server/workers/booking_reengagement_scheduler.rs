use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;
use sqlx::Row;
use serde_json::json;

pub struct BookingReengagementScheduler {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl BookingReengagementScheduler {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(60 * 60 * 24), // Run daily
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
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
                    // Identify dormant customers for this tenant and schedule checks.
                    // A dormant customer has historically booked more than once, but not in the last 14 days.
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
                                SELECT customer_id::text
                                FROM customer_stats
                                WHERE total_bookings > 1 AND last_booking < CURRENT_TIMESTAMP - INTERVAL '14 days';
                                "#
                            )
                            .bind(&tenant_id)
                            .fetch_all(&db.pool)
                            .await
                            .unwrap_or_default()
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                             sqlx::query_scalar(
                                r#"
                                WITH customer_stats AS (
                                    SELECT customer_id, COUNT(*) as total_bookings, MAX(start_time) as last_booking
                                    FROM bookings
                                    WHERE tenant_id = ?
                                    GROUP BY customer_id
                                )
                                SELECT customer_id
                                FROM customer_stats
                                WHERE total_bookings > 1 AND last_booking < datetime('now', '-14 days');
                                "#
                            )
                            .bind(&tenant_id)
                            .fetch_all(sqlite_pool)
                            .await
                            .unwrap_or_default()
                        }
                    };

                    for customer_id in dormant_customers {
                        // Check if a job already exists in the queue (PENDING) OR if we already created a task for them recently.
                        // We check the shared_tasks table to see if a re-engagement task already exists for this customer.
                        // This prevents spamming.
                        let task_exists: bool = match &db.store {
                            crate::db::DbStore::Postgres => {
                                let query_str = format!("%Approve Re-engagement for %"); // We can't easily join on name here without more complex query, but we can check if ANY re-engagement task exists for them. Actually, wait. Let's just check the job queue for ANY status within the last 14 days.

                                sqlx::query_scalar::<_, i64>(
                                    "SELECT COUNT(*) FROM ohc_job_queue WHERE tenant_id = $1 AND job_type = 'booking_reengagement_check' AND payload->>'customer_id' = $2 AND created_at > CURRENT_TIMESTAMP - INTERVAL '14 days'"
                                )
                                .bind(&tenant_id)
                                .bind(&customer_id)
                                .fetch_one(&db.pool)
                                .await
                                .unwrap_or(0) > 0
                            },
                            crate::db::DbStore::Sqlite(sqlite_pool) => {
                                sqlx::query_scalar::<_, i32>(
                                    "SELECT COUNT(*) FROM ohc_job_queue WHERE tenant_id = ? AND job_type = 'booking_reengagement_check' AND json_extract(payload, '$.customer_id') = ? AND created_at > datetime('now', '-14 days')"
                                )
                                .bind(&tenant_id)
                                .bind(&customer_id)
                                .fetch_one(sqlite_pool)
                                .await
                                .unwrap_or(0) > 0
                            }
                        };

                        if !task_exists {
                             let job_id = Uuid::new_v4().to_string();
                             let payload = json!({
                                 "customer_id": customer_id,
                             });

                             match &db.store {
                                 crate::db::DbStore::Postgres => {
                                      let _ = sqlx::query(
                                          "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES ($1, $2, 'booking_reengagement_check', $3, 'PENDING', CURRENT_TIMESTAMP)"
                                      )
                                      .bind(&job_id)
                                      .bind(&tenant_id)
                                      .bind(payload)
                                      .execute(&db.pool)
                                      .await;
                                 },
                                 crate::db::DbStore::Sqlite(sqlite_pool) => {
                                       let _ = sqlx::query(
                                          "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES (?, ?, 'booking_reengagement_check', ?, 'PENDING', CURRENT_TIMESTAMP)"
                                      )
                                      .bind(&job_id)
                                      .bind(&tenant_id)
                                      .bind(payload.to_string())
                                      .execute(sqlite_pool)
                                      .await;
                                 }
                             }
                        }
                    }
                }
            }
        });
    }
}
