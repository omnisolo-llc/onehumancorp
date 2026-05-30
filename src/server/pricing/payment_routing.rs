/// 💰 Miser Cost Analysis:
/// Stripe charges a flat fee of $0.25 plus 0.25% for instant payouts,
/// or a flat fee for standard payouts. By batching small payouts under $100,
/// we save the fixed fee for every individual payout that would otherwise have been initiated.
pub fn should_batch_payout(amount_cents: i64) -> bool {
    // Transaction Fee Optimization
    // To minimize Stripe transfer fees, small payouts under $100 are batched.
    amount_cents < 10000
}

/// 💰 Miser Cost Analysis:
/// Stripe transaction fees are significantly lower for ACH compared to credit cards.
/// Credit card fees are typically 2.9% + $0.30, whereas ACH fees are 0.8% capped at $5.00.
/// By routing transactions over $50 via ACH, we substantially reduce payment processing costs.
pub fn route_payment(amount_cents: i64) -> &'static str {
    // Transaction Fee Optimization
    // To minimize Stripe transaction fees, high-value transactions are routed via ACH instead of Credit Card.
    if amount_cents >= 5000 {
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
    }

    #[test]
    fn test_should_batch_payout() {
        assert_eq!(should_batch_payout(100), true);
        assert_eq!(should_batch_payout(9999), true);
        assert_eq!(should_batch_payout(10000), false);
        assert_eq!(should_batch_payout(50000), false);
    }
}
