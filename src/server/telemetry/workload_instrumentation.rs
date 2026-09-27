use tracing::{info, span, Level, Span};
use std::time::Instant;

pub struct WorkloadInstrumentation {
    name: String,
    start_time: Instant,
    span: Span,
}

impl WorkloadInstrumentation {
    pub fn new(name: &str) -> Self {
        let span = span!(Level::INFO, "workload", name = name);
        Self {
            name: name.to_string(),
            start_time: Instant::now(),
            span,
        }
    }

    pub fn record_outcome(&self, success: bool, cost: f64) {
        let duration = self.start_time.elapsed();
        let _enter = self.span.enter();
        info!(
            workload = %self.name,
            duration_ms = duration.as_millis(),
            success = success,
            cost = cost,
            "Workload outcome recorded"
        );
    }
}
