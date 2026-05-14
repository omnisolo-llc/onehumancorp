#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use tracing_core::{Event, Metadata};

    // A mock subscriber to capture logs in memory for testing
    #[derive(Clone)]
    struct MockSubscriber {
        logs: Arc<Mutex<Vec<String>>>,
    }

    impl tracing::Subscriber for MockSubscriber {
        fn enabled(&self, _metadata: &Metadata<'_>) -> bool { true }
        fn new_span(&self, _span: &tracing_core::span::Attributes<'_>) -> tracing_core::span::Id { tracing_core::span::Id::from_u64(1) }
        fn record(&self, _span: &tracing_core::span::Id, _values: &tracing_core::span::Record<'_>) {}
        fn record_follows_from(&self, _span: &tracing_core::span::Id, _follows: &tracing_core::span::Id) {}
        fn event(&self, event: &Event<'_>) {
            let mut visitor = StringVisitor::new();
            event.record(&mut visitor);
            if let Ok(mut logs) = self.logs.lock() {
                logs.push(visitor.content);
            }
        }
        fn enter(&self, _span: &tracing_core::span::Id) {}
        fn exit(&self, _span: &tracing_core::span::Id) {}
    }

    struct StringVisitor {
        content: String,
    }

    impl StringVisitor {
        fn new() -> Self {
            Self { content: String::new() }
        }
    }

    impl tracing::field::Visit for StringVisitor {
        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
            self.content.push_str(&format!("{}={:?} ", field.name(), value));
        }
    }

    #[test]
    fn test_pii_leakage_in_multitenant_logs() {
        let logs = Arc::new(Mutex::new(Vec::new()));
        let subscriber = MockSubscriber { logs: logs.clone() };

        let _guard = tracing::subscriber::set_default(subscriber);

        // Simulate multi-tenant data logging that SHOULD be redacted by Tracker or safely handled
        let mut props = std::collections::HashMap::new();
        props.insert("tenant_id".to_string(), "org-12345".to_string());
        props.insert("password".to_string(), "secret-pass".to_string());
        props.insert("email".to_string(), "user@acme.com".to_string());
        props.insert("ssn".to_string(), "000-00-0000".to_string());

        let tracker = crate::analytics::Tracker::new();
        tracker.track_event("user_login", props);

        // Let's also do a direct tracing call representing a rogue developer
        tracing::info!("Rogue log with safe data: {}", "all-good");

        let captured = logs.lock().unwrap();
        for log in captured.iter() {
            let lower_log = log.to_lowercase();
            // Tracker generically logs props count without printing the map contents directly to avoid leakage
            assert!(!lower_log.contains("secret-pass"), "PII leakage detected: password in logs");
            assert!(!lower_log.contains("user@acme.com"), "PII leakage detected: email in logs");
            assert!(!lower_log.contains("000-00-0000"), "PII leakage detected: SSN in logs");
            assert!(!lower_log.contains("org-12345"), "PII leakage detected: tenant_id in logs");
        }
    }
}
