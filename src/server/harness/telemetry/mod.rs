pub mod store;

pub use store::ViolationStore;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub trait SandboxTelemetryEmitter: Send + Sync {
    fn record_violation(&self, agent_id: &str, task_id: &str, reason: &str);
}

#[derive(Default)]
pub struct DefaultSandboxTelemetryEmitter;

impl SandboxTelemetryEmitter for DefaultSandboxTelemetryEmitter {
    fn record_violation(&self, agent_id: &str, task_id: &str, reason: &str) {
        ::server_telemetry::record_bubblewrap_violation(agent_id, task_id, reason);
    }
}

pub struct MockTelemetryEmitter {
    pub violation_count: Arc<AtomicUsize>,
}

impl MockTelemetryEmitter {
    pub fn new() -> Self {
        Self {
            violation_count: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl SandboxTelemetryEmitter for MockTelemetryEmitter {
    fn record_violation(&self, _agent_id: &str, _task_id: &str, _reason: &str) {
        self.violation_count.fetch_add(1, Ordering::SeqCst);
    }
}
