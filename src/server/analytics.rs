use std::collections::HashMap;
use serde_json::{json, Value};
use crate::telemetry::redact_interface_pii;

pub struct Tracker;

impl Tracker {
    pub fn new() -> Self {
        Tracker
    }

    pub fn track_event(&self, name: &str, props: HashMap<String, String>) {
        let val: Value = json!(props);
        let redacted = redact_interface_pii(val);
        tracing::info!("Event tracked: {}, props: {}", name, redacted);
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}
