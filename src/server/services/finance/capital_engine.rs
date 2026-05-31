use sqlx::PgPool;
use uuid::Uuid;
use crate::api::finance::{CashflowForecast, CapitalOffer};

pub struct CapitalEngine {
    db_pool: PgPool,
}

impl CapitalEngine {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn evaluate_and_offer(&self, forecast: &CashflowForecast) -> Result<Option<CapitalOffer>, String> {
        if forecast.net_position >= 0.0 {
            return Ok(None);
        }

        let deficit = forecast.net_position.abs();
        let offer_amount = deficit;
        let fee_percentage = 0.05;
        let repayment_rate = 0.10;
        let offer_id = Uuid::new_v4().to_string();

        let offer = CapitalOffer {
            offer_id: offer_id.clone(),
            tenant_id: forecast.tenant_id.clone(),
            forecast_id: Some(forecast.forecast_id.clone()),
            amount: offer_amount,
            fee_percentage,
            repayment_rate,
            status: "PENDING".to_string(),
        };

        let _ = sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(&forecast.tenant_id)
            .execute(&self.db_pool)
            .await;

        sqlx::query(
            "
            INSERT INTO capital_offers (offer_id, tenant_id, forecast_id, amount, fee_percentage, repayment_rate, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "
        )
        .bind(offer.offer_id.clone())
        .bind(offer.tenant_id.clone())
        .bind(offer.forecast_id.clone())
        .bind(offer.amount)
        .bind(offer.fee_percentage)
        .bind(offer.repayment_rate)
        .bind(offer.status.clone())
        .execute(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Some(offer))
    }
}
