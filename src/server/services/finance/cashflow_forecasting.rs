use sqlx::PgPool;
use uuid::Uuid;
use chrono::{NaiveDate, Utc, Duration};
use crate::api::finance::CashflowForecast;

pub struct CashflowEngine {
    db_pool: PgPool,
}

impl CashflowEngine {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn generate_forecast(&self, tenant_id: &str) -> Result<CashflowForecast, String> {
        let target_date = (Utc::now() + Duration::days(7)).date_naive();
        let expected_inflow = 500.0;
        let expected_outflow = 2500.0;
        let net_position = expected_inflow - expected_outflow;
        let risk_level = if net_position < 0.0 { "HIGH" } else { "LOW" };

        let forecast_id = Uuid::new_v4().to_string();

        let forecast = CashflowForecast {
            forecast_id: forecast_id.clone(),
            tenant_id: tenant_id.to_string(),
            target_date,
            expected_inflow,
            expected_outflow,
            net_position,
            risk_level: risk_level.to_string(),
        };

        let _ = sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(tenant_id)
            .execute(&self.db_pool)
            .await;

        sqlx::query(
            "
            INSERT INTO cashflow_forecasts (forecast_id, tenant_id, target_date, expected_inflow, expected_outflow, net_position, risk_level)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "
        )
        .bind(forecast.forecast_id.clone())
        .bind(forecast.tenant_id.clone())
        .bind(forecast.target_date)
        .bind(forecast.expected_inflow)
        .bind(forecast.expected_outflow)
        .bind(forecast.net_position)
        .bind(forecast.risk_level.clone())
        .execute(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(forecast)
    }
}
