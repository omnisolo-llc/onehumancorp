use crate::queue::{Job, SqliteTaskQueue};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

const SECRET: &str = "startup-canary-password-never-log";

fn child(name: &str, url: Option<&str>, standalone: bool) -> bool {
    if std::env::var("OHC_STARTUP_TEST_CHILD").as_deref() == Ok(name) {
        let _ = tracing_subscriber::fmt().with_ansi(false).try_init();
        return false;
    }
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args(["--exact", &format!("tests::{name}"), "--nocapture"])
        .env("OHC_STARTUP_TEST_CHILD", name)
        .env("OMNISOLO_STANDALONE_MODE", standalone.to_string())
        .env("OMNISOLO_DATABASE_URL", "sqlite::memory:");
    if let Some(url) = url {
        command.env("REDIS_URL", url);
    } else {
        command.env_remove("REDIS_URL");
    }
    let output = command.output().unwrap();
    let log = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !log.contains(SECRET),
        "Startup exposed the credential canary"
    );
    assert!(output.status.success(), "{log}");
    assert!(
        log.contains("test result: ok. 1 passed; 0 failed;"),
        "exact test did not execute: {log}"
    );
    true
}

async fn database() -> crate::db::DB {
    let sqlite = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    SqliteTaskQueue::new(sqlite.clone()).init().await.unwrap();
    crate::db::DB {
        pool: sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://fixture@127.0.0.1:1/unused")
            .unwrap(),
        store: crate::db::DbStore::Sqlite(sqlite),
    }
}

async fn failure(standalone: bool, kind: std::io::ErrorKind) {
    let _ = tracing_subscriber::fmt().with_ansi(false).try_init();
    let db = database().await;
    let started = Instant::now();
    let result = tokio::time::timeout(
        Duration::from_secs(8),
        tokio::spawn(async move { crate::initialize(&db, standalone).await }),
    )
    .await
    .expect("startup exceeded its hard test deadline");
    assert!(result.is_ok(), "Startup must return an error, not panic");
    let Err(error) = result.unwrap() else {
        panic!("Required Redis startup incorrectly succeeded")
    };
    assert!(!format!("{error:?} {error}").contains(SECRET));
    let io = error
        .downcast_ref::<std::io::Error>()
        .expect("typed startup failure");
    assert_eq!(io.kind(), kind);
    assert!(
        started.elapsed() < Duration::from_secs(8),
        "startup did not meet its bound"
    );
}

fn job() -> Job {
    let now = chrono::Utc::now();
    Job {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: "startup-test-tenant".into(),
        parent_task_id: "startup-test-parent".into(),
        job_type: "sub_agent".into(),
        payload: "{}".into(),
        status: "QUEUED".into(),
        retry_count: 0,
        max_retries: 3,
        next_retry_at: now,
        locked_until: None,
        created_at: now,
        updated_at: now,
    }
}

async fn sqlite_roundtrip(standalone: bool) {
    let db = database().await;
    let queue = crate::initialize(&db, standalone).await.unwrap();
    let job = job();
    queue.enqueue(job.clone()).await.unwrap();
    let crate::db::DbStore::Sqlite(pool) = &db.store else {
        unreachable!()
    };
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM local_queue_jobs WHERE id = ?")
        .bind(&job.id)
        .fetch_one(pool)
        .await
        .unwrap();
    assert_eq!(count, 1, "fallback must remain the actual database queue");
}

#[tokio::test]
async fn malformed_cloud_url_returns_redacted_error() {
    if child(
        "malformed_cloud_url_returns_redacted_error",
        Some(&format!("https://user:{SECRET}@127.0.0.1/")),
        false,
    ) {
        return;
    }
    failure(false, std::io::ErrorKind::InvalidInput).await;
}

#[tokio::test]
async fn malformed_standalone_rate_limit_url_returns_redacted_error() {
    if child(
        "malformed_standalone_rate_limit_url_returns_redacted_error",
        Some(&format!("https://user:{SECRET}@127.0.0.1/")),
        true,
    ) {
        return;
    }
    failure(true, std::io::ErrorKind::InvalidInput).await;
}

