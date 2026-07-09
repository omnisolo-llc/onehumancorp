use super::locks::{StandaloneLock, DistributedLock, RedisLock};

#[tokio::test]
async fn test_standalone_lock_acquire() {
    let lock = StandaloneLock::new();
    let task_id = "test_task_1";

    let mut guard1 = lock.acquire(task_id).await.unwrap();
    // Should be locked now
    guard1.release().await;

    let mut guard2 = lock.acquire(task_id).await.unwrap();
    guard2.release().await;
}

#[tokio::test]
async fn test_acquire_resource_standalone_lock() {
    let lock = StandaloneLock::new();
    let tenant_id = "tenant-1";
    let resource_type = "inventory";
    let resource_id = "item-123";

    let mut _guard1 = lock.acquire_resource(tenant_id, resource_type, resource_id).await.unwrap();

    // The resource should be locked
    let lock_clone = lock.local_locks.lock().await;
    let key = format!("ohc:lock:{}:{}:{}", tenant_id, resource_type, resource_id);
    assert!(lock_clone.contains_key(&key));
    drop(lock_clone);

    // Dropping guard1 should release the lock eventually, but StandaloneLock doesn't remove it from map
}

#[tokio::test]
async fn test_redis_lock_guard_drop_safety() {
    let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let client = match redis::Client::open(redis_url.clone()) {
        Ok(c) => c,
        Err(_) => return,
    };

    // Only proceed if Redis is reachable
    let mut conn = match client.get_multiplexed_tokio_connection().await {
        Ok(c) => c,
        Err(_) => return,
    };

    let lock = RedisLock::new(client.clone());
    let task_id = "test_drop_safety";
    let key = format!("ohc:lock:task:{}", task_id);

    // Clean up before test
    let _: () = redis::cmd("DEL").arg(&key).query_async(&mut conn).await.unwrap();

    let mut guard = lock.acquire(task_id).await.unwrap();

    // Verify it is locked in Redis
    let val1: Option<String> = redis::cmd("GET").arg(&key).query_async(&mut conn).await.unwrap();
    assert!(val1.is_some(), "Lock should be set");

    // Simulate lock expiration and another process acquiring it by manually overwriting the key
    let other_val = "other_process_uuid";
    let _: () = redis::cmd("SET").arg(&key).arg(other_val).query_async(&mut conn).await.unwrap();

    // Now drop the guard. It should try to delete the lock, but fail because the value doesn't match
    guard.release().await;

    // Verify the other process's lock is still there
    let val2: Option<String> = redis::cmd("GET").arg(&key).query_async(&mut conn).await.unwrap();
    assert_eq!(val2.unwrap(), other_val, "Lock should not be deleted by the expired guard");

    // Clean up after test
    let _: () = redis::cmd("DEL").arg(&key).query_async(&mut conn).await.unwrap();
}
