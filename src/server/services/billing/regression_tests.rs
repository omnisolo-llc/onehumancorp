use super::auditor::{AuditEvent, CostAuditor, event_pipeline};
use super::service::MyBillingService;
use server_auth::orchestration::AuthInfo;
use server_omnisolo::billing::{TokenUsage, billing_service_server::BillingService};
use server_pricing::calculator::CostConfig;
use std::sync::Arc;
use tonic::{Code, Request};

fn config() -> CostConfig {
    CostConfig {
        cost_per_input_token: 0.001,
        cost_per_output_token: 0.002,
        ..Default::default()
    }
}
fn usage(tenant: &str, input: i64) -> TokenUsage {
    TokenUsage {
        organization_id: tenant.into(),
        agent_id: "shared-agent-name".into(),
        model: "test-model".into(),
        prompt_tokens: input,
        completion_tokens: 0,
        cached_tokens: 0,
        cost_usd: 0.0,
        occurred_at_unix: 0,
    }
}
fn authenticated(value: TokenUsage, tenant: &str) -> Request<TokenUsage> {
    let mut request = Request::new(value);
    request.extensions_mut().insert(AuthInfo {
        spiffe_id: "spiffe://test/worker".into(),
        org_id: tenant.into(),
        agent_id: "shared-agent-name".into(),
    });
    request
}

#[tokio::test]
async fn ingress_is_accounted_once_and_exports_do_not_feed_back() {
    let (auditor, ingress, mut exports) = event_pipeline(config());
    ingress
        .send(AuditEvent {
            tenant_id: "tenant-a".into(),
            agent_id: "shared".into(),
            input_tokens: 100,
            output_tokens: 50,
            cached_input_tokens: 0,
            local_embedding_tokens: 0,
        })
        .unwrap();
    let output = tokio::time::timeout(std::time::Duration::from_secs(2), exports.recv())
        .await
        .expect("usage pipeline stalled")
        .expect("export missing");
    assert_eq!(output.event.input_tokens, 100);
    assert_eq!(output.cost_usd, 0.2);
    for _ in 0..4 {
        tokio::task::yield_now().await;
    }
    assert_eq!(auditor.get_tenant_tokens("tenant-a"), 150);
    assert_eq!(auditor.get_tenant_cost("tenant-a"), 0.2);
    assert!(matches!(
        exports.try_recv(),
        Err(tokio::sync::mpsc::error::TryRecvError::Empty)
    ));
    drop(ingress);
    drop(auditor);
    assert!(
        tokio::time::timeout(std::time::Duration::from_secs(2), exports.recv())
            .await
            .expect("pipeline did not close")
            .is_none()
    );
}

#[tokio::test]
async fn tenant_summary_isolates_identical_agent_ids() {
    let service = MyBillingService::new(Arc::new(CostAuditor::new(config())));
    service
        .track_token_usage(authenticated(usage("tenant-a", 100), "tenant-a"))
        .await
        .unwrap();
    service
        .track_token_usage(authenticated(usage("tenant-b", 900), "tenant-b"))
        .await
        .unwrap();
    for (tenant, expected_tokens, expected_cost) in [("tenant-a", 100, 0.1), ("tenant-b", 900, 0.9)]
    {
        let response = service
            .get_cost_summary(authenticated(usage(tenant, 0), tenant))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(response.organization_id, tenant);
        assert_eq!(response.total_tokens, expected_tokens);
        assert_eq!(response.total_cost_usd, expected_cost);
        assert_eq!(response.agents.len(), 1);
        assert_eq!(response.agents[0].token_used, expected_tokens);
        assert_eq!(response.agents[0].cost_usd, expected_cost);
    }
}

#[tokio::test]
async fn tenant_summary_rejects_missing_blank_and_mismatched_identity() {
    let service = MyBillingService::new(Arc::new(CostAuditor::new(config())));
    assert_eq!(
        service
            .get_cost_summary(Request::new(usage("tenant-a", 0)))
            .await
            .unwrap_err()
            .code(),
        Code::Unauthenticated
    );
    assert_eq!(
        service
            .get_cost_summary(authenticated(usage("tenant-a", 0), ""))
            .await
            .unwrap_err()
            .code(),
        Code::Unauthenticated
    );
    assert_eq!(
        service
            .get_cost_summary(authenticated(usage("tenant-b", 0), "tenant-a"))
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );
    assert_eq!(
        service
            .track_token_usage(authenticated(usage("tenant-b", 1), "tenant-a"))
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );
}

#[tokio::test]
async fn invalid_usage_cannot_decrease_counters_or_overflow() {
    let auditor = Arc::new(CostAuditor::new(config()));
    let service = MyBillingService::new(auditor.clone());
    assert_eq!(
        service
            .track_token_usage(authenticated(usage("tenant-a", -1), "tenant-a"))
            .await
            .unwrap_err()
            .code(),
        Code::InvalidArgument
    );
    let mut overflow = usage("tenant-a", i64::MAX);
    overflow.completion_tokens = 1;
    assert_eq!(
        service
            .track_token_usage(authenticated(overflow, "tenant-a"))
            .await
            .unwrap_err()
            .code(),
        Code::InvalidArgument
    );
    assert_eq!(auditor.get_tenant_tokens("tenant-a"), 0);
    assert!(auditor.tenant_agent_snapshot("tenant-a").is_empty());
}
