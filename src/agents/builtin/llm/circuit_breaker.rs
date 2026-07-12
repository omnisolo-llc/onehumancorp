use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;
use tokio::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug)]
pub struct CircuitBreaker {
    failures: AtomicUsize,
    opened_at: Mutex<Option<Instant>>,
    max_failures: usize,
    reset_timeout: Duration,
    probe_in_flight: AtomicBool,
}

impl CircuitBreaker {
    pub fn new(max_failures: usize, reset_timeout: Duration) -> Self {
        Self {
            failures: AtomicUsize::new(0),
            opened_at: Mutex::new(None),
            max_failures: max_failures.max(1),
            reset_timeout,
            probe_in_flight: AtomicBool::new(false),
        }
    }

    pub fn allow(&self) -> bool {
        match self.state() {
            CircuitState::Closed => true,
            CircuitState::Open => false,
            CircuitState::HalfOpen => self
                .probe_in_flight
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok(),
        }
    }

    pub fn record_success(&self) {
        self.failures.store(0, Ordering::Release);
        *self
            .opened_at
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
        self.probe_in_flight.store(false, Ordering::Release);
    }

    pub fn record_failure(&self) {
        self.failures.fetch_add(1, Ordering::AcqRel);
        *self
            .opened_at
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Instant::now());
        self.probe_in_flight.store(false, Ordering::Release);
    }

    pub fn record_transport_error(&self, error: &reqwest::Error) {
        if error.is_timeout() || error.is_connect() || error.is_body() {
            self.record_failure();
        } else {
            self.record_non_failure();
        }
    }

    pub fn record_http_status(&self, status: reqwest::StatusCode) {
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS || status.is_server_error() {
            self.record_failure();
        } else {
            self.record_non_failure();
        }
    }

    /// Release a half-open probe after an error that must not affect breaker state.
    pub fn record_non_failure(&self) {
        self.probe_in_flight.store(false, Ordering::Release);
    }

    pub fn state(&self) -> CircuitState {
        if self.failures.load(Ordering::Acquire) < self.max_failures {
            return CircuitState::Closed;
        }

        let opened_at = *self
            .opened_at
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match opened_at {
            Some(opened_at) if opened_at.elapsed() >= self.reset_timeout => CircuitState::HalfOpen,
            _ => CircuitState::Open,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CircuitBreaker, CircuitState};
    use std::time::Duration;

    #[tokio::test(start_paused = true)]
    async fn allows_only_one_half_open_probe_and_resets_on_success() {
        let breaker = CircuitBreaker::new(2, Duration::from_secs(30));
        breaker.record_failure();
        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Open);
        assert!(!breaker.allow());

        tokio::time::advance(Duration::from_secs(30)).await;
        assert_eq!(breaker.state(), CircuitState::HalfOpen);
        assert!(breaker.allow());
        assert!(!breaker.allow());

        breaker.record_success();
        assert_eq!(breaker.state(), CircuitState::Closed);
        assert!(breaker.allow());
    }

    #[tokio::test(start_paused = true)]
    async fn failed_probe_reopens_breaker_for_a_full_timeout() {
        let breaker = CircuitBreaker::new(1, Duration::from_secs(10));
        breaker.record_failure();
        tokio::time::advance(Duration::from_secs(10)).await;
        assert!(breaker.allow());

        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Open);
        assert!(!breaker.allow());
        tokio::time::advance(Duration::from_secs(9)).await;
        assert!(!breaker.allow());
        tokio::time::advance(Duration::from_secs(1)).await;
        assert!(breaker.allow());
    }

    #[test]
    fn instances_are_independent() {
        let first = CircuitBreaker::new(1, Duration::from_secs(60));
        let second = CircuitBreaker::new(1, Duration::from_secs(60));
        first.record_failure();

        assert_eq!(first.state(), CircuitState::Open);
        assert_eq!(second.state(), CircuitState::Closed);
        assert!(second.allow());
    }

    #[test]
    fn only_rate_limits_and_server_errors_count_as_http_failures() {
        let breaker = CircuitBreaker::new(1, Duration::from_secs(60));
        breaker.record_http_status(reqwest::StatusCode::UNAUTHORIZED);
        assert_eq!(breaker.state(), CircuitState::Closed);

        breaker.record_http_status(reqwest::StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(breaker.state(), CircuitState::Open);

        breaker.record_success();
        breaker.record_http_status(reqwest::StatusCode::BAD_GATEWAY);
        assert_eq!(breaker.state(), CircuitState::Open);
    }
}
