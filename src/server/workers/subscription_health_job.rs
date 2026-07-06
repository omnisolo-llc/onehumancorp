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
                match &db.store {
                    crate::db::DbStore::Postgres => {
                        let _ = sqlx::query(
                            r#"
                            INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at)
                            SELECT gen_random_uuid()::text, tenant_id, 'subscription_health',
                                   json_build_object('subscriber_id', id, 'customer_id', customer_id)::text,
                                   'PENDING', NOW()
                            FROM subscribers
                            WHERE status IN ('ACTIVE', 'PAST_DUE')
                            ON CONFLICT DO NOTHING
                            "#
                        )
                        .execute(&db.pool)
                        .await;
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                         let rows = sqlx::query("SELECT id, tenant_id, customer_id FROM subscribers WHERE status IN ('ACTIVE', 'PAST_DUE')").fetch_all(sqlite_pool).await.unwrap_or_default();
                        for row in rows {
                            use sqlx::Row;
                            let subscriber_id: String = row.get("id");
                            let tenant_id: String = row.get("tenant_id");
                            let customer_id: String = row.get("customer_id");
                            let payload = serde_json::json!({
                                "subscriber_id": subscriber_id,
                                "customer_id": customer_id,
                            });
                            let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at) VALUES (?, ?, 'subscription_health', ?, 'PENDING', CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING")
                            .bind(Uuid::new_v4().to_string()).bind(tenant_id).bind(payload.to_string()).execute(sqlite_pool).await;
                        }
                    }
                }
            }
        });
    }
}
