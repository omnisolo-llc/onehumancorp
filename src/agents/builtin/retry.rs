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
    fn test_exponential_backoff_no_jitter() {
        let strategy = ExponentialBackoffWithJitter::new(100, 0);
        assert_eq!(strategy.next_backoff(0), Duration::from_millis(100)); // 100 * 2^0
        assert_eq!(strategy.next_backoff(1), Duration::from_millis(200)); // 100 * 2^1
        assert_eq!(strategy.next_backoff(2), Duration::from_millis(400)); // 100 * 2^2
        assert_eq!(strategy.next_backoff(3), Duration::from_millis(800)); // 100 * 2^3
    }

    #[test]
    fn test_exponential_backoff_with_jitter() {
        let strategy = ExponentialBackoffWithJitter::new(100, 50);

        let backoff0 = strategy.next_backoff(0);
        assert!(backoff0 >= Duration::from_millis(100) && backoff0 < Duration::from_millis(150));

        let backoff1 = strategy.next_backoff(1);
        assert!(backoff1 >= Duration::from_millis(200) && backoff1 < Duration::from_millis(250));

        let backoff2 = strategy.next_backoff(2);
        assert!(backoff2 >= Duration::from_millis(400) && backoff2 < Duration::from_millis(450));
    }

    #[test]
    fn test_default() {
        let strategy = ExponentialBackoffWithJitter::default();
        let backoff0 = strategy.next_backoff(0);
        assert!(backoff0 >= Duration::from_millis(500) && backoff0 < Duration::from_millis(600));
    }
}
