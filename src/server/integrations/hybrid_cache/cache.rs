use async_trait::async_trait;
use dashmap::DashMap;
use redis::AsyncCommands;
use std::env;
use std::sync::Arc;
use std::time::Duration;

#[async_trait]
pub trait CacheDriver: Send + Sync {
    async fn get_cache(&self, tenant_id: &str, key: &str) -> Result<Option<Vec<u8>>, String>;
    async fn set_cache(
        &self,
        tenant_id: &str,
        key: &str,
        value: Vec<u8>,
        ttl: Duration,
    ) -> Result<(), String>;
    async fn delete_cache(&self, tenant_id: &str, key: &str) -> Result<(), String>;
}

pub struct StandaloneDriver {
    // In-memory cache using DashMap
    // Key format: "tenant_id:key", Value: (data, expiration_timestamp_ms)
    store: Arc<DashMap<String, (Vec<u8>, u64)>>,
}

impl StandaloneDriver {
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
        }
    }

    fn now_ms() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

#[async_trait]
impl CacheDriver for StandaloneDriver {
    async fn get_cache(&self, tenant_id: &str, key: &str) -> Result<Option<Vec<u8>>, String> {
        let full_key = format!("{}:{}", tenant_id, key);
        if let Some(entry) = self.store.get(&full_key) {
            let (data, expires_at) = entry.value();
            if *expires_at > Self::now_ms() {
                return Ok(Some(data.clone()));
            } else {
                // Expired, drop ref then remove
                drop(entry);
                self.store.remove(&full_key);
            }
        }
        Ok(None)
    }

    async fn set_cache(
        &self,
        tenant_id: &str,
        key: &str,
        value: Vec<u8>,
        ttl: Duration,
    ) -> Result<(), String> {
        let full_key = format!("{}:{}", tenant_id, key);
        let expires_at = Self::now_ms() + ttl.as_millis() as u64;
        self.store.insert(full_key, (value, expires_at));
        Ok(())
    }

    async fn delete_cache(&self, tenant_id: &str, key: &str) -> Result<(), String> {
        let full_key = format!("{}:{}", tenant_id, key);
        self.store.remove(&full_key);
        Ok(())
    }
}

pub struct CloudDriver {
    client: redis::Client,
}

impl CloudDriver {
    pub fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self { client })
    }
}

#[async_trait]
impl CacheDriver for CloudDriver {
    async fn get_cache(&self, tenant_id: &str, key: &str) -> Result<Option<Vec<u8>>, String> {
        let full_key = format!("{}:{}", tenant_id, key);
        let mut con = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| e.to_string())?;
        let result: Option<Vec<u8>> = con.get(&full_key).await.map_err(|e| e.to_string())?;
        Ok(result)
    }

    async fn set_cache(
        &self,
        tenant_id: &str,
        key: &str,
        value: Vec<u8>,
        ttl: Duration,
    ) -> Result<(), String> {
        let full_key = format!("{}:{}", tenant_id, key);
        let mut con = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| e.to_string())?;
        let ttl_secs = ttl.as_secs().max(1); // Minimum 1 second TTL
        let _: () = con.set_ex(&full_key, value, ttl_secs)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn delete_cache(&self, tenant_id: &str, key: &str) -> Result<(), String> {
        let full_key = format!("{}:{}", tenant_id, key);
        let mut con = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| e.to_string())?;
        let _: () = con.del(&full_key).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub struct CacheManager {
    driver: Box<dyn CacheDriver>,
}

impl CacheManager {
    pub fn new() -> Result<Self, String> {
        let is_multitenant = env::var("OHC_MULTITENANT").unwrap_or_else(|_| "false".to_string()) == "true";
        if is_multitenant {
            let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
            let driver = CloudDriver::new(&redis_url)?;
            Ok(Self {
                driver: Box::new(driver),
            })
        } else {
            Ok(Self {
                driver: Box::new(StandaloneDriver::new()),
            })
        }
    }

    pub async fn get_cache(&self, tenant_id: &str, key: &str) -> Result<Option<Vec<u8>>, String> {
        self.driver.get_cache(tenant_id, key).await
    }

    pub async fn set_cache(
        &self,
        tenant_id: &str,
        key: &str,
        value: Vec<u8>,
        ttl: Duration,
    ) -> Result<(), String> {
        self.driver.set_cache(tenant_id, key, value, ttl).await
    }

    pub async fn delete_cache(&self, tenant_id: &str, key: &str) -> Result<(), String> {
        self.driver.delete_cache(tenant_id, key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_standalone_driver() {
        let driver = StandaloneDriver::new();
        let tenant_id = "test_tenant";
        let key = "test_key";
        let value = b"hello world".to_vec();

        // Test set and get
        driver.set_cache(tenant_id, key, value.clone(), Duration::from_secs(2)).await.unwrap();
        let cached = driver.get_cache(tenant_id, key).await.unwrap();
        assert_eq!(cached, Some(value));

        // Test delete
        driver.delete_cache(tenant_id, key).await.unwrap();
        let deleted = driver.get_cache(tenant_id, key).await.unwrap();
        assert_eq!(deleted, None);

        // Test expiration
        driver.set_cache(tenant_id, "expire_key", b"exp".to_vec(), Duration::from_millis(10)).await.unwrap();
        sleep(Duration::from_millis(20)).await;
        let expired = driver.get_cache(tenant_id, "expire_key").await.unwrap();
        assert_eq!(expired, None);
    }

    #[tokio::test]
    async fn test_cloud_driver() {
        // Just verify connection failure when redis is not available
        let redis_url = "redis://127.0.0.1:9999/";
        let driver = CloudDriver::new(&redis_url).unwrap();
        let res = driver.get_cache("tenant", "key").await;
        assert!(res.is_err());

        let res2 = driver.set_cache("tenant", "key", vec![], Duration::from_secs(1)).await;
        assert!(res2.is_err());

        let res3 = driver.delete_cache("tenant", "key").await;
        assert!(res3.is_err());
    }

    #[tokio::test]
    async fn test_cache_manager_multitenant_false() {
        unsafe { env::set_var("OHC_MULTITENANT", "false"); }
        let manager = CacheManager::new().unwrap();

        let tenant_id = "tenant_a";
        let key = "key_a";
        let value = b"val_a".to_vec();

        manager.set_cache(tenant_id, key, value.clone(), Duration::from_secs(1)).await.unwrap();
        let cached = manager.get_cache(tenant_id, key).await.unwrap();
        assert_eq!(cached, Some(value));
    }
}
