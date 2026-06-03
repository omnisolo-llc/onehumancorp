use std::sync::Arc;
use crm::models::{Customer360, InteractionTimeline, LoyaltyLedger};
use crm::repo::CrmRepository;
use chrono::Utc;
use uuid::Uuid;

pub struct LifecycleEngine {
    repo: Arc<CrmRepository>,
}

impl LifecycleEngine {
    pub fn new(repo: Arc<CrmRepository>) -> Self {
        Self { repo }
    }

    pub async fn process_order_completed(&self, tenant_id: &str, customer_id: &str, amount: f64) -> Result<(), String> {
        // 1. Get or create customer
        let customer = self.repo.get_customer(tenant_id, customer_id).await?;
        if customer.is_none() {
            let new_cust = Customer360 {
                id: customer_id.to_string(),
                tenant_id: tenant_id.to_string(),
                email: None,
                phone: None,
                mood: "Active".to_string(),
                preferences: sqlx::types::Json(serde_json::json!({})),
                created_at: Some(Utc::now()),
                updated_at: Some(Utc::now()),
            };
            self.repo.create_customer(new_cust).await?;
        }

        // 2. Record Interaction
        let interaction = InteractionTimeline {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            customer_id: customer_id.to_string(),
            source: "OrderCompleted".to_string(),
            sentiment: "Positive".to_string(),
            occurred_at: Some(Utc::now()),
        };
        self.repo.record_interaction(interaction).await?;

        // 3. Update Loyalty Ledger
        let mut loyalty = self.repo.get_loyalty(tenant_id, customer_id).await?.unwrap_or_else(|| LoyaltyLedger {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            customer_id: customer_id.to_string(),
            points_balance: 0,
            tier_name: "Standard".to_string(),
            last_updated: Some(Utc::now()),
        });

        let points_earned = amount.floor() as i32;
        loyalty.points_balance += points_earned;

        if loyalty.points_balance >= 500 {
            loyalty.tier_name = "Top 5% Spender".to_string();
            self.repo.update_customer_mood(tenant_id, customer_id, "VIP").await?;
        } else if loyalty.points_balance >= 100 {
            loyalty.tier_name = "Frequent Buyer".to_string();
            self.repo.update_customer_mood(tenant_id, customer_id, "Active").await?;
        }

        loyalty.last_updated = Some(Utc::now());
        self.repo.upsert_loyalty(loyalty).await?;

        Ok(())
    }

    pub async fn evaluate_at_risk_customers(&self, tenant_id: &str, customer_id: &str, days_since_last_interaction: i64) -> Result<(), String> {
        if days_since_last_interaction > 21 {
             self.repo.update_customer_mood(tenant_id, customer_id, "Needs Attention").await?;
        }
        Ok(())
    }
}
