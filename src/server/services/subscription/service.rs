use crate::domain::subscription::{
    FulfillmentSchedule, FulfillmentStatus, SubscriptionPlan, Subscriber, SubscriptionStatus,
};
use crate::db::{DB, DbStore};
use sqlx::PgPool as DbPool;
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

#[async_trait::async_trait]
pub trait DunningNotifier: Send + Sync {
    async fn send_payment_failure_sms(&self, subscriber_id: &str, message: &str) -> Result<(), String>;
}

#[async_trait::async_trait]
pub trait DunningMessageGenerator: Send + Sync {
    async fn generate_payment_failure_message(&self, subscriber_id: &str, business_name: &str) -> String;
}

pub struct CriticalSmsDunningNotifier;
pub struct LlmDunningMessageGenerator;

#[async_trait::async_trait]
impl DunningNotifier for CriticalSmsDunningNotifier {
    async fn send_payment_failure_sms(&self, _subscriber_id: &str, message: &str) -> Result<(), String> {
        crate::dispatch_critical_sms("failed_payment", message).await
    }
}

#[async_trait::async_trait]
impl DunningMessageGenerator for LlmDunningMessageGenerator {
    async fn generate_payment_failure_message(&self, subscriber_id: &str, business_name: &str) -> String {
        let fallback = build_payment_failure_sms(business_name);
        let prompt = format!(
            "Write a concise, helpful SMS for subscription payment recovery. Business: {}. Subscriber id: {}. Mention the payment could not be processed and ask them to update their saved payment method. Avoid blame and keep it under 240 characters.",
            business_name,
            subscriber_id
        );
        match std::env::var("OHC_LLM_PROVIDER").as_deref() {
            Ok("minimax") => {
                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                crate::minimax::MinimaxClient::new(api_key)
                    .reason(&prompt)
                    .await
                    .unwrap_or(fallback)
            }
            _ => crate::minimax::LocalLLMClient::new()
                .reason(&prompt)
                .await
                .unwrap_or(fallback),
        }
    }
}

pub fn build_payment_failure_sms(business_name: &str) -> String {
    format!(
        "{} subscription payment could not be processed. Please update the saved payment method to keep the subscription active.",
        business_name
    )
}

pub async fn send_dunning_sms<N: DunningNotifier, G: DunningMessageGenerator>(
    notifier: &N,
    generator: &G,
    subscriber_id: &str,
    business_name: &str,
) -> Result<(), String> {
    let message = generator.generate_payment_failure_message(subscriber_id, business_name).await;
    notifier.send_payment_failure_sms(subscriber_id, &message).await
}

pub struct SubscriptionService {
    db: Arc<DB>,
}

impl SubscriptionService {
    pub fn new(db: Arc<DbPool>) -> Self {
        Self {
            db: Arc::new(DB {
                pool: db.as_ref().clone(),
                store: DbStore::Postgres,
            }),
        }
    }

    pub fn new_for_db(db: Arc<DB>) -> Self {
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

        self.ensure_subscription_schema().await?;

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    "INSERT INTO subscription_plans
                        (id, tenant_id, name, description, price_cents, currency, frequency, created_at, updated_at)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
                )
                .bind(&plan.id)
                .bind(&plan.tenant_id)
                .bind(&plan.name)
                .bind(&plan.description)
                .bind(plan.amount)
                .bind(&plan.currency)
                .bind(&plan.interval)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query(
                    "INSERT INTO subscription_plans
                        (id, tenant_id, name, description, price_cents, currency, frequency, created_at, updated_at)
                     VALUES (?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
                )
                .bind(&plan.id)
                .bind(&plan.tenant_id)
                .bind(&plan.name)
                .bind(&plan.description)
                .bind(plan.amount)
                .bind(&plan.currency)
                .bind(&plan.interval)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

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
            predicted_restock_date: None,
            health_score: Some(100),
            last_health_check_at: None,
            churn_risk_status: Some("healthy".to_string()),
        };

        self.ensure_subscription_schema().await?;

