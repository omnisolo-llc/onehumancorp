pub fn validate_business_name(name: &str) -> bool {
    name.trim().len() >= 3
}

pub fn validate_product_price(price: &str) -> bool {
    price.parse::<f64>().is_ok() && price.parse::<f64>().unwrap() >= 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_business_name() {
        assert!(validate_business_name("Bakery"));
        assert!(validate_business_name("   Bakery   "));
        assert!(!validate_business_name("Ba"));
        assert!(!validate_business_name(""));
    }

    #[test]
    fn test_validate_product_price() {
        assert!(validate_product_price("10.00"));
        assert!(validate_product_price("0"));
        assert!(validate_product_price("0.0"));
        assert!(!validate_product_price("-1.0"));
        assert!(!validate_product_price("abc"));
        assert!(!validate_product_price(""));
    }
}
