use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::global;
use std::sync::OnceLock;

static MEMORIES_PROCESSED_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();
static BATCH_PROCESSING_DURATION: OnceLock<Histogram<f64>> = OnceLock::new();
static CONSOLIDATION_ERRORS_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();

pub fn get_memories_processed_total() -> &'static Counter<u64> {
    MEMORIES_PROCESSED_TOTAL.get_or_init(|| {
        global::meter("ohc.autodream")
            .u64_counter("autodream.memories.processed.total")
            .with_description("Total number of memories processed by AutoDream")
            .build()
    })
}

pub fn get_batch_processing_duration() -> &'static Histogram<f64> {
    BATCH_PROCESSING_DURATION.get_or_init(|| {
        global::meter("ohc.autodream")
            .f64_histogram("autodream.batch.processing.duration.seconds")
            .with_description("Duration of AutoDream batch processing")
            .build()
    })
}

pub fn get_consolidation_errors_total() -> &'static Counter<u64> {
    CONSOLIDATION_ERRORS_TOTAL.get_or_init(|| {
        global::meter("ohc.autodream")
            .u64_counter("autodream.consolidation.errors.total")
            .with_description("Total number of errors during AutoDream memory consolidation")
            .build()
    })
}
