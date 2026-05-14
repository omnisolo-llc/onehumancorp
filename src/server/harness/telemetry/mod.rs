pub mod store;

pub use store::ViolationStore;

use std::sync::OnceLock;
use opentelemetry::global;
use opentelemetry::metrics::{Counter, Histogram};

static BUBBLEWRAP_SPAWN_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();
static BUBBLEWRAP_EXECUTION_LATENCY: OnceLock<Histogram<f64>> = OnceLock::new();
static BUBBLEWRAP_VIOLATION_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();

pub fn get_bubblewrap_spawn_total() -> &'static Counter<u64> {
    BUBBLEWRAP_SPAWN_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.sandbox");
        meter.u64_counter("ohc_sandbox_bubblewrap_spawn_total")
            .with_description("Total number of Bubblewrap spawns")
            .build()
    })
}

pub fn get_bubblewrap_execution_latency() -> &'static Histogram<f64> {
    BUBBLEWRAP_EXECUTION_LATENCY.get_or_init(|| {
        let meter = global::meter("ohc.sandbox");
        meter.f64_histogram("ohc_sandbox_bubblewrap_execution_latency")
            .with_description("Latency of Bubblewrap executions")
            .build()
    })
}

pub fn get_bubblewrap_violation_total() -> &'static Counter<u64> {
    BUBBLEWRAP_VIOLATION_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.sandbox");
        meter.u64_counter("ohc_sandbox_bubblewrap_violation_total")
            .with_description("Total number of Bubblewrap sandbox violations")
            .build()
    })
}

pub fn record_bubblewrap_spawn() {
    get_bubblewrap_spawn_total().add(1, &[]);
}

pub fn record_bubblewrap_execution_latency(latency: f64) {
    get_bubblewrap_execution_latency().record(latency, &[]);
}

pub fn record_bubblewrap_violation() {
    get_bubblewrap_violation_total().add(1, &[]);
}
