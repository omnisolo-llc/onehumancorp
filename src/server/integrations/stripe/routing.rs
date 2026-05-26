#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethod {
    CreditCard,
    Ach,
    Razorpay,
}

pub struct PaymentRouter;

impl PaymentRouter {
    pub const CARD_FEE_PERCENTAGE: f64 = 0.029;
    pub const CARD_FEE_FIXED: f64 = 0.30;
    pub const ACH_FEE_PERCENTAGE: f64 = 0.008;
    pub const ACH_FEE_CAP: f64 = 5.0;
    pub const ACH_MIN_AMOUNT: f64 = 50.0;

    /// Returns the optimal payment method based on the transaction amount.
    /// Stripe Credit Card fee: 2.9% + $0.30
    /// Stripe ACH fee: 0.8%, capped at $5.00
    pub fn optimize_payment_method(amount_usd: f64) -> PaymentMethod {
        Self::optimize_payment_method_with_currency(amount_usd, "USD")
    }

    pub fn optimize_payment_method_with_currency(amount: f64, currency: &str) -> PaymentMethod {
        if currency.eq_ignore_ascii_case("INR") {
            return PaymentMethod::Razorpay;
        }
        let amount_usd = amount;

        let card_fee = (amount_usd * Self::CARD_FEE_PERCENTAGE) + Self::CARD_FEE_FIXED;
        let ach_fee = (amount_usd * Self::ACH_FEE_PERCENTAGE).min(Self::ACH_FEE_CAP);

        let ach_min = std::env::var("ACH_MIN_AMOUNT").unwrap_or_else(|_| Self::ACH_MIN_AMOUNT.to_string()).parse::<f64>().unwrap_or(Self::ACH_MIN_AMOUNT); if ach_fee < card_fee && amount_usd >= ach_min {
            PaymentMethod::Ach
        } else {
            PaymentMethod::CreditCard
        }
    }

    /// Calculates the potential savings in USD if the optimal payment method is used
    /// instead of defaulting to Credit Card.
    pub fn calculate_fee_savings(amount_usd: f64) -> f64 {
        let card_fee = (amount_usd * Self::CARD_FEE_PERCENTAGE) + Self::CARD_FEE_FIXED;
        let ach_fee = (amount_usd * Self::ACH_FEE_PERCENTAGE).min(Self::ACH_FEE_CAP);

        let ach_min = std::env::var("ACH_MIN_AMOUNT").unwrap_or_else(|_| Self::ACH_MIN_AMOUNT.to_string()).parse::<f64>().unwrap_or(Self::ACH_MIN_AMOUNT); if ach_fee < card_fee && amount_usd >= ach_min {
            let savings = card_fee - ach_fee;
            (savings * 100.0).round() / 100.0
        } else {
            0.0
        }
    }

    /// Simulates batching multiple payouts into a single transaction to minimize fees.
    /// It calculates the fee savings compared to processing them individually via CreditCard.
    pub fn batch_payouts(amounts: Vec<f64>) -> f64 {
        let mut individual_fees = 0.0;
        let mut total_amount = 0.0;

        for amount in amounts {
            individual_fees += (amount * Self::CARD_FEE_PERCENTAGE) + Self::CARD_FEE_FIXED;
            total_amount += amount;
        }

        let batched_fee = if total_amount > 0.0 {
            let ach_min = std::env::var("ACH_MIN_AMOUNT").unwrap_or_else(|_| Self::ACH_MIN_AMOUNT.to_string()).parse::<f64>().unwrap_or(Self::ACH_MIN_AMOUNT);
            if total_amount >= ach_min {
                (total_amount * Self::ACH_FEE_PERCENTAGE).min(Self::ACH_FEE_CAP)
            } else {
                (total_amount * Self::CARD_FEE_PERCENTAGE) + Self::CARD_FEE_FIXED
            }
        } else {
            0.0
        };

        if individual_fees > batched_fee {
            let savings = individual_fees - batched_fee;
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

    #[test]
    fn test_batch_payouts() {
        // Amounts: 10.0, 20.0, 30.0
        // Individual Card fees:
        // 10.0: 0.29 + 0.30 = 0.59
        // 20.0: 0.58 + 0.30 = 0.88
        // 30.0: 0.87 + 0.30 = 1.17
        // Total Individual fees: 2.64
        // Total Amount: 60.0 (ACH eligible)
        // Batched ACH fee: 60.0 * 0.008 = 0.48
        // Savings: 2.64 - 0.48 = 2.16

        let savings = PaymentRouter::batch_payouts(vec![10.0, 20.0, 30.0]);
        assert_eq!(savings, 2.16);

        // Under threshold amounts
        // Amounts: 5.0, 5.0
        // Total amount: 10.0 (Not ACH eligible)
        // Individual Card fees:
        // 5.0: 0.145 + 0.30 = 0.445 * 2 = 0.89
        // Batched Card fee: 10.0 * 0.029 + 0.30 = 0.29 + 0.30 = 0.59
        // Savings: 0.89 - 0.59 = 0.30
        let savings_under = PaymentRouter::batch_payouts(vec![5.0, 5.0]);
        assert_eq!(savings_under, 0.30);
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

    #[test]
    fn test_zero_amount() {
        assert_eq!(PaymentRouter::optimize_payment_method(0.0), PaymentMethod::CreditCard);
        assert_eq!(PaymentRouter::calculate_fee_savings(0.0), 0.0);
    }

    #[test]
    fn test_negative_amount() {
        assert_eq!(PaymentRouter::optimize_payment_method(-10.0), PaymentMethod::CreditCard);
        assert_eq!(PaymentRouter::calculate_fee_savings(-10.0), 0.0);
    }
}

#[cfg(test)]
mod razorpay_tests {
    use super::*;

    #[test]
    fn test_optimize_payment_method_inr() {
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "INR"), PaymentMethod::Razorpay);
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(10000.0, "inr"), PaymentMethod::Razorpay);
    }
}
