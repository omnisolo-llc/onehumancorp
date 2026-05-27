pub fn should_batch_payout(amount_cents: i64) -> bool {
    // Transaction Fee Optimization
    // To minimize Stripe transfer fees, small payouts under $100 are batched.
    amount_cents < 10000
}

pub fn route_payment(amount_cents: i64) -> &'static str {
    // Transaction Fee Optimization
    // To minimize Stripe transaction fees, high-value transactions are routed via ACH instead of Credit Card.
    if amount_cents > 50000 {
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
        assert_eq!(route_payment(50000), "CreditCard");
        assert_eq!(route_payment(50001), "ACH");
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
