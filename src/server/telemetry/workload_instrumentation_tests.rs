use super::workload_instrumentation::WorkloadInstrumentation;
use std::time::Duration;

#[test]
fn test_workload_instrumentation() {
    let instrumentation = WorkloadInstrumentation::new("test_workload");
    std::thread::sleep(Duration::from_millis(10));
    instrumentation.record_outcome(true, 0.05);
}
