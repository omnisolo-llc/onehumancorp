use super::redlock::Redlock;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_redlock_acquire_release() {
    let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    let redlock = Redlock::new(&redis_url).expect("Failed to create Redlock client");

    let tenant_id = "test_tenant";
    let resource_type = "stock";
    let resource_id = "item_123";
    let ttl_ms = 1000;

    // Test acquire lock
    let lock_val = redlock.acquire_lock(tenant_id, resource_type, resource_id, ttl_ms).await;
    assert!(lock_val.is_ok(), "Should acquire lock");
    let lock_val = lock_val.unwrap();

    // Test double acquire lock (should fail)
    let lock_val2 = redlock.acquire_lock(tenant_id, resource_type, resource_id, ttl_ms).await;
    assert!(lock_val2.is_err(), "Should not acquire lock if already locked");

    // Test release lock
    let release_res = redlock.release_lock(tenant_id, resource_type, resource_id, &lock_val).await;
    assert!(release_res.is_ok(), "Should release lock");

    // Test acquire lock after release
    let lock_val3 = redlock.acquire_lock(tenant_id, resource_type, resource_id, ttl_ms).await;
    assert!(lock_val3.is_ok(), "Should acquire lock after it was released");
}
