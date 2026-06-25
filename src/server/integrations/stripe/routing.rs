use redis::AsyncCommands;
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethod {
    CreditCard,
    Ach,
    Razorpay,
    MercadoPago,
    Alipay,
    Sepa,
    Bacs,
    Ideal,
}

pub struct PaymentRouter;

impl PaymentRouter {
    pub const CARD_FEE_PERCENTAGE: f64 = 0.029;
    pub const CARD_FEE_FIXED: f64 = 0.30;
    pub const ACH_FEE_PERCENTAGE: f64 = 0.008;
    pub const ACH_FEE_CAP: f64 = 5.0;
    pub const ACH_MIN_AMOUNT: f64 = 50.0;
    pub const BATCH_PAYOUT_THRESHOLD_CENTS: i64 = 10000;

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
        if currency.eq_ignore_ascii_case("EUR") {
            return PaymentMethod::Sepa;
        }
        if currency.eq_ignore_ascii_case("GBP") {
            return PaymentMethod::Bacs;
        }

        let amount_usd = amount;
        let card_fee = (amount_usd * Self::CARD_FEE_PERCENTAGE) + Self::CARD_FEE_FIXED;
        let ach_fee = (amount_usd * Self::ACH_FEE_PERCENTAGE).min(Self::ACH_FEE_CAP);
        let ach_min = std::env::var("ACH_MIN_AMOUNT").unwrap_or_else(|_| Self::ACH_MIN_AMOUNT.to_string()).parse::<f64>().unwrap_or(Self::ACH_MIN_AMOUNT);
        if ach_fee < card_fee && amount_usd >= ach_min {
            PaymentMethod::Ach
        } else {
            PaymentMethod::CreditCard
        }
    }

    /// Calculates the potential savings in USD if the optimal payment method is used
    /// instead of defaulting to Credit Card.
    pub fn calculate_fee_savings(amount_usd: f64) -> f64 {
        Self::calculate_fee_savings_with_currency(amount_usd, "USD")
    }

    pub fn calculate_fee_savings_with_currency(amount: f64, currency: &str) -> f64 {
        if currency.eq_ignore_ascii_case("EUR") {
            // EU cards typically charge 1.5% + €0.25 vs SEPA at 0.8% + €0.20 (capped at €5.00)
            let card_fee = (amount * 0.015) + 0.25;
            let sepa_fee = (amount * 0.008 + 0.20).min(5.0);
            if sepa_fee < card_fee {
                let savings = card_fee - sepa_fee;
                return (savings * 100.0).round() / 100.0;
            }
            return 0.0;
        } else if currency.eq_ignore_ascii_case("GBP") {
            // UK cards typically charge 1.5% + £0.20 vs Bacs at 1% + £0.20 (capped at £2.00)
            let card_fee = (amount * 0.015) + 0.20;
            let bacs_fee = (amount * 0.010 + 0.20).min(2.0);
            if bacs_fee < card_fee {
                let savings = card_fee - bacs_fee;
                return (savings * 100.0).round() / 100.0;
            }
            return 0.0;
        }

        let amount_usd = amount;
        let card_fee = (amount_usd * Self::CARD_FEE_PERCENTAGE) + Self::CARD_FEE_FIXED;
        let ach_fee = (amount_usd * Self::ACH_FEE_PERCENTAGE).min(Self::ACH_FEE_CAP);

        let ach_min = std::env::var("ACH_MIN_AMOUNT").unwrap_or_else(|_| Self::ACH_MIN_AMOUNT.to_string()).parse::<f64>().unwrap_or(Self::ACH_MIN_AMOUNT);
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

pub struct ExchangeRateCache;

impl ExchangeRateCache {
    /// Gets the exchange rate from a base currency to a target currency.
    /// If not cached, it mocks an API call and stores it in Redis with a 1-hour TTL.
    pub async fn get_exchange_rate(
        redis_client: &redis::Client,
        base_currency: &str,
        target_currency: &str,
    ) -> Result<f64, String> {
        let mut conn = redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| format!("Redis connection error: {}", e))?;

        let cache_key = format!("ohc:exchange_rate:{}:{}", base_currency.to_uppercase(), target_currency.to_uppercase());

        // 1. Try to get from cache
        let cached_rate: Option<String> = conn.get(&cache_key).await.ok();
        if let Some(rate_str) = cached_rate {
            if let Ok(rate) = rate_str.parse::<f64>() {
                return Ok(rate);
            }
        }

        // 2. Mock external exchange rate API
        let rate = match (base_currency.to_uppercase().as_str(), target_currency.to_uppercase().as_str()) {
            ("EUR", "USD") => 1.08,
            ("USD", "EUR") => 0.93,
            ("GBP", "USD") => 1.25,
            ("USD", "GBP") => 0.80,
            _ => 1.0,
        };

        // 3. Cache it with 1-hour TTL (3600 seconds)
        let _: () = conn
            .set_ex(&cache_key, rate.to_string(), 3600)
            .await
            .map_err(|e| format!("Redis set error: {}", e))?;

        Ok(rate)
    }
}

#[cfg(test)]
mod sepa_bacs_tests {
    use super::*;

    #[test]
    fn test_optimize_payment_method_eur() {
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "EUR"), PaymentMethod::Sepa);
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "eur"), PaymentMethod::Sepa);
    }

    #[test]
    fn test_optimize_payment_method_gbp() {
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "GBP"), PaymentMethod::Bacs);
        assert_eq!(PaymentRouter::optimize_payment_method_with_currency(100.0, "gbp"), PaymentMethod::Bacs);
    }

    #[test]
    fn test_calculate_fee_savings_eur() {
        // EU Card: 100 * 0.015 + 0.25 = 1.75
        // SEPA: 100 * 0.008 + 0.20 = 1.00
        // Savings = 0.75
        assert_eq!(PaymentRouter::calculate_fee_savings_with_currency(100.0, "EUR"), 0.75);
    }

    #[test]
    fn test_calculate_fee_savings_gbp() {
        // UK Card: 100 * 0.015 + 0.20 = 1.70
        // Bacs: 100 * 0.010 + 0.20 = 1.20
        // Savings = 0.50
        assert_eq!(PaymentRouter::calculate_fee_savings_with_currency(100.0, "GBP"), 0.50);
    }
}
