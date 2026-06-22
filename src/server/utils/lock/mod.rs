use redsync::{RedisInstance, Redsync};
use std::time::Duration;

pub struct InventoryLockManager {
    redsync: Redsync<RedisInstance>,
}

impl InventoryLockManager {
    pub fn new(redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let instance = RedisInstance::new(redis_url).map_err(|e| e.to_string())?;
        let redsync = Redsync::new(vec![instance]);
        Ok(Self { redsync })
    }

    pub fn acquire_lock(&self, tenant_id: &str, product_id: &str, ttl_ms: u64) -> Result<redsync::Lock, Box<dyn std::error::Error>> {
        let resource_name = format!("ohc:lock:{}:inventory:{}", tenant_id, product_id);
        match self.redsync.lock(&resource_name, Duration::from_millis(ttl_ms)) {
            Ok(lock) => Ok(lock),
            Err(e) => Err(format!("Failed to acquire lock: {:?}", e).into()),
        }
    }

    pub fn release_lock(&self, lock: &redsync::Lock) -> Result<(), Box<dyn std::error::Error>> {
        match self.redsync.unlock(lock) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to release lock: {:?}", e).into()),
        }
    }
}
