use crate::db::DB;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

const POSTGRES_AT_RISK_SUBSCRIPTIONS_SQL: &str = r#"
    SELECT
        s.id AS subscription_id,
        s.tenant_id,
        s.customer_id,
        c.name AS customer_name,
        s.current_period_end,
        COALESCE(
            (SELECT MAX(a.created_at)
             FROM appointments a
             WHERE a.tenant_id = s.tenant_id AND a.customer_id = s.customer_id),
            s.created_at
        ) AS last_activity_date
    FROM subscriptions s
    JOIN customers c ON c.tenant_id = s.tenant_id AND c.id = s.customer_id
    WHERE s.status = 'active'
      AND s.current_period_end <= NOW() + INTERVAL '7 days'
      AND s.cancel_at_period_end = FALSE
      AND COALESCE(
            (SELECT MAX(a.created_at)
             FROM appointments a
             WHERE a.tenant_id = s.tenant_id AND a.customer_id = s.customer_id),
            s.created_at
          ) <= NOW() - INTERVAL '30 days'
"#;

const POSTGRES_ENQUEUE_RETENTION_JOB_SQL: &str = "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES ($1, $2, 'agent_customer_success', $3::jsonb, 'PENDING', NOW()) ON CONFLICT DO NOTHING";

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
                        let result = async {
                            let mut transaction = db.pool.begin().await?;
                            sqlx::query("SET LOCAL ROLE ohc_bypassrls")
                                .execute(&mut *transaction)
                                .await?;
                            let rows = sqlx::query(POSTGRES_AT_RISK_SUBSCRIPTIONS_SQL)
                                .fetch_all(&mut *transaction)
                                .await?;

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

                                sqlx::query(POSTGRES_ENQUEUE_RETENTION_JOB_SQL)
                                    .bind(Uuid::new_v4().to_string())
                                    .bind(tenant_id)
                                    .bind(serde_json::json!({
                                        "event_type": "tenant.subscription.churn_risk",
                                        "payload": payload
                                    }).to_string())
                                    .execute(&mut *transaction)
                                    .await?;
                            }

                            transaction.commit().await
                        }
                        .await;
                        if let Err(error) = result {
                            tracing::warn!("subscription retention job failed: {}", error);
                        }
                    }
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

#[cfg(test)]
mod tests {
    use super::{POSTGRES_AT_RISK_SUBSCRIPTIONS_SQL, POSTGRES_ENQUEUE_RETENTION_JOB_SQL};

    #[test]
    fn retention_query_matches_the_field_operations_appointments_schema() {
        assert!(POSTGRES_AT_RISK_SUBSCRIPTIONS_SQL.contains("appointments"));
        assert!(POSTGRES_AT_RISK_SUBSCRIPTIONS_SQL.contains("created_at"));
        assert!(!POSTGRES_AT_RISK_SUBSCRIPTIONS_SQL.contains("appointment_date"));
    }

    #[test]
    fn postgres_retention_jobs_cast_bound_json_to_jsonb() {
        assert!(POSTGRES_ENQUEUE_RETENTION_JOB_SQL.contains("$3::jsonb"));
    }
}
