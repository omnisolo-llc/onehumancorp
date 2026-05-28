use tonic::{Request, Response, Status};
use std::sync::Arc;
// use crate::db::DB;
use chrono::Utc;
use uuid::Uuid;
use ::server_auth::orchestration::AuthInfo;

// Assuming the generated proto exposes this:
use ::finance_proto::ohc::finance::{
    finance_service_server::FinanceService, GetOffersRequest, GetOffersResponse,
    AcceptOfferRequest, AcceptOfferResponse, GetAdvancesRequest, GetAdvancesResponse,
    CapitalOffer, CapitalAdvance
};

pub struct MyFinanceService {
    // db: Arc<DB>,
}

impl MyFinanceService {
    pub fn new() -> Self {
        Self { }
    }

    // Simulate real risk calculation via DB ledger fetch
    async fn get_risk_score(&self, _tenant_id: &str) -> f32 {
        // Here we would run complex queries against UniversalWalletLedger and SalesLedger
        // like "SELECT SUM(amount) FROM ledger WHERE tenant_id = $1 AND timestamp > NOW() - INTERVAL '30 days'"
        // For this mission, we're building the core service logic shell
        85.0
    }
}

#[tonic::async_trait]
impl FinanceService for MyFinanceService {
    async fn get_offers(
        &self,
        request: Request<GetOffersRequest>,
    ) -> Result<Response<GetOffersResponse>, Status> {
        let auth_info = request.extensions().get::<AuthInfo>().cloned();
        let req = request.into_inner();

        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => req.tenant_id.clone(),
        };

        // Real logic wrapper: only offer if score is high enough
        let score = self.get_risk_score(&tenant_id).await;
        if score < 60.0 {
            return Ok(Response::new(GetOffersResponse { offers: vec![] }));
        }

        // We fetch and store these models in DB in full implementation
        let offer = CapitalOffer {
            id: format!("offer_{}", Uuid::new_v4()),
            tenant_id: tenant_id.clone(),
            offer_amount: 1500.0,
            fee_percentage: 10.0,
            repayment_rate: 8.0,
            status: "PENDING".to_string(),
            expires_at_unix: Utc::now().timestamp() + 86400 * 7,
        };

        Ok(Response::new(GetOffersResponse {
            offers: vec![offer],
        }))
    }

    async fn accept_offer(
        &self,
        request: Request<AcceptOfferRequest>,
    ) -> Result<Response<AcceptOfferResponse>, Status> {
        let auth_info = request.extensions().get::<AuthInfo>().cloned();
        let req = request.into_inner();

        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => req.tenant_id.clone(),
        };

        // Validate offer exist and belong to tenant, and generate Advance
        let advance = CapitalAdvance {
            id: format!("adv_{}", Uuid::new_v4()),
            tenant_id: tenant_id.clone(),
            offer_id: req.offer_id.clone(),
            total_amount: 1500.0,
            amount_repaid: 0.0,
            remaining_balance: 1500.0 + (1500.0 * 0.10),
            status: "ACTIVE".to_string(),
        };

        // Trigger deposit instantly into OHC Treasury Wallet
        // Execute queries into Ledger

        Ok(Response::new(AcceptOfferResponse { success: true, advance: Some(advance) }))
    }

    async fn get_advances(
        &self,
        request: Request<GetAdvancesRequest>,
    ) -> Result<Response<GetAdvancesResponse>, Status> {
        let auth_info = request.extensions().get::<AuthInfo>().cloned();
        let _req = request.into_inner();

        let _tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => _req.tenant_id.clone(),
        };

        Ok(Response::new(GetAdvancesResponse {
            advances: vec![],
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;

    #[tokio::test]
    async fn test_get_offers() {
        let service = MyFinanceService::new();

        let req = Request::new(GetOffersRequest {
            tenant_id: "test_tenant".to_string(),
        });

        let res = service.get_offers(req).await.unwrap().into_inner();
        assert_eq!(res.offers.len(), 1);
        assert_eq!(res.offers[0].tenant_id, "test_tenant");
        assert_eq!(res.offers[0].offer_amount, 1500.0);
    }
}
