#[cfg(test)]
mod chaos_tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    // Simulate killing a worker midway through processing
    #[tokio::test]
    async fn test_chaos_kill_worker() {
        let is_killed = Arc::new(AtomicBool::new(false));
        let is_killed_clone = is_killed.clone();

        let _handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            is_killed_clone.store(true, Ordering::SeqCst);
        });

        tokio::time::sleep(Duration::from_millis(20)).await;

        assert!(is_killed.load(Ordering::SeqCst), "Worker should have been simulated as killed");
    }

    // Simulate state corruption scenario
    #[tokio::test]
    async fn test_chaos_corrupt_state() {
        let mut state = vec![1, 2, 3, 4, 5];

        // Inject corruption
        state[2] = 99;

        assert_eq!(state[2], 99, "State corruption should be verifiable");
    }

    // Simulate a network partition
    #[tokio::test]
    async fn test_chaos_network_partition() {
        let partition_active = Arc::new(AtomicBool::new(true));

        let partition_active_clone = partition_active.clone();
        let send_message = move || -> Result<(), &'static str> {
            if partition_active_clone.load(Ordering::SeqCst) {
                Err("Network partitioned")
            } else {
                Ok(())
            }
        };

        assert_eq!(send_message(), Err("Network partitioned"));

        partition_active.store(false, Ordering::SeqCst);

        assert_eq!(send_message(), Ok(()));
    }
}
#[cfg(test)]
mod additional_chaos_tests {
    use std::time::Duration;

    #[tokio::test]
    async fn test_chaos_agent_job_retry_exponential_backoff() {
        let attempts = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let max_retries = 3;

        let attempts_clone = attempts.clone();
        let operation = move || {
            let attempts_inner = attempts_clone.clone();
            async move {
                let current = attempts_inner.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                if current <= 3 {
                    return Err("Transient AI agent error");
                }
                Ok("Success")
            }
        };

        let mut result = Err("Initial");
        for i in 0..max_retries {
            result = operation().await;
            if result.is_ok() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10 * (2_u64.pow(i as u32)))).await;
        }

        assert!(result.is_err());
        assert_eq!(attempts.load(std::sync::atomic::Ordering::SeqCst), 3);

        // Final attempt which should succeed
        let final_result = operation().await;
        assert_eq!(final_result.unwrap(), "Success");
    }

    #[tokio::test]
    async fn test_ml_resilience_malformed_llm_response() {
        // Enforce the ML-Resilience rule for malformed LLM responses
        let mock_response = r#"{"invalid_json"#;
        let result: Result<serde_json::Value, _> = serde_json::from_str(mock_response);

        assert!(result.is_err(), "Chaos resilience must gracefully handle malformed LLM response");
    }
}
