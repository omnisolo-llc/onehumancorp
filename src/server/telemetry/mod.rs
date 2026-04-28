use std::time::Duration;
use opentelemetry::metrics::{Counter, Histogram, Meter, Unit};
use opentelemetry::{global, KeyValue};
use std::sync::OnceLock;

static METRICS_INIT: std::sync::Once = std::sync::Once::new();
static mut SYNC_LATENCY: Option<Histogram<f64>> = None;
static mut SYNC_PAYLOAD_BYTES: Option<Counter<u64>> = None;

fn get_meter() -> Meter {
    global::meter("ohc.server.sipdb")
}

pub fn init_telemetry() {
    METRICS_INIT.call_once(|| {
        let meter = get_meter();
        let latency = meter
            .f64_histogram("ohc_sync_latency_seconds")
            .with_description("Latency of SIPDB sync operations in seconds")
            .with_unit(Unit::new("s"))
            .init();

        let payload = meter
            .u64_counter("ohc_sync_payload_bytes")
            .with_description("Size of payload synced from SIPDB")
            .with_unit(Unit::new("bytes"))
            .init();

        unsafe {
            SYNC_LATENCY = Some(latency);
            SYNC_PAYLOAD_BYTES = Some(payload);
        }
    });
}

pub fn record_sync_latency(ctx: &str, duration: Duration) {
    init_telemetry();
    if let Some(ref metric) = unsafe { SYNC_LATENCY.as_ref() } {
        metric.record(duration.as_secs_f64(), &[KeyValue::new("context", ctx.to_string())]);
    }
}

pub fn record_sync_payload_size(ctx: &str, bytes: usize) {
    init_telemetry();
    if let Some(ref metric) = unsafe { SYNC_PAYLOAD_BYTES.as_ref() } {
        metric.add(bytes as u64, &[KeyValue::new("context", ctx.to_string())]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry::global;

    #[test]
    fn test_record_sync_latency() {
        init_telemetry();
        record_sync_latency("test_latency", Duration::from_millis(50));
        assert!(unsafe { SYNC_LATENCY.is_some() });
    }

    #[test]
    fn test_record_sync_payload_size() {
        init_telemetry();
        record_sync_payload_size("test_payload", 1024);
        assert!(unsafe { SYNC_PAYLOAD_BYTES.is_some() });
    }
}
