use ::server_domain::subscription::{SubscriptionPlan, Subscriber, SubscriptionStatus};
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
            name: product_id.to_string(),
            description: "".to_string(),
            amount: 0,
            currency: "usd".to_string(),
            interval: interval.to_string(),
            active: true,
            created_at: Utc::now().timestamp(),
        };

        let db = self.db.clone();

        sqlx::query("
            INSERT INTO subscription_plans (id, tenant_id, product_id, interval, interval_count, status, discount_percentage)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        ")
        .bind(&plan.id)
        .bind(&plan.tenant_id)
        .bind(product_id)
        .bind(&plan.interval)
        .bind(interval_count)
        .bind("active")
        .bind(discount_percentage)
        .execute(&*db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(plan)
    }

    pub async fn subscribe_customer(&self, tenant_id: &str, plan_id: &str, customer_id: &str) -> Result<Subscriber, String> {
        let subscription = Subscriber {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            plan_id: plan_id.to_string(),
            customer_id: customer_id.to_string(),
            stripe_subscription_id: "".to_string(),
            status: SubscriptionStatus::Active,
            current_period_end: Utc::now().timestamp() + 30 * 24 * 60 * 60, // 30 days
            created_at: Utc::now().timestamp(),
        };

        let db = self.db.clone();

        let status_str = match subscription.status {
            SubscriptionStatus::Active => "active",
            SubscriptionStatus::Canceled => "canceled",
            SubscriptionStatus::PastDue => "past_due",
            _ => "active",
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
        .bind(Utc::now().timestamp() as f64)
        .bind(subscription.current_period_end as f64)
        .bind(false)
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
