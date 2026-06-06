use ::server_domain::subscription::{SubscriptionPlan, Subscriber, SubscriptionStatus};
use sqlx::PgPool as DbPool;
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
        server_lib::dispatch_critical_sms("failed_payment", message).await
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
                server_lib::minimax::MinimaxClient::new(api_key)
                    .reason(&prompt)
                    .await
                    .unwrap_or(fallback)
            }
            _ => server_lib::minimax::LocalLLMClient::new()
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

        send_dunning_sms(notifier, generator, subscriber_id, "Your business").await?;

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

#[cfg(test)]
mod tests {
    use super::*;
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
}
