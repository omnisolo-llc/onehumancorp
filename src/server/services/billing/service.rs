use tonic::{Request, Response, Status};
use ::server_ohc::billing::*;
use ::server_ohc::billing::billing_service_server::BillingService;
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
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();

        let req = request.into_inner();

        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => req.organization_id.clone(),
        };

        let event = AuditEvent {
            agent_id: req.agent_id.clone(),
            tenant_id,
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
        for (agent_id, cost, token_used, roi, eff, storage_bytes) in self.auditor.get_agent_costs_snapshot() {
            let pct = if total_cost > 0.0 { (cost / total_cost) as f32 } else { 0.0 };
            agents.push(AgentCostSummary {
                agent_id,
                cost_usd: cost,
                token_used,
                roi,
                efficiency: eff,
                pct,
                storage_usage_bytes: storage_bytes,
            });
        }

        Ok(Response::new(CostSummary {
            organization_id: org_id,
            total_cost_usd: total_cost,
            total_tokens,
            projected_monthly_usd: total_cost * 30.0, // Rough estimate
            agents,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::server_pricing::calculator::CostConfig;

    #[tokio::test]
    async fn test_track_token_usage() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.002,
            ..Default::default()
        };
        let auditor = Arc::new(CostAuditor::new(config));
        let service = MyBillingService::new(auditor.clone());

        let req = TokenUsage {
            agent_id: "agent_x".to_string(),
            organization_id: "org_y".to_string(),
            model: "model_z".to_string(),
            prompt_tokens: 1000,
            completion_tokens: 500,
            cost_usd: 0.0,
            occurred_at_unix: 0,
        };

        let mut request = Request::new(req.clone());
        request.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "spiffe://test".to_string(),
            org_id: "org_y".to_string(),
            agent_id: "agent_x".to_string(),
        });
        let response = service.track_token_usage(request).await;

        assert!(response.is_ok());
        let resp_inner = response.unwrap().into_inner();
        assert_eq!(resp_inner.agent_id, "agent_x");

        let cost = auditor.get_agent_cost("agent_x");
        assert_eq!(cost, 2.0); // 1000*0.001 + 500*0.002 = 1.0 + 1.0 = 2.0
    }

    #[tokio::test]
    async fn test_get_cost_summary() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.002,
            ..Default::default()
        };
        let auditor = Arc::new(CostAuditor::new(config));
        let service = MyBillingService::new(auditor.clone());

        // Track some usage
        let req = TokenUsage {
            agent_id: "agent_x".to_string(),
            organization_id: "org_y".to_string(),
            model: "model_z".to_string(),
            prompt_tokens: 1000,
            completion_tokens: 500,
            cost_usd: 0.0,
            occurred_at_unix: 0,
        };
        let mut req_req = Request::new(req);
        req_req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "spiffe://test".to_string(),
            org_id: "org_y".to_string(),
            agent_id: "agent_x".to_string(),
        });
        let _ = service.track_token_usage(req_req).await;

        let req_summary = TokenUsage {
            agent_id: "".to_string(),
            organization_id: "org_y".to_string(),
            model: "".to_string(),
            prompt_tokens: 0,
            completion_tokens: 0,
            cost_usd: 0.0,
            occurred_at_unix: 0,
        };

        let response = service.get_cost_summary(Request::new(req_summary)).await;
        assert!(response.is_ok());
        let summary = response.unwrap().into_inner();

        assert_eq!(summary.organization_id, "org_y");
        assert_eq!(summary.total_cost_usd, 2.0);
        assert_eq!(summary.total_tokens, 500); // 500 completion tokens
        assert_eq!(summary.agents.len(), 1);

        let agent_summary = &summary.agents[0];
        assert_eq!(agent_summary.agent_id, "agent_x");
        assert_eq!(agent_summary.cost_usd, 2.0);
        assert_eq!(agent_summary.token_used, 500);
        assert_eq!(agent_summary.pct, 1.0);
    }
}
