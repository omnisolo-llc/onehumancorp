use super::*;
use redis::AsyncCommands;
use std::sync::Arc;

// CLIENT KILL affects only the explicitly owned service. Serialize these tests
// against one another; UUID keys keep data isolated without flushing the DB.
static SERVICE: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

async fn owned_service() -> (
    tokio::sync::MutexGuard<'static, ()>,
    redis::Client,
    redis::aio::MultiplexedConnection,
) {
    let guard = SERVICE.lock().await;
    assert_eq!(
        std::env::var("OHC_REDIS_SERVICE_ISOLATION").as_deref(),
        Ok("1")
    );
    let url = std::env::var("OHC_TEST_REDIS_URL").expect("owned disposable Redis URL required");
    assert!(
        url.starts_with("redis://127.0.0.1:"),
        "only an owned loopback Redis fixture is allowed"
    );
    let client = redis::Client::open(url).unwrap();
    let observer = client.get_multiplexed_tokio_connection().await.unwrap();
    (guard, client, observer)
}

fn key() -> String {
    format!("ohc-reconnect:{}", uuid::Uuid::new_v4())
}
fn job() -> Job {
    Job {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: "tenant-fixture".into(),
        parent_task_id: "parent".into(),
        job_type: "fixture-role".into(),
        payload: "{}".into(),
        status: "QUEUED".into(),
        retry_count: 0,
        max_retries: 0,
        next_retry_at: Utc::now(),
        locked_until: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}
async fn disconnect(observer: &mut redis::aio::MultiplexedConnection) {
    let killed: i64 = redis::cmd("CLIENT")
        .arg("KILL")
        .arg("TYPE")
        .arg("NORMAL")
        .arg("SKIPME")
        .arg("YES")
        .query_async(observer)
        .await
        .unwrap();
    assert!(
        killed > 0,
        "test must actually disconnect an initialized adapter"
    );
}

#[tokio::test]
async fn cache_recovers_redis_reads_after_an_established_socket_dies() {
    let (_guard, client, mut observer) = owned_service().await;
    let cache = cache::HybridCache::<String>::new(Some(client));
    assert_eq!(cache.get(&key()).await, None); // Establish the actual L2 socket.
    disconnect(&mut observer).await;
    let sentinel = key();
    let _: () = observer
        .set_ex(&sentinel, serde_json::to_string("recovered").unwrap(), 60)
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            if cache.get(&sentinel).await == Some("recovered".to_string()) {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("initialized cache must reconnect instead of retaining a dead socket forever");
    cache.invalidate(&sentinel).await;
    assert!(!observer.exists::<_, bool>(&sentinel).await.unwrap());
}

async fn queue_ready(queue: &RedisTaskQueue, blocking: bool) {
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let result = if blocking {
                queue.get_blocking_connection().await
            } else {
                queue.get_connection().await
            };
            if let Ok(mut connection) = result
                && matches!(
                    redis::cmd("PING")
                        .query_async::<String>(&mut connection)
                        .await
                        .as_deref(),
                    Ok("PONG")
                )
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("queue must reconnect its established socket; PING is safe to repeat");
}

#[tokio::test]
async fn queue_recovers_both_sockets_without_replaying_enqueue() {
    let (_guard, _, mut observer) = owned_service().await;
    let url = std::env::var("OHC_TEST_REDIS_URL").unwrap();
    let queue_name = key();
    let queue = RedisTaskQueue::connect_for_startup(&url, &queue_name)
        .await
        .unwrap();
    let first = job();
    queue.enqueue(first.clone()).await.unwrap();
    assert_eq!(
        queue
            .dequeue(vec!["fixture-role".into()])
            .await
            .unwrap()
            .unwrap()
            .id,
        first.id
    );
    queue.complete(&first.id, &first.tenant_id).await.unwrap();
    disconnect(&mut observer).await;
    queue_ready(&queue, false).await;
    queue_ready(&queue, true).await;
    let second = job();
    queue.enqueue(second.clone()).await.unwrap();
    assert_eq!(observer.llen::<_, i64>(&queue_name).await.unwrap(), 1);
    assert_eq!(
        queue
            .dequeue(vec!["fixture-role".into()])
            .await
            .unwrap()
            .unwrap()
            .id,
        second.id
    );
    queue.complete(&second.id, &second.tenant_id).await.unwrap();
    assert_eq!(observer.llen::<_, i64>(&queue_name).await.unwrap(), 0);
}

#[tokio::test]
async fn blocking_queue_read_does_not_delay_enqueue_acknowledgement_or_cache_delete() {
    let (_guard, client, mut observer) = owned_service().await;
    let queue = Arc::new(
        RedisTaskQueue::connect_for_startup(&std::env::var("OHC_TEST_REDIS_URL").unwrap(), &key())
            .await
            .unwrap(),
    );
    let first = job();
    queue.enqueue(first.clone()).await.unwrap();
    queue
        .dequeue(vec!["fixture-role".into()])
        .await
        .unwrap()
        .unwrap();
    let cache = cache::HybridCache::<String>::new(Some(client));
    let cached = key();
    cache
        .set(&cached, "stale".into(), Duration::from_secs(60))
        .await;
    let consumer = queue.clone();
    let blocked = tokio::spawn(async move { consumer.dequeue(vec!["fixture-role".into()]).await });
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            let clients: String = redis::cmd("CLIENT")
                .arg("LIST")
                .query_async(&mut observer)
                .await
                .unwrap();
            if clients.lines().any(|line| {
                line.contains("cmd=blpop")
                    && line.split_whitespace().any(|part| {
                        part.strip_prefix("flags=")
                            .is_some_and(|flags| flags.contains('b'))
                    })
            }) {
                break;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("observe actual blocked BLPOP");
    tokio::time::timeout(
        Duration::from_millis(400),
        queue.complete(&first.id, &first.tenant_id),
    )
    .await
    .expect("ack cannot wait for BLPOP")
    .unwrap();
    tokio::time::timeout(Duration::from_millis(400), cache.invalidate(&cached))
        .await
        .expect("cache invalidation cannot wait for BLPOP");
    assert!(!observer.exists::<_, bool>(&cached).await.unwrap());
    let second = job();
    tokio::time::timeout(Duration::from_millis(400), queue.enqueue(second.clone()))
        .await
        .expect("enqueue cannot wait for BLPOP")
        .unwrap();
    assert_eq!(blocked.await.unwrap().unwrap().unwrap().id, second.id);
    queue.complete(&second.id, &second.tenant_id).await.unwrap();
}

#[tokio::test]
async fn concurrent_blocking_reads_do_not_expire_while_waiting_for_wire_admission() {
    let (_guard, _, _) = owned_service().await;
    let queue = Arc::new(
        RedisTaskQueue::connect_for_startup(&std::env::var("OHC_TEST_REDIS_URL").unwrap(), &key())
            .await
            .unwrap(),
    );
    let mut consumers = tokio::task::JoinSet::new();
    for _ in 0..3 {
        let queue = queue.clone();
        consumers.spawn(async move { queue.dequeue(vec!["fixture-role".into()]).await });
    }
    tokio::time::timeout(Duration::from_secs(5), async {
        while let Some(result) = consumers.join_next().await {
            assert!(
                result
                    .unwrap()
                    .expect("queued BLPOP must receive its own one-second response window")
                    .is_none()
            );
        }
    })
    .await
    .expect("three empty blocking reads finish with bounded wire admission");
}

#[tokio::test]
async fn post_pop_processing_write_uses_the_command_connection() {
    let (_guard, _, mut observer) = owned_service().await;
    let queue =
        RedisTaskQueue::connect_for_startup(&std::env::var("OHC_TEST_REDIS_URL").unwrap(), &key())
            .await
            .unwrap();
    let mut command = queue.get_connection().await.unwrap();
    let command_id: i64 = redis::cmd("CLIENT")
        .arg("ID")
        .query_async(&mut command)
        .await
        .unwrap();
    let queued = job();
    queue.enqueue(queued.clone()).await.unwrap();
    assert_eq!(
        queue
            .dequeue(vec!["fixture-role".into()])
            .await
            .unwrap()
            .unwrap()
            .id,
        queued.id
    );
    let clients: String = redis::cmd("CLIENT")
        .arg("LIST")
        .query_async(&mut observer)
        .await
        .unwrap();
    let command_line = clients
        .lines()
        .find(|line| {
            line.split_whitespace()
                .any(|part| part == format!("id={command_id}"))
        })
        .unwrap();
    assert!(
        command_line
            .split_whitespace()
            .any(|part| part == "cmd=hset"),
        "post-pop bookkeeping must use the command socket: {command_line}"
    );
    queue.complete(&queued.id, &queued.tenant_id).await.unwrap();
}

#[path = "fault.rs"]
mod fault;

#[tokio::test]
async fn lost_enqueue_reply_is_reported_and_never_replayed() {
    let server = fault::FaultServer::start("RPUSH", false).await;
    let queue = RedisTaskQueue::connect_for_startup(&server.url, "fault-queue")
        .await
        .unwrap();
    assert!(
        queue.enqueue(job()).await.is_err(),
        "a lost receipt cannot become success"
    );
    queue_ready(&queue, false).await;
    assert_eq!(
        server.count("RPUSH"),
        1,
        "reconnection must not resend the uncertain enqueue"
    );
    queue.enqueue(job()).await.unwrap();
    assert_eq!(
        server.count("RPUSH"),
        2,
        "only the separately requested enqueue is sent"
    );
}

#[tokio::test]
async fn lost_pop_reply_is_reported_and_never_replayed() {
    let server = fault::FaultServer::start("BLPOP", false).await;
    let queue = RedisTaskQueue::connect_for_startup(&server.url, "fault-queue")
        .await
        .unwrap();
    assert!(queue.dequeue(vec!["fixture-role".into()]).await.is_err());
    queue_ready(&queue, true).await;
    assert_eq!(
        server.count("BLPOP"),
        1,
        "reconnection cannot consume another job for the failed pop"
    );
}

#[tokio::test]
async fn lost_acknowledgement_reply_is_reported_and_never_replayed() {
    let server = fault::FaultServer::start("HDEL", false).await;
    let queue = RedisTaskQueue::connect_for_startup(&server.url, "fault-queue")
        .await
        .unwrap();
    assert!(
        queue
            .complete("seeded-job", "tenant-fixture")
            .await
            .is_err()
    );
    queue_ready(&queue, false).await;
    assert_eq!(
        server.count("HDEL"),
        1,
        "lost acknowledgement must not be replayed"
    );
}

#[tokio::test]
async fn cache_query_deadline_includes_an_unresponsive_server() {
    let server = fault::FaultServer::start("GET", true).await;
    let cache =
        cache::HybridCache::<String>::new(Some(redis::Client::open(server.url.as_str()).unwrap()));
    assert_eq!(
        tokio::time::timeout(Duration::from_secs(1), cache.get("uncached"))
            .await
            .expect("optional Redis GET must not hang the request"),
        None
    );
    assert_eq!(server.count("GET"), 1);
}

#[tokio::test]
async fn queue_mutation_deadline_does_not_retry_an_unresponsive_server() {
    let server = fault::FaultServer::start("RPUSH", true).await;
    let queue = RedisTaskQueue::connect_for_startup(&server.url, "fault-queue")
        .await
        .unwrap();
    assert!(
        tokio::time::timeout(Duration::from_secs(1), queue.enqueue(job()))
            .await
            .expect("queue command deadline must be bounded")
            .is_err()
    );
    assert_eq!(server.count("RPUSH"), 1);
}

#[tokio::test]
async fn concurrent_callers_share_one_replacement_queue_connection() {
    let (_guard, _, mut observer) = owned_service().await;
    let queue = Arc::new(
        RedisTaskQueue::connect_for_startup(&std::env::var("OHC_TEST_REDIS_URL").unwrap(), &key())
            .await
            .unwrap(),
    );
    let mut original = queue.get_connection().await.unwrap();
    let old_id: i64 = redis::cmd("CLIENT")
        .arg("ID")
        .query_async(&mut original)
        .await
        .unwrap();
    let _: i64 = redis::cmd("CLIENT")
        .arg("KILL")
        .arg("ID")
        .arg(old_id)
        .query_async(&mut observer)
        .await
        .unwrap();
    let mut tasks = tokio::task::JoinSet::new();
    for _ in 0..8 {
        let queue = queue.clone();
        tasks.spawn(async move {
            queue_ready(&queue, false).await;
            let mut connection = queue.get_connection().await.unwrap();
            redis::cmd("CLIENT")
                .arg("ID")
                .query_async::<i64>(&mut connection)
                .await
                .unwrap()
        });
    }
    let mut replacements = std::collections::HashSet::new();
    while let Some(result) = tasks.join_next().await {
        replacements.insert(result.unwrap());
    }
    assert_eq!(replacements.len(), 1);
    assert!(!replacements.contains(&old_id));
}

#[tokio::test]
async fn failed_reconnection_during_outage_can_recover_when_service_returns() {
    let server = fault::FaultServer::start("RPUSH", false).await;
    let queue = RedisTaskQueue::connect_for_startup(&server.url, "fault-queue")
        .await
        .unwrap();
    server.set_available(false);
    assert!(queue.enqueue(job()).await.is_err()); // Original socket loses its reply.
    let during_outage = tokio::time::timeout(Duration::from_secs(1), queue.enqueue(job()))
        .await
        .expect("failed reconnect must remain bounded");
    assert!(during_outage.is_err());
    assert_eq!(
        server.count("RPUSH"),
        1,
        "new connections are unavailable and the first command cannot be replayed"
    );
    server.set_available(true);
    queue_ready(&queue, false).await;
    queue.enqueue(job()).await.unwrap();
    assert_eq!(server.count("RPUSH"), 2);
}

#[tokio::test]
async fn invalidation_subscription_returns_after_its_socket_is_killed() {
    let (_guard, _, mut observer) = owned_service().await;
    assert_eq!(
        std::env::var("REDIS_URL").unwrap(),
        std::env::var("OHC_TEST_REDIS_URL").unwrap(),
        "set REDIS_URL to the same owned fixture before launching the process"
    );
    struct AbortOnDrop(tokio::task::JoinHandle<()>);
    impl Drop for AbortOnDrop {
        fn drop(&mut self) {
            self.0.abort();
        }
    }
    let service = AbortOnDrop(tokio::spawn(invalidator::start_cache_invalidator(
        sqlx::PgPool,
    )));
    async fn wait_for_subscription(observer: &mut redis::aio::MultiplexedConnection) {
        tokio::time::timeout(Duration::from_secs(3), async {
            loop {
                let counts: Vec<(String, usize)> = redis::cmd("PUBSUB")
                    .arg("NUMSUB")
                    .arg("cache_invalidation_events")
                    .query_async(observer)
                    .await
                    .unwrap();
                if counts[0].1 == 1 {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("invalidation service must establish or restore its subscription");
    }
    wait_for_subscription(&mut observer).await;
    let killed: usize = redis::cmd("CLIENT")
        .arg("KILL")
        .arg("TYPE")
        .arg("PUBSUB")
        .query_async(&mut observer)
        .await
        .unwrap();
    assert_eq!(killed, 1, "actually close the service subscription socket");
    wait_for_subscription(&mut observer).await;
    assert!(!service.0.is_finished());
}
