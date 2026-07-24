use std::time::Duration;

pub trait RetryStrategy: Send + Sync {
    fn next_backoff(&self, attempt: usize) -> Duration;
    fn max_retries(&self, requested_max: usize) -> usize {
        requested_max
    }
}

pub struct ExponentialBackoffWithJitter {
    base_ms: u64,
    jitter_max_ms: u64,
}

impl Default for ExponentialBackoffWithJitter {
    fn default() -> Self {
        Self {
            base_ms: 500,
            jitter_max_ms: 100,
        }
    }
}

impl ExponentialBackoffWithJitter {
    pub fn new(base_ms: u64, jitter_max_ms: u64) -> Self {
        Self {
            base_ms,
            jitter_max_ms,
        }
    }
}

impl RetryStrategy for ExponentialBackoffWithJitter {
    fn next_backoff(&self, attempt: usize) -> Duration {
        let base_backoff = self.base_ms * (1 << attempt);
        use rand::Rng;
        let jitter = if self.jitter_max_ms > 0 {
            rand::thread_rng().gen_range(0..self.jitter_max_ms)
        } else {
            0
        };
        Duration::from_millis(base_backoff + jitter)
    }
}

/// A retry strategy that enforces the Stripe constraint of exactly 2 max retries.
pub struct StripeRetryStrategy {
    inner: Box<dyn RetryStrategy>,
}

impl StripeRetryStrategy {
    pub fn new(inner: Box<dyn RetryStrategy>) -> Self {
        Self { inner }
    }
}

impl RetryStrategy for StripeRetryStrategy {
    fn next_backoff(&self, attempt: usize) -> Duration {
        self.inner.next_backoff(attempt)
    }

    fn max_retries(&self, requested_max: usize) -> usize {
        std::cmp::min(requested_max, 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockRetryStrategy;
    impl RetryStrategy for MockRetryStrategy {
        fn next_backoff(&self, _attempt: usize) -> Duration {
            Duration::from_millis(10)
        }
    }

    #[test]
    fn test_stripe_retry_strategy_max_retries() {
        let strategy = StripeRetryStrategy::new(Box::new(MockRetryStrategy));

        // Requesting 5 should be clamped to 2
        assert_eq!(strategy.max_retries(5), 2);

        // Requesting 1 should remain 1
        assert_eq!(strategy.max_retries(1), 1);

        // Requesting 0 should remain 0
        assert_eq!(strategy.max_retries(0), 0);
    }

    #[test]
    fn test_stripe_retry_strategy_next_backoff() {
        let strategy = StripeRetryStrategy::new(Box::new(MockRetryStrategy));
        assert_eq!(strategy.next_backoff(0), Duration::from_millis(10));
    }
}
