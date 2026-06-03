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
}
