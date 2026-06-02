pub struct ForecastEngine {}

impl ForecastEngine {
    pub fn new() -> Self {
        Self {}
    }

    /// Predicts cashflow for the next 30 days based on simple heuristics.
    /// Returns a tuple of (predicted_balance_cents, alert_message).
    /// Positive values imply a surplus, negative values imply a shortfall.
    pub fn predict_30_day_cashflow(&self, current_balance_cents: i64, avg_daily_expenses_cents: i64, avg_daily_revenue_cents: i64) -> (i64, Option<String>) {
        let predicted_revenue = avg_daily_revenue_cents * 30;
        let predicted_expenses = avg_daily_expenses_cents * 30;
        let predicted_balance = current_balance_cents + predicted_revenue - predicted_expenses;

        let mut alert = None;
        if predicted_balance < 0 {
            alert = Some(format!(
                "You might have a ${} shortfall next month. Let's resolve it.",
                predicted_balance.abs() / 100
            ));
        }

        (predicted_balance, alert)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_cashflow_surplus() {
        let engine = ForecastEngine::new();
        let (balance, alert) = engine.predict_30_day_cashflow(100000, 1000, 2000); // $1000, $10/day, $20/day
        assert_eq!(balance, 130000); // 100000 + 60000 - 30000
        assert_eq!(alert, None);
    }

    #[test]
    fn test_predict_cashflow_shortfall() {
        let engine = ForecastEngine::new();
        let (balance, alert) = engine.predict_30_day_cashflow(10000, 2000, 500); // $100, $20/day, $5/day
        assert_eq!(balance, -35000); // 10000 + 15000 - 60000
        assert!(alert.is_some());
        assert_eq!(alert.unwrap(), "You might have a $350 shortfall next month. Let's resolve it.");
    }
}
