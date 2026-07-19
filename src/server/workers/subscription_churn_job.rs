use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;
use serde_json::json;

pub struct SubscriptionChurnJob {
    pub db: Arc<DB>,
}

impl SubscriptionChurnJob {
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
                    // Find active subscriptions and check health score based on time since last booking/engagement
                    let subscriptions: Vec<(String, String)> = match &db.store {
                        crate::db::DbStore::Postgres => {
                            sqlx::query_as::<_, (String, String)>(
                                r#"
                                SELECT s.id, s.customer_id
                                FROM subscriptions s
                                WHERE s.tenant_id = $1 AND s.status = 'active'
                                "#
                            )
                            .bind(&tenant_id)
                            .fetch_all(&db.pool)
                            .await
                            .unwrap_or_default()
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            sqlx::query_as::<_, (String, String)>(
                                r#"
                                SELECT s.id, s.customer_id
                                FROM subscriptions s
                                WHERE s.tenant_id = $1 AND s.status = 'active'
                                "#
                            )
                            .bind(&tenant_id)
                            .fetch_all(&db.pool)
                            .await
                            .unwrap_or_default()
                        }
                    };

                    for (subscription_id, customer_id) in subscriptions {
                        // Calculate health score (simple heuristic: 1.0 - (days since last booking / 60))
                        let days_since_last_booking: i64 = match &db.store {
                            crate::db::DbStore::Postgres => {
                                sqlx::query_scalar::<_, Option<i64>>(
                                    r#"
                                    SELECT EXTRACT(DAY FROM CURRENT_TIMESTAMP - MAX(start_time))::bigint
                                    FROM bookings
                                    WHERE tenant_id = $1 AND customer_id = $2::uuid
                                    "#
                                )
                                .bind(&tenant_id)
                                .bind(&customer_id)
                                .fetch_one(&db.pool)
                                .await
                                .unwrap_or(None)
                                .unwrap_or(30)
                            },
                            crate::db::DbStore::Sqlite(_) => {
                                sqlx::query_scalar::<_, Option<i64>>(
                                    r#"
                                    SELECT CAST(julianday('now') - julianday(MAX(start_time)) AS INTEGER)
                                    FROM bookings
                                    WHERE tenant_id = $1 AND customer_id = $2
                                    "#
                                )
                                .bind(&tenant_id)
                                .bind(&customer_id)
                                .fetch_one(&db.pool)
                                .await
                                .unwrap_or(None)
                                .unwrap_or(30)
                            }
                        };

                        let mut health_score = 1.0 - (days_since_last_booking as f64 / 60.0);
                        if health_score < 0.0 { health_score = 0.0; }
                        if health_score > 1.0 { health_score = 1.0; }

                        // Update the health score in DB
                        match &db.store {
                            crate::db::DbStore::Postgres => {
                                let _ = sqlx::query("UPDATE subscriptions SET health_score = $1, updated_at = NOW() WHERE id = $2")
                                    .bind(health_score)
                                    .bind(&subscription_id)
                                    .execute(&db.pool)
                                    .await;
                            },
                            crate::db::DbStore::Sqlite(_) => {
                                let _ = sqlx::query("UPDATE subscriptions SET health_score = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
                                    .bind(health_score)
                                    .bind(&subscription_id)
                                    .execute(&db.pool)
                                    .await;
                            }
                        }

                        // Check if at-risk (health_score < 0.4) and not already in job queue
                        if health_score < 0.4 {
                            let in_queue: bool = match &db.store {
                                crate::db::DbStore::Postgres => {
                                    sqlx::query_scalar::<_, i64>(
                                        "SELECT COUNT(*) FROM ohc_job_queue WHERE tenant_id = $1 AND job_type = 'subscription_churn_prediction' AND status = 'PENDING' AND payload->>'subscription_id' = $2"
                                    )
                                    .bind(&tenant_id)
                                    .bind(&subscription_id)
                                    .fetch_one(&db.pool)
                                    .await
                                    .unwrap_or(0) > 0
                                },
                                crate::db::DbStore::Sqlite(_) => {
                                    sqlx::query_scalar::<_, i64>(
                                        "SELECT COUNT(*) FROM ohc_job_queue WHERE tenant_id = $1 AND job_type = 'subscription_churn_prediction' AND status = 'PENDING' AND json_extract(payload, '$.subscription_id') = $2"
                                    )
                                    .bind(&tenant_id)
                                    .bind(&subscription_id)
                                    .fetch_one(&db.pool)
                                    .await
                                    .unwrap_or(0) > 0
                                }
                            };

                            if !in_queue {
                                let payload = json!({
                                    "customer_id": customer_id,
                                    "subscription_id": subscription_id,
                                    "health_score": health_score,
                                    "days_since_last_booking": days_since_last_booking
                                });
                                let job_id = Uuid::new_v4().to_string();

                                match &db.store {
                                    crate::db::DbStore::Postgres => {
                                        let _ = sqlx::query(
                                            "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'subscription_churn_prediction', $3, 'PENDING') ON CONFLICT DO NOTHING"
                                        )
                                        .bind(&job_id)
                                        .bind(&tenant_id)
                                        .bind(&payload)
                                        .execute(&db.pool)
                                        .await;
                                    },
                                    crate::db::DbStore::Sqlite(_) => {
                                        let _ = sqlx::query(
                                            "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'subscription_churn_prediction', ?, 'PENDING') ON CONFLICT DO NOTHING"
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
                }
            }
        });
    }
}
