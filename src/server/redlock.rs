use redis::AsyncCommands;
use std::time::Duration;
use uuid::Uuid;

pub struct Redlock {
    client: redis::Client,
}

impl Redlock {
    pub fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self { client })
    }

    pub fn lock_key(tenant_id: &str, resource_type: &str, resource_id: &str) -> String {
        format!("ohc:lock:{}:{}:{}", tenant_id, resource_type, resource_id)
    }

    pub async fn acquire(&self, key: &str, ttl: Duration) -> Result<Option<String>, String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;
        let lock_value = Uuid::new_v4().to_string();

        // Use SET NX PX
        let result: Option<String> = redis::cmd("SET")
            .arg(key)
            .arg(&lock_value)
            .arg("NX")
            .arg("PX")
            .arg(ttl.as_millis() as u64)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        if result == Some("OK".to_string()) {
            Ok(Some(lock_value))
        } else {
            Ok(None)
        }
    }

    pub async fn release(&self, key: &str, lock_value: &str) -> Result<bool, String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;

        let script = redis::Script::new(
            "if redis.call('get', KEYS[1]) == ARGV[1] then
                 return redis.call('del', KEYS[1])
             else
                 return 0
             end",
        );

        let result: i32 = script
            .key(key)
            .arg(lock_value)
            .invoke_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result == 1)
    }
}
