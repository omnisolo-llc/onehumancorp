use std::collections::HashMap;

pub struct Tracker;

impl Tracker {
    pub fn new() -> Self {
        Tracker
    }

    pub fn track_event(&self, name: &str, props: HashMap<String, String>) {
        let redacted_props = crate::telemetry::redact_interface_pii(serde_json::to_value(props).unwrap_or(serde_json::Value::Null));
        tracing::info!("Event tracked: {}, props: {}", name, redacted_props);
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}
