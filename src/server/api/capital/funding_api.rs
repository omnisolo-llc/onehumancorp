use tonic::{Request, Response, Status};
use funding_prost::{
    funding_service_server::FundingService,
    ListFundingOpportunitiesRequest, ListFundingOpportunitiesResponse,
    ApproveFundingOpportunityRequest, ApproveFundingOpportunityResponse,
    FundingOpportunity as ProtoFundingOpportunity,
};
use server_services_capital::funding_engine::FundingEngine;
use std::sync::Arc;
use sqlx::PgPool;

pub struct FundingApi {
    engine: Arc<FundingEngine>,
}

impl FundingApi {
    pub fn new(pool: PgPool) -> Self {
        Self {
            engine: Arc::new(FundingEngine::new(pool)),
        }
    }
}

#[tonic::async_trait]
impl FundingService for FundingApi {
    async fn list_funding_opportunities(
        &self,
        request: Request<ListFundingOpportunitiesRequest>,
    ) -> Result<Response<ListFundingOpportunitiesResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = req.tenant_id;

        let opps = self.engine.get_opportunities(&tenant_id).await.map_err(|e| {
            Status::internal(format!("Database error: {}", e))
        })?;

        let proto_opps = opps.into_iter().map(|o| ProtoFundingOpportunity {
            id: o.id,
            tenant_id: o.tenant_id,
            grant_name: o.grant_name,
            amount: o.amount,
            draft_proposal_text: o.draft_proposal_text.unwrap_or_default(),
            status: o.status,
            deadline: o.deadline.map(|d| d.to_rfc3339()).unwrap_or_default(),
        }).collect();

        Ok(Response::new(ListFundingOpportunitiesResponse {
            opportunities: proto_opps,
        }))
    }

    async fn approve_funding_opportunity(
        &self,
        request: Request<ApproveFundingOpportunityRequest>,
    ) -> Result<Response<ApproveFundingOpportunityResponse>, Status> {
        let req = request.into_inner();

        let success = self.engine.approve_opportunity(&req.tenant_id, &req.opportunity_id).await.map_err(|e| {
            Status::internal(format!("Database error: {}", e))
        })?;

        Ok(Response::new(ApproveFundingOpportunityResponse {
            success,
        }))
    }
}
