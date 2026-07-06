use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;

pub struct SubscriptionHealthJob {
    pub db: Arc<DB>,
}

impl SubscriptionHealthJob {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Run hourly
            loop {
                interval.tick().await;

                // Find subscriptions with low health score (e.g., no activity in 30 days)
                match &db.store {
                    crate::db::DbStore::Postgres => {
                        let rows = sqlx::query(
                            r#"
                            SELECT s.id, s.tenant_id, s.customer_id
                            FROM subscriptions s
                            WHERE s.status = 'active'
                            AND NOT EXISTS (
                                SELECT 1 FROM bookings b
                                WHERE b.customer_id = s.customer_id::uuid
                                AND b.created_at >= NOW() - INTERVAL '30 days'
                            )
                            "#
                        )
                        .fetch_all(&db.pool)
                        .await
                        .unwrap_or_default();

                        for row in rows {
                            use sqlx::Row;
                            let subscription_id: String = row.get("id");
                            let tenant_id: String = row.get("tenant_id");
                            let customer_id: String = row.get("customer_id");

                            let payload = serde_json::json!({
                                "subscription_id": subscription_id,
                                "customer_id": customer_id,
                                "reason": "no bookings in 30 days"
                            });

                            let _ = sqlx::query(
                                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES ($1, $2, 'subscription_health_check', $3, 'PENDING', NOW()) ON CONFLICT DO NOTHING"
                            )
                            .bind(Uuid::new_v4().to_string())
                            .bind(tenant_id)
                            .bind(payload.to_string())
                            .execute(&db.pool)
                            .await;
                        }
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                         let rows = sqlx::query(
                            r#"
                            SELECT s.id, s.tenant_id, s.customer_id
                            FROM subscriptions s
                            WHERE s.status = 'active'
                            AND NOT EXISTS (
                                SELECT 1 FROM bookings b
                                WHERE b.customer_id = s.customer_id
                                AND b.created_at >= datetime('now', '-30 days')
                            )
                            "#
                        )
                        .fetch_all(sqlite_pool)
                        .await
                        .unwrap_or_default();

                        for row in rows {
                            use sqlx::Row;
                            let subscription_id: String = row.get("id");
                            let tenant_id: String = row.get("tenant_id");
                            let customer_id: String = row.get("customer_id");

                            let payload = serde_json::json!({
                                "subscription_id": subscription_id,
                                "customer_id": customer_id,
                                "reason": "no bookings in 30 days"
                            });

                            let _ = sqlx::query(
                                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES (?, ?, 'subscription_health_check', ?, 'PENDING', CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING"
                            )
                            .bind(Uuid::new_v4().to_string())
                            .bind(tenant_id)
                            .bind(payload.to_string())
                            .execute(sqlite_pool)
                            .await;
                        }
                    }
                }
            }
        });
    }
}
