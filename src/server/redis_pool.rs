use std::sync::{Arc, OnceLock};

pub struct RedisPool {
    client: redis::Client,
    command_connection: tokio::sync::OnceCell<redis::aio::ConnectionManager>,
}

impl RedisPool {
    pub fn new(url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(url)?;
        Ok(Self {
            client,
            command_connection: tokio::sync::OnceCell::new(),
        })
    }

    pub fn client(&self) -> &redis::Client {
        &self.client
    }

    pub async fn get_async_connection(
        &self,
    ) -> Result<redis::aio::ConnectionManager, redis::RedisError> {
        self.command_connection
            .get_or_try_init(|| self.client.get_connection_manager())
            .await
            .cloned()
    }

    pub async fn get_pubsub(&self) -> Result<redis::aio::PubSub, redis::RedisError> {
        self.client.get_async_pubsub().await
    }
}

static REDIS_POOL: OnceLock<Arc<RedisPool>> = OnceLock::new();

pub fn get_redis_pool() -> Option<&'static Arc<RedisPool>> {
    if crate::is_standalone_runtime() {
        return None;
    }
    Some(REDIS_POOL.get_or_init(|| {
        let url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        Arc::new(RedisPool::new(&url).expect("Failed to create Redis pool"))
    }))
}

pub fn get_redis_client() -> Option<redis::Client> {
    get_redis_pool().map(|pool| pool.client().clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn redis_fixture() -> (RedisPool, tokio::task::JoinHandle<()>) {
        use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let pool = RedisPool::new(&format!("redis://{}", listener.local_addr().unwrap())).unwrap();
        let task = tokio::spawn(async move {
            let mut connection_id = 0;
            while let Ok((stream, _)) = listener.accept().await {
                connection_id += 1;
                let id = connection_id;
                tokio::spawn(async move {
                    let mut stream = BufReader::new(stream);
                    loop {
                        let mut line = String::new();
                        if stream.read_line(&mut line).await.unwrap_or(0) == 0 {
                            break;
                        }
                        let count: usize = line.trim().strip_prefix('*').unwrap().parse().unwrap();
                        let mut command = Vec::new();
                        for _ in 0..count {
                            line.clear();
                            stream.read_line(&mut line).await.unwrap();
                            let length: usize =
                                line.trim().strip_prefix('$').unwrap().parse().unwrap();
                            let mut argument = vec![0; length + 2];
                            stream.read_exact(&mut argument).await.unwrap();
                            command.push(String::from_utf8(argument[..length].to_vec()).unwrap());
                        }
                        let response = if command == ["CLIENT", "ID"] {
                            format!(":{id}\r\n")
                        } else if command == ["SHUTDOWN"] {
                            break;
                        } else {
                            "+OK\r\n".to_string()
                        };
                        if stream.write_all(response.as_bytes()).await.is_err() {
                            break;
                        }
                    }
                });
            }
        });
        (pool, task)
    }

    #[tokio::test]
    async fn command_connections_are_reused() {
        let (pool, server) = redis_fixture().await;
        let mut first = pool.get_async_connection().await.unwrap();
        let first_id: i64 = redis::cmd("CLIENT")
            .arg("ID")
            .query_async(&mut first)
            .await
            .unwrap();
        let mut second = pool.get_async_connection().await.unwrap();
        let second_id: i64 = redis::cmd("CLIENT")
            .arg("ID")
            .query_async(&mut second)
            .await
            .unwrap();
        server.abort();
        assert_eq!(
            first_id, second_id,
            "pool must reuse the established command connection"
        );
    }

    #[tokio::test]
    async fn concurrent_callers_share_connection_initialization() {
        let (pool, server) = redis_fixture().await;
        let pool = Arc::new(pool);
        let mut tasks = tokio::task::JoinSet::new();
        for _ in 0..8 {
            let pool = pool.clone();
            tasks.spawn(async move {
                let mut connection = pool.get_async_connection().await.unwrap();
                redis::cmd("CLIENT")
                    .arg("ID")
                    .query_async::<i64>(&mut connection)
                    .await
                    .unwrap()
            });
        }
        let mut ids = std::collections::HashSet::new();
        while let Some(result) = tasks.join_next().await {
            ids.insert(result.unwrap());
        }
        server.abort();
        assert_eq!(ids.len(), 1);
    }

    #[tokio::test]
    async fn command_connection_recovers_after_disconnect() {
        let (pool, server) = redis_fixture().await;
        let mut connection = pool.get_async_connection().await.unwrap();
        let first_id: i64 = redis::cmd("CLIENT")
            .arg("ID")
            .query_async(&mut connection)
            .await
            .unwrap();
        assert!(
            redis::cmd("SHUTDOWN")
                .query_async::<()>(&mut connection)
                .await
                .is_err()
        );
        let next_id = tokio::time::timeout(std::time::Duration::from_secs(5), async {
            loop {
                let mut connection = pool.get_async_connection().await.unwrap();
                if let Ok(id) = redis::cmd("CLIENT")
                    .arg("ID")
                    .query_async::<i64>(&mut connection)
                    .await
                {
                    break id;
                }
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("connection manager reconnects");
        server.abort();
        assert!(next_id > first_id);
    }

    #[test]
    fn test_get_redis_pool_returns_same_instance() {
        let pool1 = get_redis_pool();
        let pool2 = get_redis_pool();
        if let (Some(p1), Some(p2)) = (pool1, pool2) {
            assert!(Arc::ptr_eq(p1, p2));
        }
    }

    #[test]
    fn test_returns_none_in_standalone_mode() {
        const CHILD: &str = "OHC_REDIS_STANDALONE_TEST_CHILD";
        if std::env::var_os(CHILD).is_some() {
            assert!(get_redis_pool().is_none());
            return;
        }
        // Configuration is cached globally; verify this mode in a fresh process.
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "redis_pool::tests::test_returns_none_in_standalone_mode",
            ])
            .env(CHILD, "1")
            .env("OHC_STANDALONE_MODE", "true")
            .status()
            .unwrap();
        assert!(status.success());
    }
}
