#[cfg(test)]
mod compliance_tests {
    use crate::telemetry::{redact_interface_pii, is_sensitive_key};
    use serde_json::json;

    #[test]
    fn test_pii_redaction_comprehensive() {
        let raw = json!({
            "user": {
                "name": "John Doe",
                "email": "john@example.com",
                "phone": "555-1234",
                "address": "123 Main St"
            },
            "financial": {
                "account_number": "123456789",
                "routing_number": "987654321",
                "iban": "DE123456789",
                "swift": "ABCDEFGH",
                "cvc": "123",
                "credit_card": "1111-2222-3333-4444"
            },
            "identity": {
                "ssn": "000-00-0000",
                "tax_id": "99-9999999",
                "driver_license": "D1234567",
                "passport": "P1234567"
            },
            "metadata": {
                "tenant_id": "tenant-123",
                "organization_id": "org-456",
                "status": "active"
            }
        });

        let redacted = redact_interface_pii(raw);

        // Sensitive fields should be redacted
        assert_eq!(redacted["user"]["name"], "[REDACTED]");
        assert_eq!(redacted["user"]["email"], "[REDACTED]");
        assert_eq!(redacted["user"]["phone"], "[REDACTED]");
        assert_eq!(redacted["user"]["address"], "[REDACTED]");

        assert_eq!(redacted["financial"]["account_number"], "[REDACTED]");
        assert_eq!(redacted["financial"]["routing_number"], "[REDACTED]");
        assert_eq!(redacted["financial"]["iban"], "[REDACTED]");
        assert_eq!(redacted["financial"]["swift"], "[REDACTED]");
        assert_eq!(redacted["financial"]["cvc"], "[REDACTED]");
        assert_eq!(redacted["financial"]["credit_card"], "[REDACTED]");

        assert_eq!(redacted["identity"]["ssn"], "[REDACTED]");
        assert_eq!(redacted["identity"]["tax_id"], "[REDACTED]");
        assert_eq!(redacted["identity"]["driver_license"], "[REDACTED]");
        assert_eq!(redacted["identity"]["passport"], "[REDACTED]");

        // Non-PII infrastructure identifiers should NOT be redacted
        assert_eq!(redacted["metadata"]["tenant_id"], "tenant-123");
        assert_eq!(redacted["metadata"]["organization_id"], "org-456");
        assert_eq!(redacted["metadata"]["status"], "active");
    }

    #[test]
    fn test_is_sensitive_key_coverage() {
        let sensitive_keys = vec![
            "password", "secret", "api_key", "token", "auth_token", "session_id",
            "account_number", "routing_number", "iban", "swift", "tax_id",
            "driver_license", "medicare", "social_security", "cvc", "cvv"
        ];
        for key in sensitive_keys {
            assert!(is_sensitive_key(key), "Key '{}' should be sensitive", key);
        }

        let safe_keys = vec!["tenant_id", "organization_id", "status", "count"];
        for key in safe_keys {
            assert!(!is_sensitive_key(key), "Key '{}' should NOT be sensitive", key);
        }
    }

    #[test]
    fn test_standalone_telemetry_sovereignty() {
        use crate::config;

        // Simulate Standalone Mode with Telemetry Disabled
        temp_env::with_vars(vec![
            ("OHC_MULTITENANT", Some("false")),
            ("OHC_TELEMETRY_ENABLED", Some("false")),
        ], || {
            let cfg = config::load().unwrap();
            assert!(!cfg.telemetry_enabled, "Telemetry must be disabled by default in Standalone mode");
        });

        // Simulate Standalone Mode with Explicit Telemetry Enabled
        temp_env::with_vars(vec![
            ("OHC_MULTITENANT", Some("false")),
            ("OHC_TELEMETRY_ENABLED", Some("true")),
        ], || {
            let cfg = config::load().unwrap();
            assert!(cfg.telemetry_enabled, "Telemetry should be enabled when explicitly opted-in in Standalone mode");
        });
    }
}
