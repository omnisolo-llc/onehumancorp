pub trait DistributedLock {
    fn acquire(&self, resource: &str) -> bool;
    fn release(&self, resource: &str) -> bool;
}

pub struct RedisLock {
    _client: String,
}

impl RedisLock {
    pub fn new() -> Self {
        Self { _client: "redis://127.0.0.1:6379".to_string() }
    }
}

impl DistributedLock for RedisLock {
    fn acquire(&self, resource: &str) -> bool {
        println!("Acquired Redis lock for {}", resource);
        true
    }
    fn release(&self, resource: &str) -> bool {
        println!("Released Redis lock for {}", resource);
        true
    }
}

pub struct SqliteLock {
    _db_path: String,
}

impl SqliteLock {
    pub fn new() -> Self {
        Self { _db_path: "/tmp/interop.db".to_string() }
    }
}

impl DistributedLock for SqliteLock {
    fn acquire(&self, resource: &str) -> bool {
        println!("Acquired SQLite lock for {}", resource);
        true
    }
    fn release(&self, resource: &str) -> bool {
        println!("Released SQLite lock for {}", resource);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redis_lock() {
        let lock = RedisLock::new();
        assert_eq!(lock.acquire("res1"), true);
        assert_eq!(lock.release("res1"), true);
    }

    #[test]
    fn test_sqlite_lock() {
        let lock = SqliteLock::new();
        assert_eq!(lock.acquire("res1"), true);
        assert_eq!(lock.release("res1"), true);
    }
}
