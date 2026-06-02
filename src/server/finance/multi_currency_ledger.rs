use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReconciliationResult {
    pub base_amount: f64,
    pub margin_diff: f64,
    pub flagged_for_review: bool,
}

pub struct MultiCurrencyLedger {
    pub default_currency: String,
    pub real_time_rates: Mutex<HashMap<String, f64>>,
}

impl MultiCurrencyLedger {
    pub fn new(default_currency: &str) -> Self {
        let mut rates = HashMap::new();
        rates.insert("USD".to_string(), 1.0);
        rates.insert("EUR".to_string(), 0.92);
        rates.insert("AED".to_string(), 3.67);
        rates.insert("BRL".to_string(), 5.15);
        rates.insert("GBP".to_string(), 0.79);

        Self {
            default_currency: default_currency.to_string(),
            real_time_rates: Mutex::new(rates),
        }
    }

    pub fn reconcile_transaction(&self, amount: f64, local_currency: &str, cached_rate: f64) -> ReconciliationResult {
        let rates = self.real_time_rates.lock().unwrap();
        let current_rate = rates.get(local_currency).unwrap_or(&1.0);

        let base_amount_charged = amount / cached_rate;
        let actual_base_amount = amount / current_rate;
        let diff = base_amount_charged - actual_base_amount;

        let flagged = diff.abs() / actual_base_amount > 0.05;

        ReconciliationResult {
            base_amount: actual_base_amount,
            margin_diff: diff,
            flagged_for_review: flagged,
        }
    }
}
