use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Clone)]
pub struct CircuitBreaker {
    state: Arc<Mutex<CircuitState>>,
    failure_count: Arc<AtomicUsize>,
    failure_threshold: usize,
    reset_timeout: Duration,
    last_failure_time: Arc<Mutex<Option<Instant>>>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, reset_timeout: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(CircuitState::Closed)),
            failure_count: Arc::new(AtomicUsize::new(0)),
            failure_threshold,
            reset_timeout,
            last_failure_time: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn execute<F, T, E>(&self, operation: F) -> Result<T, E>
    where
        F: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Debug,
    {
        let mut state = self.state.lock().await;

        if *state == CircuitState::Open {
            let last_failure = self.last_failure_time.lock().await.unwrap();
            if last_failure.elapsed() > self.reset_timeout {
                *state = CircuitState::HalfOpen;
            } else {
                panic!("Circuit breaker is open");
            }
        }

        drop(state);

        let result = operation.await;

        let mut state = self.state.lock().await;
        match &result {
            Ok(_) => {
                if *state == CircuitState::HalfOpen {
                    *state = CircuitState::Closed;
                    self.failure_count.store(0, Ordering::SeqCst);
                }
            }
            Err(_) => {
                let current_failures = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;
                if current_failures >= self.failure_threshold {
                    *state = CircuitState::Open;
                    let mut last_failure = self.last_failure_time.lock().await;
                    *last_failure = Some(Instant::now());
                } else if *state == CircuitState::HalfOpen {
                    *state = CircuitState::Open;
                    let mut last_failure = self.last_failure_time.lock().await;
                    *last_failure = Some(Instant::now());
                }
            }
        }

        result
    }
}
