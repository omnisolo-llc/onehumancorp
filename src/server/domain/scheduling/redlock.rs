use sqlx::PgPool;
use std::time::Duration;
use uuid::Uuid;
use redis::AsyncCommands;
use redis::aio::Connection;
use redis::Client;

pub struct Redlock {
    client: Client,
}

impl Redlock {
    pub fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }

    pub async fn acquire_lock(&self, tenant_id: &str, resource_type: &str, resource_id: &str, ttl_ms: u64) -> Result<String, redis::RedisError> {
        let lock_key = format!("ohc:lock:{}:{}:{}", tenant_id, resource_type, resource_id);
        let lock_val = Uuid::new_v4().to_string();

        let mut conn = self.client.get_async_connection().await?;

        let result: bool = redis::cmd("SET")
            .arg(&lock_key)
            .arg(&lock_val)
            .arg("NX")
            .arg("PX")
            .arg(ttl_ms)
            .query_async(&mut conn)
            .await?;

        if result {
            Ok(lock_val)
        } else {
            Err(redis::RedisError::from((redis::ErrorKind::IoError, "Lock already acquired")))
        }
    }

    pub async fn release_lock(&self, tenant_id: &str, resource_type: &str, resource_id: &str, lock_val: &str) -> Result<(), redis::RedisError> {
        let lock_key = format!("ohc:lock:{}:{}:{}", tenant_id, resource_type, resource_id);
        let mut conn = self.client.get_async_connection().await?;

        let script = redis::Script::new(
            "if redis.call('get', KEYS[1]) == ARGV[1] then return redis.call('del', KEYS[1]) else return 0 end"
        );

        let _result: i32 = script.key(&lock_key).arg(lock_val).invoke_async(&mut conn).await?;
        Ok(())
    }
}
