use tonic::{Request, Response, Status};
use crate::ohc::billing::*;
use crate::ohc::billing::billing_service_server::BillingService;
use crate::services::billing::auditor::{CostAuditor, AuditEvent};
use std::sync::Arc;

pub struct MyBillingService {
    auditor: Arc<CostAuditor>,
}

impl MyBillingService {
    pub fn new(auditor: Arc<CostAuditor>) -> Self {
        Self { auditor }
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
        for (agent_id, cost, _roi, _eff) in self.auditor.get_agent_costs_snapshot() {
            agents.push(AgentCostSummary {
                agent_id,
                cost_usd: cost,
                token_used: 0, // Need to update Auditor to return tokens per agent too if needed
            });
        }

        Ok(Response::new(CostSummary {
            organization_id: org_id,
            total_cost_usd: total_cost,
            total_tokens: total_tokens,
            projected_monthly_usd: total_cost * 30.0, // Rough estimate
            agents,
        }))
    }
}
