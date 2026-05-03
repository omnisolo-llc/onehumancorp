use opentelemetry::global;
use opentelemetry::metrics::{Counter, Histogram};
use std::sync::OnceLock;

fn harness_execution_duration() -> &'static Histogram<f64> {
    static HARNESS_EXECUTION_DURATION: OnceLock<Histogram<f64>> = OnceLock::new();
    HARNESS_EXECUTION_DURATION.get_or_init(|| {
        let meter = global::meter("ohc-harness");
        meter.f64_histogram("harness_execution_duration_seconds")
            .with_description("Duration of harness executions in seconds")
            .build()
    })
}

fn harness_tool_invocations() -> &'static Counter<u64> {
    static HARNESS_TOOL_INVOCATIONS: OnceLock<Counter<u64>> = OnceLock::new();
    HARNESS_TOOL_INVOCATIONS.get_or_init(|| {
        let meter = global::meter("ohc-harness");
        meter.u64_counter("harness_tool_invocations_total")
            .with_description("Total number of tool invocations within the harness sandbox")
            .build()
    })
}

fn harness_violations() -> &'static Counter<u64> {
    static HARNESS_VIOLATIONS: OnceLock<Counter<u64>> = OnceLock::new();
    HARNESS_VIOLATIONS.get_or_init(|| {
        let meter = global::meter("ohc-harness");
        meter.u64_counter("harness_violations_total")
            .with_description("Total number of harness execution violations")
            .build()
    })
}

pub fn record_harness_execution_duration(duration: f64) {
    harness_execution_duration().record(duration, &[]);
}

pub fn increment_harness_tool_invocations() {
    harness_tool_invocations().add(1, &[]);
}

pub fn increment_harness_violations() {
    harness_violations().add(1, &[]);
}
