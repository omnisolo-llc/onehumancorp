use std::time::Duration;
use crate::circuit::CircuitBreaker;

#[derive(Clone)]
pub struct ResilienceConfig {
    pub task_timeout: Duration,
    pub max_retries: u32,
    pub circuit_breaker: CircuitBreaker,
}

impl Default for ResilienceConfig {
    fn default() -> Self {
        Self {
            task_timeout: Duration::from_secs(60),
            max_retries: 3,
            circuit_breaker: CircuitBreaker::new(5, Duration::from_secs(30)),
        }
    }
}
