use tonic::{Request, Response, Status};
use crate::proto::billing::*;
use crate::proto::billing::billing_service_server::BillingService;
use crate::services::billing::auditor::{CostAuditor, AuditEvent};
use std::sync::Arc;

pub struct MyBillingService {
    auditor: Arc<CostAuditor>,
    cache: std::sync::Arc<crate::utils::cache::HybridCache<CostSummary>>,
}

impl MyBillingService {
    pub fn new(auditor: Arc<CostAuditor>, redis_client: Option<redis::Client>) -> Self {
        let cache = std::sync::Arc::new(crate::utils::cache::HybridCache::new(redis_client.clone()));
        Self { auditor, cache }
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
            cached_input_tokens: req.cached_tokens,
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

        let cache_key = format!("cost_summary:{}", org_id);

        if let Some((cached, is_stale)) = self.cache.get_with_swr(&cache_key).await {
            if is_stale {
                let auditor = self.auditor.clone();
                let cache_key_bg = cache_key.clone();
                let org_id_bg = org_id.clone();
                let cache = self.cache.clone();
                tokio::spawn(async move {
                    let total_cost = auditor.get_total_cost();
                    let total_tokens = auditor.get_total_tokens();

                    let mut agents = Vec::new();
                    for (agent_id, cost, token_used, roi, eff, storage_bytes) in auditor.get_agent_costs_snapshot() {
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

                    let response = CostSummary {
                        organization_id: org_id_bg,
                        total_cost_usd: total_cost,
                        total_tokens: total_tokens,
                        projected_monthly_usd: total_cost * 30.0,
                        agents,
                    };
                    cache.set(&cache_key_bg, response, std::time::Duration::from_secs(60)).await;
                });
            }
            return Ok(Response::new(cached));
        }

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

        let response = CostSummary {
            organization_id: org_id,
            total_cost_usd: total_cost,
            total_tokens,
            projected_monthly_usd: total_cost * 30.0, // Rough estimate
            agents,
        };

        self.cache.set(&cache_key, response.clone(), std::time::Duration::from_secs(60)).await;

        Ok(Response::new(response))
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
        let service = MyBillingService::new(auditor.clone(), None);

        let req = TokenUsage {
            agent_id: "agent_x".to_string(),
            organization_id: "org_y".to_string(),
            model: "model_z".to_string(),
            prompt_tokens: 1000,
            completion_tokens: 500,
            cost_usd: 0.0,
            occurred_at_unix: 0,
            cached_tokens: 0,
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
        let service = MyBillingService::new(auditor.clone(), None);

        // Track some usage
        let req = TokenUsage {
            agent_id: "agent_x".to_string(),
            organization_id: "org_y".to_string(),
            model: "model_z".to_string(),
            prompt_tokens: 1000,
            completion_tokens: 500,
            cost_usd: 0.0,
            occurred_at_unix: 0,
            cached_tokens: 0,
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
            cached_tokens: 0,
        };

        let response = service.get_cost_summary(Request::new(req_summary)).await;
        assert!(response.is_ok());
        let summary = response.unwrap().into_inner();

        assert_eq!(summary.organization_id, "org_y");
        assert_eq!(summary.total_cost_usd, 2.0);
        assert_eq!(summary.total_tokens, 1500); // 1000 prompt + 500 completion tokens
        assert_eq!(summary.agents.len(), 1);

        let agent_summary = &summary.agents[0];
        assert_eq!(agent_summary.agent_id, "agent_x");
        assert_eq!(agent_summary.cost_usd, 2.0);
        assert_eq!(agent_summary.token_used, 1500);
        assert_eq!(agent_summary.pct, 1.0);
    }


    #[tokio::test]
    async fn test_get_cost_summary_cache() {
        let config = CostConfig::default();
        let auditor = Arc::new(CostAuditor::new(config));

        let service = MyBillingService::new(auditor.clone(), None);

        let req_summary = TokenUsage {
            agent_id: "".to_string(),
            organization_id: "org_y".to_string(),
            model: "".to_string(),
            prompt_tokens: 0,
            completion_tokens: 0,
            cost_usd: 0.0,
            occurred_at_unix: 0,
            cached_tokens: 0,
        };

        // First call should set cache
        let _ = service.get_cost_summary(Request::new(req_summary.clone())).await;

        let cache_key = format!("cost_summary:org_y");

        let cached = service.cache.get(&cache_key).await;
        assert!(cached.is_some(), "Cost summary should be cached");
        assert_eq!(cached.unwrap().organization_id, "org_y");
    }
}
