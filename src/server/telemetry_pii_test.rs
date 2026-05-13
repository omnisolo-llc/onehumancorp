#[cfg(test)]
mod telemetry_pii_tests {
    use crate::telemetry::redact_interface_pii;
    use serde_json::json;

    #[test]
    fn test_email_redaction() {
        let data = json!({ "user_email": " Maya.Smith@example.com " });
        let redacted = redact_interface_pii(data);
        assert_eq!(redacted["user_email"], "[REDACTED]"); // Key match

        let data_raw = json!({ "msg": "Send to maya@ohc.app" });
        let redacted_raw = redact_interface_pii(data_raw);
        assert_eq!(redacted_raw["msg"], "[EMAIL_REDACTED]"); // Value match
    }

    #[test]
    fn test_ssn_redaction() {
        let data = json!({ "info": "123-45-6789" });
        let redacted = redact_interface_pii(data);
        assert_eq!(redacted["info"], "[SSN_REDACTED]");
    }

    #[test]
    fn test_credit_card_redaction() {
        let visa = json!({ "payment": "4111111111111111" });
        assert_eq!(redact_interface_pii(visa)["payment"], "[CARD_REDACTED]");

        let mc = json!({ "payment": "5100000000000000" });
        assert_eq!(redact_interface_pii(mc)["payment"], "[CARD_REDACTED]");
    }

    #[test]
    fn test_ip_redaction() {
        let data = json!({ "ip_address": "192.168.1.1" });
        let redacted = redact_interface_pii(data);
        assert_eq!(redacted["ip_address"], "[REDACTED]"); // Key match

        let data_val = json!({ "host": "10.0.0.1" });
        let redacted_val = redact_interface_pii(data_val);
        assert_eq!(redacted_val["host"], "[IP_REDACTED]"); // Value match
    }

    #[test]
    fn test_nested_pii_redaction() {
        let data = json!({
            "event": "login",
            "metadata": {
                "deep": {
                    "ssn": "000-00-0000",
                    "safe": "data"
                },
                "list": ["user@test.com", "not-an-email"]
            }
        });
        let redacted = redact_interface_pii(data);
        // "ssn" key is in is_sensitive_key list, so it gets [REDACTED]
        assert_eq!(redacted["metadata"]["deep"]["ssn"], "[REDACTED]");
        assert_eq!(redacted["metadata"]["deep"]["safe"], "data");
        assert_eq!(redacted["metadata"]["list"][0], "[EMAIL_REDACTED]");
        assert_eq!(redacted["metadata"]["list"][1], "not-an-email");
    }

    #[test]
    fn test_embedded_pii_redaction() {
        let data = json!({
            "msg": "The user with SSN 123-45-6789 logged in from 10.0.0.1"
        });
        let redacted = redact_interface_pii(data);
        // Note: Currently we redact the WHOLE string if it matches a PII pattern.
        // If we want to redact only the PART of the string, we'd need to use regex::replace.
        // For now, based on the implementation, it redacts the whole value.
        assert_eq!(redacted["msg"], "[SSN_REDACTED]"); // SSN matches first
    }
}
