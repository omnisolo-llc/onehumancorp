use std::time::Duration;
use uuid::Uuid;
use redis::AsyncCommands;

pub struct RedisLock {
    client: redis::Client,
}

impl RedisLock {
    pub fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self {
            client,
        })
    }

    async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection, String> {
        self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())
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
