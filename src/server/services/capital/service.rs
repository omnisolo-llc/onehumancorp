use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::engine::RiskAssessmentEngine;
use super::models::{CapitalOffer, CapitalContract, CapitalContractStatus};

#[derive(Clone)]
pub struct CapitalService {
    risk_engine: RiskAssessmentEngine,
    pool: PgPool,
}

impl CapitalService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            risk_engine: RiskAssessmentEngine::new(pool.clone()),
            pool,
        }
    }

    pub async fn trigger_offer_for_booking(&self, tenant_id: &str, booking_amount: f64) -> Result<Option<CapitalOffer>, String> {
        self.risk_engine.generate_offer(tenant_id, booking_amount).await
    }

    pub async fn get_active_offer(&self, tenant_id: &str) -> Result<Option<CapitalOffer>, String> {
        let tenant_uuid = Uuid::parse_str(tenant_id).map_err(|e| e.to_string())?;

        let row_opt = sqlx::query(
            "SELECT id, advance_amount, flat_fee, repayment_percentage FROM capital_offers WHERE tenant_id = $1 AND status = 'Offered' ORDER BY created_at DESC LIMIT 1"
        )
        .bind(tenant_uuid)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(row) = row_opt {
            Ok(Some(CapitalOffer {
                id: row.get::<Uuid, _>(0).to_string(),
                tenant_id: tenant_id.to_string(),
                advance_amount: row.get(1),
                flat_fee: row.get(2),
                repayment_percentage: row.get(3),
                status: CapitalContractStatus::Offered,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn accept_offer(&self, tenant_id: &str, offer_id: &str) -> Result<CapitalContract, String> {
        let offer_uuid = Uuid::parse_str(offer_id).map_err(|e| e.to_string())?;
        let tenant_uuid = Uuid::parse_str(tenant_id).map_err(|e| e.to_string())?;

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        // 1. Verify offer
        let row = sqlx::query(
            "SELECT advance_amount, flat_fee, repayment_percentage FROM capital_offers WHERE id = $1 AND tenant_id = $2 AND status = 'Offered'"
        )
        .bind(offer_uuid)
        .bind(tenant_uuid)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| "Invalid offer ID".to_string())?;

        let advance_amount: f64 = row.get(0);
        let flat_fee: f64 = row.get(1);
        let repayment_percentage: f64 = row.get(2);

        // 2. Mark offer as accepted
        sqlx::query("UPDATE capital_offers SET status = 'Accepted', updated_at = NOW() WHERE id = $1")
            .bind(offer_uuid)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        // 3. Create contract
        let contract_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO capital_contracts (id, tenant_id, advance_amount, flat_fee, repayment_percentage, repaid_amount, status)
             VALUES ($1, $2, $3, $4, $5, 0, 'Active')"
        )
        .bind(contract_id)
        .bind(tenant_uuid)
        .bind(advance_amount)
        .bind(flat_fee)
        .bind(repayment_percentage)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(CapitalContract {
            id: contract_id.to_string(),
            tenant_id: tenant_id.to_string(),
            advance_amount,
            flat_fee,
            repayment_percentage,
            repaid_amount: 0.0,
            status: CapitalContractStatus::Active,
        })
    }

    pub async fn get_active_contract(&self, tenant_id: &str) -> Result<Option<CapitalContract>, String> {
        let tenant_uuid = Uuid::parse_str(tenant_id).map_err(|e| e.to_string())?;

        let row_opt = sqlx::query(
            "SELECT id, advance_amount, flat_fee, repayment_percentage, repaid_amount FROM capital_contracts WHERE tenant_id = $1 AND status = 'Active' LIMIT 1"
        )
        .bind(tenant_uuid)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(row) = row_opt {
            Ok(Some(CapitalContract {
                id: row.get::<Uuid, _>(0).to_string(),
                tenant_id: tenant_id.to_string(),
                advance_amount: row.get(1),
                flat_fee: row.get(2),
                repayment_percentage: row.get(3),
                repaid_amount: row.get(4),
                status: CapitalContractStatus::Active,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn process_revenue(&self, tenant_id: &str, amount: f64) -> Result<(), String> {
        let tenant_uuid = Uuid::parse_str(tenant_id).map_err(|e| e.to_string())?;

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row_opt = sqlx::query(
            "SELECT id, advance_amount, flat_fee, repayment_percentage, repaid_amount FROM capital_contracts WHERE tenant_id = $1 AND status = 'Active' FOR UPDATE"
        )
        .bind(tenant_uuid)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(row) = row_opt {
            let contract_id: Uuid = row.get(0);
            let advance_amount: f64 = row.get(1);
            let flat_fee: f64 = row.get(2);
            let repayment_percentage: f64 = row.get(3);
            let repaid_amount: f64 = row.get(4);

            let interception_amount = amount * repayment_percentage;
            let total_owed = advance_amount + flat_fee;
            let remaining_balance = total_owed - repaid_amount;

            let actual_repayment = interception_amount.min(remaining_balance);
            let new_repaid_amount = repaid_amount + actual_repayment;

            let mut status = "Active";
            if new_repaid_amount >= total_owed {
                status = "Repaid";
            }

            sqlx::query(
                "UPDATE capital_contracts SET repaid_amount = $1, status = $2, updated_at = NOW() WHERE id = $3"
            )
            .bind(new_repaid_amount)
            .bind(status)
            .bind(contract_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::get_pool;

    // A note about testing: Since this uses DB pool, we'll implement isolated testing
    // via integration testing suites.
}
