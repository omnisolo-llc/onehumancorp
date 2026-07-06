use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;

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
            let mut interval = tokio::time::interval(Duration::from_secs(86400)); // Run daily
            loop {
                interval.tick().await;

                tracing::info!("Running SubscriptionRetentionJob to find at-risk subscriptions");

                match &db.store {
                    crate::db::DbStore::Postgres => {
                        let rows = sqlx::query(
                            r#"
                            SELECT
                                s.id as subscription_id,
                                s.tenant_id,
                                s.customer_id,
                                c.name as customer_name,
                                s.current_period_end,
                                COALESCE(
                                    (SELECT MAX(created_at) FROM appointments WHERE customer_id = s.customer_id),
                                    s.created_at
                                ) as last_activity_date
                            FROM subscriptions s
                            JOIN customers c ON s.customer_id = c.id
                            WHERE s.status = 'active'
                            AND s.current_period_end <= NOW() + INTERVAL '7 days'
                            AND s.cancel_at_period_end = FALSE
                            AND COALESCE(
                                    (SELECT MAX(created_at) FROM appointments WHERE customer_id = s.customer_id),
                                    s.created_at
                                ) <= NOW() - INTERVAL '30 days'
                            "#
                        )
                        .fetch_all(&db.pool)
                        .await
                        .unwrap_or_default();

                        for row in rows {
                            use sqlx::Row;
                            let subscription_id: String = row.get("subscription_id");
                            let tenant_id: String = row.get("tenant_id");
                            let customer_id: String = row.get("customer_id");
                            let customer_name: String = row.get("customer_name");

                            let payload = serde_json::json!({
                                "subscription_id": subscription_id,
                                "customer_id": customer_id,
                                "customer_name": customer_name,
                                "health_score": "LOW",
                                "reason": "No recent activity in 30 days and approaching renewal"
                            });

                            let _ = sqlx::query(
                                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES ($1, $2, 'agent_customer_success', $3, 'PENDING', NOW()) ON CONFLICT DO NOTHING"
                            )
                            .bind(Uuid::new_v4().to_string())
                            .bind(tenant_id)
                            .bind(serde_json::json!({
                                "event_type": "tenant.subscription.churn_risk",
                                "payload": payload
                            }).to_string())
                            .execute(&db.pool)
                            .await;
                        }
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                         let rows = sqlx::query(
                            r#"
                            SELECT
                                s.id as subscription_id,
                                s.tenant_id,
                                s.customer_id,
                                c.name as customer_name,
                                s.current_period_end,
                                COALESCE(
                                    (SELECT MAX(created_at) FROM appointments WHERE customer_id = s.customer_id),
                                    s.created_at
                                ) as last_activity_date
                            FROM subscriptions s
                            JOIN customers c ON s.customer_id = c.id
                            WHERE s.status = 'active'
                            AND datetime(s.current_period_end) <= datetime('now', '+7 days')
                            AND s.cancel_at_period_end = 0
                            AND datetime(COALESCE(
                                    (SELECT MAX(created_at) FROM appointments WHERE customer_id = s.customer_id),
                                    s.created_at
                                )) <= datetime('now', '-30 days')
                            "#
                        )
                        .fetch_all(sqlite_pool)
                        .await
                        .unwrap_or_default();

                        for row in rows {
                            use sqlx::Row;
                            let subscription_id: String = row.get("subscription_id");
                            let tenant_id: String = row.get("tenant_id");
                            let customer_id: String = row.get("customer_id");
                            let customer_name: String = row.get("customer_name");

                            let payload = serde_json::json!({
                                "subscription_id": subscription_id,
                                "customer_id": customer_id,
                                "customer_name": customer_name,
                                "health_score": "LOW",
                                "reason": "No recent activity in 30 days and approaching renewal"
                            });

                            let _ = sqlx::query(
                                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES (?, ?, 'agent_customer_success', ?, 'PENDING', CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING"
                            )
                            .bind(Uuid::new_v4().to_string())
                            .bind(tenant_id)
                            .bind(serde_json::json!({
                                "event_type": "tenant.subscription.churn_risk",
                                "payload": payload
                            }).to_string())
                            .execute(sqlite_pool)
                            .await;
                        }
                    }
                }
            }
        });
    }
}
