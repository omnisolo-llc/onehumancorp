use std::time::Duration;
use tokio::time::sleep;

pub struct SubscriptionDunningWorker {
    pub pool: sqlx::PgPool,
}

impl SubscriptionDunningWorker {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn run(&self) {
        loop {
            if let Err(e) = self.process_jobs().await {
                tracing::error!("Error processing subscription dunning jobs: {}", e);
            }
            sleep(Duration::from_secs(60)).await;
        }
    }

    pub async fn process_jobs(&self) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // 1. Fetch pending subscriptions that need payment failure retries using SKIP LOCKED
        let rows = sqlx::query(
            "SELECT id, tenant_id, customer_id FROM subscriptions
             WHERE status = 'past_due' AND updated_at < CURRENT_TIMESTAMP - INTERVAL '1 day'
             ORDER BY updated_at ASC
             LIMIT 5
             FOR UPDATE SKIP LOCKED"
        )
        .fetch_all(&mut *tx)
        .await?;

        for row in rows {
            use sqlx::Row;
            let id: String = row.get("id");
            let tenant_id: String = row.get("tenant_id");
            let customer_id: String = row.get("customer_id");

            tracing::info!("Triggering AI dunning workflow for subscription {}, tenant {}, customer {}", id, tenant_id, customer_id);

            // Queue a job for the Finance & Payments agent to send an SMS/Email
            let job_id = uuid::Uuid::new_v4().to_string();
            let payload = serde_json::json!({
                "action": "send_dunning_notice",
                "subscription_id": id,
                "customer_id": customer_id,
            });

            sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
                 VALUES ($1, $2, 'finance_agent_dunning', $3)"
            )
            .bind(&job_id)
            .bind(&tenant_id)
            .bind(payload)
            .execute(&mut *tx)
            .await?;

            // Update subscription to avoid immediate re-polling
            sqlx::query("UPDATE subscriptions SET updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                .bind(&id)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_jobs_no_error() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = sqlx::postgres::PgPoolOptions::new().connect(&database_url).await.unwrap();

        // Setup data for test
        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-dunning', 'Dunning Test Tenant') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        sqlx::query("
            CREATE TABLE IF NOT EXISTS subscription_plans (
                id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, product_id TEXT NOT NULL, interval TEXT NOT NULL,
                interval_count INTEGER NOT NULL DEFAULT 1, status TEXT NOT NULL DEFAULT 'active',
                discount_percentage INTEGER DEFAULT 0, created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS subscriptions (
                id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, customer_id TEXT NOT NULL, plan_id TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'active', current_period_start TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                current_period_end TIMESTAMPTZ NOT NULL, cancel_at_period_end BOOLEAN NOT NULL DEFAULT FALSE,
                canceled_at TIMESTAMPTZ, created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            INSERT INTO subscription_plans (id, tenant_id, product_id, interval) VALUES ('plan-1', 'tenant-dunning', 'prod-1', 'monthly') ON CONFLICT DO NOTHING;
        ").execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO subscriptions (id, tenant_id, customer_id, plan_id, status, current_period_end, updated_at)
             VALUES ('sub-1', 'tenant-dunning', 'cust-1', 'plan-1', 'past_due', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP - INTERVAL '2 days')
             ON CONFLICT DO NOTHING")
             .execute(&pool).await.unwrap();

        let worker = SubscriptionDunningWorker::new(pool.clone());
        let result = worker.process_jobs().await;
        assert!(result.is_ok());

        // Verify the job was created
        let (job_count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_job_queue WHERE job_type = 'finance_agent_dunning'")
            .fetch_one(&pool).await.unwrap();
        assert!(job_count > 0);

        // Verify the subscription updated_at was bumped
        let (updated_recent,): (bool,) = sqlx::query_as("SELECT updated_at > CURRENT_TIMESTAMP - INTERVAL '1 hour' FROM subscriptions WHERE id = 'sub-1'")
            .fetch_one(&pool).await.unwrap();
        assert!(updated_recent);
    }
}
