use std::collections::HashMap;

pub struct Tracker;

impl Tracker {
    pub fn new() -> Self {
        Tracker
    }

    pub fn track_event(&self, name: &str, props: HashMap<String, String>) {
        tracing::info!("Event tracked: {}, props: {:?}", name, props);
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}
