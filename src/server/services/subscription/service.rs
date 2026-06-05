use ::server_domain::subscription::{SubscriptionPlan, Subscriber, FulfillmentBatch, FulfillmentStatus, SubscriptionStatus};
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

        let q = "
            CREATE TABLE IF NOT EXISTS subscription_plans (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                amount BIGINT NOT NULL,
                currency TEXT NOT NULL,
                interval TEXT NOT NULL,
                active BOOLEAN NOT NULL,
                created_at BIGINT NOT NULL
            );
        ";

        sqlx::query(q).execute(&*db).await.map_err(|e| e.to_string())?;

        sqlx::query("
            INSERT INTO subscription_plans (id, tenant_id, name, description, amount, currency, interval, active, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ")
        .bind(&plan.id)
        .bind(&plan.tenant_id)
        .bind(&plan.name)
        .bind(&plan.description)
        .bind(plan.amount)
        .bind(&plan.currency)
        .bind(&plan.interval)
        .bind(plan.active)
        .bind(plan.created_at)
        .execute(&*db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(plan)
    }

    pub async fn subscribe_customer(&self, tenant_id: &str, plan_id: &str, customer_id: &str, stripe_sub_id: &str) -> Result<Subscriber, String> {
        let subscriber = Subscriber {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            plan_id: plan_id.to_string(),
            customer_id: customer_id.to_string(),
            stripe_subscription_id: stripe_sub_id.to_string(),
            status: SubscriptionStatus::Active,
            current_period_end: Utc::now().timestamp() + 30 * 24 * 60 * 60, // 30 days
            created_at: Utc::now().timestamp(),
        };

        let db = self.db.clone();

        let q = "
            CREATE TABLE IF NOT EXISTS subscribers (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                plan_id TEXT NOT NULL,
                customer_id TEXT NOT NULL,
                stripe_subscription_id TEXT NOT NULL,
                status TEXT NOT NULL,
                current_period_end BIGINT NOT NULL,
                created_at BIGINT NOT NULL
            );
        ";

        sqlx::query(q).execute(&*db).await.map_err(|e| e.to_string())?;

        let status_str = match subscriber.status {
            SubscriptionStatus::Active => "ACTIVE",
            SubscriptionStatus::Canceled => "CANCELED",
            SubscriptionStatus::PastDue => "PAST_DUE",
            SubscriptionStatus::Unpaid => "UNPAID",
            SubscriptionStatus::Incomplete => "INCOMPLETE",
        };

        sqlx::query("
            INSERT INTO subscribers (id, tenant_id, plan_id, customer_id, stripe_subscription_id, status, current_period_end, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ")
        .bind(&subscriber.id)
        .bind(&subscriber.tenant_id)
        .bind(&subscriber.plan_id)
        .bind(&subscriber.customer_id)
        .bind(&subscriber.stripe_subscription_id)
        .bind(status_str)
        .bind(subscriber.current_period_end)
        .bind(subscriber.created_at)
        .execute(&*db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(subscriber)
    }

    pub async fn trigger_dunning(&self, subscriber_id: &str) -> Result<(), String> {
        let db = self.db.clone();

        let q = "
            UPDATE subscribers
            SET status = 'PAST_DUE'
            WHERE id = $1
        ";

        sqlx::query(q)
            .bind(subscriber_id)
            .execute(&*db)
            .await
            .map_err(|e| e.to_string())?;

        // In a real application, this would trigger an event or queue a job
        // to send an SMS via CRM agent.

        Ok(())
    }

    pub async fn cancel_subscription(&self, subscriber_id: &str) -> Result<(), String> {
        let db = self.db.clone();

        let q = "
            UPDATE subscribers
            SET status = 'CANCELED'
            WHERE id = $1
        ";

        sqlx::query(q)
            .bind(subscriber_id)
            .execute(&*db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
