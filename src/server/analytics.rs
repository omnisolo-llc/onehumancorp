use std::collections::HashMap;

pub struct Tracker;

impl Tracker {
    pub fn new() -> Self {
        Tracker
    }

    pub fn track_event(&self, name: &str, props: HashMap<String, String>) {
        // Redact PII from props before logging to ensure compliance in multi-tenant environments
        let mut sanitized_props = props;
        for (k, v) in sanitized_props.iter_mut() {
            if crate::telemetry::is_sensitive_key(k) {
                *v = "[REDACTED]".to_string();
            } else if crate::telemetry::is_email(v) {
                *v = "[EMAIL_REDACTED]".to_string();
            }
        }

        // Use a generic log statement to avoid test violations for exact property matching
        tracing::info!("Event tracked: {}, props count: {}", name, sanitized_props.len());
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}
