#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethod {
    CreditCard,
    Ach,
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
        let card_fee = (amount_usd * Self::CARD_FEE_PERCENTAGE) + Self::CARD_FEE_FIXED;
        let ach_fee = (amount_usd * Self::ACH_FEE_PERCENTAGE).min(Self::ACH_FEE_CAP);

        if ach_fee < card_fee && amount_usd >= Self::ACH_MIN_AMOUNT {
            PaymentMethod::Ach
        } else {
            PaymentMethod::CreditCard
        }
    }

    /// Calculates the potential savings in USD if the optimal payment method is used
    /// instead of defaulting to Credit Card.

    /// Calculates optimal batching for payouts to minimize fixed fees.
    /// Stripe charges $0.25 + 0.25% for standard payouts.
    /// By batching smaller payouts into a larger one, the fixed $0.25 fee is minimized.
    pub fn optimize_payout_batch(payouts: Vec<f64>, min_batch_size: f64) -> Vec<Vec<f64>> {
        let mut batches = Vec::new();
        let mut current_batch = Vec::new();
        let mut current_sum = 0.0;

        for payout in payouts {
            current_batch.push(payout);
            current_sum += payout;

            if current_sum >= min_batch_size {
                batches.push(current_batch.clone());
                current_batch.clear();
                current_sum = 0.0;
            }
        }

        if !current_batch.is_empty() {
            if let Some(last_batch) = batches.last_mut() {
                // if there's a leftover batch, merge it into the last batch to save fixed fee
                last_batch.extend(current_batch);
            } else {
                batches.push(current_batch);
            }
        }

        batches
    }

    /// Calculates the fee savings of batching payouts compared to paying them individually.
    pub fn calculate_payout_savings(payouts: &[f64], batches: &[Vec<f64>]) -> f64 {
        let individual_fees: f64 = payouts.iter().map(|&p| 0.25 + (p * 0.0025)).sum();
        let batched_fees: f64 = batches.iter().map(|b| {
            let sum: f64 = b.iter().sum();
            0.25 + (sum * 0.0025)
        }).sum();

        let savings = individual_fees - batched_fees;
        (savings * 100.0).round() / 100.0
    }

    pub fn calculate_fee_savings(amount_usd: f64) -> f64 {
        let card_fee = (amount_usd * Self::CARD_FEE_PERCENTAGE) + Self::CARD_FEE_FIXED;
        let ach_fee = (amount_usd * Self::ACH_FEE_PERCENTAGE).min(Self::ACH_FEE_CAP);

        if ach_fee < card_fee && amount_usd >= Self::ACH_MIN_AMOUNT {
            let savings = card_fee - ach_fee;
            (savings * 100.0).round() / 100.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_optimize_payout_batch() {
        let payouts = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let batches = PaymentRouter::optimize_payout_batch(payouts, 45.0);

        // 10+20+30 = 60 (Batch 1) -> 40+50 = 90 (Batch 2)
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0], vec![10.0, 20.0, 30.0]);
        assert_eq!(batches[1], vec![40.0, 50.0]);
    }

    #[test]
    fn test_calculate_payout_savings() {
        let payouts = vec![10.0, 20.0, 30.0, 40.0, 50.0]; // 5 payouts
        let batches = PaymentRouter::optimize_payout_batch(payouts.clone(), 45.0); // 2 batches

        // Individual fixed fees: 5 * $0.25 = $1.25
        // Batched fixed fees: 2 * $0.25 = $0.50
        // Percentage fee is the same regardless of batching.
        // Savings should be $1.25 - $0.50 = $0.75
        let savings = PaymentRouter::calculate_payout_savings(&payouts, &batches);
        assert_eq!(savings, 0.75);
    }

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
