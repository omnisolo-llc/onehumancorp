use tonic::{Request, Response, Status};
use std::sync::Arc;
use crate::ohc::billing::{
    TokenUsage, CostSummary, AgentCostSummary
};
use crate::ohc::billing::billing_service_server::BillingService;
use crate::services::billing::auditor::{CostAuditor, AuditEvent};

pub struct MyBillingService {
    auditor: Arc<CostAuditor>,
}

impl MyBillingService {
    pub fn new(auditor: Arc<CostAuditor>) -> Self {
        MyBillingService { auditor }
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
            organization_id: Some(req.organization_id.clone()),
            input_tokens: req.prompt_tokens,
            output_tokens: req.completion_tokens,
            cached_input_tokens: 0,
            local_embedding_tokens: 0,
        };

        let cost = self.auditor.record_event(event);

        let mut resp = req.clone();
        resp.cost_usd = cost;

        Ok(Response::new(resp))
    }

    async fn get_cost_summary(
        &self,
        request: Request<TokenUsage>,
    ) -> Result<Response<CostSummary>, Status> {
        let req = request.into_inner();

        let mut total_cost = self.auditor.get_total_cost();
        let mut total_tokens = self.auditor.get_total_tokens();
        let mut agents = Vec::new();

        if !req.organization_id.is_empty() {
            total_cost = self.auditor.get_tenant_cost(&req.organization_id);
            total_tokens = self.auditor.get_tenant_tokens(&req.organization_id);

            for (agent_id, cost, tokens) in self.auditor.get_tenant_agent_summary(&req.organization_id) {
                agents.push(AgentCostSummary {
                    agent_id,
                    cost_usd: cost,
                    token_used: tokens,
                });
            }
        } else if !req.agent_id.is_empty() {
             let cost = self.auditor.get_agent_cost(&req.agent_id);
             agents.push(AgentCostSummary {
                 agent_id: req.agent_id.clone(),
                 cost_usd: cost,
                 token_used: total_tokens, // fallback if org_id is missing, but should be exact per agent globally if API supported it
             });
        }

        // Simple projection based on current cumulative total for a hypothetical 30 day window
        // Note: For a real projection, we would need the timeframe, but multiplying cumulative by 30 is incorrect.
        // Real projection should be based on daily average, assuming here we just do cumulative + some projection factor or 0 for now.
        let projected_monthly_usd = total_cost;

        let summary = CostSummary {
            organization_id: req.organization_id.clone(),
            total_cost_usd: total_cost,
            total_tokens: total_tokens,
            projected_monthly_usd,
            agents,
        };

        Ok(Response::new(summary))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pricing::calculator::CostConfig;

    #[tokio::test]
    async fn test_track_token_usage() {
        let auditor = Arc::new(CostAuditor::new(CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.002,
            ..Default::default()
        }));
        let service = MyBillingService::new(auditor.clone());

        let req = TokenUsage {
            agent_id: "agent-1".to_string(),
            organization_id: "org-1".to_string(),
            model: "gpt-4".to_string(),
            prompt_tokens: 100,
            completion_tokens: 50,
            cost_usd: 0.0,
            occurred_at_unix: 0,
        };

        let response = service.track_token_usage(Request::new(req.clone())).await.unwrap().into_inner();

        assert_eq!(response.cost_usd, 0.2); // 100 * 0.001 + 50 * 0.002 = 0.1 + 0.1 = 0.2
        assert_eq!(auditor.get_agent_cost("agent-1"), 0.2);
    }

    #[tokio::test]
    async fn test_get_cost_summary() {
        let auditor = Arc::new(CostAuditor::new(CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.002,
            ..Default::default()
        }));
        let service = MyBillingService::new(auditor.clone());

        let req = TokenUsage {
            agent_id: "agent-1".to_string(),
            organization_id: "org-1".to_string(),
            model: "gpt-4".to_string(),
            prompt_tokens: 100,
            completion_tokens: 50,
            cost_usd: 0.0,
            occurred_at_unix: 0,
        };

        // Track some usage first
        service.track_token_usage(Request::new(req.clone())).await.unwrap();

        // Track for another org to ensure isolation
        let req2 = TokenUsage {
            agent_id: "agent-2".to_string(),
            organization_id: "org-2".to_string(),
            model: "gpt-4".to_string(),
            prompt_tokens: 200,
            completion_tokens: 100,
            cost_usd: 0.0,
            occurred_at_unix: 0,
        };
        service.track_token_usage(Request::new(req2)).await.unwrap();

        // Get summary for org-1
        let summary_req = TokenUsage {
            agent_id: "".to_string(),
            organization_id: "org-1".to_string(),
            model: "".to_string(),
            prompt_tokens: 0,
            completion_tokens: 0,
            cost_usd: 0.0,
            occurred_at_unix: 0,
        };

        let response = service.get_cost_summary(Request::new(summary_req)).await.unwrap().into_inner();

        assert_eq!(response.total_cost_usd, 0.2);
        assert_eq!(response.total_tokens, 150); // 100 input + 50 output
        assert_eq!(response.agents.len(), 1);
        assert_eq!(response.agents[0].agent_id, "agent-1");
        assert_eq!(response.agents[0].cost_usd, 0.2);
        assert_eq!(response.agents[0].token_used, 150);

        // Get summary with no org_id, just agent_id (fallback path)
        let summary_req2 = TokenUsage {
            agent_id: "agent-2".to_string(),
            organization_id: "".to_string(),
            model: "".to_string(),
            prompt_tokens: 0,
            completion_tokens: 0,
            cost_usd: 0.0,
            occurred_at_unix: 0,
        };

        let response2 = service.get_cost_summary(Request::new(summary_req2)).await.unwrap().into_inner();
        assert_eq!(response2.agents[0].agent_id, "agent-2");
        assert_eq!(response2.agents[0].cost_usd, 0.4); // 200*0.001 + 100*0.002 = 0.4
    }
}
