use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethod {
    CreditCard,
    Ach,
    Razorpay,
    MercadoPago,
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
        if currency.eq_ignore_ascii_case("BRL") || currency.eq_ignore_ascii_case("MXN") {
            return PaymentMethod::MercadoPago;
        }
        let amount_usd = amount;

        let card_fee = (amount_usd * Self::CARD_FEE_PERCENTAGE) + Self::CARD_FEE_FIXED;
        let ach_fee = (amount_usd * Self::ACH_FEE_PERCENTAGE).min(Self::ACH_FEE_CAP);

        let ach_min = std::env::var("ACH_MIN_AMOUNT")
            .unwrap_or_else(|_| Self::ACH_MIN_AMOUNT.to_string())
            .parse::<f64>()
            .unwrap_or(Self::ACH_MIN_AMOUNT);

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

        let ach_min = std::env::var("ACH_MIN_AMOUNT")
            .unwrap_or_else(|_| Self::ACH_MIN_AMOUNT.to_string())
            .parse::<f64>()
            .unwrap_or(Self::ACH_MIN_AMOUNT);

        if ach_fee < card_fee && amount_usd >= ach_min {
            let savings = card_fee - ach_fee;
            (savings * 100.0).round() / 100.0
        } else {
            0.0
        }
    }
}

const BATCH_PAYOUT_THRESHOLD_CENTS: i64 = 10000;
const ACH_ROUTING_THRESHOLD_CENTS: i64 = 5000;

pub fn should_batch_payout(amount_cents: i64) -> bool {
    // Transaction Fee Optimization
    // To minimize Stripe transfer fees, small payouts under $100 are batched.
    amount_cents < BATCH_PAYOUT_THRESHOLD_CENTS
}

pub fn route_payment(amount_cents: i64) -> &'static str {
    // Transaction Fee Optimization
    // To minimize Stripe transaction fees, high-value transactions are routed via ACH instead of Credit Card.
    if amount_cents >= ACH_ROUTING_THRESHOLD_CENTS {
        "ACH"
    } else {
        "CreditCard"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_payment() {
        assert_eq!(route_payment(100), "CreditCard");
        assert_eq!(route_payment(4999), "CreditCard");
        assert_eq!(route_payment(5000), "ACH");
        assert_eq!(route_payment(100000), "ACH");
        assert_eq!(route_payment(0), "CreditCard");
        assert_eq!(route_payment(-100), "CreditCard");
    }

    #[test]
    fn test_should_batch_payout() {
        assert_eq!(should_batch_payout(100), true);
        assert_eq!(should_batch_payout(9999), true);
        assert_eq!(should_batch_payout(10000), false);
        assert_eq!(should_batch_payout(50000), false);
        assert_eq!(should_batch_payout(0), true);
        assert_eq!(should_batch_payout(-100), true);
    }

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
    fn test_calculate_fee_savings_large_amount() {
        let savings = PaymentRouter::calculate_fee_savings(1000.0);
        // Card fee: 29.30
        // ACH fee: 5.00
        // Savings: 24.30
        assert_eq!(savings, 24.30);
    }

    #[test]
    fn test_optimize_payment_method_inr() {
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "INR"), PaymentMethod::Razorpay);
    }

    #[test]
    fn test_optimize_payment_method_mercadopago() {
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "BRL"), PaymentMethod::MercadoPago);
    }
}
