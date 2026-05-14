use crate::miser::{MiserRecommendation, get_active_recommendations};
use crate::steering::{steer_request, SteeringConfig, ModelTier};
use crate::budget::BudgetManager;

pub struct MiserEngine {
    pub config: SteeringConfig,
}

impl MiserEngine {
    pub fn new() -> Self {
        Self {
            config: SteeringConfig::default(),
        }
    }

    pub fn get_recommendations(&self, tenant_id: &str, budget: &BudgetManager) -> Vec<MiserRecommendation> {
        let mut recs = get_active_recommendations();

        let remaining = budget.get_remaining_cents();
        if remaining < 100 {
            recs.push(MiserRecommendation {
                id: "low_budget_warning".to_string(),
                title: "Budget Running Low".to_string(),
                description: "Your monthly AI budget is almost exhausted. We've automatically switched you to Economy models to keep things running.".to_string(),
                impact: "Prevent service interruption".to_string(),
                action_label: "Add Funds".to_string(),
                action_type: "BUDGET_MANAGEMENT".to_string(),
                potential_savings_cents: 0,
                priority: 0,
            });
        }

        recs
    }

    pub fn select_model(&self, prompt: &str, budget: &BudgetManager) -> ModelTier {
        steer_request(prompt, budget.get_remaining_cents(), &self.config)
    }
}
