#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingStrategy {
    NearestCent, // 10.42 -> 10.42
    Cosmetic99,  // 10.42 -> 10.99
    Nearest100,  // JPY: 1432 -> 1400 or 1500
}

pub fn determine_rounding_strategy(locale: &str, target_currency: &str) -> RoundingStrategy {
    match target_currency.to_uppercase().as_str() {
        "JPY" => RoundingStrategy::Nearest100,
        "USD" | "EUR" | "GBP" | "CAD" | "AUD" => RoundingStrategy::Cosmetic99,
        _ => RoundingStrategy::NearestCent,
    }
}

pub fn round_price_cosmetic(price_cents: i64, target_currency: &str) -> i64 {
    let strategy = determine_rounding_strategy("en_US", target_currency);
    match strategy {
        RoundingStrategy::Cosmetic99 => {
            // e.g. 5420 cents -> $54.20.  If it's over $x.00, bump to $x.99 or $(x-1).99
            let dollars = price_cents / 100;
            if price_cents % 100 == 0 {
                price_cents // already an exact dollar, maybe keep it or make it .99? Let's leave exact alone.
            } else {
                (dollars * 100) + 99
            }
        }
        RoundingStrategy::Nearest100 => {
            // 1432 -> 1400
            ((price_cents + 50) / 100) * 100
        }
        RoundingStrategy::NearestCent => price_cents,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosmetic_rounding_usd() {
        assert_eq!(round_price_cosmetic(5420, "USD"), 5499);
        assert_eq!(round_price_cosmetic(5400, "USD"), 5400); // Exact dollars stay exact
        assert_eq!(round_price_cosmetic(99, "USD"), 99);
    }

    #[test]
    fn test_rounding_jpy() {
        assert_eq!(round_price_cosmetic(1432, "JPY"), 1400);
        assert_eq!(round_price_cosmetic(1450, "JPY"), 1500);
    }
}
