use tonic::{Request, Response, Status};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use chrono::Utc;
use tracing::{info, error};

use server_ohc::capital::{
    capital_engine_service_server::CapitalEngineService,
    CapitalOffer, CapitalAdvance, GetOffersRequest, GetOffersResponse,
    AcceptOfferRequest, AcceptOfferResponse, ProcessSaleRequest, ProcessSaleResponse,
};

pub struct CapitalEngineServiceImpl {
    db: Pool<Postgres>,
}

impl CapitalEngineServiceImpl {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl CapitalEngineService for CapitalEngineServiceImpl {
    async fn get_offers(
        &self,
        request: Request<GetOffersRequest>,
    ) -> Result<Response<GetOffersResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = req.tenant_id;

        info!("Fetching capital offers for tenant: {}", tenant_id);

        let rows = sqlx::query(
            "SELECT id, tenant_id, amount_cents, fee_cents, sweep_percentage, status, created_at FROM capital_offers WHERE tenant_id = $1"
        )
        .bind(&tenant_id)
        .fetch_all(&self.db)
        .await
        .map_err(|e| {
            error!("Failed to fetch offers: {:?}", e);
            Status::internal("Failed to fetch offers")
        })?;

        let mut offers = Vec::new();
        for row in rows {
            offers.push(CapitalOffer {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                amount_cents: row.get("amount_cents"),
                fee_cents: row.get("fee_cents"),
                sweep_percentage: row.get("sweep_percentage"),
                status: row.get("status"),
                created_at_unix: row.try_get::<chrono::DateTime<Utc>, _>("created_at").map(|t| t.timestamp()).unwrap_or(0),
            });
        }

        Ok(Response::new(GetOffersResponse { offers }))
    }

    async fn accept_offer(
        &self,
        request: Request<AcceptOfferRequest>,
    ) -> Result<Response<AcceptOfferResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = req.tenant_id;
        let offer_id = req.offer_id;

        info!("Accepting capital offer {} for tenant: {}", offer_id, tenant_id);

        let mut tx = self.db.begin().await.map_err(|e| {
            error!("Failed to begin transaction: {:?}", e);
            Status::internal("Failed to begin transaction")
        })?;

        let offer = sqlx::query(
            "SELECT amount_cents, fee_cents FROM capital_offers WHERE id = $1 AND tenant_id = $2 AND status = 'PENDING'"
        )
        .bind(&offer_id)
        .bind(&tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            error!("Failed to fetch offer: {:?}", e);
            Status::internal("Failed to fetch offer")
        })?
        .ok_or_else(|| Status::not_found("Offer not found or already accepted"))?;

        let advance_id = Uuid::new_v4().to_string();
        let amount_cents: i64 = offer.get("amount_cents");
        let fee_cents: i64 = offer.get("fee_cents");
        let total_repayment = amount_cents + fee_cents;

        sqlx::query(
            "UPDATE capital_offers SET status = 'ACCEPTED' WHERE id = $1"
        )
        .bind(&offer_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!("Failed to update offer status: {:?}", e);
            Status::internal("Failed to update offer status")
        })?;

        sqlx::query(
            "INSERT INTO capital_advances (id, tenant_id, offer_id, total_repayment_cents, repaid_cents, status) VALUES ($1, $2, $3, $4, 0, 'ACTIVE')"
        )
        .bind(&advance_id)
        .bind(&tenant_id)
        .bind(&offer_id)
        .bind(total_repayment)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!("Failed to create advance: {:?}", e);
            Status::internal("Failed to create advance")
        })?;

        tx.commit().await.map_err(|e| {
            error!("Failed to commit transaction: {:?}", e);
            Status::internal("Failed to commit transaction")
        })?;

        let advance = CapitalAdvance {
            id: advance_id,
            tenant_id,
            offer_id,
            total_repayment_cents: total_repayment,
            repaid_cents: 0,
            status: "ACTIVE".to_string(),
            created_at_unix: Utc::now().timestamp(),
        };

        Ok(Response::new(AcceptOfferResponse { advance: Some(advance) }))
    }

    async fn process_sale(
        &self,
        request: Request<ProcessSaleRequest>,
    ) -> Result<Response<ProcessSaleResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = req.tenant_id;
        let sale_amount = req.sale_amount_cents;

        info!("Processing sale of {} cents for tenant: {}", sale_amount, tenant_id);

        let mut tx = self.db.begin().await.map_err(|e| {
            error!("Failed to begin transaction: {:?}", e);
            Status::internal("Failed to begin transaction")
        })?;

        let sale_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO transaction_ledger (id, tenant_id, amount_cents, type, reference_id) VALUES ($1, $2, $3, 'SALE', $4)"
        )
        .bind(&sale_id)
        .bind(&tenant_id)
        .bind(sale_amount)
        .bind(&sale_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!("Failed to log sale: {:?}", e);
            Status::internal("Failed to log sale")
        })?;

        let advances = sqlx::query(
            r#"
            SELECT id, total_repayment_cents, repaid_cents, (SELECT sweep_percentage FROM capital_offers WHERE id = offer_id) as sweep_percentage
            FROM capital_advances
            WHERE tenant_id = $1 AND status = 'ACTIVE'
            "#
        )
        .bind(&tenant_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| {
            error!("Failed to fetch active advances: {:?}", e);
            Status::internal("Failed to fetch active advances")
        })?;

        let mut total_sweep = 0;

        for adv in advances {
            let total_repayment_cents: i64 = adv.get("total_repayment_cents");
            let repaid_cents: i64 = adv.get("repaid_cents");
            let sweep_percentage: f64 = adv.get("sweep_percentage");
            let adv_id: String = adv.get("id");

            let remaining = total_repayment_cents - repaid_cents;
            if remaining <= 0 {
                continue;
            }

            let mut sweep_amount = (sale_amount as f64 * sweep_percentage) as i64;
            if sweep_amount > remaining {
                sweep_amount = remaining;
            }

            total_sweep += sweep_amount;

            let sweep_id = Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO transaction_ledger (id, tenant_id, amount_cents, type, reference_id) VALUES ($1, $2, $3, 'SWEEP', $4)"
            )
            .bind(&sweep_id)
            .bind(&tenant_id)
            .bind(sweep_amount)
            .bind(&adv_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                error!("Failed to log sweep: {:?}", e);
                Status::internal("Failed to log sweep")
            })?;

            let new_repaid = repaid_cents + sweep_amount;
            let status = if new_repaid >= total_repayment_cents { "REPAID" } else { "ACTIVE" };

            sqlx::query(
                "UPDATE capital_advances SET repaid_cents = $1, status = $2 WHERE id = $3"
            )
            .bind(new_repaid)
            .bind(status)
            .bind(&adv_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                error!("Failed to update advance: {:?}", e);
                Status::internal("Failed to update advance")
            })?;
        }

        tx.commit().await.map_err(|e| {
            error!("Failed to commit transaction: {:?}", e);
            Status::internal("Failed to commit transaction")
        })?;

        Ok(Response::new(ProcessSaleResponse {
            success: true,
            sweep_amount_cents: total_sweep,
        }))
    }
}
