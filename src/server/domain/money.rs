use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Money {
    pub amount: i64,
    pub currency_code: String,
}

impl Money {
    pub fn new(amount: i64, currency_code: &str) -> Self {
        Self {
            amount,
            currency_code: currency_code.to_string(),
        }
    }
}
