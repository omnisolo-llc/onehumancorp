use tonic::{Request, Response, Status};
use funding_proto::ohc::funding::{
    funding_service_server::FundingService,
    ListFundingOpportunitiesRequest, ListFundingOpportunitiesResponse,
    ApproveFundingOpportunityRequest, ApproveFundingOpportunityResponse,
    FundingOpportunity as ProtoFundingOpportunity,
};
use server_services_capital::FundingEngine;
use std::sync::Arc;
use std::sync::OnceLock;
use sqlx::PgPool;
use server_utils::cache::HybridCache;

pub struct FundingApi {
    engine: Arc<FundingEngine>,
}

static FUNDING_OPPORTUNITIES_CACHE: OnceLock<HybridCache<ListFundingOpportunitiesResponse>> = OnceLock::new();

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
        let tenant_id = req.tenant_id.clone();

        let cache_key = format!("funding_opportunities:{}", tenant_id);
        let cache = FUNDING_OPPORTUNITIES_CACHE.get_or_init(|| HybridCache::new(crate::get_redis_client()));

        if let Some((cached, is_stale)) = cache.get_with_swr(&cache_key).await {
            if !is_stale {
                return Ok(Response::new(cached));
            }

            let engine_bg = self.engine.clone();
            let tenant_id_bg = tenant_id.clone();
            let cache_key_bg = cache_key.clone();
            tokio::spawn(async move {
                if let Ok(opps) = engine_bg.get_opportunities(&tenant_id_bg).await {
                    let proto_opps = opps.into_iter().map(|o| ProtoFundingOpportunity {
                        id: o.id,
                        tenant_id: o.tenant_id,
                        grant_name: o.grant_name,
                        amount: o.amount as f64,
                        draft_proposal_text: o.draft_proposal_text.unwrap_or_default(),
                        status: o.status,
                        deadline: o.deadline.unwrap_or_default(),
                    }).collect();
                    let resp = ListFundingOpportunitiesResponse {
                        opportunities: proto_opps,
                    };
                    if let Some(c) = FUNDING_OPPORTUNITIES_CACHE.get() {
                        c.set(&cache_key_bg, resp, std::time::Duration::from_secs(60)).await;
                    }
                }
            });
            return Ok(Response::new(cached));
        }

        let opps = self.engine.get_opportunities(&tenant_id).await.map_err(|e| {
            Status::internal(format!("Database error: {}", e))
        })?;

        let proto_opps = opps.into_iter().map(|o| ProtoFundingOpportunity {
            id: o.id,
            tenant_id: o.tenant_id,
            grant_name: o.grant_name,
            amount: o.amount as f64,
            draft_proposal_text: o.draft_proposal_text.unwrap_or_default(),
            status: o.status,
            deadline: o.deadline.unwrap_or_default(),
        }).collect();

        let resp = ListFundingOpportunitiesResponse {
            opportunities: proto_opps,
        };
        cache.set(&cache_key, resp.clone(), std::time::Duration::from_secs(60)).await;

        Ok(Response::new(resp))
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
