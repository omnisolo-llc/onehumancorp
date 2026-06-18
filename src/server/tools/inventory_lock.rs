use redis::{AsyncCommands, Client};
use std::time::Duration;


#[derive(Clone)]
pub struct DistributedLockManager {
    client: Client,
}

impl DistributedLockManager {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn acquire_lock(&self, tenant_id: &str, product_id: &str, lock_id: &str, duration: Duration) -> bool {
        let key = format!("ohc:lock:{}:inventory:{}", tenant_id, product_id);
        let mut conn = match self.client.get_async_connection().await {
            Ok(conn) => conn,
            Err(_) => return false,
        };

        let result: redis::RedisResult<bool> = redis::cmd("SET")
            .arg(key)
            .arg(lock_id)
            .arg("NX")
            .arg("PX")
            .arg(duration.as_millis() as u64)
            .query_async(&mut conn)
            .await;

        match result {
            Ok(success) => success,
            Err(_) => false,
        }
    }

    pub async fn release_lock(&self, tenant_id: &str, product_id: &str, lock_id: &str) -> bool {
        let key = format!("ohc:lock:{}:inventory:{}", tenant_id, product_id);
        let mut conn = match self.client.get_async_connection().await {
            Ok(conn) => conn,
            Err(_) => return false,
        };

        // Lua script to safely delete the lock only if it matches the lock_id
        let script = redis::Script::new(
            r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
            "#,
        );

        let result: redis::RedisResult<i32> = script.key(key).arg(lock_id).invoke_async(&mut conn).await;
        match result {
            Ok(val) => val == 1,
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lock_acquire_and_release() {
        // We simulate concurrency behavior by attempting to acquire locks and handling connection errors gracefully.
        let client_res = Client::open("redis://127.0.0.1:6379");
        if client_res.is_err() {
            return; // Skip if no Redis available
        }
        let client = client_res.unwrap();

        // Try to connect to see if redis is running
        let mut conn = match client.get_async_connection().await {
            Ok(c) => c,
            Err(_) => return, // Redis not running, skip test
        };

        let manager = DistributedLockManager::new(client);
        let tenant = "test_tenant";
        let product = "test_product";
        let lock_id1 = "lock1";
        let lock_id2 = "lock2";

        // 1. First lock should succeed
        let success1 = manager.acquire_lock(tenant, product, lock_id1, Duration::from_secs(5)).await;
        assert!(success1, "First lock should be acquired");

        // 2. Second lock by someone else should fail
        let success2 = manager.acquire_lock(tenant, product, lock_id2, Duration::from_secs(5)).await;
        assert!(!success2, "Second lock should fail due to existing lock");

        // 3. Releasing someone else's lock should fail
        let release_wrong = manager.release_lock(tenant, product, lock_id2).await;
        assert!(!release_wrong, "Releasing someone else's lock should fail");

        // 4. Releasing own lock should succeed
        let release_correct = manager.release_lock(tenant, product, lock_id1).await;
        assert!(release_correct, "Releasing own lock should succeed");

        // 5. Acquiring lock again after release should succeed
        let success3 = manager.acquire_lock(tenant, product, lock_id2, Duration::from_secs(5)).await;
        assert!(success3, "Lock should be available again after release");

        // Clean up
        let _ = manager.release_lock(tenant, product, lock_id2).await;
    }
}
