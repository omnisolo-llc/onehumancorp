use crate::integrations::stripe::routing::{PaymentMethod, PaymentRouter};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize_payment_method_with_currency_inr() {
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "INR"), PaymentMethod::Razorpay);
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(10000.0, "inr"), PaymentMethod::Razorpay);
    }

    #[test]
    fn test_optimize_payment_method_with_currency_brl() {
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "BRL"), PaymentMethod::MercadoPago);
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "MXN"), PaymentMethod::MercadoPago);
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "brl"), PaymentMethod::MercadoPago);
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "mxn"), PaymentMethod::MercadoPago);
    }
}
