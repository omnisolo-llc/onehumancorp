use super::lock::{LockManager, LockManagerInterface};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_standalone_lock_acquire_release() {
    unsafe { std::env::set_var("OHC_MULTITENANT", "false"); }

    let manager = LockManager::new().unwrap();
    let org_id = "test_org";
    let resource = "test_resource";

    // Acquire the lock
    let token1 = manager.acquire_lock(org_id, resource, Duration::from_secs(10)).await.unwrap();
    assert!(token1.is_some());
    let token1_val = token1.unwrap();

    // Try to acquire the same lock again, should fail
    let token2 = manager.acquire_lock(org_id, resource, Duration::from_secs(10)).await.unwrap();
    assert!(token2.is_none());

    // Release the lock
    let released = manager.release_lock(org_id, resource, &token1_val).await.unwrap();
    assert!(released);

    // Try to acquire the lock again after release, should succeed
    let token3 = manager.acquire_lock(org_id, resource, Duration::from_secs(10)).await.unwrap();
    assert!(token3.is_some());
}

#[tokio::test]
async fn test_standalone_lock_expiration() {
    unsafe { std::env::set_var("OHC_MULTITENANT", "false"); }

    let manager = LockManager::new().unwrap();
    let org_id = "test_org";
    let resource = "test_resource_exp";

    // Acquire the lock with a short TTL
    let token1 = manager.acquire_lock(org_id, resource, Duration::from_millis(100)).await.unwrap();
    assert!(token1.is_some());

    // Try to acquire immediately, should fail
    let token2 = manager.acquire_lock(org_id, resource, Duration::from_secs(10)).await.unwrap();
    assert!(token2.is_none());

    // Wait for TTL to expire
    sleep(Duration::from_millis(150)).await;

    // Try to acquire again, should succeed because previous lock expired
    let token3 = manager.acquire_lock(org_id, resource, Duration::from_secs(10)).await.unwrap();
    assert!(token3.is_some());
}

#[tokio::test]
async fn test_standalone_e2e_concurrent_agents() {
    unsafe { std::env::set_var("OHC_MULTITENANT", "false"); }

    let manager = std::sync::Arc::new(LockManager::new().unwrap());
    let org_id = "test_org_e2e";
    let resource = "shared_config_file";

    let manager_clone1 = manager.clone();
    let manager_clone2 = manager.clone();

    // Agent 1 tries to acquire lock
    let agent1 = tokio::spawn(async move {
        manager_clone1.acquire_lock(org_id, resource, Duration::from_secs(5)).await
    });

    // Wait a tiny bit to ensure Agent 1 goes first
    sleep(Duration::from_millis(10)).await;

    // Agent 2 tries to acquire lock
    let agent2 = tokio::spawn(async move {
        manager_clone2.acquire_lock(org_id, resource, Duration::from_secs(5)).await
    });

    let result1 = agent1.await.unwrap().unwrap();
    let result2 = agent2.await.unwrap().unwrap();

    // One should succeed, the other should fail
    assert!(result1.is_some());
    assert!(result2.is_none());
}
