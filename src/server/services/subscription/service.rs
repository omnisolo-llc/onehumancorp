use ::server_domain::subscription::{SubscriptionPlan, Subscriber, FulfillmentBatch, FulfillmentStatus, SubscriptionStatus, Entitlement, SubscriptionEvent};
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

        // Initialize entitlement and log event
        sqlx::query("
            INSERT INTO entitlements (id, tenant_id, subscription_id, credit_balance)
            VALUES ($1, $2, $3, $4)
        ")
        .bind(Uuid::new_v4().to_string())
        .bind(&subscriber.tenant_id)
        .bind(&subscriber.id)
        .bind(4) // e.g. 4 credits per month
        .execute(&*db)
        .await
        .map_err(|e| e.to_string())?;

        let event_payload = serde_json::json!({"action": "subscribe", "initial_credits": 4});
        self.log_subscription_event(&subscriber.tenant_id, &subscriber.id, "subscribed", event_payload).await?;

        Ok(subscriber)
    }

    pub async fn log_subscription_event(&self, tenant_id: &str, subscription_id: &str, event_type: &str, payload: serde_json::Value) -> Result<SubscriptionEvent, String> {
        let event = SubscriptionEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            subscription_id: subscription_id.to_string(),
            event_type: event_type.to_string(),
            event_payload: payload,
            clock: 1, // simplified CRDT clock
            signature: None,
            created_at: Utc::now().timestamp(),
        };

        let db = self.db.clone();

        sqlx::query("
            INSERT INTO subscription_events (id, tenant_id, subscription_id, event_type, event_payload, clock, signature, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ")
        .bind(&event.id)
        .bind(&event.tenant_id)
        .bind(&event.subscription_id)
        .bind(&event.event_type)
        .bind(&event.event_payload)
        .bind(event.clock)
        .bind(&event.signature)
        .bind(event.created_at)
        .execute(&*db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(event)
    }

    pub async fn sync_offline_events(&self, tenant_id: &str, events: Vec<SubscriptionEvent>) -> Result<(), String> {
        let db = self.db.clone();
        let mut tx = db.begin().await.map_err(|e| e.to_string())?;

        for event in events {
            sqlx::query("
                INSERT INTO subscription_events (id, tenant_id, subscription_id, event_type, event_payload, clock, signature, created_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (id) DO NOTHING
            ")
            .bind(&event.id)
            .bind(tenant_id)
            .bind(&event.subscription_id)
            .bind(&event.event_type)
            .bind(&event.event_payload)
            .bind(event.clock)
            .bind(&event.signature)
            .bind(event.created_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            if event.event_type == "credit_redeemed" {
                // Apply CRDT logic (decrement credit balance based on event)
                sqlx::query("
                    UPDATE entitlements
                    SET credit_balance = credit_balance - 1
                    WHERE tenant_id = $1 AND subscription_id = $2 AND credit_balance > 0
                ")
                .bind(tenant_id)
                .bind(&event.subscription_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
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
