#[cfg(test)]
mod pii_redaction_tests {
    use crate::telemetry::{redact_interface_pii, is_email, is_sensitive_key};
    use serde_json::json;

    #[test]
    fn test_extensive_redaction() {
        let data = json!({
            "user_id": "123",
            "username": "jdoe",
            "details": {
                "email": "private@example.com",
                "phone": "+1234567890",
                "zip": "90210",
                "postcode": "SW1A 1AA",
                "lat": 34.05,
                "long": -118.24,
                "dob": "1990-01-01",
                "passport": "X1234567",
                "ssn": "000-00-0000"
            },
            "billing": {
                "card": "4111...",
                "cvv": "123",
                "address": "123 Main St"
            },
            "network": {
                "ip_address": "192.168.1.1",
                "mac_address": "00:0a:95:9d:68:16"
            },
            "safe_metadata": {
                "version": "1.0.0",
                "feature_flag": true
            }
        });

        let redacted = redact_interface_pii(data);

        // Sensitive keys should be redacted
        assert_eq!(redacted["username"], "[REDACTED]");
        assert_eq!(redacted["details"]["email"], "[REDACTED]");
        assert_eq!(redacted["details"]["phone"], "[REDACTED]");
        assert_eq!(redacted["details"]["zip"], "[REDACTED]");
        assert_eq!(redacted["details"]["postcode"], "[REDACTED]");
        assert_eq!(redacted["details"]["lat"], "[REDACTED]");
        assert_eq!(redacted["details"]["long"], "[REDACTED]");
        assert_eq!(redacted["details"]["dob"], "[REDACTED]");
        assert_eq!(redacted["details"]["passport"], "[REDACTED]");
        assert_eq!(redacted["details"]["ssn"], "[REDACTED]");
        assert_eq!(redacted["billing"], "[REDACTED]");
        assert_eq!(redacted["network"]["ip_address"], "[REDACTED]");
        assert_eq!(redacted["network"]["mac_address"], "[REDACTED]");

        // Safe fields should remain
        assert_eq!(redacted["user_id"], "123");
        assert_eq!(redacted["safe_metadata"]["version"], "1.0.0");
        assert_eq!(redacted["safe_metadata"]["feature_flag"], true);
    }

    #[test]
    fn test_email_heuristic() {
        assert!(is_email("test@example.com"));
        assert!(is_email("user.name@sub.domain.org "));
        assert!(!is_email("not-an-email"));
        assert!(!is_email("@domain.com"));
        assert!(!is_email("user@.com"));
        assert!(!is_email("user@domain."));
        assert!(!is_email(""));
    }

    #[test]
    fn test_sensitive_key_matching() {
        assert!(is_sensitive_key("API_KEY"));
        assert!(is_sensitive_key("session_id"));
        assert!(is_sensitive_key("billing_info"));
        assert!(is_sensitive_key("Latitude"));
        assert!(is_sensitive_key("zip_code"));
        assert!(!is_sensitive_key("count"));
        assert!(!is_sensitive_key("version"));
    }
}
