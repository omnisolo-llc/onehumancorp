use tonic::{Request, Response, Status};
use ::server_ohc::capital::*;
use ::server_ohc::capital::capital_service_server::CapitalService;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use ::server_common::auth_utils::set_org_context;

use crate::services::capital::engine::CapitalEngine;
use crate::services::capital::forecaster::CashFlowForecaster;

pub struct MyCapitalService {
    pool: PgPool,
    engine: Arc<CapitalEngine>,
    forecaster: Arc<CashFlowForecaster>,
}

impl MyCapitalService {
    pub fn new(pool: PgPool) -> Self {
        let engine = Arc::new(CapitalEngine::new(pool.clone()));
        let forecaster = Arc::new(CashFlowForecaster::new(pool.clone()));
        
        Self {
            pool,
            engine,
            forecaster,
        }
    }

    async fn get_org_id(&self, metadata: &tonic::metadata::MetadataMap) -> Result<String, Status> {
        let spiffe_id_str = metadata.get("x-spiffe-id")
            .ok_or_else(|| Status::unauthenticated("missing x-spiffe-id header"))?
            .to_str()
            .map_err(|_| Status::unauthenticated("invalid x-spiffe-id header"))?;

        let (org_id, _) = ::server_auth::parse_spiffe_id(spiffe_id_str)?;

        Ok(org_id)
    }

    async fn get_business_id(&self, tenant_id: &str) -> Result<String, Status> {
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let row = sqlx::query("SELECT id FROM businesses WHERE tenant_id = $1 LIMIT 1")
            .bind(tenant_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(r.get("id")),
            None => Err(Status::not_found("No business found for this tenant")),
        }
    }
}

#[tonic::async_trait]
impl CapitalService for MyCapitalService {
    async fn get_cash_flow_predictions(
        &self,
        request: Request<GetCashFlowRequest>,
    ) -> Result<Response<CashFlowResponse>, Status> {
        let tenant_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();
        
        let business_id = if req.business_id.is_empty() {
            self.get_business_id(&tenant_id).await?
        } else {
            req.business_id
        };

        let days_ahead = if req.days_ahead > 0 { req.days_ahead } else { 30 };

        let response = self.forecaster.get_cash_flow_predictions(&tenant_id, &business_id, days_ahead).await?;

        Ok(Response::new(response))
    }

    async fn get_capital_offers(
        &self,
        request: Request<GetCapitalOffersRequest>,
    ) -> Result<Response<CapitalOffersResponse>, Status> {
        let tenant_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();
        
        let business_id = if req.business_id.is_empty() {
            self.get_business_id(&tenant_id).await?
        } else {
            req.business_id
        };

        let response = self.engine.get_capital_offers(&tenant_id, &business_id).await?;

        Ok(Response::new(response))
    }

    async fn accept_capital_offer(
        &self,
        request: Request<AcceptCapitalOfferRequest>,
    ) -> Result<Response<AcceptOfferResponse>, Status> {
        let tenant_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();

        let response = self.engine.accept_capital_offer(&tenant_id, &req.offer_id).await?;

        Ok(Response::new(response))
    }

    async fn record_sales_transaction(
        &self,
        request: Request<RecordSaleRequest>,
    ) -> Result<Response<RecordSaleResponse>, Status> {
        let tenant_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();
        
        let business_id = if req.business_id.is_empty() {
            self.get_business_id(&tenant_id).await?
        } else {
            req.business_id
        };

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, &tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let transaction_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO sales_transactions (id, tenant_id, business_id, transaction_date, amount, transaction_type, description, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&transaction_id)
        .bind(&tenant_id)
        .bind(&business_id)
        .bind(now)
        .bind(req.amount)
        .bind(&req.transaction_type)
        .bind(&req.description)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        // Process automatic repayments if there are active advances
        let _ = self.engine.process_automatic_repayment(&tenant_id, &business_id, req.amount).await;

        let transaction = SalesTransaction {
            id: transaction_id,
            tenant_id,
            business_id,
            transaction_date_unix: now.timestamp(),
            amount: req.amount,
            transaction_type: req.transaction_type,
            description: req.description,
            created_at_unix: now.timestamp(),
        };

        Ok(Response::new(RecordSaleResponse {
            transaction: Some(transaction),
            success: true,
        }))
    }