        let status_str = match subscriber.status {
            SubscriptionStatus::Active => "ACTIVE",
            SubscriptionStatus::Canceled => "CANCELED",
            SubscriptionStatus::PastDue => "PAST_DUE",
            SubscriptionStatus::Unpaid => "UNPAID",
            SubscriptionStatus::Incomplete => "INCOMPLETE",
        };

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    "INSERT INTO subscribers
                        (id, tenant_id, subscription_plan_id, customer_id, stripe_subscription_id, status, created_at, updated_at, predicted_restock_date)
                     VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $7)",
                )
                .bind(&subscriber.id)
                .bind(&subscriber.tenant_id)
                .bind(&subscriber.plan_id)
                .bind(&subscriber.customer_id)
                .bind(&subscriber.stripe_subscription_id)
                .bind(status_str)
                .bind(subscriber.predicted_restock_date)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query(
                    "INSERT INTO subscribers
                        (id, tenant_id, subscription_plan_id, customer_id, stripe_subscription_id, status, created_at, updated_at, predicted_restock_date)
                     VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, ?)",
                )
                .bind(&subscriber.id)
                .bind(&subscriber.tenant_id)
                .bind(&subscriber.plan_id)
                .bind(&subscriber.customer_id)
                .bind(&subscriber.stripe_subscription_id)
                .bind(status_str)
                .bind(subscriber.predicted_restock_date)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        Ok(subscriber)
    }

    pub async fn trigger_dunning(&self, subscriber_id: &str) -> Result<(), String> {
        self.trigger_dunning_with_notifier(subscriber_id, &CriticalSmsDunningNotifier).await
    }

    pub async fn trigger_dunning_with_notifier<N: DunningNotifier>(&self, subscriber_id: &str, notifier: &N) -> Result<(), String> {
        self.trigger_dunning_with_notifier_and_generator(subscriber_id, notifier, &LlmDunningMessageGenerator).await
    }

    pub async fn trigger_dunning_with_notifier_and_generator<N: DunningNotifier, G: DunningMessageGenerator>(
        &self,
        subscriber_id: &str,
        notifier: &N,
        generator: &G,
    ) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query("UPDATE subscribers SET status = 'PAST_DUE' WHERE id = $1")
                    .bind(subscriber_id)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE subscribers SET status = 'PAST_DUE' WHERE id = ?")
                    .bind(subscriber_id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        send_dunning_sms(notifier, generator, subscriber_id, "Your business").await?;

        Ok(())
    }

    pub async fn handle_stripe_webhook(&self, event_type: &str, subscription_id: &str) -> Result<(), String> {
        match event_type {
            "invoice.payment_succeeded" => {
                match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query("UPDATE subscribers SET status = 'ACTIVE', updated_at = CURRENT_TIMESTAMP WHERE stripe_subscription_id = $1")
                            .bind(subscription_id)
                            .execute(&self.db.pool)
                            .await
                            .map_err(|e| e.to_string())?;
                    }
                    crate::db::DbStore::Sqlite(pool) => {
                        sqlx::query("UPDATE subscribers SET status = 'ACTIVE', updated_at = CURRENT_TIMESTAMP WHERE stripe_subscription_id = ?")
                            .bind(subscription_id)
                            .execute(pool)
                            .await
                            .map_err(|e| e.to_string())?;
                    }
                }
            }
            "invoice.payment_failed" => {
                let subscriber_id: Option<String> = match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        let row = sqlx::query("SELECT id FROM subscribers WHERE stripe_subscription_id = $1")
                            .bind(subscription_id)
                            .fetch_optional(&self.db.pool)
                            .await
                            .map_err(|e| e.to_string())?;
                        row.map(|r| r.try_get("id").unwrap_or_default())
                    }
                    crate::db::DbStore::Sqlite(pool) => {
                        let row = sqlx::query("SELECT id FROM subscribers WHERE stripe_subscription_id = ?")
                            .bind(subscription_id)
                            .fetch_optional(pool)
                            .await
                            .map_err(|e| e.to_string())?;
                        row.map(|r| r.try_get("id").unwrap_or_default())
                    }
                };

                if let Some(sub_id) = subscriber_id {
                    self.trigger_dunning(&sub_id).await?;
                }
            }
            "customer.subscription.deleted" => {
                match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query("UPDATE subscribers SET status = 'CANCELED', updated_at = CURRENT_TIMESTAMP WHERE stripe_subscription_id = $1")
                            .bind(subscription_id)
                            .execute(&self.db.pool)
                            .await
                            .map_err(|e| e.to_string())?;
                    }
                    crate::db::DbStore::Sqlite(pool) => {
                        sqlx::query("UPDATE subscribers SET status = 'CANCELED', updated_at = CURRENT_TIMESTAMP WHERE stripe_subscription_id = ?")
                            .bind(subscription_id)
                            .execute(pool)
                            .await
                            .map_err(|e| e.to_string())?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn cancel_subscription(&self, subscriber_id: &str) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query("UPDATE subscribers SET status = 'CANCELED' WHERE id = $1")
                    .bind(subscriber_id)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE subscribers SET status = 'CANCELED' WHERE id = ?")
                    .bind(subscriber_id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    pub async fn generate_fulfillment_schedule(
        &self,
        tenant_id: &str,
        plan_id: &str,
        fulfillment_date: &str,
    ) -> Result<FulfillmentSchedule, String> {
        if tenant_id.trim().is_empty() {
            return Err("tenant id is required".to_string());
        }
        if plan_id.trim().is_empty() {
            return Err("subscription plan id is required".to_string());
        }
        if fulfillment_date.trim().is_empty() {
            return Err("fulfillment date is required".to_string());
        }

        self.ensure_subscription_schema().await?;

        let subscriber_count: i64 = match &self.db.store {
            DbStore::Postgres => {
                let row = sqlx::query(
                    "SELECT COUNT(*) AS subscriber_count
                     FROM subscribers
                     WHERE tenant_id = $1
                       AND subscription_plan_id = $2
                       AND UPPER(status) = 'ACTIVE'",
                )
                .bind(tenant_id)
                .bind(plan_id)
                .fetch_one(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
                row.try_get("subscriber_count").unwrap_or(0)
            }
            DbStore::Sqlite(pool) => {
                let row = sqlx::query(
                    "SELECT COUNT(*) AS subscriber_count
                     FROM subscribers
                     WHERE tenant_id = ?
                       AND subscription_plan_id = ?
                       AND UPPER(status) = 'ACTIVE'",
                )
                .bind(tenant_id)
                .bind(plan_id)
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;
                row.try_get("subscriber_count").unwrap_or(0)
            }
        };

        let batch = FulfillmentSchedule {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            plan_id: plan_id.to_string(),
            fulfillment_date: fulfillment_date.to_string(),
            subscriber_count,
            status: FulfillmentStatus::Pending,
            label_url: None,
            created_at: Utc::now().timestamp(),
        };

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    "INSERT INTO fulfillment_schedules
                        (id, tenant_id, subscription_plan_id, fulfillment_date, subscriber_count, status, created_at, updated_at)
                     VALUES ($1, $2, $3, $4::date, $5, 'PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
                )
                .bind(&batch.id)
                .bind(&batch.tenant_id)
                .bind(&batch.plan_id)
                .bind(&batch.fulfillment_date)
                .bind(batch.subscriber_count)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query(
                    "INSERT INTO fulfillment_schedules
                        (id, tenant_id, subscription_plan_id, fulfillment_date, subscriber_count, status, created_at, updated_at)
                     VALUES (?, ?, ?, ?, ?, 'PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
                )
                .bind(&batch.id)
                .bind(&batch.tenant_id)
                .bind(&batch.plan_id)
                .bind(&batch.fulfillment_date)
                .bind(batch.subscriber_count)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        Ok(batch)
    }

    pub fn fulfillment_schedule_event_payload(&self, batch: &FulfillmentSchedule) -> serde_json::Value {
        serde_json::json!({
            "batch_id": batch.id,
            "subscription_plan_id": batch.plan_id,
            "fulfillment_date": batch.fulfillment_date,
            "subscriber_count": batch.subscriber_count,
            "status": "PENDING",
        })
    }

    async fn ensure_subscription_schema(&self) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    "CREATE TABLE IF NOT EXISTS subscription_plans (
                        id TEXT PRIMARY KEY,
                        tenant_id TEXT NOT NULL,
                        name TEXT NOT NULL,
                        description TEXT,
                        price_cents BIGINT NOT NULL,
                        currency TEXT NOT NULL DEFAULT 'USD',
                        frequency TEXT NOT NULL,
                        cutoff_day INTEGER,
                        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
                    )",
                )
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
                sqlx::query(
                    "CREATE TABLE IF NOT EXISTS subscribers (
                        id TEXT PRIMARY KEY,
                        tenant_id TEXT NOT NULL,
                        customer_id TEXT NOT NULL,
                        subscription_plan_id TEXT NOT NULL REFERENCES subscription_plans(id) ON DELETE CASCADE,
                        status TEXT NOT NULL DEFAULT 'ACTIVE',
                        stripe_subscription_id TEXT,
                        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        predicted_restock_date BIGINT
                    )",
                )
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
                sqlx::query(
                    "CREATE TABLE IF NOT EXISTS fulfillment_schedules (
                        id TEXT PRIMARY KEY,
                        tenant_id TEXT NOT NULL,
                        subscription_plan_id TEXT NOT NULL REFERENCES subscription_plans(id) ON DELETE CASCADE,
                        fulfillment_date DATE NOT NULL,
                        subscriber_count INTEGER NOT NULL DEFAULT 0,
                        status TEXT NOT NULL DEFAULT 'PENDING',
                        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
                    )",
                )
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query(
                    "CREATE TABLE IF NOT EXISTS subscription_plans (
                        id TEXT PRIMARY KEY,
                        tenant_id TEXT NOT NULL,
                        name TEXT NOT NULL,
                        description TEXT,
                        price_cents INTEGER NOT NULL,
                        currency TEXT NOT NULL DEFAULT 'USD',
                        frequency TEXT NOT NULL,
                        cutoff_day INTEGER,
                        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                    )",
                )
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
                sqlx::query(
                    "CREATE TABLE IF NOT EXISTS subscribers (
                        id TEXT PRIMARY KEY,
                        tenant_id TEXT NOT NULL,
                        customer_id TEXT NOT NULL,
                        subscription_plan_id TEXT NOT NULL,
                        status TEXT NOT NULL DEFAULT 'ACTIVE',
                        stripe_subscription_id TEXT,
                        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        predicted_restock_date INTEGER
                    )",
                )
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
                sqlx::query(
                    "CREATE TABLE IF NOT EXISTS fulfillment_schedules (
                        id TEXT PRIMARY KEY,
                        tenant_id TEXT NOT NULL,
                        subscription_plan_id TEXT NOT NULL,
                        fulfillment_date TEXT NOT NULL,
                        subscriber_count INTEGER NOT NULL DEFAULT 0,
                        status TEXT NOT NULL DEFAULT 'PENDING',
                        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                    )",
                )
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::sync::{Arc, Mutex};

    struct RecordingDunningNotifier {
        sent: Arc<Mutex<Vec<(String, String)>>>,
    }

    struct FixedDunningMessageGenerator;

    #[async_trait::async_trait]
    impl DunningMessageGenerator for FixedDunningMessageGenerator {
        async fn generate_payment_failure_message(&self, _subscriber_id: &str, _business_name: &str) -> String {
            "LLM generated dunning response".to_string()
        }
    }

    #[async_trait::async_trait]
    impl DunningNotifier for RecordingDunningNotifier {
        async fn send_payment_failure_sms(&self, subscriber_id: &str, message: &str) -> Result<(), String> {
            self.sent.lock().unwrap().push((subscriber_id.to_string(), message.to_string()));
            Ok(())
        }
    }

    #[tokio::test]
    async fn dunning_notifier_sends_generated_payment_failure_sms() {
        let sent = Arc::new(Mutex::new(Vec::new()));
        let notifier = RecordingDunningNotifier { sent: sent.clone() };
        let generator = FixedDunningMessageGenerator;

        send_dunning_sms(&notifier, &generator, "sub_123", "Maya's Cakes").await.unwrap();

        let messages = sent.lock().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].0, "sub_123");
        assert_eq!(messages[0].1, "LLM generated dunning response");
    }

    async fn sqlite_subscription_service() -> SubscriptionService {
        let sqlite_pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let db = Arc::new(crate::db::DB {
            pool: pg_pool,
            store: crate::db::DbStore::Sqlite(sqlite_pool),
        });

        SubscriptionService::new_for_db(db)
    }

    #[tokio::test]
    async fn generate_fulfillment_schedule_counts_active_subscribers_from_db() {
        let service = sqlite_subscription_service().await;
        let tenant_id = "tenant-fulfillment";
        let plan = service
            .create_plan(
                tenant_id,
                "Monthly Coffee",
                "Physical coffee subscription",
                2900,
                "USD",
                "monthly",
            )
            .await
            .unwrap();

        service
            .subscribe_customer(tenant_id, &plan.id, "cus_active_1", "sub_active_1")
            .await
            .unwrap();
        service
            .subscribe_customer(tenant_id, &plan.id, "cus_active_2", "sub_active_2")
            .await
            .unwrap();
        let canceled = service
            .subscribe_customer(tenant_id, &plan.id, "cus_canceled", "sub_canceled")
            .await
            .unwrap();
        service.cancel_subscription(&canceled.id).await.unwrap();

        let batch = service
            .generate_fulfillment_schedule(tenant_id, &plan.id, "2026-06-15")
            .await
            .unwrap();

        assert_eq!(batch.tenant_id, tenant_id);
        assert_eq!(batch.plan_id, plan.id);
        assert_eq!(batch.status, FulfillmentStatus::Pending);
        assert_eq!(batch.subscriber_count, 2);
        assert_eq!(batch.fulfillment_date, "2026-06-15");

        let payload = service.fulfillment_schedule_event_payload(&batch);
        assert_eq!(payload["batch_id"], batch.id);
        assert_eq!(payload["subscription_plan_id"], plan.id);
        assert_eq!(payload["subscriber_count"], 2);
        assert_eq!(payload["fulfillment_date"], "2026-06-15");
    }
}
