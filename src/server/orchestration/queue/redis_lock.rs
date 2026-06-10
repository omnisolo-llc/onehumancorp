use std::time::Duration;
use uuid::Uuid;
use redis::AsyncCommands;

pub struct RedisLock {
    client: redis::Client,
    connection: tokio::sync::OnceCell<redis::aio::MultiplexedConnection>,
}

impl RedisLock {
    pub fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self {
            client,
            connection: tokio::sync::OnceCell::new(),
        })
    }

    async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection, String> {
        let conn = self.connection.get_or_try_init(|| async {
            self.client.get_multiplexed_tokio_connection().await
        }).await.map_err(|e| e.to_string())?;
        Ok(conn.clone())
    }

    pub async fn acquire_lock(&self, tenant_id: &str, resource_type: &str, resource_id: &str, ttl_secs: u64) -> Result<Option<String>, String> {
        let key = format!("ohc:lock:{}:{}:{}", tenant_id, resource_type, resource_id);
        let lock_val = Uuid::new_v4().to_string();

        let mut conn = self.get_connection().await?;

        let acquired: bool = redis::cmd("SET")
            .arg(&key)
            .arg(&lock_val)
            .arg("NX")
            .arg("EX")
            .arg(ttl_secs)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        if acquired {
            Ok(Some(lock_val))
        } else {
            Ok(None)
        }
    }

    pub async fn release_lock(&self, tenant_id: &str, resource_type: &str, resource_id: &str, lock_val: &str) -> Result<bool, String> {
        let key = format!("ohc:lock:{}:{}:{}", tenant_id, resource_type, resource_id);
        let mut conn = self.get_connection().await?;

        // Lua script to ensure we only delete the lock if the value matches (prevent deleting someone else's lock if ours expired)
        let script = redis::Script::new(
            r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
            "#,
        );

        let result: i32 = script
            .key(&key)
            .arg(lock_val)
            .invoke_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result == 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_redis_lock_acquire_and_release() {
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        if redis::Client::open(redis_url.as_str()).is_err() {
            return; // Skip test if redis is not available
        }

        let lock = RedisLock::new(&redis_url).unwrap();
        let tenant_id = "test_tenant";
        let resource_type = "test_resource";
        let resource_id = "res_123";

        // Acquire lock
        let lock_val = lock.acquire_lock(tenant_id, resource_type, resource_id, 10).await.unwrap();
        assert!(lock_val.is_some());
        let val = lock_val.unwrap();

        // Release lock
        let released = lock.release_lock(tenant_id, resource_type, resource_id, &val).await.unwrap();
        assert!(released);
    }

    #[tokio::test]
    async fn test_redis_lock_conflict() {
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        if redis::Client::open(redis_url.as_str()).is_err() {
            return;
        }

        let lock1 = RedisLock::new(&redis_url).unwrap();
        let lock2 = RedisLock::new(&redis_url).unwrap();

        let tenant_id = "test_tenant_conflict";
        let resource_type = "test_resource";
        let resource_id = "res_456";

        let lock_val1 = lock1.acquire_lock(tenant_id, resource_type, resource_id, 10).await.unwrap();
        assert!(lock_val1.is_some());

        let lock_val2 = lock2.acquire_lock(tenant_id, resource_type, resource_id, 10).await.unwrap();
        assert!(lock_val2.is_none());

        let val1 = lock_val1.unwrap();
        let released = lock1.release_lock(tenant_id, resource_type, resource_id, &val1).await.unwrap();
        assert!(released);
    }

    #[tokio::test]
    async fn test_redis_lock_wrong_val_release() {
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        if redis::Client::open(redis_url.as_str()).is_err() {
            return;
        }

        let lock = RedisLock::new(&redis_url).unwrap();
        let tenant_id = "test_tenant_wrong_val";
        let resource_type = "test_resource";
        let resource_id = "res_789";

        let lock_val = lock.acquire_lock(tenant_id, resource_type, resource_id, 10).await.unwrap();
        assert!(lock_val.is_some());

        let wrong_val = Uuid::new_v4().to_string();
        let released = lock.release_lock(tenant_id, resource_type, resource_id, &wrong_val).await.unwrap();
        assert!(!released);

        let val = lock_val.unwrap();
        let released = lock.release_lock(tenant_id, resource_type, resource_id, &val).await.unwrap();
        assert!(released);
    }

    #[tokio::test]
    async fn test_redis_lock_expiration() {
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        if redis::Client::open(redis_url.as_str()).is_err() {
            return;
        }

        let lock = RedisLock::new(&redis_url).unwrap();
        let tenant_id = "test_tenant_exp";
        let resource_type = "test_resource";
        let resource_id = "res_abc";

        let lock_val = lock.acquire_lock(tenant_id, resource_type, resource_id, 1).await.unwrap();
        assert!(lock_val.is_some());

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let lock_val2 = lock.acquire_lock(tenant_id, resource_type, resource_id, 10).await.unwrap();
        assert!(lock_val2.is_some());

        let val2 = lock_val2.unwrap();
        let released = lock.release_lock(tenant_id, resource_type, resource_id, &val2).await.unwrap();
        assert!(released);
    }

    #[tokio::test]
    async fn test_redis_lock_double_release() {
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        if redis::Client::open(redis_url.as_str()).is_err() {
            return;
        }

        let lock = RedisLock::new(&redis_url).unwrap();
        let tenant_id = "test_tenant_double";
        let resource_type = "test_resource";
        let resource_id = "res_def";

        let lock_val = lock.acquire_lock(tenant_id, resource_type, resource_id, 10).await.unwrap();
        assert!(lock_val.is_some());
        let val = lock_val.unwrap();

        let released1 = lock.release_lock(tenant_id, resource_type, resource_id, &val).await.unwrap();
        assert!(released1);

        let released2 = lock.release_lock(tenant_id, resource_type, resource_id, &val).await.unwrap();
        assert!(!released2);
    }
}