    async fn get_active_advances(
        &self,
        request: Request<GetAdvancesRequest>,
    ) -> Result<Response<AdvancesResponse>, Status> {
        let tenant_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();
        
        let business_id = if req.business_id.is_empty() {
            self.get_business_id(&tenant_id).await?
        } else {
            req.business_id
        };

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, &tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let rows = sqlx::query(
            "SELECT id, tenant_id, business_id, offer_id, principal_amount, flat_fee_amount, 
                    total_owed, amount_repaid, repayment_percentage, status, 
                    EXTRACT(EPOCH FROM disbursed_at)::bigint as disbursed_at_unix,
                    EXTRACT(EPOCH FROM repaid_at)::bigint as repaid_at_unix,
                    EXTRACT(EPOCH FROM created_at)::bigint as created_at_unix
             FROM capital_advances 
             WHERE tenant_id = $1 AND business_id = $2 AND status = 'active'
             ORDER BY created_at DESC"
        )
        .bind(&tenant_id)
        .bind(&business_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let mut advances = Vec::new();
        let mut total_outstanding = 0.0;

        for row in rows {
            let total_owed: f64 = row.try_get("total_owed").unwrap_or(0.0);
            let amount_repaid: f64 = row.try_get("amount_repaid").unwrap_or(0.0);
            total_outstanding += total_owed - amount_repaid;

            advances.push(CapitalAdvance {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                business_id: row.get("business_id"),
                offer_id: row.get("offer_id"),
                principal_amount: row.try_get("principal_amount").unwrap_or(0.0),
                flat_fee_amount: row.try_get("flat_fee_amount").unwrap_or(0.0),
                total_owed,
                amount_repaid,
                repayment_percentage: row.try_get("repayment_percentage").unwrap_or(0.0),
                status: row.get("status"),
                disbursed_at_unix: row.try_get("disbursed_at_unix").unwrap_or(0),
                repaid_at_unix: row.try_get("repaid_at_unix").unwrap_or(0),
                created_at_unix: row.try_get("created_at_unix").unwrap_or(0),
            });
        }

        Ok(Response::new(AdvancesResponse {
            advances,
            total_outstanding,
        }))
    }

    async fn get_capital_dashboard(
        &self,
        request: Request<EmptyCapitalRequest>,
    ) -> Result<Response<CapitalDashboardResponse>, Status> {
        let tenant_id = self.get_org_id(request.metadata()).await?;
        let business_id = self.get_business_id(&tenant_id).await?;

        // Get cash flow predictions
        let cash_flow = self.forecaster.get_cash_flow_predictions(&tenant_id, &business_id, 30).await?;

        // Get pending offers
        let offers_response = self.engine.get_capital_offers(&tenant_id, &business_id).await?;

        // Get active advances
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, &tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let advance_rows = sqlx::query(
            "SELECT id, tenant_id, business_id, offer_id, principal_amount, flat_fee_amount, 
                    total_owed, amount_repaid, repayment_percentage, status, 
                    EXTRACT(EPOCH FROM disbursed_at)::bigint as disbursed_at_unix,
                    EXTRACT(EPOCH FROM repaid_at)::bigint as repaid_at_unix,
                    EXTRACT(EPOCH FROM created_at)::bigint as created_at_unix
             FROM capital_advances 
             WHERE tenant_id = $1 AND business_id = $2 AND status = 'active'
             ORDER BY created_at DESC"
        )
        .bind(&tenant_id)
        .bind(&business_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let repaid_rows = sqlx::query(
            "SELECT COALESCE(SUM(amount_repaid), 0) as total_repaid
             FROM capital_advances 
             WHERE tenant_id = $1 AND business_id = $2"
        )
        .bind(&tenant_id)
        .bind(&business_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let mut active_advances = Vec::new();
        let mut outstanding_balance = 0.0;

        for row in advance_rows {
            let total_owed: f64 = row.try_get("total_owed").unwrap_or(0.0);
            let amount_repaid: f64 = row.try_get("amount_repaid").unwrap_or(0.0);
            outstanding_balance += total_owed - amount_repaid;

            active_advances.push(CapitalAdvance {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                business_id: row.get("business_id"),
                offer_id: row.get("offer_id"),
                principal_amount: row.try_get("principal_amount").unwrap_or(0.0),
                flat_fee_amount: row.try_get("flat_fee_amount").unwrap_or(0.0),
                total_owed,
                amount_repaid,
                repayment_percentage: row.try_get("repayment_percentage").unwrap_or(0.0),
                status: row.get("status"),
                disbursed_at_unix: row.try_get("disbursed_at_unix").unwrap_or(0),
                repaid_at_unix: row.try_get("repaid_at_unix").unwrap_or(0),
                created_at_unix: row.try_get("created_at_unix").unwrap_or(0),
            });
        }

        let total_repaid: f64 = repaid_rows.try_get("total_repaid").unwrap_or(0.0);

        // Calculate available capital based on business performance
        let available_capital = if offers_response.eligible_for_capital {
            self.engine.calculate_available_capital(&tenant_id, &business_id).await.unwrap_or(0.0)
        } else {
            0.0
        };

        Ok(Response::new(CapitalDashboardResponse {
            available_capital,
            outstanding_balance,
            total_repaid,
            active_advances_count: active_advances.len() as i32,
            pending_offers: offers_response.offers,
            active_advances,
            cash_flow: Some(cash_flow),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_creation() {
        // This is a placeholder test
        // In a real scenario, you'd set up a test database
        assert!(true);
    }
}
