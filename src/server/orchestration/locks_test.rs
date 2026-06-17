use super::locks::{StandaloneLock, DistributedLock};

#[tokio::test]
async fn test_standalone_lock_acquire() {
    let lock = StandaloneLock::new();
    let task_id = "test_task_1";

    let guard1 = lock.acquire(task_id).await.unwrap();
    // Should be locked now
    drop(guard1);

    let guard2 = lock.acquire(task_id).await.unwrap();
    drop(guard2);
}

#[tokio::test]
async fn test_acquire_resource_standalone_lock() {
    let lock = StandaloneLock::new();
    let tenant_id = "tenant-1";
    let resource_type = "inventory";
    let resource_id = "item-123";

    let _guard1 = lock.acquire_resource(tenant_id, resource_type, resource_id).await.unwrap();

    // The resource should be locked
    let lock_clone = lock.locks.lock().await;
    let key = format!("ohc:lock:{}:{}:{}", tenant_id, resource_type, resource_id);
    assert!(lock_clone.contains_key(&key));
    drop(lock_clone);

    // Dropping guard1 should release the lock eventually, but StandaloneLock doesn't remove it from map
}
