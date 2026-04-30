use async_trait::async_trait;
use redis::AsyncCommands;
use std::time::Duration;
use uuid::Uuid;
use std::sync::Arc;
use sqlx::{sqlite::{SqlitePoolOptions, SqliteConnectOptions}, Row, ConnectOptions};
use tokio::sync::OnceCell;

#[async_trait]
pub trait MeshLock: Send + Sync {
    async fn acquire(&self, timeout: Duration, expiration: Duration) -> Result<(), String>;
    async fn release(&self) -> Result<(), String>;
}

pub struct RedisLock {
    client: redis::Client,
    key: String,
    value: String,
}

impl RedisLock {
    pub fn new(client: redis::Client, key: &str) -> Self {
        RedisLock {
            client,
            key: format!("lock:{}", key),
            value: Uuid::new_v4().to_string(),
        }
    }
}

#[async_trait]
impl MeshLock for RedisLock {
    async fn acquire(&self, timeout: Duration, expiration: Duration) -> Result<(), String> {
        let mut con = self.client.get_async_connection().await.map_err(|e| e.to_string())?;
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err("timeout acquiring lock".to_string());
            }

            let res: Option<String> = redis::cmd("SET")
                .arg(&self.key)
                .arg(&self.value)
                .arg("NX")
                .arg("PX")
                .arg(expiration.as_millis() as u64)
                .query_async(&mut con)
                .await
                .map_err(|e| e.to_string())?;

            if res.is_some() {
                return Ok(());
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    async fn release(&self) -> Result<(), String> {
        let mut con = self.client.get_async_connection().await.map_err(|e| e.to_string())?;
        let script = redis::Script::new(r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#);

        let res: i32 = script.key(&self.key).arg(&self.value).invoke_async(&mut con).await.map_err(|e| e.to_string())?;

        if res == 1 {
            Ok(())
        } else {
            Err("failed to release lock: not owner or lock expired".to_string())
        }
    }
}

pub struct LocalLock {
    key: String,
    value: String,
    pool: sqlx::SqlitePool,
}

static LOCAL_POOL: OnceCell<sqlx::SqlitePool> = OnceCell::const_new();

impl LocalLock {
    pub async fn new(key: &str) -> Result<Self, String> {
        let pool = LOCAL_POOL.get_or_try_init(|| async {
            let db_path = std::env::temp_dir().join("ohc_locks.sqlite");

            let mut options = SqliteConnectOptions::new()
                .filename(&db_path)
                .create_if_missing(true)
                // Use WAL mode for better concurrency performance
                .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

            options = options.disable_statement_logging();

            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect_with(options)
                .await
                .map_err(|e| format!("Failed to connect to LocalLock SQLite database: {}", e))?;

            // Create locks table if it doesn't exist
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS locks (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL,
                    expires_at INTEGER NOT NULL
                )"
            )
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to create LocalLock locks table: {}", e))?;

            Ok::<_, String>(pool)
        }).await?;

        Ok(LocalLock {
            key: key.to_string(),
            value: Uuid::new_v4().to_string(),
            pool: pool.clone(),
        })
    }
}

#[async_trait]
impl MeshLock for LocalLock {
    async fn acquire(&self, timeout: Duration, expiration: Duration) -> Result<(), String> {
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err("timeout acquiring lock".to_string());
            }

            let now = chrono::Utc::now().timestamp_millis();
            let expires_at = now + expiration.as_millis() as i64;

            // Attempt to insert a new lock or overwrite an expired one
            let res = sqlx::query(
                "INSERT INTO locks (key, value, expires_at)
                 VALUES (?, ?, ?)
                 ON CONFLICT(key) DO UPDATE SET
                    value = excluded.value,
                    expires_at = excluded.expires_at
                 WHERE locks.expires_at < ?"
            )
            .bind(&self.key)
            .bind(&self.value)
            .bind(expires_at)
            .bind(now)
            .execute(&self.pool)
            .await;

            match res {
                Ok(result) => {
                    if result.rows_affected() > 0 {
                        // Successfully acquired the lock (either inserted or updated an expired one)
                        return Ok(());
                    } else {
                        // Conflict occurred, but no rows were updated because the existing lock hasn't expired
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                }
                Err(e) => {
                    return Err(format!("Database error acquiring LocalLock: {}", e));
                }
            }
        }
    }

