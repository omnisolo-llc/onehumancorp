use tonic::{Request, Response, Status};
use ::server_ohc::billing::*;
use ::server_ohc::billing::billing_service_server::BillingService;
use crate::services::billing::auditor::{CostAuditor, AuditEvent};
use std::sync::Arc;

pub struct MyBillingService {
    auditor: Arc<CostAuditor>,
    tracker: Arc<crate::billing::Tracker>,
}

impl MyBillingService {
    pub fn new(auditor: Arc<CostAuditor>, tracker: Arc<crate::billing::Tracker>) -> Self {
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
            total_tokens: total_tokens,
            projected_monthly_usd: total_cost * 30.0, // Rough estimate
            agents,
        }))
    }

    async fn get_my_plan(
        &self,
        request: Request<GetMyPlanRequest>,
    ) -> Result<Response<GetMyPlanResponse>, Status> {
        let req = request.into_inner();
        let org_id = req.organization_id;

        let tier = self.tracker.get_tenant_tier(&org_id).await.unwrap_or(::server_pricing::rate_limit::PlanTier::Free);
        let actions_used = self.tracker.get_tenant_actions_used(&org_id).await.unwrap_or(0);
        let storage_used = self.tracker.get_tenant_storage_used(&org_id).await.unwrap_or(0);
        let ai_actions_limit = tier.monthly_action_limit().unwrap_or(0); // 0 = unlimited in proto mapping or logic
        let storage_limit_bytes = tier.storage_limit_mb().unwrap_or(0) as i64 * 1024 * 1024;

        let estimated_next_bill = match tier {
            ::server_pricing::rate_limit::PlanTier::Free => 0.0,
            ::server_pricing::rate_limit::PlanTier::Starter => 9.0,
            ::server_pricing::rate_limit::PlanTier::Pro => 29.0,
            ::server_pricing::rate_limit::PlanTier::Business => 79.0,
        };

        Ok(Response::new(GetMyPlanResponse {
            current_tier: format!("{:?}", tier),
            ai_actions_used: actions_used,
            ai_actions_limit,
            storage_used_bytes: storage_used,
            storage_limit_bytes,
            estimated_next_bill_usd: estimated_next_bill,
        }))
    }

    async fn upgrade_plan(
        &self,
        request: Request<UpgradePlanRequest>,
    ) -> Result<Response<UpgradePlanResponse>, Status> {
        let req = request.into_inner();
        let org_id = req.organization_id;

        let target_tier_enum = match req.target_tier.as_str() {
            "Starter" => ::server_pricing::rate_limit::PlanTier::Starter,
            "Pro" => ::server_pricing::rate_limit::PlanTier::Pro,
            "Business" => ::server_pricing::rate_limit::PlanTier::Business,
            _ => ::server_pricing::rate_limit::PlanTier::Free,
        };

        // In a real app we'd trigger a Stripe checkout here
        let price = match target_tier_enum {
            ::server_pricing::rate_limit::PlanTier::Free => 0.0,
            ::server_pricing::rate_limit::PlanTier::Starter => 9.0,
            ::server_pricing::rate_limit::PlanTier::Pro => 29.0,
            ::server_pricing::rate_limit::PlanTier::Business => 79.0,
        };

        let mut checkout_url = "".to_string();
        if let Some(ref client) = self.tracker.stripe_client {
            if let Ok(url) = client.create_checkout_session("price_dummy", &org_id, price).await {
                checkout_url = url;
            }
        }

        Ok(Response::new(UpgradePlanResponse {
            success: true,
            checkout_url,
        }))
    }

    async fn cancel_plan(
        &self,
        request: Request<CancelPlanRequest>,
    ) -> Result<Response<CancelPlanResponse>, Status> {
        let _req = request.into_inner();
        Ok(Response::new(CancelPlanResponse {
            success: true,
        }))
    }

    async fn get_billing_history(
        &self,
        request: Request<GetBillingHistoryRequest>,
    ) -> Result<Response<GetBillingHistoryResponse>, Status> {
        let req = request.into_inner();
        let mut history = vec![];
        if let Some(ref client) = self.tracker.stripe_client {
            if let Ok(invoices) = client.list_invoices(&req.organization_id).await {
                for inv in invoices {
                    history.push(BillingHistoryItem {
                        invoice_id: inv.id,
                        amount_usd: inv.amount_due as f64 / 100.0,
                        status: inv.status,
                        invoice_pdf_url: inv.invoice_pdf.unwrap_or_default(),
                        date_unix: 0, // Mocked for now
                    });
                }
            }
        }
        Ok(Response::new(GetBillingHistoryResponse {
            history,
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
        let tracker = Arc::new(crate::billing::Tracker::new());
        let service = MyBillingService::new(auditor.clone(), tracker);

        let req = TokenUsage {
            agent_id: "agent_x".to_string(),
            organization_id: "org_y".to_string(),
            model: "model_z".to_string(),
            prompt_tokens: 1000,
            completion_tokens: 500,
            cost_usd: 0.0,
            occurred_at_unix: 0,
        };

        let request = Request::new(req.clone());
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
        let tracker = Arc::new(crate::billing::Tracker::new());
        let service = MyBillingService::new(auditor.clone(), tracker);

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
        let _ = service.track_token_usage(Request::new(req)).await;

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

#[cfg(test)]
mod more_tests {
    use super::*;
    use ::server_pricing::calculator::CostConfig;

    #[tokio::test]
    async fn test_get_my_plan() {
        let config = CostConfig::default();
        let auditor = Arc::new(CostAuditor::new(config));
        let tracker = Arc::new(crate::billing::Tracker::new());
        let service = MyBillingService::new(auditor, tracker);

        let req = GetMyPlanRequest {
            organization_id: "org_123".to_string(),
        };

        let response = service.get_my_plan(Request::new(req)).await;
        assert!(response.is_ok());
        let plan = response.unwrap().into_inner();
        assert_eq!(plan.current_tier, "Free");
        assert_eq!(plan.ai_actions_limit, 100);
    }

    #[tokio::test]
    async fn test_upgrade_plan() {
        let config = CostConfig::default();
        let auditor = Arc::new(CostAuditor::new(config));
        let tracker = Arc::new(crate::billing::Tracker::new());
        let service = MyBillingService::new(auditor, tracker);

        let req = UpgradePlanRequest {
            organization_id: "org_123".to_string(),
            target_tier: "Pro".to_string(),
        };

        let response = service.upgrade_plan(Request::new(req)).await;
        assert!(response.is_ok());
        let result = response.unwrap().into_inner();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_cancel_plan() {
        let config = CostConfig::default();
        let auditor = Arc::new(CostAuditor::new(config));
        let tracker = Arc::new(crate::billing::Tracker::new());
        let service = MyBillingService::new(auditor, tracker);

        let req = CancelPlanRequest {
            organization_id: "org_123".to_string(),
        };

        let response = service.cancel_plan(Request::new(req)).await;
        assert!(response.is_ok());
        assert!(response.unwrap().into_inner().success);
    }
}