#[tokio::test]
async fn empty_configured_cloud_url_is_invalid() {
    if child("empty_configured_cloud_url_is_invalid", Some(""), false) {
        return;
    }
    failure(false, std::io::ErrorKind::InvalidInput).await;
}

#[tokio::test]
async fn absent_cloud_redis_preserves_database_queue() {
    if child("absent_cloud_redis_preserves_database_queue", None, false) {
        return;
    }
    sqlite_roundtrip(false).await;
}

#[tokio::test]
async fn absent_standalone_redis_preserves_database_queue() {
    if child(
        "absent_standalone_redis_preserves_database_queue",
        None,
        true,
    ) {
        return;
    }
    sqlite_roundtrip(true).await;
}

#[tokio::test]
async fn standalone_does_not_require_optional_redis_service() {
    if child(
        "standalone_does_not_require_optional_redis_service",
        Some("redis://127.0.0.1:1/"),
        true,
    ) {
        return;
    }
    sqlite_roundtrip(true).await;
}

#[tokio::test]
async fn unreachable_cloud_redis_fails_without_fallback() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("redis://{}", listener.local_addr().unwrap());
    drop(listener);
    if child(
        "unreachable_cloud_redis_fails_without_fallback",
        Some(&url),
        false,
    ) {
        return;
    }
    failure(false, std::io::ErrorKind::Other).await;
}

#[tokio::test]
async fn unresponsive_cloud_redis_has_bounded_startup() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("redis://{}", listener.local_addr().unwrap());
    // The parent retains an owned local listener that never speaks Redis.
    if child(
        "unresponsive_cloud_redis_has_bounded_startup",
        Some(&url),
        false,
    ) {
        return;
    }
    failure(false, std::io::ErrorKind::TimedOut).await;
}

