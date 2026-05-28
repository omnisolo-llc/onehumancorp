use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::models::{CapitalOffer, CapitalContractStatus};

#[derive(Clone)]
pub struct RiskAssessmentEngine {
    pool: PgPool,
}

impl RiskAssessmentEngine {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn record_revenue(&self, tenant_id: &str, amount: f64) -> Result<(), String> {
        // Here we'd insert into actual tenant ledger. For the capital engine scope,
        // we'll assume the ledger tracks this, or we track aggregated revenue.
        // For demonstration, simulating success:
        Ok(())
    }

    pub async fn calculate_pre_approved_limit(&self, tenant_id: &str) -> Result<f64, String> {
        // Fetch real historical revenue sum from orders/ledger table
        let tenant_uuid = Uuid::parse_str(tenant_id).map_err(|e| e.to_string())?;

        // Simulating ledger fetch for now
        let total_revenue: f64 = sqlx::query("SELECT COALESCE(SUM(total), 0) FROM orders WHERE tenant_id = $1 AND status = 'completed'")
            .bind(tenant_uuid)
            .fetch_one(&self.pool)
            .await
            .map(|r| r.get::<f64, _>(0))
            .unwrap_or(0.0);

        // Simple heuristic: 10% of total historical revenue
        Ok(total_revenue * 0.10)
    }

    pub async fn generate_offer(&self, tenant_id: &str, triggered_by_amount: f64) -> Result<Option<CapitalOffer>, String> {
        let limit = self.calculate_pre_approved_limit(tenant_id).await?;

        if limit > 0.0 && triggered_by_amount <= limit * 2.0 {
            let advance_amount = triggered_by_amount.min(limit);
            let flat_fee = advance_amount * 0.10;
            let repayment_percentage = 0.10;

            let offer_id = Uuid::new_v4();
            let tenant_uuid = Uuid::parse_str(tenant_id).map_err(|e| e.to_string())?;

            sqlx::query(
                "INSERT INTO capital_offers (id, tenant_id, advance_amount, flat_fee, repayment_percentage, status)
                 VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(offer_id)
            .bind(tenant_uuid)
            .bind(advance_amount)
            .bind(flat_fee)
            .bind(repayment_percentage)
            .bind("Offered")
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            Ok(Some(CapitalOffer {
                id: offer_id.to_string(),
                tenant_id: tenant_id.to_string(),
                advance_amount,
                flat_fee,
                repayment_percentage,
                status: CapitalContractStatus::Offered,
            }))
        } else {
            Ok(None)
        }
    }
}
