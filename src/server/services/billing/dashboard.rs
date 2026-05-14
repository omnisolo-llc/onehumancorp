use std::sync::Arc;
use crate::services::billing::auditor::CostAuditor;

pub struct CostTransparencyDashboard {
    auditor: Arc<CostAuditor>,
    tracker: Arc<crate::billing::Tracker>,
}

impl CostTransparencyDashboard {
    pub fn new(auditor: Arc<CostAuditor>, tracker: Arc<crate::billing::Tracker>) -> Self {
        Self { auditor, tracker }
    }

    pub async fn generate_summary(&self, organization_id: &str) -> String {
        let tier = self.tracker.get_tenant_tier(organization_id).await.unwrap_or(::server_pricing::rate_limit::PlanTier::Free);
        let actions_used = self.tracker.get_tenant_actions_used(organization_id).await.unwrap_or(0);
        let storage_used = self.tracker.get_tenant_storage_used(organization_id).await.unwrap_or(0);

        let ai_actions_limit = tier.monthly_action_limit();
        let storage_limit_mb = tier.storage_limit_mb().unwrap_or(500);

        let ai_status = match ai_actions_limit {
            Some(limit) => format!("You have used {} out of {} AI actions this month.", actions_used, limit),
            None => format!("You have used {} AI actions this month. (Unlimited actions on your current plan)", actions_used),
        };

        let storage_used_mb = storage_used / (1024 * 1024);
        let storage_status = format!("You are currently using {} MB out of {} MB of your storage quota.", storage_used_mb, storage_limit_mb);

        let estimated_bill = match tier {
            ::server_pricing::rate_limit::PlanTier::Free => 0.0,
            ::server_pricing::rate_limit::PlanTier::Starter => 9.0,
            ::server_pricing::rate_limit::PlanTier::Pro => 29.0,
            ::server_pricing::rate_limit::PlanTier::Business => 79.0,
        };

        let total_cost = self.auditor.get_total_cost();
        let infrastructure_cost_summary = format!("Total Infrastructure Cost Generated: ${:.2}", total_cost);

        format!(
            "# My Plan Dashboard\n\n\
            **Current Plan:** {:?}\n\n\
            {}\n\n\
            {}\n\n\
            **Estimated Next Bill:** ${:.2}\n\n\
            ---\n\
            *Infrastructure Insights:*\n\
            {}\n\
            ",
            tier, ai_status, storage_status, estimated_bill, infrastructure_cost_summary
        )
    }
}
