use redis::{AsyncCommands, Client};
use uuid::Uuid;
use std::time::Duration;

#[allow(dead_code)]
pub struct Redlock {
    client: Client,
}

#[allow(dead_code)]
pub struct Lock {
    pub key: String,
    pub val: String,
}

impl Redlock {
    #[allow(dead_code)]
    pub fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }

    #[allow(dead_code)]
    pub async fn lock(&self, resource: &str, ttl: Duration) -> Result<Option<Lock>, redis::RedisError> {
        let mut con = self.client.get_async_connection().await?;
        let val = Uuid::new_v4().to_string();
        let key = format!("ohc:lock:{}", resource);

        let acquired: bool = redis::cmd("SET")
            .arg(&key)
            .arg(&val)
            .arg("NX")
            .arg("PX")
            .arg(ttl.as_millis() as u64)
            .query_async(&mut con)
            .await?;

        if acquired {
            Ok(Some(Lock { key, val }))
        } else {
            Ok(None)
        }
    }

    #[allow(dead_code)]
    pub async fn unlock(&self, lock: &Lock) -> Result<bool, redis::RedisError> {
        let mut con = self.client.get_async_connection().await?;

        // Use Lua script to ensure atomicity: only delete if the value matches
        let script = redis::Script::new(
            r#"
            if redis.call("get",KEYS[1]) == ARGV[1] then
                return redis.call("del",KEYS[1])
            else
                return 0
            end
            "#,
        );

        let result: i32 = script
            .key(&lock.key)
            .arg(&lock.val)
            .invoke_async(&mut con)
            .await?;

        Ok(result == 1)
    }
}
