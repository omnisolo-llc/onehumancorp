use super::funding_engine::*;
use server_ohc::app::funding_engine_service_server::FundingEngineService;
use server_ohc::app::{GetFundingOpportunitiesRequest, SubmitFundingOpportunityRequest};
use tonic::Request;

#[tokio::test]
async fn test_get_funding_opportunities() {
    let engine = FundingEngineImpl::new();
    let request = Request::new(GetFundingOpportunitiesRequest {
        tenant_id: "test-tenant".to_string(),
    });

    let response = engine.get_funding_opportunities(request).await.unwrap().into_inner();

    assert_eq!(response.opportunities.len(), 1);
    assert_eq!(response.opportunities[0].grant_name, "Downtown Revitalization Grant");
    assert_eq!(response.opportunities[0].amount, 10000);
    assert_eq!(response.opportunities[0].status, "Drafted");
}

#[tokio::test]
async fn test_submit_funding_opportunity() {
    let engine = FundingEngineImpl::new();
    let request = Request::new(SubmitFundingOpportunityRequest {
        tenant_id: "test-tenant".to_string(),
        opportunity_id: "test-id".to_string(),
    });

    let response = engine.submit_funding_opportunity(request).await.unwrap().into_inner();
    assert_eq!(response.success, true);
}
