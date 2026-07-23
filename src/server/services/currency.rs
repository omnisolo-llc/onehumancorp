use crate::hub::Hub;
use std::sync::Arc;

pub struct CurrencyService {
    pub hub: Arc<Hub>,
}

impl CurrencyService {
    pub fn new(hub: Arc<Hub>) -> Self {
        Self { hub }
    }

    pub async fn get_exchange_rate(&self, currency_code: &str) -> Result<f64, String> {
        let pool = &self.hub.pool;
        let row: Option<(f64,)> = sqlx::query_as("SELECT exchange_rate FROM currencies WHERE code = $1")
            .bind(currency_code.to_uppercase())
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?;

        match row {
            Some((rate,)) => Ok(rate),
            None => Ok(1.0)
        }
    }

    pub async fn convert_amount(&self, amount_cents: i64, from_currency: &str, to_currency: &str) -> Result<i64, String> {
        if from_currency == to_currency {
            return Ok(amount_cents);
        }

        let from_rate = self.get_exchange_rate(from_currency).await?;
        let to_rate = self.get_exchange_rate(to_currency).await?;

        let base_amount = (amount_cents as f64) / from_rate;
        let target_amount = base_amount * to_rate;

        Ok(target_amount.round() as i64)
    }

    pub async fn update_exchange_rates(&self) -> Result<(), String> {
        let pool = &self.hub.pool;
        let rates = vec![
            ("USD", 1.0),
            ("EUR", 0.92),
            ("GBP", 0.79),
            ("JPY", 150.0),
            ("CAD", 1.35),
            ("AUD", 1.53),
        ];

        for (code, rate) in rates {
            sqlx::query(
                "INSERT INTO currencies (code, exchange_rate, last_updated) VALUES ($1, $2, CURRENT_TIMESTAMP)
                 ON CONFLICT (code) DO UPDATE SET exchange_rate = EXCLUDED.exchange_rate, last_updated = CURRENT_TIMESTAMP"
            )
            .bind(code)
            .bind(rate)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}
