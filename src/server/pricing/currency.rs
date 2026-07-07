use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalizedPrice {
    pub original_cents: i64,
    pub original_currency: String,
    pub localized_cents: i64,
    pub localized_currency: String,
    pub exchange_rate: f64,
}

pub fn determine_currency_from_ip(ip: &str, browser_locale: &str) -> String {
    // A simplified heuristic for edge detection logic. In real scenarios,
    // Cloudflare headers or a GeoIP database are queried.
    let locale_lower = browser_locale.to_lowercase();
    if locale_lower.contains("en-gb") || ip.starts_with("82.") {
        "GBP".to_string()
    } else if locale_lower.contains("fr") || locale_lower.contains("de") || ip.starts_with("80.") {
        "EUR".to_string()
    } else if locale_lower.contains("en-ca") {
        "CAD".to_string()
    } else if locale_lower.contains("en-au") {
        "AUD".to_string()
    } else if locale_lower.contains("ja") {
        "JPY".to_string()
    } else {
        "USD".to_string() // Fallback
    }
}

pub fn get_available_lpms(currency: &str) -> Vec<String> {
    match currency {
        "EUR" => vec!["cartes_bancaires".to_string(), "ideal".to_string(), "bancontact".to_string(), "giropay".to_string(), "card".to_string(), "apple_pay".to_string()],
        "GBP" => vec!["bacs_debit".to_string(), "card".to_string(), "apple_pay".to_string()],
        "CAD" => vec!["interac_present".to_string(), "card".to_string(), "apple_pay".to_string()],
        "AUD" => vec!["becs_debit".to_string(), "card".to_string(), "apple_pay".to_string()],
        "JPY" => vec!["konbini".to_string(), "card".to_string(), "apple_pay".to_string()],
        _ => vec!["card".to_string(), "apple_pay".to_string(), "google_pay".to_string()],
    }
}

pub fn apply_cosmetic_rounding(amount_cents: i64, currency: &str) -> i64 {
    // For JPY, there are no cents. Round to nearest 100.
    if currency == "JPY" {
        let remainder = amount_cents % 10000; // Since it's stored in 'cents', JPY 100 is 10000
        if remainder > 0 {
            return amount_cents - remainder + 10000;
        }
        return amount_cents;
    }

    // Typical western currencies (USD, EUR, GBP, CAD, AUD):
    // Convert to .99 or .00 if close.
    // We get the remainder of cents (0-99).
    let cents = amount_cents % 100;

    // If it's something like 18.43, standard cosmetic rounding dictates
    // pushing it up to .99 for charming pricing if it's past .50, or .49 if below,
    // or just generally to .99. Let's force .99 for anything above .00 for simplicity
    // to match retail "charming prices" expectations in the spec.
    if cents == 0 {
        amount_cents
    } else {
        amount_cents - cents + 99
    }
}

pub fn calculate_localized_price(
    original_cents: i64,
    original_currency: &str,
    target_currency: &str,
    exchange_rate: f64,
) -> LocalizedPrice {
    if original_currency == target_currency {
        return LocalizedPrice {
            original_cents,
            original_currency: original_currency.to_string(),
            localized_cents: original_cents,
            localized_currency: target_currency.to_string(),
            exchange_rate: 1.0,
        };
    }

    let converted_raw = (original_cents as f64 * exchange_rate).round() as i64;
    let cosmetically_rounded = apply_cosmetic_rounding(converted_raw, target_currency);

    LocalizedPrice {
        original_cents,
        original_currency: original_currency.to_string(),
        localized_cents: cosmetically_rounded,
        localized_currency: target_currency.to_string(),
        exchange_rate,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_currency() {
        assert_eq!(determine_currency_from_ip("192.168.1.1", "en-US"), "USD");
        assert_eq!(determine_currency_from_ip("82.1.1.1", "en-US"), "GBP");
        assert_eq!(determine_currency_from_ip("1.1.1.1", "fr-FR"), "EUR");
        assert_eq!(determine_currency_from_ip("1.1.1.1", "en-CA"), "CAD");
    }

    #[test]
    fn test_get_lpms() {
        let eur_lpms = get_available_lpms("EUR");
        assert!(eur_lpms.contains(&"giropay".to_string()));
        assert!(eur_lpms.contains(&"cartes_bancaires".to_string()));

        let usd_lpms = get_available_lpms("USD");
        assert!(!usd_lpms.contains(&"giropay".to_string()));
    }

    #[test]
    fn test_cosmetic_rounding() {
        // $18.43 -> $18.99
        assert_eq!(apply_cosmetic_rounding(1843, "USD"), 1899);
        // $20.00 -> $20.00
        assert_eq!(apply_cosmetic_rounding(2000, "USD"), 2000);
        // €49.12 -> €49.99
        assert_eq!(apply_cosmetic_rounding(4912, "EUR"), 4999);

        // JPY 1450 (which is 145000 in cents) -> JPY 1500 (150000 cents)
        assert_eq!(apply_cosmetic_rounding(145000, "JPY"), 150000);
    }

    #[test]
    fn test_calculate_localized_price() {
        // 20 USD to EUR at 0.92 rate = 18.40 EUR -> rounded to 18.99 EUR
        let result = calculate_localized_price(2000, "USD", "EUR", 0.92);
        assert_eq!(result.localized_cents, 1899);
        assert_eq!(result.localized_currency, "EUR");
        assert_eq!(result.exchange_rate, 0.92);

        // Same currency
        let same = calculate_localized_price(2000, "USD", "USD", 1.2); // rate ignored
        assert_eq!(same.localized_cents, 2000);
        assert_eq!(same.exchange_rate, 1.0);
    }
}
