#[cfg(test)]
mod tests {
    use serde_json::json;
    use ::server_telemetry::redact_interface_pii;

    #[test]
    fn test_pii_redaction_regex_patterns() {
        // is_sensitive_key matches keywords. We test regex on keys that don't match keywords.
        let payload_regex = json!({
            "msg": "Call me at 555-123-4567",
            "identifier": "test@example.com",
            "data": "Card 4111-1111-1111-1111"
        });

        let redacted_regex = redact_interface_pii(payload_regex);
        assert_eq!(redacted_regex["msg"], serde_json::Value::String("[PHONE_REDACTED]".to_string()));
        assert_eq!(redacted_regex["identifier"], serde_json::Value::String("[EMAIL_REDACTED]".to_string()));
        assert_eq!(redacted_regex["data"], serde_json::Value::String("[CREDIT_CARD_REDACTED]".to_string()));

        let raw_email = json!("test@test.com");
        let redacted_raw = redact_interface_pii(raw_email);
        assert_eq!(redacted_raw, serde_json::Value::String("[EMAIL_REDACTED]".to_string()));
    }

    #[test]
    fn test_infrastructure_id_preservation() {
        // tenant_id and organization_id should NOT be redacted to support billing attribution
        let payload = json!({
            "tenant_id": "tenant-123",
            "organization_id": "org-456",
            "agent_id": "agent-789",
            "password": "secret-password"
        });

        let redacted = redact_interface_pii(payload);

        assert_eq!(redacted["tenant_id"], serde_json::Value::String("tenant-123".to_string()));
        assert_eq!(redacted["organization_id"], serde_json::Value::String("org-456".to_string()));
        assert_eq!(redacted["agent_id"], serde_json::Value::String("agent-789".to_string()));
        assert_eq!(redacted["password"], serde_json::Value::String("[REDACTED]".to_string()));
    }

    #[test]
    fn test_standalone_data_sovereignty_defaults() {
        temp_env::with_vars(
            [
                ("STANDALONE_MODE", Some("true")),
                ("OHC_TELEMETRY_ENABLED", None), // Default case
            ],
            || {
                let config = ::server_config::load().expect("Failed to load config");
                assert_eq!(config.telemetry_enabled, false, "Telemetry must default to OFF in Standalone mode");
            }
        );
    }

    #[test]
    fn test_standalone_data_sovereignty_opt_in() {
        temp_env::with_vars(
            [
                ("STANDALONE_MODE", Some("true")),
                ("OHC_TELEMETRY_ENABLED", Some("true")),
            ],
            || {
                let config = ::server_config::load().expect("Failed to load config");
                assert_eq!(config.telemetry_enabled, true, "Telemetry should be ON when explicitly opted-in");
            }
        );
    }

    #[test]
    fn test_no_pii_logging_statements_extended() {
        use walkdir::WalkDir;
        use std::fs;
        use std::env;
        use std::path::PathBuf;

        let mut violations = Vec::new();
        let mut search_dirs = vec![PathBuf::from("src")];

        if let Ok(runfiles_dir) = env::var("RUNFILES_DIR") {
            let runfiles = PathBuf::from(&runfiles_dir);
            if let Ok(workspace) = env::var("TEST_WORKSPACE") {
                let prefixed = runfiles.join(&workspace).join("src");
                if prefixed.exists() {
                    search_dirs.push(prefixed);
                }
            }
        }

        let mut checked_files = 0;

        for dir in &search_dirs {
            if dir.exists() {
                let walker = WalkDir::new(&dir).into_iter().filter_entry(|e| {
                    let path = e.path();
                    !path.to_string_lossy().contains("compliance_test.rs") &&
                    !path.to_string_lossy().contains("telemetry_test.rs") &&
                    !path.to_string_lossy().contains("external")
                });

                for entry in walker
                    .filter_map(Result::ok)
                    .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs" || ext == "go" || ext == "ts"))
                {
                    checked_files += 1;
                    let content = fs::read_to_string(entry.path()).unwrap_or_default();

                    for (i, line) in content.lines().enumerate() {
                        let lower_line = line.to_lowercase();

                        // Look for common logging/printing patterns
                        if lower_line.contains("tracing::") ||
                           lower_line.contains("info!") ||
                           lower_line.contains("error!") ||
                           lower_line.contains("println!") ||
                           lower_line.contains("console.log")
                        {
                            // Check for sensitive keywords in the same line
                            let sensitive_words = [
                                "password", "secret", "api_key", "auth_token", "ssn",
                                "credit_card", "cvv", "cvc", "iban", "social_security"
                            ];

                            for word in &sensitive_words {
                                if lower_line.contains(word) {
                                    violations.push(format!("{}:{} - Found potential PII '{}' in logging statement", entry.path().display(), i + 1, word));
                                }
                            }
                        }
                    }
                }
            }
        }

        if checked_files > 0 {
            assert!(
                violations.is_empty(),
                "Found PII logging violations:\n{}",
                violations.join("\n")
            );
        }
    }
}
