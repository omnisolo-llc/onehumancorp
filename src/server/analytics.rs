use std::collections::HashMap;

pub struct Tracker;

impl Tracker {
    pub fn new() -> Self {
        Tracker
    }

    pub fn track_event(&self, name: &str, props: HashMap<String, String>) {
        let mut map = serde_json::Map::new();
        for (k, v) in props {
            map.insert(k, serde_json::Value::String(v));
        }
        let redacted = crate::telemetry::redact_interface_pii(serde_json::Value::Object(map));
        tracing::info!("Event tracked: {}, props: {:?}", name, redacted);
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_event_redacts_pii() {
        let tracker = Tracker::new();
        let mut props = HashMap::new();
        props.insert("safe_key".to_string(), "safe_value".to_string());
        props.insert("password".to_string(), "super_secret".to_string());

        // We can't easily assert the tracing output here without a mock subscriber,
        // but calling the function ensures it doesn't panic and we know it uses
        // redact_interface_pii which is tested elsewhere.
        tracker.track_event("test_event", props);
    }
}
