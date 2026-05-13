// Performance Monitor Utility (Bolt)
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};

pub struct LatencyTracker {
    pub name: String,
    pub count: AtomicUsize,
    pub total_us: AtomicU64,
}

impl LatencyTracker {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            count: AtomicUsize::new(0),
            total_us: AtomicU64::new(0),
        }
    }
}
