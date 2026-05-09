#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethod {
    CreditCard,
    Ach,
}

pub struct PaymentRouter;

impl PaymentRouter {
    const ACH_CAP: f64 = 5.0;
    const ACH_RATE: f64 = 0.008;
    const CARD_RATE: f64 = 0.029;
    const CARD_FIXED: f64 = 0.30;
    const ACH_THRESHOLD: f64 = 50.0;

    /// Returns the optimal payment method based on the transaction amount.
    /// Stripe Credit Card fee: 2.9% + $0.30
    /// Stripe ACH fee: 0.8%, capped at $5.00
    pub fn optimize_payment_method(amount_usd: f64) -> PaymentMethod {
        let card_fee = (amount_usd * Self::CARD_RATE) + Self::CARD_FIXED;
        let ach_fee = (amount_usd * Self::ACH_RATE).min(Self::ACH_CAP);

        if ach_fee < card_fee && amount_usd >= Self::ACH_THRESHOLD {
            PaymentMethod::Ach
        } else {
            PaymentMethod::CreditCard
        }
    }

    /// Calculates the potential savings in USD if the optimal payment method is used
    /// instead of defaulting to Credit Card.
    pub fn calculate_fee_savings(amount_usd: f64) -> f64 {
        let card_fee = (amount_usd * Self::CARD_RATE) + Self::CARD_FIXED;
        let ach_fee = (amount_usd * Self::ACH_RATE).min(Self::ACH_CAP);

        if ach_fee < card_fee && amount_usd >= Self::ACH_THRESHOLD {
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
        assert_eq!(PaymentRouter::optimize_payment_method(1000.0), PaymentMethod::Ach);
    }

    #[test]
    fn test_calculate_fee_savings_large_amount() {
        let savings = PaymentRouter::calculate_fee_savings(1000.0);
        assert_eq!(savings, 24.30);
    }
}

#[cfg(test)]
mod extra_tests {
    use super::*;

    #[test]
    fn test_optimize_payment_method_medium_amount() {
        assert_eq!(PaymentRouter::optimize_payment_method(50.0), PaymentMethod::Ach);
    }

    #[test]
    fn test_optimize_payment_method_just_below_threshold() {
        assert_eq!(PaymentRouter::optimize_payment_method(49.99), PaymentMethod::CreditCard);
    }

    #[test]
    fn test_calculate_fee_savings_small_amount() {
        assert_eq!(PaymentRouter::calculate_fee_savings(10.0), 0.0);
    }

    #[test]
    fn test_calculate_fee_savings_medium_amount() {
        assert_eq!(PaymentRouter::calculate_fee_savings(50.0), 1.35);
    }

    #[test]
    fn test_calculate_fee_savings_massive_amount() {
        assert_eq!(PaymentRouter::calculate_fee_savings(10_000.0), 285.30);
    }
}
