use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;

pub struct SubscriptionChurnPredictionJob {
    pub db: Arc<DB>,
}

impl SubscriptionChurnPredictionJob {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(3600 * 24)); // Run daily
            loop {
                interval.tick().await;

                match &db.store {
                    crate::db::DbStore::Postgres => {
                        let rows = sqlx::query(
                            r#"
                            SELECT id, tenant_id, customer_id, health_score
                            FROM subscribers
                            WHERE status = 'ACTIVE'
                            AND (health_score < 50 OR last_engagement_at <= NOW() - INTERVAL '30 days')
                            "#
                        )
                        .fetch_all(&db.pool)
                        .await
                        .unwrap_or_default();

                        for row in rows {
                            use sqlx::Row;
                            let subscriber_id: String = row.get("id");
                            let tenant_id: String = row.get("tenant_id");
                            let customer_id: String = row.get("customer_id");
                            let health_score: i32 = row.get("health_score");

                            let payload = serde_json::json!({
                                "subscriber_id": subscriber_id,
                                "customer_id": customer_id,
                                "health_score": health_score,
                                "reason": if health_score < 50 { "low_health_score" } else { "low_engagement" }
                            });

                            let _ = sqlx::query(
                                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES ($1, $2, 'subscription_churn_risk', $3, 'PENDING', NOW()) ON CONFLICT DO NOTHING"
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
                            SELECT id, tenant_id, customer_id, health_score
                            FROM subscribers
                            WHERE status = 'ACTIVE'
                            AND (health_score < 50 OR datetime(last_engagement_at) <= datetime('now', '-30 days'))
                            "#
                        )
                        .fetch_all(sqlite_pool)
                        .await
                        .unwrap_or_default();

                        for row in rows {
                            use sqlx::Row;
                            let subscriber_id: String = row.get("id");
                            let tenant_id: String = row.get("tenant_id");
                            let customer_id: String = row.get("customer_id");
                            let health_score: i32 = row.get("health_score");

                            let payload = serde_json::json!({
                                "subscriber_id": subscriber_id,
                                "customer_id": customer_id,
                                "health_score": health_score,
                                "reason": if health_score < 50 { "low_health_score" } else { "low_engagement" }
                            });

                            let _ = sqlx::query(
                                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES (?, ?, 'subscription_churn_risk', ?, 'PENDING', CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING"
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
