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
}
