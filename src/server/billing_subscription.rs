use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SubscriptionPlan {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub amount_cents: u64,
    pub currency: String,
    pub interval: String, // e.g., "month"
    pub stripe_price_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CustomerSubscription {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub plan_id: String,
    pub status: String, // "active", "past_due", "canceled"
    pub current_period_end: u64,
    pub stripe_subscription_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DunningAction {
    pub id: String,
    pub tenant_id: String,
    pub subscription_id: String,
    pub status: String, // "pending", "email_sent", "resolved"
}

pub struct SubscriptionEngine {
    pub plans: Arc<Mutex<HashMap<String, SubscriptionPlan>>>,
    pub subscriptions: Arc<Mutex<HashMap<String, CustomerSubscription>>>,
    pub dunning_actions: Arc<Mutex<HashMap<String, DunningAction>>>,
}

impl SubscriptionEngine {
    pub fn new() -> Self {
        Self {
            plans: Arc::new(Mutex::new(HashMap::new())),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            dunning_actions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_plan(&self, tenant_id: &str, name: &str, amount_cents: u64) -> SubscriptionPlan {
        let plan = SubscriptionPlan {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            name: name.to_string(),
            amount_cents,
            currency: "usd".to_string(),
            interval: "month".to_string(),
            stripe_price_id: Some(format!("price_{}", Uuid::new_v4())),
        };
        self.plans.lock().await.insert(plan.id.clone(), plan.clone());
        plan
    }

    pub async fn subscribe_customer(&self, tenant_id: &str, customer_id: &str, plan_id: &str) -> CustomerSubscription {
        let sub = CustomerSubscription {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            customer_id: customer_id.to_string(),
            plan_id: plan_id.to_string(),
            status: "active".to_string(),
            current_period_end: 0,
            stripe_subscription_id: Some(format!("sub_{}", Uuid::new_v4())),
        };
        self.subscriptions.lock().await.insert(sub.id.clone(), sub.clone());
        sub
    }

    pub async fn handle_payment_failed(&self, tenant_id: &str, subscription_id: &str) {
        let mut subs = self.subscriptions.lock().await;
        if let Some(sub) = subs.get_mut(subscription_id) {
            sub.status = "past_due".to_string();

            let action = DunningAction {
                id: Uuid::new_v4().to_string(),
                tenant_id: tenant_id.to_string(),
                subscription_id: subscription_id.to_string(),
                status: "pending".to_string(),
            };
            self.dunning_actions.lock().await.insert(action.id.clone(), action);
        }
    }

    pub async fn process_dunning(&self) -> Vec<DunningAction> {
        let mut actions = self.dunning_actions.lock().await;
        let mut processed = Vec::new();
        for (_, action) in actions.iter_mut() {
            if action.status == "pending" {
                action.status = "email_sent".to_string();
                processed.push(action.clone());
            }
        }
        processed
    }
}
