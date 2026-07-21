use std::time::Duration;

pub trait RetryStrategy: Send + Sync {
    fn next_backoff(&self, attempt: usize) -> Duration;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_backoff_default() {
        let strategy = ExponentialBackoffWithJitter::default();
        // Base is 500ms, jitter is 100ms max.
        let backoff_0 = strategy.next_backoff(0).as_millis() as u64;
        assert!(backoff_0 >= 500 && backoff_0 <= 600, "Attempt 0 failed bounds: {}", backoff_0);

        let backoff_1 = strategy.next_backoff(1).as_millis() as u64;
        assert!(backoff_1 >= 1000 && backoff_1 <= 1100, "Attempt 1 failed bounds: {}", backoff_1);

        let backoff_2 = strategy.next_backoff(2).as_millis() as u64;
        assert!(backoff_2 >= 2000 && backoff_2 <= 2100, "Attempt 2 failed bounds: {}", backoff_2);
    }

    #[test]
    fn test_exponential_backoff_custom() {
        let strategy = ExponentialBackoffWithJitter::new(100, 50);
        let backoff_0 = strategy.next_backoff(0).as_millis() as u64;
        assert!(backoff_0 >= 100 && backoff_0 <= 150, "Attempt 0 failed bounds: {}", backoff_0);

        let backoff_1 = strategy.next_backoff(1).as_millis() as u64;
        assert!(backoff_1 >= 200 && backoff_1 <= 250, "Attempt 1 failed bounds: {}", backoff_1);
    }

    #[test]
    fn test_exponential_backoff_no_jitter() {
        let strategy = ExponentialBackoffWithJitter::new(100, 0);
        let backoff_0 = strategy.next_backoff(0).as_millis() as u64;
        assert_eq!(backoff_0, 100, "Attempt 0 failed bounds: {}", backoff_0);

        let backoff_1 = strategy.next_backoff(1).as_millis() as u64;
        assert_eq!(backoff_1, 200, "Attempt 1 failed bounds: {}", backoff_1);
    }
}
