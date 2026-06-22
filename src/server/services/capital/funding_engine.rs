use tonic::{Request, Response, Status};
use server_ohc::app::funding_engine_service_server::FundingEngineService;
use server_ohc::app::{
    GetFundingOpportunitiesRequest, GetFundingOpportunitiesResponse,
    SubmitFundingOpportunityRequest, SubmitFundingOpportunityResponse, FundingOpportunity,
};
use uuid::Uuid;

pub struct FundingEngineImpl {}

impl FundingEngineImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl FundingEngineService for FundingEngineImpl {
    async fn get_funding_opportunities(
        &self,
        request: Request<GetFundingOpportunitiesRequest>,
    ) -> Result<Response<GetFundingOpportunitiesResponse>, Status> {
        let _req = request.into_inner();

        let mut opportunities = Vec::new();
        // Return a mocked opportunity
        opportunities.push(FundingOpportunity {
            id: Uuid::new_v4().to_string(),
            tenant_id: "test-tenant".to_string(),
            grant_name: "Downtown Revitalization Grant".to_string(),
            amount: 10000,
            draft_proposal_text: "You have a 92% match based on your location and revenue. The Legal Agent has drafted the required 500-word essay detailing how you will use the funds for a new oven.".to_string(),
            status: "Drafted".to_string(),
            deadline: "2024-12-31".to_string(),
        });

        Ok(Response::new(GetFundingOpportunitiesResponse { opportunities }))
    }

    async fn submit_funding_opportunity(
        &self,
        request: Request<SubmitFundingOpportunityRequest>,
    ) -> Result<Response<SubmitFundingOpportunityResponse>, Status> {
        let _req = request.into_inner();
        Ok(Response::new(SubmitFundingOpportunityResponse { success: true }))
    }
}
