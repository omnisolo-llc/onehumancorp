use ::server_domain::subscription::{SubscriptionPlan, Subscription, SubscriptionStatus};
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
            INSERT INTO subscription_plans (id, tenant_id, product_id, interval, interval_count, status, discount_percentage)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        ")
        .bind(&plan.id)
        .bind(&plan.tenant_id)
        .bind(&plan.product_id)
        .bind(&plan.interval)
        .bind(plan.interval_count)
        .bind(&plan.status)
        .bind(plan.discount_percentage)
        .execute(&*db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(plan)
    }

    pub async fn subscribe_customer(&self, tenant_id: &str, plan_id: &str, customer_id: &str) -> Result<Subscription, String> {
        let subscription = Subscription {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            plan_id: plan_id.to_string(),
            customer_id: customer_id.to_string(),
            status: SubscriptionStatus::Active,
            current_period_start: Utc::now().timestamp(),
            current_period_end: Utc::now().timestamp() + 30 * 24 * 60 * 60, // 30 days
            cancel_at_period_end: false,
            canceled_at: None,
            created_at: Utc::now().timestamp(),
            updated_at: Utc::now().timestamp(),
        };

        let db = self.db.clone();

        let status_str = match subscription.status {
            SubscriptionStatus::Active => "active",
            SubscriptionStatus::Canceled => "canceled",
            SubscriptionStatus::PastDue => "past_due",
            SubscriptionStatus::Paused => "paused",
        };

        sqlx::query("
            INSERT INTO subscriptions (id, tenant_id, customer_id, plan_id, status, current_period_start, current_period_end, cancel_at_period_end)
            VALUES ($1, $2, $3, $4, $5, to_timestamp($6), to_timestamp($7), $8)
        ")
        .bind(&subscription.id)
        .bind(&subscription.tenant_id)
        .bind(&subscription.customer_id)
        .bind(&subscription.plan_id)
        .bind(status_str)
        .bind(subscription.current_period_start as f64)
        .bind(subscription.current_period_end as f64)
        .bind(subscription.cancel_at_period_end)
        .execute(&*db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(subscription)
    }

    pub async fn trigger_dunning(&self, subscription_id: &str) -> Result<(), String> {
        let db = self.db.clone();

        let q = "
            UPDATE subscriptions
            SET status = 'past_due'
            WHERE id = $1
        ";

        sqlx::query(q)
            .bind(subscription_id)
            .execute(&*db)
            .await
            .map_err(|e| e.to_string())?;

        // In a real application, this would trigger an event or queue a job
        // to send an SMS via CRM agent.

        Ok(())
    }

    pub async fn cancel_subscription(&self, subscription_id: &str) -> Result<(), String> {
        let db = self.db.clone();

        let q = "
            UPDATE subscriptions
            SET status = 'canceled', canceled_at = CURRENT_TIMESTAMP
            WHERE id = $1
        ";

        sqlx::query(q)
            .bind(subscription_id)
            .execute(&*db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
