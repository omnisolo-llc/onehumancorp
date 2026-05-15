use std::sync::Arc;
use redis::{AsyncCommands, Client};
use async_trait::async_trait;
use uuid::Uuid;
use super::provider::{DistributedLock, LockManager, LockConfig};

#[derive(Clone)]
pub struct RedisLockManager {
    client: Client,
    owner_id: String,
}

impl RedisLockManager {
    pub fn new(client: Client, owner_id: String) -> Self {
        Self {
            client,
            owner_id,
        }
    }
}

pub struct RedisLock {
    manager: RedisLockManager,
    key: String,
    released: bool,
}

#[async_trait]
impl DistributedLock for RedisLock {
    async fn release(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.released {
            return Ok(());
        }

        let mut con = self.manager.client.get_multiplexed_tokio_connection().await?;

        // Use a Lua script to only release if we are the owner
        let script = redis::Script::new(
            r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
            "#
        );

        let _: i32 = script.key(&self.key).arg(&self.manager.owner_id).invoke_async(&mut con).await?;

        self.released = true;
        Ok(())
    }
}

impl Drop for RedisLock {
    fn drop(&mut self) {
        if !self.released {
            let manager = self.manager.clone();
            let key = self.key.clone();
            tokio::spawn(async move {
                if let Ok(mut con) = manager.client.get_multiplexed_tokio_connection().await {
                    let script = redis::Script::new(
                        r#"
                        if redis.call("get", KEYS[1]) == ARGV[1] then
                            return redis.call("del", KEYS[1])
                        else
                            return 0
                        end
                        "#
                    );

                    let _: Result<i32, _> = script.key(&key).arg(&manager.owner_id).invoke_async(&mut con).await;
                }
            });
        }
    }
}

#[async_trait]
impl LockManager for RedisLockManager {
    async fn acquire(&self, config: LockConfig) -> Result<Box<dyn DistributedLock>, Box<dyn std::error::Error + Send + Sync>> {
        let mut attempts = 0;
        let mut con = self.client.get_multiplexed_tokio_connection().await?;
        let ttl_millis = config.ttl.as_millis() as u64;

        loop {
            let acquired: bool = redis::cmd("SET")
                .arg(&config.key)
                .arg(&self.owner_id)
                .arg("NX")
                .arg("PX")
                .arg(ttl_millis)
                .query_async(&mut con)
                .await?;

            if acquired {
                return Ok(Box::new(RedisLock {
                    manager: self.clone(),
                    key: config.key,
                    released: false,
                }));
            }

            attempts += 1;
            if attempts > config.retry_count {
                return Err(format!("Failed to acquire lock for key {} after {} attempts", config.key, attempts).into());
            }

            tokio::time::sleep(config.retry_delay).await;
        }
    }
}
