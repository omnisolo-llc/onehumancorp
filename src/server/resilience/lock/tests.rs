use super::*;
use std::time::Duration;

#[tokio::test]
async fn test_memory_lock() {
    let manager1 = MemoryLockManager::new("owner1".to_string());
    let manager2 = MemoryLockManager::new("owner2".to_string());

    let mut config = LockConfig::default();
    config.key = "test-lock".to_string();
    config.retry_count = 0;

    let mut lock1 = manager1.acquire(config.clone()).await.expect("Failed to acquire lock1");

    let lock2_res = manager2.acquire(config.clone()).await;
    assert!(lock2_res.is_err(), "owner2 should not acquire while owner1 holds lock");

    lock1.release().await.expect("Failed to release lock1");

    let _lock2 = manager2.acquire(config).await.expect("owner2 should acquire after release");
}
