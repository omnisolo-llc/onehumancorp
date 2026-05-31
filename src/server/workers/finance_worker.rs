use tokio::time::{sleep, Duration};
use sqlx::PgPool;
use crate::services::finance::cashflow_forecasting::CashflowEngine;
use crate::services::finance::capital_engine::CapitalEngine;

pub struct FinanceWorker {
    cashflow_engine: CashflowEngine,
    capital_engine: CapitalEngine,
    db_pool: PgPool,
}

impl FinanceWorker {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            cashflow_engine: CashflowEngine::new(db_pool.clone()),
            capital_engine: CapitalEngine::new(db_pool.clone()),
            db_pool,
        }
    }

    pub async fn run(&self) {
        loop {
            tracing::info!("FinanceWorker running cashflow analysis...");
            self.process_tenants().await;
            sleep(Duration::from_secs(3600)).await;
        }
    }

    async fn process_tenants(&self) {
        let tenants_result = sqlx::query("SELECT DISTINCT tenant_id FROM capital_offers").fetch_all(&self.db_pool).await;

        let mut active_tenants = vec!["default-tenant".to_string()];
        if let Ok(rows) = tenants_result {
            for row in rows {
                use sqlx::Row;
                if let Ok(t) = row.try_get::<String, _>("tenant_id") {
                    active_tenants.push(t);
                }
            }
        }

        for tenant_id_str in active_tenants {
            let tenant_id = tenant_id_str.as_str();
            let forecast = self.cashflow_engine.generate_forecast(tenant_id).await.unwrap_or_else(|_| {
                crate::api::finance::CashflowForecast {
                    forecast_id: "error".to_string(),
                    tenant_id: tenant_id.to_string(),
                    target_date: chrono::Utc::now().naive_utc().date(),
                    expected_inflow: 0.0,
                    expected_outflow: 0.0,
                    net_position: 0.0,
                    risk_level: "LOW".to_string(),
                }
            });
            if forecast.risk_level == "HIGH" {
                if let Ok(Some(offer)) = self.capital_engine.evaluate_and_offer(&forecast).await {
                    tracing::info!("Generated capital offer {} for tenant {}", offer.offer_id, tenant_id);
                }
            }
        }
    }
}
