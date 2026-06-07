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
        assert!(should_batch_payout(100));
        assert!(should_batch_payout(9999));
        assert!(!(should_batch_payout(10000)));
        assert!(!(should_batch_payout(50000)));
        assert!(should_batch_payout(0));
        assert!(should_batch_payout(-100));
    }
}
