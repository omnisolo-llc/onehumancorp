#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethod {
    CreditCard,
    Ach,
}

pub struct PaymentRouter;

impl PaymentRouter {
    /// Returns the optimal payment method based on the transaction amount.
    /// Stripe Credit Card fee: 2.9% + $0.30
    /// Stripe ACH fee: 0.8%, capped at $5.00
    pub fn optimize_payment_method(amount_usd: f64) -> PaymentMethod {
        let card_fee = (amount_usd * 0.029) + 0.30;
        let mut ach_fee = amount_usd * 0.008;
        if ach_fee > 5.0 {
            ach_fee = 5.0;
        }

        if ach_fee < card_fee && amount_usd >= 50.0 {
            PaymentMethod::Ach
        } else {
            PaymentMethod::CreditCard
        }
    }

    /// Calculates the potential savings in USD if the optimal payment method is used
    /// instead of defaulting to Credit Card.
    pub fn calculate_fee_savings(amount_usd: f64) -> f64 {
        let card_fee = (amount_usd * 0.029) + 0.30;
        let mut ach_fee = amount_usd * 0.008;
        if ach_fee > 5.0 {
            ach_fee = 5.0;
        }

        if ach_fee < card_fee && amount_usd >= 50.0 {
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
        assert_eq!(PaymentRouter::optimize_payment_method(10.0), PaymentMethod::CreditCard);
    }

    #[test]
    fn test_optimize_payment_method_large_amount() {
        // Amount: $1000.00
        // Card fee: $29.00 + $0.30 = $29.30
        // ACH fee: $8.00 capped at $5.00 = $5.00
        assert_eq!(PaymentRouter::optimize_payment_method(1000.0), PaymentMethod::Ach);
    }

    #[test]
    fn test_calculate_fee_savings_large_amount() {
        let savings = PaymentRouter::calculate_fee_savings(1000.0);
        // Card fee: 29.30
        // ACH fee: 5.00
        // Savings: 24.30
        assert_eq!(savings, 24.30);
    }
}

#[cfg(test)]
mod extra_tests {
    use super::*;

    #[test]
    fn test_optimize_payment_method_medium_amount() {
        // Card fee: 50.0 * 0.029 + 0.30 = 1.45 + 0.30 = 1.75
        // ACH fee: 50.0 * 0.008 = 0.40
        // Because 0.40 < 1.75 and amount is >= 50.0, ACH should be preferred.
        assert_eq!(PaymentRouter::optimize_payment_method(50.0), PaymentMethod::Ach);
    }

    #[test]
    fn test_optimize_payment_method_just_below_threshold() {
        // Amount: 49.99
        // Card fee: 49.99 * 0.029 + 0.30 = 1.44971 + 0.30 = 1.74971
        // ACH fee: 49.99 * 0.008 = 0.39992
        // Although ACH is cheaper, amount is < 50.0, so CreditCard is preferred.
        assert_eq!(PaymentRouter::optimize_payment_method(49.99), PaymentMethod::CreditCard);
    }

    #[test]
    fn test_calculate_fee_savings_small_amount() {
        // For small amounts under 50.0, savings is 0 since we stick with CreditCard
        assert_eq!(PaymentRouter::calculate_fee_savings(10.0), 0.0);
    }

    #[test]
    fn test_calculate_fee_savings_medium_amount() {
        // Amount: 50.0
        // Card fee: 50.0 * 0.029 + 0.30 = 1.75
        // ACH fee: 50.0 * 0.008 = 0.40
        // Savings: 1.75 - 0.40 = 1.35
        assert_eq!(PaymentRouter::calculate_fee_savings(50.0), 1.35);
    }

    #[test]
    fn test_calculate_fee_savings_massive_amount() {
        // Amount: 10_000.0
        // Card fee: 10_000.0 * 0.029 + 0.30 = 290.30
        // ACH fee: capped at 5.00
        // Savings: 290.30 - 5.00 = 285.30
        assert_eq!(PaymentRouter::calculate_fee_savings(10_000.0), 285.30);
    }
}
