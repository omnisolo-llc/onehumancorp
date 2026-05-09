#[cfg(test)]
mod privacy_tests {
    use crate::telemetry::{redact_interface_pii, is_sensitive_key};
    use serde_json::json;

    #[test]
    fn test_email_redaction() {
        let input = json!({ "email_addr": "user@example.com" });
        let redacted = redact_interface_pii(input);
        assert_eq!(redacted["email_addr"], "[REDACTED]");

        let input_nested = json!({ "data": "Contact me at admin@ohc.app" });
        let redacted_nested = redact_interface_pii(input_nested);
        assert_eq!(redacted_nested["data"], "[REDACTED]");
    }

    #[test]
    fn test_phone_redaction() {
        let input = json!({ "phone_number": "123-456-7890" });
        let redacted = redact_interface_pii(input);
        assert_eq!(redacted["phone_number"], "[REDACTED]");

        let input_intl = json!({ "callback": "+1 555 123 4567" });
        let redacted_intl = redact_interface_pii(input_intl);
        assert_eq!(redacted_intl["callback"], "[REDACTED]");
    }

    #[test]
    fn test_credit_card_redaction() {
        let input = json!({ "cc": "1234-5678-9012-3456" });
        let redacted = redact_interface_pii(input);
        assert_eq!(redacted["cc"], "[REDACTED]");

        let input_raw = json!({ "payload": "Paid with 4111 1111 1111 1111" });
        let redacted_raw = redact_interface_pii(input_raw);
        assert_eq!(redacted_raw["payload"], "[REDACTED]");
    }

    #[test]
    fn test_sensitive_keys() {
        assert!(is_sensitive_key("password"));
        assert!(is_sensitive_key("API_KEY"));
        assert!(is_sensitive_key("tax_id"));
        assert!(is_sensitive_key("national_id"));
        assert!(is_sensitive_key("medical_record"));
        assert!(is_sensitive_key("health_data"));
        assert!(is_sensitive_key("insurance_policy"));
        assert!(!is_sensitive_key("public_id"));
    }

    #[tokio::test]
    async fn test_standalone_telemetry_opt_in_enforcement() {
        // Mocking config to simulate standalone mode with telemetry disabled
        temp_env::with_vars(vec![("OHC_STANDALONE", Some("true")), ("OHC_TELEMETRY_ENABLED", Some("false"))], || {
             let cfg = crate::config::load().unwrap();
             assert_eq!(cfg.standalone, true);
             assert_eq!(cfg.telemetry_enabled, false);
        });

        // Mocking config to simulate standalone mode with telemetry explicitly enabled
        temp_env::with_vars(vec![("OHC_STANDALONE", Some("true")), ("OHC_TELEMETRY_ENABLED", Some("true"))], || {
             let cfg = crate::config::load().unwrap();
             assert_eq!(cfg.standalone, true);
             assert_eq!(cfg.telemetry_enabled, true);
        });
    }
}
