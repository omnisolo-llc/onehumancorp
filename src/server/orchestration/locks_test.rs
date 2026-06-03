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
