#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethod {
    CreditCard,
    Ach,
}

#[derive(Debug, Clone)]
pub struct PaymentFeeConfig {
    pub card_percent: f64,
    pub card_fixed: f64,
    pub ach_percent: f64,
    pub ach_cap: f64,
    pub ach_min_amount: f64,
}

impl Default for PaymentFeeConfig {
    fn default() -> Self {
        Self {
            card_percent: 0.029,
            card_fixed: 0.30,
            ach_percent: 0.008,
            ach_cap: 5.00,
            ach_min_amount: 50.0,
        }
    }
}

pub struct PaymentRouter;

impl PaymentRouter {
    /// Returns the optimal payment method based on the transaction amount.
    pub fn optimize_payment_method(amount_usd: f64, config: Option<&PaymentFeeConfig>) -> PaymentMethod {
        let default_config = PaymentFeeConfig::default();
        let cfg = config.unwrap_or(&default_config);

        let card_fee = (amount_usd * cfg.card_percent) + cfg.card_fixed;
        let mut ach_fee = amount_usd * cfg.ach_percent;
        if ach_fee > cfg.ach_cap {
            ach_fee = cfg.ach_cap;
        }

        if ach_fee < card_fee && amount_usd >= cfg.ach_min_amount {
            PaymentMethod::Ach
        } else {
            PaymentMethod::CreditCard
        }
    }

    /// Calculates the potential savings in USD if the optimal payment method is used
    /// instead of defaulting to Credit Card.
    pub fn calculate_fee_savings(amount_usd: f64, config: Option<&PaymentFeeConfig>) -> f64 {
        let default_config = PaymentFeeConfig::default();
        let cfg = config.unwrap_or(&default_config);

        let card_fee = (amount_usd * cfg.card_percent) + cfg.card_fixed;
        let mut ach_fee = amount_usd * cfg.ach_percent;
        if ach_fee > cfg.ach_cap {
            ach_fee = cfg.ach_cap;
        }

        if ach_fee < card_fee && amount_usd >= cfg.ach_min_amount {
            let savings = card_fee - ach_fee;
            (savings * 100.0).round() / 100.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize_payment_method_small_amount() {
        assert_eq!(PaymentRouter::optimize_payment_method(10.0, None), PaymentMethod::CreditCard);
    }

    #[test]
    fn test_optimize_payment_method_large_amount() {
        // Amount: $1000.00
        // Card fee: $29.00 + $0.30 = $29.30
        // ACH fee: $8.00 capped at $5.00 = $5.00
        assert_eq!(PaymentRouter::optimize_payment_method(1000.0, None), PaymentMethod::Ach);
    }

    #[test]
    fn test_calculate_fee_savings_large_amount() {
        let savings = PaymentRouter::calculate_fee_savings(1000.0, None);
        // Card fee: 29.30
        // ACH fee: 5.00
        // Savings: 24.30
        assert_eq!(savings, 24.30);
    }

    #[test]
    fn test_optimize_payment_method_custom_config() {
        let config = PaymentFeeConfig {
            card_percent: 0.02,
            card_fixed: 0.10,
            ach_percent: 0.01,
            ach_cap: 10.0,
            ach_min_amount: 10.0,
        };

        // Amount $20
        // Card: 20 * 0.02 + 0.10 = 0.50
        // ACH: 20 * 0.01 = 0.20
        assert_eq!(PaymentRouter::optimize_payment_method(20.0, Some(&config)), PaymentMethod::Ach);
        assert_eq!(PaymentRouter::calculate_fee_savings(20.0, Some(&config)), 0.30);
    }
}
