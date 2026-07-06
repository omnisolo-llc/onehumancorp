use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;
use sqlx::Row;

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
            let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Run hourly
            loop {
                interval.tick().await;

                match &db.store {
                    crate::db::DbStore::Postgres => {
                        let rows = sqlx::query(
                            r#"
                            SELECT s.id, s.tenant_id, s.customer_id
                            FROM subscriptions s
                            WHERE s.status = 'active'
                            AND s.current_period_end BETWEEN NOW() AND NOW() + INTERVAL '7 days'
                            AND NOT EXISTS (
                                SELECT 1 FROM bookings b
                                WHERE b.customer_id::text = s.customer_id
                                AND b.tenant_id = s.tenant_id
                                AND b.created_at >= NOW() - INTERVAL '21 days'
                            )
                            "#
                        )
                        .fetch_all(&db.pool)
                        .await
                        .unwrap_or_default();

                        for row in rows {
                            let subscription_id: String = row.get("id");
                            let tenant_id: String = row.get("tenant_id");
                            let customer_id: String = row.get("customer_id");

                            let payload = serde_json::json!({
                                "subscription_id": subscription_id,
                                "customer_id": customer_id
                            });

                            let _ = sqlx::query(
                                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES ($1, $2, 'subscription_retention', $3, 'PENDING', NOW()) ON CONFLICT DO NOTHING"
                            )
                            .bind(Uuid::new_v4().to_string())
                            .bind(tenant_id)
                            .bind(sqlx::types::Json(payload))
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
                            AND s.current_period_end BETWEEN datetime('now') AND datetime('now', '+7 days')
                            AND NOT EXISTS (
                                SELECT 1 FROM bookings b
                                WHERE b.customer_id = s.customer_id
                                AND b.tenant_id = s.tenant_id
                                AND b.created_at >= datetime('now', '-21 days')
                            )
                            "#
                        )
                        .fetch_all(sqlite_pool)
                        .await
                        .unwrap_or_default();

                        for row in rows {
                            let subscription_id: String = row.get("id");
                            let tenant_id: String = row.get("tenant_id");
                            let customer_id: String = row.get("customer_id");

                            let payload = serde_json::json!({
                                "subscription_id": subscription_id,
                                "customer_id": customer_id
                            });

                            let _ = sqlx::query(
                                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES (?, ?, 'subscription_retention', ?, 'PENDING', CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING"
                            )
                            .bind(Uuid::new_v4().to_string())
                            .bind(tenant_id)
                            .bind(sqlx::types::Json(payload))
                            .execute(sqlite_pool)
                            .await;
                        }
                    }
                }
            }
        });
    }
}
