use tonic::{Request, Response, Status};
use std::sync::Arc;

use crate::ohc::billing::billing_service_server::BillingService;
use crate::ohc::billing::{TokenUsage, CostSummary, AgentCostSummary};
use crate::services::billing::auditor::CostAuditor;

pub struct BillingServiceServerImpl {
    auditor: Arc<CostAuditor>,
    pool: Option<sqlx::PgPool>,
}

impl BillingServiceServerImpl {
    pub fn new(auditor: Arc<CostAuditor>, pool: Option<sqlx::PgPool>) -> Self {
        BillingServiceServerImpl { auditor, pool }
    }
}

#[tonic::async_trait]
impl BillingService for BillingServiceServerImpl {
    async fn track_token_usage(
        &self,
        request: Request<TokenUsage>,
    ) -> Result<Response<TokenUsage>, Status> {
        let req = request.into_inner();

        let event = crate::services::billing::auditor::AuditEvent {
            agent_id: req.agent_id.clone(),
            input_tokens: req.prompt_tokens,
            output_tokens: req.completion_tokens,
            cached_input_tokens: 0,
            local_embedding_tokens: 0,
        };

        let cost = self.auditor.record_event(event);

        if let Some(pool) = &self.pool {
            let total_tokens = (req.prompt_tokens + req.completion_tokens) as f32;
            let pool_clone = pool.clone();
            let model = req.model.clone();
            let org_id = req.organization_id.clone();
            let agent_id = req.agent_id.clone();
            tokio::spawn(async move {
                let _ = crate::telemetry::record_llm_token_usage(&pool_clone, total_tokens, &model, &org_id, &agent_id).await;
            });
        }

        let mut resp = req.clone();
        resp.cost_usd = cost;

        Ok(Response::new(resp))
    }

    async fn get_cost_summary(
        &self,
        request: Request<TokenUsage>,
    ) -> Result<Response<CostSummary>, Status> {
        let req = request.into_inner();

        let total_cost_usd = self.auditor.get_total_cost();
        let total_tokens = self.auditor.get_total_tokens();

        let projected_monthly_usd = total_cost_usd;

        let mut agents = Vec::new();
        if total_cost_usd > 0.0 {
            agents.push(AgentCostSummary {
                agent_id: req.agent_id.clone(),
                cost_usd: total_cost_usd,
                token_used: total_tokens,
            });
        }

        let summary = CostSummary {
            organization_id: req.organization_id,
            total_cost_usd,
            total_tokens,
            projected_monthly_usd,
            agents,
        };

        Ok(Response::new(summary))
    }
}
