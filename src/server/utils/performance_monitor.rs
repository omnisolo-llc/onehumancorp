use std::time::Instant;
use tracing::info;

pub struct LatencyGuard {
    name: String,
    start: Instant,
}

impl LatencyGuard {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start: Instant::now(),
        }
    }
}

impl Drop for LatencyGuard {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        info!(target: "performance", operation = %self.name, latency_ms = %elapsed.as_millis(), "Latency report");

        // In a real scenario, we could also record this to a metric buffer
        // ::server_telemetry::buffer_latency(&self.name, elapsed);
    }
}

#[macro_export]
macro_rules! track_latency {
    ($name:expr) => {
        let _latency_guard = crate::utils::performance_monitor::LatencyGuard::new($name);
    };
}
