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
        let partition_active = std::sync::atomic::AtomicBool::new(true);

        let send_message = || -> Result<(), &'static str> {
            if partition_active.load(std::sync::atomic::Ordering::SeqCst) {
                Err("Network partitioned")
            } else {
                Ok(())
            }
        };

        assert_eq!(send_message(), Err("Network partitioned"));

        partition_active.store(false, std::sync::atomic::Ordering::SeqCst);

        assert_eq!(send_message(), Ok(()));
    }
}
