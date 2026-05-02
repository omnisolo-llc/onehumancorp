use std::collections::HashMap;

pub struct Tracker;

impl Tracker {
    pub fn new() -> Self {
        Tracker
    }

    pub fn track_event(&self, name: &str, props: HashMap<String, String>) {
        use serde_json::json;
        let props_val = json!(props);
        let redacted = crate::telemetry::redact_interface_pii(props_val);
        let redacted_str = serde_json::to_string(&redacted).unwrap_or_else(|_| "{}".to_string());
        tracing::info!("Event tracked: {}, props: {}", name, redacted_str);
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}
