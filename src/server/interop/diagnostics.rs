
pub mod mesh_diagnostics {
    use std::sync::atomic::{AtomicU64, Ordering};
    use tokio::time::{sleep, Duration};

    pub struct DiagnosticProbe {
        total_pings: AtomicU64,
        failed_pings: AtomicU64,
        recovered_pings: AtomicU64,
    }

    impl DiagnosticProbe {
        pub fn new() -> Self {
            Self {
                total_pings: AtomicU64::new(0),
                failed_pings: AtomicU64::new(0),
                recovered_pings: AtomicU64::new(0),
            }
        }

        pub fn record_ping(&self) {
            self.total_pings.fetch_add(1, Ordering::Relaxed);
        }

        pub fn record_failure(&self) {
            self.failed_pings.fetch_add(1, Ordering::Relaxed);
        }

        pub fn record_recovery(&self) {
            self.recovered_pings.fetch_add(1, Ordering::Relaxed);
        }

        pub fn health_score(&self) -> f64 {
            let total = self.total_pings.load(Ordering::Relaxed);
            let failed = self.failed_pings.load(Ordering::Relaxed);
            if total == 0 {
                return 100.0;
            }
            let success = total.saturating_sub(failed);
            (success as f64 / total as f64) * 100.0
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_diagnostic_probe_scoring() {
            let probe = DiagnosticProbe::new();
            assert_eq!(probe.health_score(), 100.0);

            probe.record_ping();
            probe.record_ping();
            probe.record_failure();

            assert_eq!(probe.health_score(), 50.0);
        }

        #[test]
        fn test_diagnostic_probe_recovery() {
            let probe = DiagnosticProbe::new();
            probe.record_recovery();
            assert_eq!(probe.recovered_pings.load(Ordering::Relaxed), 1);
        }
    }
}
