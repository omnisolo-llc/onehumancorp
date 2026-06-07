use opentelemetry::metrics::Counter;
use std::sync::OnceLock;
use opentelemetry::global;

static HARNESS_SECURITY_DIVERGENCE_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();

pub fn get_harness_security_divergence_total() -> &'static Counter<u64> {
    HARNESS_SECURITY_DIVERGENCE_TOTAL.get_or_init(|| {
        let meter = global::meter("ohc.sandbox");
        meter
            .u64_counter("ohc_harness_security_divergence_total")
            .with_description("Total number of regex and AST divergence events in bash parsing")
            .build()
    })
}

pub fn record_harness_security_divergence() {
    let counter = get_harness_security_divergence_total();
    counter.add(1, &[]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_harness_security_divergence() {
        record_harness_security_divergence();
        let counter = get_harness_security_divergence_total();
        // Since OpenTelemetry counters are not easily observable directly without setting up an exporter,
        // just making sure calling it doesn't panic.
        counter.add(0, &[]);
    }
}
