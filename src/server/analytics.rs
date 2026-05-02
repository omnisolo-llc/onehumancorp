use std::collections::HashMap;
use serde_json::{json, Value};
use crate::telemetry::redact_interface_pii;

pub struct Tracker;

impl Tracker {
    pub fn new() -> Self {
        Tracker
    }

    pub fn track_event(&self, name: &str, props: HashMap<String, String>) {
        let props_val: Value = json!(props);
        let redacted_props = redact_interface_pii(props_val);

        match serde_json::from_value::<HashMap<String, String>>(redacted_props) {
            Ok(safe_props) => tracing::info!("Event tracked: {}, props: {:?}", name, safe_props),
            Err(_) => tracing::info!("Event tracked: {} (failed to parse redacted props)", name),
        }
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}
