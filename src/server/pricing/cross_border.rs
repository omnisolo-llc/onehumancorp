pub struct CrossBorderEngine;

impl CrossBorderEngine {
    /// Detects currency based on IP address
    pub fn detect_currency_from_ip(ip: &str) -> &'static str {
        if ip.starts_with("193.") || ip.starts_with("2.") || ip.starts_with("80.") || ip.starts_with("90.") {
            "EUR"
        } else if ip.starts_with("13.") || ip.starts_with("20.") {
            "GBP"
        } else if ip.starts_with("100.") || ip.starts_with("200.") {
            "CAD"
        } else {
            "USD" // Default
        }
    }

    /// Detects preferred LPM based on locale
    pub fn get_available_lpms(locale: &str) -> Vec<&'static str> {
        match locale {
            "fr-FR" | "fr" => vec!["Cartes Bancaires", "Apple Pay", "Credit Card"],
            "de-DE" | "de" => vec!["Giropay", "Apple Pay", "Credit Card"],
            "nl-NL" | "nl" => vec!["iDEAL", "Apple Pay", "Credit Card"],
            _ => vec!["Credit Card", "Apple Pay", "Google Pay"],
        }
    }

    /// Cosmetically rounds a converted price
    /// e.g. 18.43 -> 18.99, 14.82 -> 14.99
    pub fn cosmetic_round(price: f64) -> f64 {
        let int_part = price.trunc();
        int_part + 0.99
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_to_currency() {
        assert_eq!(CrossBorderEngine::detect_currency_from_ip("193.168.1.1"), "EUR");
        assert_eq!(CrossBorderEngine::detect_currency_from_ip("13.1.2.3"), "GBP");
        assert_eq!(CrossBorderEngine::detect_currency_from_ip("8.8.8.8"), "USD");
    }

    #[test]
    fn test_lpms() {
        assert_eq!(CrossBorderEngine::get_available_lpms("fr-FR"), vec!["Cartes Bancaires", "Apple Pay", "Credit Card"]);
        assert_eq!(CrossBorderEngine::get_available_lpms("de"), vec!["Giropay", "Apple Pay", "Credit Card"]);
        assert_eq!(CrossBorderEngine::get_available_lpms("en-US"), vec!["Credit Card", "Apple Pay", "Google Pay"]);
    }

    #[test]
    fn test_cosmetic_rounding() {
        assert_eq!(CrossBorderEngine::cosmetic_round(18.43), 18.99);
        assert_eq!(CrossBorderEngine::cosmetic_round(14.82), 14.99);
        assert_eq!(CrossBorderEngine::cosmetic_round(100.0), 100.99);
    }
}