struct RedisFixture {
    process: Child,
    directory: std::path::PathBuf,
    url: String,
}
impl Drop for RedisFixture {
    fn drop(&mut self) {
        let _ = self.process.kill();
        let _ = self.process.wait();
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}
async fn redis_fixture() -> RedisFixture {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let directory =
        std::env::temp_dir().join(format!("ohc-redis-startup-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&directory).unwrap();
    let process = Command::new("redis-server")
        .args([
            "--bind",
            "127.0.0.1",
            "--port",
            &port.to_string(),
            "--save",
            "",
            "--appendonly",
            "no",
            "--requirepass",
            "public-local-test-password",
            "--daemonize",
            "no",
            "--dir",
        ])
        .arg(&directory)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("official redis-server prerequisite required");
    let fixture = RedisFixture {
        process,
        directory,
        url: format!("redis://:public-local-test-password@127.0.0.1:{port}/"),
    };
    let client = redis::Client::open(fixture.url.as_str()).unwrap();
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            if client.get_multiplexed_tokio_connection().await.is_ok() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("owned Redis fixture did not start");
    fixture
}

#[tokio::test]
async fn valid_authenticated_redis_retains_queue_behavior() {
    if std::env::var("OHC_STARTUP_TEST_CHILD").is_err() {
        let fixture = redis_fixture().await;
        assert!(child(
            "valid_authenticated_redis_retains_queue_behavior",
            Some(&fixture.url),
            false
        ));
        return;
    }
    let db = database().await;
    let queue = crate::initialize(&db, false).await.unwrap();
    let job = job();
    queue.enqueue(job.clone()).await.unwrap();
    drop(queue);
    let queue = crate::initialize(&db, false).await.unwrap();
    let received = queue
        .dequeue(vec!["sub_agent".into()])
        .await
        .unwrap()
        .unwrap();
    assert_eq!(received.id, job.id);
    assert_eq!(received.tenant_id, job.tenant_id);
    queue.complete(&job.id, &job.tenant_id).await.unwrap();
}

#[tokio::test]
async fn wrong_password_fails_with_redacted_error() {
    if std::env::var("OHC_STARTUP_TEST_CHILD").is_err() {
        let fixture = redis_fixture().await;
        let url = fixture.url.replace("public-local-test-password", SECRET);
        assert!(child(
            "wrong_password_fails_with_redacted_error",
            Some(&url),
            false
        ));
        return;
    }
    failure(false, std::io::ErrorKind::Other).await;
}

#[test]
fn startup_is_early_and_pruning_reuses_the_initialized_queue() {
    let source = include_str!("../../src/server/lib.rs");
    let preflight = source
        .find("// Validate Redis startup before spawning workers.")
        .expect("Redis startup must be validated early");
    assert!(preflight < source[preflight..].find("// Initialize database").unwrap() + preflight);
    assert!(source.contains("let omnisolo_job_queue_prune = omnisolo_job_queue.clone();"));
    assert!(
        !source.contains("Failed to initialize Redis client for RateLimiter at"),
        "credential-bearing panic must be removed"
    );
    assert!(
        !source.contains("crate::queue::RedisTaskQueue::new("),
        "bootstrap must use the checked constructor"
    );
}

#[test]
fn absent_optional_pool_is_disabled() {
    if child("absent_optional_pool_is_disabled", None, false) {
        return;
    }
    assert!(crate::redis_pool::get_redis_pool().is_none());
}

#[test]
fn standalone_optional_pool_ignores_malformed_config() {
    if child(
        "standalone_optional_pool_ignores_malformed_config",
        Some(&format!("https://user:{SECRET}@127.0.0.1/")),
        true,
    ) {
        return;
    }
    assert!(crate::redis_pool::get_redis_pool().is_none());
}

#[test]
fn malformed_optional_pool_is_unavailable_without_panic() {
    if child(
        "malformed_optional_pool_is_unavailable_without_panic",
        Some(&format!("https://user:{SECRET}@127.0.0.1/")),
        false,
    ) {
        return;
    }
    assert!(crate::redis_pool::get_redis_pool().is_none());
}

#[tokio::test]
async fn checked_constructor_redacts_malformed_url() {
    let result = crate::queue::RedisTaskQueue::connect_for_startup(
        &format!("https://user:{SECRET}@127.0.0.1/"),
        "startup-test-queue",
    )
    .await;
    let Err(error) = result else {
        panic!("malformed queue URL must fail")
    };
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(!format!("{error:?} {error}").contains(SECRET));
}

#[tokio::test]
async fn blocked_dequeue_does_not_block_enqueue_or_pruning() {
    use crate::queue::TaskQueue;
    let fixture = redis_fixture().await;
    let queue = std::sync::Arc::new(
        crate::queue::RedisTaskQueue::connect_for_startup(&fixture.url, "startup-concurrent-queue")
            .await
            .unwrap(),
    );
    let consumer = queue.clone();
    let received = tokio::spawn(async move { consumer.dequeue(vec!["sub_agent".into()]).await });
    let client = redis::Client::open(fixture.url.as_str()).unwrap();
    let mut observer = client.get_multiplexed_tokio_connection().await.unwrap();
    // Observe the real server's blocked BLPOP, rather than assuming that a task
    // has reached its await point after a fixed sleep.
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            let clients: String = redis::cmd("CLIENT")
                .arg("LIST")
                .query_async(&mut observer)
                .await
                .unwrap();
            if clients.lines().any(|line| {
                line.contains("cmd=blpop")
                    && line.split_whitespace().any(|field| {
                        field
                            .strip_prefix("flags=")
                            .is_some_and(|flags| flags.contains('b'))
                    })
            }) {
                break;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("dequeue did not enter a blocked Redis state");
    let pruned = tokio::time::timeout(Duration::from_millis(400), queue.cleanup_stale_jobs()).await;
    let expected = job();
    let enqueued =
        tokio::time::timeout(Duration::from_millis(400), queue.enqueue(expected.clone())).await;
    let received = tokio::time::timeout(Duration::from_secs(2), received)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert!(pruned.is_ok(), "pruning was blocked behind BLPOP");
    assert_eq!(pruned.unwrap().unwrap(), 0);
    assert!(enqueued.is_ok(), "enqueue was blocked behind BLPOP");
    enqueued.unwrap().unwrap();
    assert_eq!(received.unwrap().id, expected.id);
    queue
        .complete(&expected.id, &expected.tenant_id)
        .await
        .unwrap();
}
