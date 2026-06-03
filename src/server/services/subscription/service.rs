use crate::domain::subscription::{SubscriptionPlan, Subscriber, FulfillmentBatch, FulfillmentStatus, SubscriptionStatus};
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

    pub async fn create_plan(&self, tenant_id: &str, name: &str, description: &str, amount: i64, currency: &str, interval: &str) -> Result<SubscriptionPlan, String> {
        let plan = SubscriptionPlan {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            amount,
            currency: currency.to_string(),
            interval: interval.to_string(),
            active: true,
            created_at: Utc::now().timestamp(),
        };

        let db = self.db.clone();

        sqlx::query("
            INSERT INTO subscription_plans (id, tenant_id, product_id, interval, discount_percentage, status, created_at)
            VALUES ($1, $2, $3, $4, $5, 'active', CURRENT_TIMESTAMP)
        ")
        .bind(&plan.id)
        .bind(&plan.tenant_id)
        .bind(&plan.name) // Using product_id as name for now based on schema
        .bind(&plan.interval)
        .bind(0) // default discount
        .execute(&*db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(plan)
    }

    pub async fn subscribe_customer(&self, tenant_id: &str, plan_id: &str, customer_id: &str, _stripe_sub_id: &str) -> Result<Subscriber, String> {
        let subscriber = Subscriber {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            plan_id: plan_id.to_string(),
            customer_id: customer_id.to_string(),
            stripe_subscription_id: _stripe_sub_id.to_string(),
            status: SubscriptionStatus::Active,
            current_period_end: Utc::now().timestamp() + 30 * 24 * 60 * 60, // 30 days
            created_at: Utc::now().timestamp(),
        };

        let db = self.db.clone();

        let status_str = match subscriber.status {
            SubscriptionStatus::Active => "active",
            SubscriptionStatus::Canceled => "canceled",
            SubscriptionStatus::PastDue => "past_due",
            SubscriptionStatus::Unpaid => "unpaid",
            SubscriptionStatus::Incomplete => "incomplete",
        };

        sqlx::query("
            INSERT INTO subscriptions (id, tenant_id, customer_id, plan_id, status, current_period_start, current_period_end)
            VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP + INTERVAL '1 month')
        ")
        .bind(&subscriber.id)
        .bind(&subscriber.tenant_id)
        .bind(&subscriber.customer_id)
        .bind(&subscriber.plan_id)
        .bind(status_str)
        .execute(&*db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(subscriber)
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
            SET status = 'canceled'
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
