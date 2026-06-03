use ::server_domain::subscription::{SubscriptionPlan, Subscription, FulfillmentBatch, FulfillmentStatus, SubscriptionStatus};
use sqlx::PgPool as DbPool;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct SubscriptionService {
    db: Arc<DbPool>,
}

impl SubscriptionService {
    pub fn new(db: Arc<DbPool>) -> Self {
        Self { db }
    }

    pub async fn create_plan(&self, tenant_id: &str, product_id: &str, interval: &str, interval_count: i32, discount_percentage: i32) -> Result<SubscriptionPlan, String> {
        let plan = SubscriptionPlan {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            product_id: product_id.to_string(),
            interval: interval.to_string(),
            interval_count,
            status: "active".to_string(),
            discount_percentage,
            created_at: Utc::now().timestamp(),
            updated_at: Utc::now().timestamp(),
        };

        let db = self.db.clone();

        sqlx::query("
            INSERT INTO subscription_plans (id, tenant_id, product_id, interval, interval_count, status, discount_percentage, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, to_timestamp($8), to_timestamp($9))
        ")
        .bind(&plan.id)
        .bind(&plan.tenant_id)
        .bind(&plan.product_id)
        .bind(&plan.interval)
        .bind(plan.interval_count)
        .bind(&plan.status)
        .bind(plan.discount_percentage)
        .bind(plan.created_at)
        .bind(plan.updated_at)
        .execute(&*db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(plan)
    }

    pub async fn create_subscription(&self, tenant_id: &str, customer_id: &str, plan_id: &str) -> Result<Subscription, String> {
        let now = Utc::now().timestamp();
        // Just mock the period_end by adding 30 days for testing
        let current_period_end = now + (30 * 24 * 60 * 60);

        let sub = Subscription {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            customer_id: customer_id.to_string(),
            plan_id: plan_id.to_string(),
            status: "active".to_string(),
            current_period_start: now,
            current_period_end,
            cancel_at_period_end: false,
            canceled_at: None,
            created_at: now,
            updated_at: now,
        };

        let db = self.db.clone();

        sqlx::query("
            INSERT INTO subscriptions (id, tenant_id, customer_id, plan_id, status, current_period_start, current_period_end, cancel_at_period_end, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, to_timestamp($6), to_timestamp($7), $8, to_timestamp($9), to_timestamp($10))
        ")
        .bind(&sub.id)
        .bind(&sub.tenant_id)
        .bind(&sub.customer_id)
        .bind(&sub.plan_id)
        .bind(&sub.status)
        .bind(sub.current_period_start)
        .bind(sub.current_period_end)
        .bind(sub.cancel_at_period_end)
        .bind(sub.created_at)
        .bind(sub.updated_at)
        .execute(&*db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(sub)
    }

    pub async fn pause_subscription(&self, tenant_id: &str, subscription_id: &str) -> Result<(), String> {
        let db = self.db.clone();
        sqlx::query("UPDATE subscriptions SET status = 'paused', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
            .bind(subscription_id)
            .bind(tenant_id)
            .execute(&*db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn resume_subscription(&self, tenant_id: &str, subscription_id: &str) -> Result<(), String> {
        let db = self.db.clone();
        sqlx::query("UPDATE subscriptions SET status = 'active', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
            .bind(subscription_id)
            .bind(tenant_id)
            .execute(&*db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn cancel_subscription(&self, tenant_id: &str, subscription_id: &str) -> Result<(), String> {
        let db = self.db.clone();
        sqlx::query("UPDATE subscriptions SET status = 'canceled', cancel_at_period_end = TRUE, canceled_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
            .bind(subscription_id)
            .bind(tenant_id)
            .execute(&*db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subscription_service() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();

        let pool = Arc::new(pool);
        let service = SubscriptionService::new(pool);

        let tenant_id = "test_tenant";
        let product_id = "test_product";

        let plan = service.create_plan(tenant_id, product_id, "monthly", 1, 0).await.unwrap();
        assert_eq!(plan.tenant_id, tenant_id);

        let sub = service.create_subscription(tenant_id, "cust_1", &plan.id).await.unwrap();
        assert_eq!(sub.status, "active");

        service.pause_subscription(tenant_id, &sub.id).await.unwrap();
        service.resume_subscription(tenant_id, &sub.id).await.unwrap();
        service.cancel_subscription(tenant_id, &sub.id).await.unwrap();
    }
}
