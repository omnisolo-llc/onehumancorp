use std::time::Duration;

pub struct ResiliencePolicy {
    pub max_retries: usize,
    pub initial_backoff: Duration,
    pub timeout: Duration,
}

impl Default for ResiliencePolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff: Duration::from_millis(500),
            timeout: Duration::from_secs(60),
        }
    }
}

pub fn calculate_backoff(attempt: usize, policy: &ResiliencePolicy) -> Duration {
    if attempt == 0 {
        return Duration::from_secs(0);
    }
    policy.initial_backoff * (1 << (attempt - 1)) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_backoff() {
        let policy = ResiliencePolicy::default();
        assert_eq!(calculate_backoff(1, &policy), Duration::from_millis(500));
        assert_eq!(calculate_backoff(2, &policy), Duration::from_millis(1000));
        assert_eq!(calculate_backoff(3, &policy), Duration::from_millis(2000));
    }
}
