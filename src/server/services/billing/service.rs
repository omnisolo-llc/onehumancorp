use super::auditor::{AuditEvent, CostAuditor};
use server_omnisolo::billing::billing_service_server::BillingService;
use server_omnisolo::billing::*;
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct MyBillingService {
    auditor: Arc<CostAuditor>,
}

impl MyBillingService {
    pub fn new(auditor: Arc<CostAuditor>) -> Self {
        Self { auditor }
    }
}

// Keep the large transport status off the successful authorization stack path.
// RPC boundaries below unwrap it without changing the status code or message.
fn tenant_for<T>(request: &Request<T>, requested: &str) -> Result<String, Box<Status>> {
    let auth = request
        .extensions()
        .get::<::server_auth::orchestration::AuthInfo>()
        .ok_or_else(|| Status::unauthenticated("Authenticated organization required"))?;
    if auth.org_id.trim().is_empty() {
        return Err(Box::new(Status::unauthenticated(
            "Authenticated organization required",
        )));
    }
    if !requested.is_empty() && requested != auth.org_id {
        return Err(Box::new(Status::permission_denied(
            "Organization does not match authenticated identity",
        )));
    }
    Ok(auth.org_id.clone())
}

#[tonic::async_trait]
impl BillingService for MyBillingService {
    async fn track_token_usage(
        &self,
        request: Request<TokenUsage>,
    ) -> Result<Response<TokenUsage>, Status> {
        let tenant_id =
            tenant_for(&request, &request.get_ref().organization_id).map_err(|status| *status)?;
        let mut req = request.into_inner();
        if req.agent_id.trim().is_empty()
            || [req.prompt_tokens, req.completion_tokens, req.cached_tokens]
                .iter()
                .any(|value| *value < 0)
            || req
                .prompt_tokens
                .checked_add(req.completion_tokens)
                .is_none()
        {
            return Err(Status::invalid_argument(
                "Valid agent and nonnegative bounded usage required",
            ));
        }
        req.organization_id = tenant_id.clone();

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
        let org_id =
            tenant_for(&request, &request.get_ref().organization_id).map_err(|status| *status)?;
        let snapshot = self.auditor.tenant_agent_snapshot(&org_id);
        let total_cost: f64 = snapshot.iter().map(|(_, usage)| usage.cost_usd).sum();
        let total_tokens = snapshot.iter().fold(0_i64, |total, (_, usage)| {
            total.saturating_add(usage.tokens)
        });

        let mut agents = Vec::new();
        for (agent_id, usage) in snapshot {
            let cost = usage.cost_usd;
            let token_used = usage.tokens;
            let pct = if total_cost > 0.0 {
                (cost / total_cost) as f32
            } else {
                0.0
            };
            agents.push(AgentCostSummary {
                agent_id,
                cost_usd: cost,
                token_used,
                // No tenant-scoped revenue/storage source was supplied to this
                // legacy RPC; never substitute another tenant's global values.
                roi: 0.0,
                efficiency: self.auditor.calculate_efficiency(cost, token_used),
                pct,
                storage_usage_bytes: 0,
            });
        }

        Ok(Response::new(CostSummary {
            organization_id: org_id,
            total_cost_usd: total_cost,
            total_tokens,
            projected_monthly_usd: total_cost, // Observed-only until a real period is supplied.
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
            cached_tokens: 0,
        };

        let mut request = Request::new(req.clone());
        request
            .extensions_mut()
            .insert(::server_auth::orchestration::AuthInfo {
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
            cached_tokens: 0,
        };
        let mut req_req = Request::new(req);
        req_req
            .extensions_mut()
            .insert(::server_auth::orchestration::AuthInfo {
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

        let mut request = Request::new(req_summary);
        request
            .extensions_mut()
            .insert(::server_auth::orchestration::AuthInfo {
                spiffe_id: "spiffe://test".to_string(),
                org_id: "org_y".to_string(),
                agent_id: "agent_x".to_string(),
            });
        let response = service.get_cost_summary(request).await;
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
}
