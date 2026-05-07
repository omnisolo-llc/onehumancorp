use std::collections::HashMap;
use serde_json::{Value, Map};
use crate::telemetry::redact_interface_pii;

pub struct Tracker;

impl Tracker {
    pub fn new() -> Self {
        Tracker
    }

    pub fn track_event(&self, name: &str, props: HashMap<String, String>) {
        let mut map = Map::new();
        for (k, v) in props {
            map.insert(k, Value::String(v));
        }
        let redacted = redact_interface_pii(Value::Object(map));
        tracing::info!("Event tracked: {}, props: {:?}", name, redacted);
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}
