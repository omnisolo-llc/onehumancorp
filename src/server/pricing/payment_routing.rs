#[derive(Debug, PartialEq)]
pub enum PaymentMethod {
    CreditCard,
    ACH,
}

#[derive(Debug)]
pub struct RoutingDecision {
    pub method: PaymentMethod,
    pub original_fee: f64,
    pub routed_fee: f64,
    pub savings: f64,
}

pub fn calculate_credit_card_fee(amount: f64) -> f64 {
    (amount * 0.029) + 0.30
}

pub fn calculate_ach_fee(amount: f64) -> f64 {
    let fee = amount * 0.008;
    if fee > 5.00 {
        5.00
    } else {
        fee
    }
}

pub fn route_payment(amount: f64) -> RoutingDecision {
    let cc_fee = calculate_credit_card_fee(amount);
    let ach_fee = calculate_ach_fee(amount);

    // If transaction is >= $50.00 AND ACH fee is lower, use ACH
    if amount >= 50.00 && ach_fee < cc_fee {
        RoutingDecision {
            method: PaymentMethod::ACH,
            original_fee: cc_fee,
            routed_fee: ach_fee,
            savings: cc_fee - ach_fee,
        }
    } else {
        RoutingDecision {
            method: PaymentMethod::CreditCard,
            original_fee: cc_fee,
            routed_fee: cc_fee,
            savings: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cc_fee() {
        assert_eq!((calculate_credit_card_fee(100.0) * 100.0).round() / 100.0, 3.20);
    }

    #[test]
    fn test_ach_fee() {
        assert_eq!(calculate_ach_fee(100.0), 0.80);
        assert_eq!(calculate_ach_fee(1000.0), 5.00); // capped
    }

    #[test]
    fn test_route_payment_small() {
        let dec = route_payment(20.0);
        assert_eq!(dec.method, PaymentMethod::CreditCard);
        assert_eq!(dec.savings, 0.0);
    }

    #[test]
    fn test_route_payment_large() {
        let dec = route_payment(100.0);
        assert_eq!(dec.method, PaymentMethod::ACH);
        // CC: 3.20, ACH: 0.80, Savings: 2.40
        assert_eq!((dec.savings * 100.0).round() / 100.0, 2.40);

        let dec2 = route_payment(1000.0);
        assert_eq!(dec2.method, PaymentMethod::ACH);
        // CC: 29.30, ACH: 5.00, Savings: 24.30
        assert_eq!((dec2.savings * 100.0).round() / 100.0, 24.30);
    }
}
