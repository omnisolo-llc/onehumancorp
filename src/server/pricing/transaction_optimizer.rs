#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethod {
    CreditCard,
    ACH,
    Wallet,
}

pub struct TransactionOptimizer {
    pub ach_threshold: f64,
}

impl TransactionOptimizer {
    pub fn new() -> Self {
        Self {
            ach_threshold: 50.00,
        }
    }

    pub fn recommend_payment_method(&self, amount: f64) -> PaymentMethod {
        if amount >= self.ach_threshold {
            PaymentMethod::ACH
        } else {
            PaymentMethod::CreditCard
        }
    }

    pub fn calculate_fee(&self, amount: f64, method: &PaymentMethod) -> f64 {
        match method {
            PaymentMethod::CreditCard => {
                // Typical Stripe CC fee: 2.9% + $0.30
                (amount * 0.029) + 0.30
            }
            PaymentMethod::ACH => {
                // Typical Stripe ACH fee: 0.8%, capped at $5.00
                let fee = amount * 0.008;
                if fee > 5.00 {
                    5.00
                } else {
                    fee
                }
            }
            PaymentMethod::Wallet => {
                (amount * 0.029) + 0.30
            }
        }
    }
}
