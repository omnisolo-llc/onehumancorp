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
