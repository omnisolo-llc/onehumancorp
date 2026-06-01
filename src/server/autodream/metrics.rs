use opentelemetry::{global, metrics::{Counter, Histogram}};
use std::sync::OnceLock;

static MEMORIES_PROCESSED_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();
static BATCH_PROCESSING_DURATION_MS: OnceLock<Histogram<f64>> = OnceLock::new();
static CONSOLIDATION_ERRORS_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();

pub fn get_memories_processed_total() -> &'static Counter<u64> {
    MEMORIES_PROCESSED_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.autodream");
        meter
            .u64_counter("ohc_autodream_memories_processed_total")
            .with_description("Total memories processed")
            .build()
    })
}

pub fn get_batch_processing_duration_ms_histogram() -> &'static Histogram<f64> {
    BATCH_PROCESSING_DURATION_MS.get_or_init(|| {
        let meter = global::meter("ohc.autodream");
        meter
            .f64_histogram("ohc_autodream_batch_processing_duration_ms")
            .with_description("Batch processing duration in milliseconds")
            .build()
    })
}

pub fn get_consolidation_errors_total() -> &'static Counter<u64> {
    CONSOLIDATION_ERRORS_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.autodream");
        meter
            .u64_counter("ohc_autodream_consolidation_errors_total")
            .with_description("Total consolidation errors")
            .build()
    })
}

pub fn record_memories_processed(mode: &str, count: u64) {
    let counter = get_memories_processed_total();
    counter.add(count, &[
        opentelemetry::KeyValue::new("mode", mode.to_string()),
    ]);
}

pub fn record_batch_processing_duration_ms(mode: &str, duration_ms: f64) {
    let histogram = get_batch_processing_duration_ms_histogram();
    histogram.record(duration_ms, &[
        opentelemetry::KeyValue::new("mode", mode.to_string()),
    ]);
}

pub fn record_consolidation_error(mode: &str, error: &str) {
    let counter = get_consolidation_errors_total();
    counter.add(1, &[
        opentelemetry::KeyValue::new("mode", mode.to_string()),
        opentelemetry::KeyValue::new("error", error.to_string()),
    ]);
}
