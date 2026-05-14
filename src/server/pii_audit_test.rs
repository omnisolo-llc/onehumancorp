#[cfg(test)]
mod tests {
    use serde_json::json;
    use ::server_telemetry::redact_interface_pii;

    #[test]
    fn test_pii_leakage_prevention() {
        let payload = json!({
            "tenant_id": "org_123",
            "session_id": "sess_456",
            "user_email": "test@example.com",
            "safe_metric": 42
        });

        let redacted = redact_interface_pii(payload);

        assert_eq!(redacted["tenant_id"], "[REDACTED]");
        assert_eq!(redacted["session_id"], "[REDACTED]");
        assert_eq!(redacted["safe_metric"], 42);
        assert_eq!(redacted["user_email"], "[REDACTED]");
    }

    #[test]
    fn test_standalone_telemetry_audit() {
        let is_telemetry_enabled = ::server_config::get().telemetry_enabled;
        assert!(!is_telemetry_enabled, "Standalone telemetry must be off by default for privacy");

        temp_env::with_vars(vec![("OHC_MULTITENANT", Some("false"))], || {
            let mode = ::server_telemetry::get_deployment_mode();
            assert_eq!(mode, "Standalone", "System must report Standalone mode when multitenant is off");
        });
    }
}
