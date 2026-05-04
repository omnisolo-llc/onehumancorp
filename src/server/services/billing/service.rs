use tonic::{Request, Response, Status};
use crate::ohc::billing::*;
use crate::ohc::billing::billing_service_server::BillingService;
use crate::services::billing::auditor::{CostAuditor, AuditEvent};
use std::sync::Arc;

pub struct MyBillingService {
    auditor: Arc<CostAuditor>,
    tracker: crate::billing::Tracker,
}

impl MyBillingService {
    pub fn new(auditor: Arc<CostAuditor>, tracker: crate::billing::Tracker) -> Self {
        Self { auditor, tracker }
    }
}

#[tonic::async_trait]
impl BillingService for MyBillingService {
    async fn track_token_usage(
        &self,
        request: Request<TokenUsage>,
    ) -> Result<Response<TokenUsage>, Status> {
        let req = request.into_inner();

        let event = AuditEvent {
            agent_id: req.agent_id.clone(),
            input_tokens: req.prompt_tokens,
            output_tokens: req.completion_tokens,
            cached_input_tokens: 0, // Proto doesn't have it yet, maybe add it later
            local_embedding_tokens: 0,
        };

        self.auditor.record_event(event);

        Ok(Response::new(req))
    }

    async fn get_cost_summary(
        &self,
        request: Request<TokenUsage>,
    ) -> Result<Response<CostSummary>, Status> {
        let req = request.into_inner();
        let org_id = req.organization_id;

        let total_cost = self.auditor.get_total_cost();
        let total_tokens = self.auditor.get_total_tokens();

        let mut agents = Vec::new();
        for (agent_id, cost, roi, eff) in self.auditor.get_agent_costs_snapshot() {
            let pct = if total_cost > 0.0 { (cost / total_cost) as f32 } else { 0.0 };
            agents.push(AgentCostSummary {
                agent_id,
                cost_usd: cost,
                token_used: 0, // Need to update Auditor to return tokens per agent too if needed
                roi,
                efficiency: eff,
                pct,
            });
        }

        let tier = self.tracker.get_tenant_tier(&org_id).await.unwrap_or(crate::pricing::rate_limit::PlanTier::Free);
        let tier_name = match tier {
            crate::pricing::rate_limit::PlanTier::Free => "Free",
            crate::pricing::rate_limit::PlanTier::Starter => "Starter",
            crate::pricing::rate_limit::PlanTier::Pro => "Pro",
            crate::pricing::rate_limit::PlanTier::Business => "Business",
        }.to_string();

        let action_limit = tier.monthly_action_limit().map(|l| l as i64).unwrap_or(-1);
        let storage_limit_bytes = tier.storage_limit_mb().map(|l| (l as i64) * 1024 * 1024).unwrap_or(-1);

        let (_actions_used, storage_used_bytes) = self.tracker.get_tenant_usage_stats(&org_id).await.unwrap_or((0, 0));

        Ok(Response::new(CostSummary {
            organization_id: org_id,
            total_cost_usd: total_cost,
            total_tokens: total_tokens,
            projected_monthly_usd: total_cost * 30.0, // Rough estimate
            agents,
            current_plan: tier_name,
            action_limit,
            storage_used_bytes,
            storage_limit_bytes,
            plan_status: "Active".to_string(), // Would come from Stripe
            renewal_date: "Next Month".to_string(), // Would come from Stripe
        }))
    }
}
