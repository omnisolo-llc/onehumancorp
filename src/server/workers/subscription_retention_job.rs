use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;
use serde_json::json;

pub struct SubscriptionRetentionJob {
    pub db: Arc<DB>,
}

impl SubscriptionRetentionJob {
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
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        sqlx::query_scalar("SELECT id FROM tenants")
                            .fetch_all(sqlite_pool)
                            .await
                            .unwrap_or_default()
                    }
                };

                for tenant_id in tenants {
                    // Find active subscriptions where the customer hasn't ordered/booked in > 21 days
                    let customers: Vec<String> = match &db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query_scalar(
                                r#"
                                WITH customer_activity AS (
                                    SELECT customer_id, MAX(created_at) as last_activity
                                    FROM orders
                                    WHERE tenant_id = $1
                                    GROUP BY customer_id
                                    UNION ALL
                                    SELECT customer_id, MAX(start_time) as last_activity
                                    FROM bookings
                                    WHERE tenant_id = $1
                                    GROUP BY customer_id
                                ),
                                max_activity AS (
                                    SELECT customer_id, MAX(last_activity) as last_active_at
                                    FROM customer_activity
                                    GROUP BY customer_id
                                )
                                SELECT s.customer_id
                                FROM subscriptions s
                                LEFT JOIN max_activity ma ON s.customer_id = ma.customer_id
                                WHERE s.tenant_id = $1
                                  AND s.status = 'active'
                                  AND (ma.last_active_at IS NULL OR ma.last_active_at < CURRENT_TIMESTAMP - INTERVAL '21 days')
                                  AND s.customer_id NOT IN (
                                      SELECT payload->>'customer_id'
                                      FROM ohc_job_queue
                                      WHERE tenant_id = $1 AND job_type = 'subscription_retention_check' AND status = 'PENDING'
                                  )
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
                                WITH customer_activity AS (
                                    SELECT customer_id, MAX(created_at) as last_activity
                                    FROM orders
                                    WHERE tenant_id = ?
                                    GROUP BY customer_id
                                    UNION ALL
                                    SELECT customer_id, MAX(start_time) as last_activity
                                    FROM bookings
                                    WHERE tenant_id = ?
                                    GROUP BY customer_id
                                ),
                                max_activity AS (
                                    SELECT customer_id, MAX(last_activity) as last_active_at
                                    FROM customer_activity
                                    GROUP BY customer_id
                                )
                                SELECT s.customer_id
                                FROM subscriptions s
                                LEFT JOIN max_activity ma ON s.customer_id = ma.customer_id
                                WHERE s.tenant_id = ?
                                  AND s.status = 'active'
                                  AND (ma.last_active_at IS NULL OR ma.last_active_at < datetime('now', '-21 days'))
                                  AND s.customer_id NOT IN (
                                      SELECT json_extract(payload, '$.customer_id')
                                      FROM ohc_job_queue
                                      WHERE tenant_id = ? AND job_type = 'subscription_retention_check' AND status = 'PENDING'
                                  )
                                "#
                            )
                            .bind(&tenant_id)
                            .bind(&tenant_id)
                            .bind(&tenant_id)
                            .bind(&tenant_id)
                            .fetch_all(sqlite_pool)
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
                                    "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'subscription_retention_check', $3, 'PENDING') ON CONFLICT DO NOTHING"
                                )
                                .bind(&job_id)
                                .bind(&tenant_id)
                                .bind(&payload)
                                .execute(&db.pool)
                                .await;
                            },
                            crate::db::DbStore::Sqlite(sqlite_pool) => {
                                let _ = sqlx::query(
                                    "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'subscription_retention_check', ?, 'PENDING') ON CONFLICT DO NOTHING"
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
        });
    }
}
