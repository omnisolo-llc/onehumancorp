use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethod {
    CreditCard,
    Ach,
    Razorpay,
    MercadoPago,
    Alipay,
}

pub struct PaymentRouter;

static ACH_MIN_CACHE: OnceLock<f64> = OnceLock::new();

impl PaymentRouter {
    pub const CARD_FEE_PERCENTAGE: f64 = 0.029;
    pub const CARD_FEE_FIXED: f64 = 0.30;
    pub const ACH_FEE_PERCENTAGE: f64 = 0.008;
    pub const ACH_FEE_CAP: f64 = 5.0;
    pub const ACH_MIN_AMOUNT: f64 = 50.0;
    pub const BATCH_PAYOUT_THRESHOLD_CENTS: i64 = 10000;

    fn get_ach_min_amount() -> f64 {
        *ACH_MIN_CACHE.get_or_init(|| {
            std::env::var("ACH_MIN_AMOUNT")
                .ok()
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(Self::ACH_MIN_AMOUNT)
        })
    }

    pub fn should_batch_payout(amount_cents: i64) -> bool {
        // Transaction Fee Optimization
        // To minimize Stripe transfer fees, small payouts under $100 are batched.
        amount_cents < Self::BATCH_PAYOUT_THRESHOLD_CENTS
    }

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
        if currency.eq_ignore_ascii_case("BRL") || currency.eq_ignore_ascii_case("MXN") {
            return PaymentMethod::MercadoPago;
        }
        if currency.eq_ignore_ascii_case("CNY") {
            return PaymentMethod::Alipay;
        }
        let amount_usd = amount;

        let card_fee = (amount_usd * Self::CARD_FEE_PERCENTAGE) + Self::CARD_FEE_FIXED;
        let ach_fee = (amount_usd * Self::ACH_FEE_PERCENTAGE).min(Self::ACH_FEE_CAP);

        let ach_min = Self::get_ach_min_amount();
        if ach_fee < card_fee && amount_usd >= ach_min {
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

        let ach_min = Self::get_ach_min_amount();
        if ach_fee < card_fee && amount_usd >= ach_min {
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
    fn test_optimize_payment_method_boundary() {
        // Amount: $50.00
        // Card fee: 50 * 0.029 + 0.30 = 1.75
        // ACH fee: 50 * 0.008 = 0.40
        assert_eq!(PaymentRouter::optimize_payment_method(50.0), PaymentMethod::Ach);
        assert_eq!(PaymentRouter::optimize_payment_method(49.99), PaymentMethod::CreditCard);
    }

    #[test]
    fn test_optimize_payment_method_large_amount() {
        // Amount: $1000.00
        // Card fee: $29.00 + $0.30 = $29.30
        // ACH fee: $8.00 capped at $5.00 = $5.00
        assert_eq!(PaymentRouter::optimize_payment_method(1000.0), PaymentMethod::Ach);
    }

    #[test]
    fn test_optimize_payment_method_alipay_currency() {
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "CNY"), PaymentMethod::Alipay);
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "cny"), PaymentMethod::Alipay);
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

#[cfg(test)]
mod mercadopago_tests {
    use super::*;

    #[test]
    fn test_optimize_payment_method_mercadopago() {
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "BRL"), PaymentMethod::MercadoPago);
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "MXN"), PaymentMethod::MercadoPago);
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "brl"), PaymentMethod::MercadoPago);
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "mxn"), PaymentMethod::MercadoPago);
    }
}

#[cfg(test)]
mod alipay_tests {
    use super::*;

    #[test]
    fn test_optimize_payment_method_alipay() {
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "CNY"), PaymentMethod::Alipay);
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(1000.0, "cny"), PaymentMethod::Alipay);
    }
}

#[cfg(test)]
mod batch_payout_tests {
    use super::*;

    #[test]
    fn test_should_batch_payout() {
        assert!(PaymentRouter::should_batch_payout(5000));
        assert!(!PaymentRouter::should_batch_payout(10000));
        assert!(!PaymentRouter::should_batch_payout(15000));
    }
}

#[cfg(test)]
mod cost_analysis_tests {
    use super::*;

    #[test]
    fn test_cost_analysis_large_payment_ach_vs_card() {
        // Scenario: A single large payment of $5,000.
        let amount_usd = 5000.0;

        let card_fee = (amount_usd * PaymentRouter::CARD_FEE_PERCENTAGE) + PaymentRouter::CARD_FEE_FIXED;
        let ach_fee = (amount_usd * PaymentRouter::ACH_FEE_PERCENTAGE).min(PaymentRouter::ACH_FEE_CAP);

        // Assert actual computed fees with precision tolerance
        assert!((card_fee - 145.30).abs() < 1e-6, "Expected 145.30, got {}", card_fee);
        assert!((ach_fee - 5.00).abs() < 1e-6, "Expected 5.00, got {}", ach_fee);

        // Total Savings
        let savings = card_fee - ach_fee;
        assert!((savings - 140.30).abs() < 1e-6, "Expected 140.30, got {}", savings);

        // Assert the routing logic matches
        assert_eq!(PaymentRouter::optimize_payment_method(amount_usd), PaymentMethod::Ach);
        assert!((PaymentRouter::calculate_fee_savings(amount_usd) - 140.30).abs() < 1e-6, "Expected fee savings 140.30");
    }
}