    async fn release(&self) -> Result<(), String> {
        let res = sqlx::query("DELETE FROM locks WHERE key = ? AND value = ?")
            .bind(&self.key)
            .bind(&self.value)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Database error releasing LocalLock: {}", e))?;

        if res.rows_affected() > 0 {
            Ok(())
        } else {
            Err("failed to release lock: not owner or lock expired".to_string())
        }
    }
}

pub async fn create_lock(redis_url: Option<&str>, standalone: bool, key: &str) -> Arc<dyn MeshLock> {
    if standalone {
        match LocalLock::new(key).await {
            Ok(lock) => return Arc::new(lock),
            Err(e) => eprintln!("Failed to initialize LocalLock: {}", e),
        }
    } else if let Some(url) = redis_url {
        match redis::Client::open(url) {
            Ok(client) => return Arc::new(RedisLock::new(client, key)),
            Err(e) => eprintln!("Failed to connect to Redis for MeshLock: {}", e),
        }
    }

    // Fallback if cloud redis fails or local fails
    // Ideally we shouldn't fail if we just fall back, but we log the error.
    match LocalLock::new(key).await {
        Ok(lock) => Arc::new(lock),
        Err(e) => {
            eprintln!("Critical fallback failure for LocalLock: {}", e);
            // Returning an invalid lock in an absolute crisis (or panic)
            panic!("Cannot provide any lock mechanisms.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_local_lock_acquire_and_release() {
        let key = "test_acquire_release";
        let lock = LocalLock::new(key).await.unwrap();

        // Ensure clean state
        let _ = lock.release().await;

        let res = lock.acquire(Duration::from_secs(2), Duration::from_secs(10)).await;
        assert!(res.is_ok());

        let release_res = lock.release().await;
        assert!(release_res.is_ok());
    }

    #[tokio::test]
    async fn test_local_lock_mutual_exclusion() {
        let key = "test_mutual_exclusion";
        let lock1 = LocalLock::new(key).await.unwrap();
        let lock2 = LocalLock::new(key).await.unwrap();

        // Ensure clean state
        let _ = lock1.release().await;
        let _ = lock2.release().await;
        // manually delete from db to be totally clean just in case
        let _ = sqlx::query("DELETE FROM locks WHERE key = ?").bind(key).execute(&lock1.pool).await;

        // lock1 acquires the lock
        let res1 = lock1.acquire(Duration::from_secs(2), Duration::from_secs(10)).await;
        assert!(res1.is_ok());

        // lock2 attempts to acquire the lock but should fail since it's already held
        let res2 = lock2.acquire(Duration::from_millis(500), Duration::from_secs(10)).await;
        assert!(res2.is_err());

        // lock1 releases the lock
        let release_res = lock1.release().await;
        assert!(release_res.is_ok());

        // lock2 should now be able to acquire the lock
        let res3 = lock2.acquire(Duration::from_secs(2), Duration::from_secs(10)).await;
        assert!(res3.is_ok());

        let _ = lock2.release().await;
    }

    #[tokio::test]
    async fn test_local_lock_expiration() {
        let key = "test_expiration";
        let lock1 = LocalLock::new(key).await.unwrap();
        let lock2 = LocalLock::new(key).await.unwrap();

        // Ensure clean state
        let _ = lock1.release().await;
        let _ = sqlx::query("DELETE FROM locks WHERE key = ?").bind(key).execute(&lock1.pool).await;

        // lock1 acquires the lock with a 0-second expiration effectively (since it takes some ms to run)
        let res1 = lock1.acquire(Duration::from_secs(2), Duration::from_millis(10)).await;
        assert!(res1.is_ok());

        tokio::time::sleep(Duration::from_millis(100)).await;

        // lock2 attempts to acquire the lock and should succeed because lock1 expired
        let res2 = lock2.acquire(Duration::from_secs(2), Duration::from_secs(10)).await;
        assert!(res2.is_ok());

        let _ = lock2.release().await;
    }

    #[tokio::test]
    async fn test_create_lock_factory() {
        let lock = create_lock(None, true, "test_factory").await;

        let res = lock.acquire(Duration::from_secs(1), Duration::from_secs(10)).await;
        assert!(res.is_ok());

        let release_res = lock.release().await;
        assert!(release_res.is_ok());
    }
}
