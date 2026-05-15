use super::*;

    #[test]
    fn test_analytics_tracker_pii_redaction() {
        let tracker = Tracker::new();
        let mut props = HashMap::new();
        props.insert("user_id".to_string(), "12345".to_string());
        props.insert("password".to_string(), "super_secret".to_string());
        props.insert("email".to_string(), "user@example.com".to_string());
        props.insert("contact".to_string(), "contact@test.com".to_string());
        props.insert("billing_address".to_string(), "123 Main St".to_string());

        let sanitized = tracker.sanitize_props(props);

        assert_eq!(sanitized.get("user_id").unwrap(), "12345");
        assert_eq!(sanitized.get("password").unwrap(), "[REDACTED]");
        assert_eq!(sanitized.get("email").unwrap(), "[REDACTED]");
        assert_eq!(sanitized.get("contact").unwrap(), "[EMAIL_REDACTED]");
        assert_eq!(sanitized.get("billing_address").unwrap(), "[REDACTED]");
    }

    #[test]
    fn test_analytics_pii_redaction_cross_mode() {
        temp_env::with_vars(
            [
                ("OHC_STANDALONE", Some("true")),
            ],
            || {
                let tracker = Tracker::new();
                let mut props = HashMap::new();
                props.insert("user_id".to_string(), "12345".to_string());
                props.insert("password".to_string(), "super_secret".to_string());
                props.insert("email".to_string(), "user@example.com".to_string());
                props.insert("contact".to_string(), "contact@test.com".to_string());
                props.insert("billing_address".to_string(), "123 Main St".to_string());

                let sanitized = tracker.sanitize_props(props);

                assert_eq!(sanitized.get("user_id").unwrap(), "12345");
                assert_eq!(sanitized.get("password").unwrap(), "[REDACTED]");
                assert_eq!(sanitized.get("email").unwrap(), "[REDACTED]");
                assert_eq!(sanitized.get("contact").unwrap(), "[EMAIL_REDACTED]");
                assert_eq!(sanitized.get("billing_address").unwrap(), "[REDACTED]");
            },
        );
    }
