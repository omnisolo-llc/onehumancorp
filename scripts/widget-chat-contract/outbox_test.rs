use super::*;

fn redis_test_url() -> String {
    let raw = std::env::var("OHC_WIDGET_TEST_REDIS_URL")
        .expect("explicit disposable loopback Redis prerequisite required");
    let url = url::Url::parse(&raw).expect("valid Redis test URL");
    let host = url
        .host_str()
        .expect("explicit loopback IP")
        .trim_start_matches('[')
        .trim_end_matches(']');
    let address: std::net::IpAddr = host.parse().expect("explicit loopback IP");
    assert!(
        url.scheme() == "redis"
            && !raw.chars().any(char::is_control)
            && address.is_loopback()
            && url.port().is_some()
            && url.username().is_empty()
            && url.password().is_none()
            && matches!(url.path(), "" | "/" | "/0")
            && url.query().is_none()
            && url.fragment().is_none()
    );
    raw
}

fn redis_case_child(name: &str) -> bool {
    if std::env::var("OHC_WIDGET_REDIS_CHILD").as_deref() == Ok(name) {
        return true;
    }
    let raw = redis_test_url();
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", name, "--nocapture"])
        .env("OHC_WIDGET_REDIS_CHILD", name)
        .env("REDIS_URL", raw)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "Redis child failed: {}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("test result: ok. 1 passed; 0 failed;"),
        "isolated Redis child must execute exactly the requested real test"
    );
    false
}

#[tokio::test]
async fn redis_does_not_receive_an_event_for_a_rejected_commit() {
    const NAME: &str =
        "tests::postgres::outbox::redis_does_not_receive_an_event_for_a_rejected_commit";
    if !redis_case_child(NAME) {
        return;
    }
    use futures_util::StreamExt;
    let f = PgFixture::new().await;
    let conversation = f.seed_conversation(false).await;
    sqlx::raw_sql("CREATE FUNCTION reject_redis_commit() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'synthetic deferred rejection'; END $$;CREATE CONSTRAINT TRIGGER reject_redis_commit AFTER INSERT ON chat_messages DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_redis_commit();")
        .execute(&f.admin).await.unwrap();
    let client = redis::Client::open(std::env::var("REDIS_URL").unwrap()).unwrap();
    let mut subscription = client.get_async_pubsub().await.unwrap();
    subscription
        .subscribe(format!("unified:chat:{}", f.a))
        .await
        .unwrap();
    let response = f
        .request(
            false,
            "POST",
            "/api/widget/messages",
            Some(&f.token),
            json!({"tenant_id":f.a,"conversation_id":conversation,"content":"never committed"}),
        )
        .await;
    let received =
        tokio::time::timeout(Duration::from_millis(250), subscription.on_message().next()).await;
    let counts = f.counts().await;
    f.finish().await;
    assert_eq!(response.0, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(counts, (1, 0));
    assert!(
        received.is_err(),
        "Redis must never publish uncommitted message content"
    );
}

#[tokio::test]
async fn redis_publication_acknowledgement_retains_committed_outbox_evidence() {
    const NAME: &str = "tests::postgres::outbox::redis_publication_acknowledgement_retains_committed_outbox_evidence";
    if !redis_case_child(NAME) {
        return;
    }
    use futures_util::StreamExt;
    let f = PgFixture::new().await;
    let conversation = f.seed_conversation(false).await;
    let client = redis::Client::open(std::env::var("REDIS_URL").unwrap()).unwrap();
    let mut subscription = client.get_async_pubsub().await.unwrap();
    subscription
        .subscribe(format!("unified:chat:{}", f.a))
        .await
        .unwrap();
    let response=f.request(false,"POST","/api/widget/messages",Some(&f.token),json!({"tenant_id":f.a,"conversation_id":conversation,"content":"committed Redis publication 🦀"})).await;
    let message = tokio::time::timeout(Duration::from_secs(2), subscription.on_message().next())
        .await
        .unwrap()
        .unwrap();
    let received: Value = serde_json::from_str(&message.get_payload::<String>().unwrap()).unwrap();
    let event: (String, String, Value) =
        sqlx::query_as("SELECT id,status,payload FROM ohc_job_queue")
            .fetch_one(&f.admin)
            .await
            .unwrap();
    let persisted: Uuid = sqlx::query_scalar("SELECT id FROM chat_messages")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(response.0, StatusCode::OK);
    assert_eq!(received["message"], response.1);
    assert_eq!(received["event_id"], persisted.to_string());
    assert_eq!(event.0, persisted.to_string());
    assert_eq!(event.1, "COMPLETED");
    assert_eq!(event.2["publication"]["transport"], "redis_pubsub");
    assert_eq!(event.2["publication"]["subscriber_count"], 1);
    assert!(
        event.2.get("delivered").is_none(),
        "Redis acknowledgement is not recipient receipt"
    );
}

#[tokio::test]
async fn redis_zero_subscribers_preserves_pending_intent_for_restart_worker() {
    const NAME: &str = "tests::postgres::outbox::redis_zero_subscribers_preserves_pending_intent_for_restart_worker";
    if !redis_case_child(NAME) {
        return;
    }
    use futures_util::StreamExt;
    let f = PgFixture::new().await;
    let conversation = f.seed_conversation(false).await;
    let response=f.request(false,"POST","/api/widget/messages",Some(&f.token),json!({"tenant_id":f.a,"conversation_id":conversation,"content":"recovered after producer exited"})).await;
    assert_eq!(response.0, StatusCode::OK);
    let state: (String, i32, Value, bool) = sqlx::query_as(
        "SELECT status,retry_count,payload,next_retry_at>clock_timestamp() FROM ohc_job_queue",
    )
    .fetch_one(&f.admin)
    .await
    .unwrap();
    assert_eq!(state.0, "PENDING");
    assert_eq!(state.1, 1);
    assert!(state.2.get("publication").is_none());
    assert!(state.3);
    // The same production role convention used by the server, scoped to this
    // disposable schema. The application role remains LOGIN/NOBYPASSRLS.
    sqlx::raw_sql(&format!("DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname='ohc_bypassrls') THEN CREATE ROLE ohc_bypassrls NOLOGIN BYPASSRLS; END IF; END $$;GRANT USAGE ON SCHEMA {} TO ohc_bypassrls;GRANT SELECT,UPDATE ON ALL TABLES IN SCHEMA {} TO ohc_bypassrls;GRANT ohc_bypassrls TO {};",f.schema,f.schema,f.role)).execute(&f.admin).await.unwrap();
    sqlx::query("UPDATE ohc_job_queue SET next_retry_at=clock_timestamp()-interval '1 second'")
        .execute(&f.admin)
        .await
        .unwrap();
    let client = redis::Client::open(std::env::var("REDIS_URL").unwrap()).unwrap();
    let mut subscription = client.get_async_pubsub().await.unwrap();
    subscription
        .subscribe(format!("unified:chat:{}", f.a))
        .await
        .unwrap();
    // Execute the exact extracted startup block; no producer call performs this retry.
    let shutdown = crate::actual_outbox_bootstrap(
        Arc::new(crate::db::DB {
            pool: f.pool.clone(),
            store: crate::db::DbStore::Postgres,
        }),
        true,
    );
    let message = tokio::time::timeout(Duration::from_secs(3), subscription.on_message().next())
        .await
        .unwrap()
        .unwrap();
    let received: Value = serde_json::from_str(&message.get_payload::<String>().unwrap()).unwrap();
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            let status: String = sqlx::query_scalar("SELECT status FROM ohc_job_queue")
                .fetch_one(&f.admin)
                .await
                .unwrap();
            if status == "COMPLETED" {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    shutdown.send(true).unwrap();
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM ohc_job_queue")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(received["message"], response.1);
    assert_eq!(received["event_id"], response.1["id"]);
    assert_eq!(count, 1);
}

#[tokio::test]
async fn concurrent_relays_claim_once_and_cannot_cross_tenants() {
    use crate::chat_outbox::{RelayOutcome, relay_chat_event};
    use futures_util::StreamExt;
    let f = PgFixture::new().await;
    let conversation = f.seed_conversation(false).await;
    let response = f
        .request(
            false,
            "POST",
            "/api/widget/messages",
            Some(&f.token),
            json!({"tenant_id":f.a,"conversation_id":conversation,"content":"one claimed event"}),
        )
        .await;
    assert_eq!(response.0, StatusCode::OK);
    let event = response.1["id"].as_str().unwrap();
    let redis = crate::redis_pool::RedisPool::new(&redis_test_url()).unwrap();
    let mut subscription = redis.get_pubsub().await.unwrap();
    subscription
        .subscribe(format!("unified:chat:{}", f.a))
        .await
        .unwrap();
    assert_eq!(
        relay_chat_event(&f.pool, &redis, f.b, event).await.unwrap(),
        RelayOutcome::Idle
    );
    let (a, b) = tokio::join!(
        relay_chat_event(&f.admin, &redis, f.a, event),
        relay_chat_event(&f.admin, &redis, f.a, event)
    );
    let results = [a.unwrap(), b.unwrap()];
    assert_eq!(
        results
            .iter()
            .filter(|r| matches!(r, RelayOutcome::Published { .. }))
            .count(),
        1
    );
    assert_eq!(
        results
            .iter()
            .filter(|r| matches!(r, RelayOutcome::Idle))
            .count(),
        1
    );
    let message = tokio::time::timeout(Duration::from_secs(2), subscription.on_message().next())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&message.get_payload::<String>().unwrap()).unwrap()["event_id"],
        event
    );
    assert!(
        tokio::time::timeout(Duration::from_millis(100), subscription.on_message().next())
            .await
            .is_err()
    );
    f.finish().await;
}

#[tokio::test]
async fn malformed_or_foreign_event_identity_cannot_publish_content() {
    use crate::chat_outbox::{RelayOutcome, relay_chat_event};
    use futures_util::StreamExt;
    let f = PgFixture::new().await;
    let conversation = f.seed_conversation(false).await;
    let response=f.request(false,"POST","/api/widget/messages",Some(&f.token),json!({"tenant_id":f.a,"conversation_id":conversation,"content":"private canonical content"})).await;
    let event = response.1["id"].as_str().unwrap();
    let redis = crate::redis_pool::RedisPool::new(&redis_test_url()).unwrap();
    let mut subscription = redis.get_pubsub().await.unwrap();
    subscription.psubscribe("unified:chat:*").await.unwrap();
    sqlx::query("UPDATE ohc_job_queue SET payload=jsonb_set(payload,'{message,tenant_id}',$1)")
        .bind(sqlx::types::Json(json!(f.b)))
        .execute(&f.admin)
        .await
        .unwrap();
    assert_eq!(
        relay_chat_event(&f.pool, &redis, f.a, event).await.unwrap(),
        RelayOutcome::Failed
    );
    assert!(
        tokio::time::timeout(Duration::from_millis(100), subscription.on_message().next())
            .await
            .is_err()
    );
    let status: String = sqlx::query_scalar("SELECT status FROM ohc_job_queue")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(status, "FAILED");
}

#[tokio::test]
async fn failed_redis_connection_is_bounded_and_retries_event_without_reposting() {
    use crate::chat_outbox::{RelayOutcome, relay_chat_event};
    let f = PgFixture::new().await;
    let conversation = f.seed_conversation(false).await;
    let response=f.request(false,"POST","/api/widget/messages",Some(&f.token),json!({"tenant_id":f.a,"conversation_id":conversation,"content":"pending during Redis outage"})).await;
    let event = response.1["id"].as_str().unwrap();
    let unavailable = crate::redis_pool::RedisPool::new("redis://127.0.0.1:1").unwrap();
    let outcome = tokio::time::timeout(
        Duration::from_secs(2),
        relay_chat_event(&f.pool, &unavailable, f.a, event),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(outcome, RelayOutcome::RetryScheduled);
    let state: (String, i32) = sqlx::query_as("SELECT status,retry_count FROM ohc_job_queue")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    assert_eq!(state, ("PENDING".into(), 1));
    sqlx::query("UPDATE ohc_job_queue SET retry_count=9,next_retry_at=clock_timestamp()-interval '1 second'").execute(&f.admin).await.unwrap();
    assert_eq!(
        relay_chat_event(&f.pool, &unavailable, f.a, event)
            .await
            .unwrap(),
        RelayOutcome::Failed
    );
    let state: (String, i32) = sqlx::query_as("SELECT status,retry_count FROM ohc_job_queue")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    let counts = f.counts().await;
    f.finish().await;
    assert_eq!(state, ("FAILED".into(), 10));
    assert_eq!(counts, (1, 1));
}

#[tokio::test]
async fn cancelled_relay_releases_claim_and_real_redis_can_retry_same_event() {
    use crate::chat_outbox::{RelayOutcome, relay_chat_event};
    use futures_util::StreamExt;
    let f = PgFixture::new().await;
    let conversation = f.seed_conversation(false).await;
    let response=f.request(false,"POST","/api/widget/messages",Some(&f.token),json!({"tenant_id":f.a,"conversation_id":conversation,"content":"survives relay interruption"})).await;
    let event = response.1["id"].as_str().unwrap().to_string();
    // This socket only injects a stalled connection. Successful publication below
    // requires the actual isolated Redis server and an actual subscription.
    let stalled = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let broken =
        crate::redis_pool::RedisPool::new(&format!("redis://{}", stalled.local_addr().unwrap()))
            .unwrap();
    let pool = f.pool.clone();
    let tenant = f.a;
    let claimed_event = event.clone();
    let task =
        tokio::spawn(async move { relay_chat_event(&pool, &broken, tenant, &claimed_event).await });
    let held = tokio::time::timeout(Duration::from_secs(2), stalled.accept())
        .await
        .unwrap()
        .unwrap();
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    drop(held);
    let state: (String, i32) = tokio::time::timeout(
        Duration::from_secs(2),
        sqlx::query_as("SELECT status,retry_count FROM ohc_job_queue").fetch_one(&f.admin),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(state, ("PENDING".into(), 0));
    let redis = crate::redis_pool::RedisPool::new(&redis_test_url()).unwrap();
    let mut subscription = redis.get_pubsub().await.unwrap();
    subscription
        .subscribe(format!("unified:chat:{}", f.a))
        .await
        .unwrap();
    assert!(matches!(
        relay_chat_event(&f.pool, &redis, f.a, &event)
            .await
            .unwrap(),
        RelayOutcome::Published { .. }
    ));
    let message = tokio::time::timeout(Duration::from_secs(2), subscription.on_message().next())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&message.get_payload::<String>().unwrap()).unwrap()["event_id"],
        event
    );
    f.finish().await;
}

#[tokio::test]
async fn reassigned_parent_prevents_pending_event_publication() {
    use crate::chat_outbox::{RelayOutcome, relay_chat_event};
    use futures_util::StreamExt;
    let f = PgFixture::new().await;
    let conversation = f.seed_conversation(false).await;
    let response = f
        .request(
            false,
            "POST",
            "/api/widget/messages",
            Some(&f.token),
            json!({"tenant_id":f.a,"conversation_id":conversation,"content":"no longer owned"}),
        )
        .await;
    let event = response.1["id"].as_str().unwrap();
    sqlx::query("UPDATE chat_inboxes SET tenant_id=$1 WHERE id=$2")
        .bind(f.b)
        .bind(f.ai)
        .execute(&f.admin)
        .await
        .unwrap();
    let redis = crate::redis_pool::RedisPool::new(&redis_test_url()).unwrap();
    let mut subscription = redis.get_pubsub().await.unwrap();
    subscription.psubscribe("unified:chat:*").await.unwrap();
    assert_eq!(
        relay_chat_event(&f.pool, &redis, f.a, event).await.unwrap(),
        RelayOutcome::Failed
    );
    assert!(
        tokio::time::timeout(Duration::from_millis(100), subscription.on_message().next())
            .await
            .is_err()
    );
    f.finish().await;
}

#[tokio::test]
async fn lost_database_ack_preserves_event_identity_for_at_least_once_retry() {
    use crate::chat_outbox::{RelayOutcome, relay_chat_event};
    use futures_util::StreamExt;
    let f = PgFixture::new().await;
    let conversation = f.seed_conversation(false).await;
    let response=f.request(false,"POST","/api/widget/messages",Some(&f.token),json!({"tenant_id":f.a,"conversation_id":conversation,"content":"deduplicate this stable event"})).await;
    let event = response.1["id"].as_str().unwrap();
    sqlx::raw_sql("CREATE FUNCTION reject_relay_ack() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'synthetic acknowledgement failure'; END $$;CREATE CONSTRAINT TRIGGER reject_relay_ack AFTER UPDATE ON ohc_job_queue DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_relay_ack();").execute(&f.admin).await.unwrap();
    let redis = crate::redis_pool::RedisPool::new(&redis_test_url()).unwrap();
    let mut subscription = redis.get_pubsub().await.unwrap();
    subscription
        .subscribe(format!("unified:chat:{}", f.a))
        .await
        .unwrap();
    assert!(relay_chat_event(&f.pool, &redis, f.a, event).await.is_err());
    let first = tokio::time::timeout(Duration::from_secs(2), subscription.on_message().next())
        .await
        .unwrap()
        .unwrap();
    let status: String = sqlx::query_scalar("SELECT status FROM ohc_job_queue")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    assert_eq!(status, "PENDING");
    sqlx::raw_sql("DROP TRIGGER reject_relay_ack ON ohc_job_queue")
        .execute(&f.admin)
        .await
        .unwrap();
    assert!(matches!(
        relay_chat_event(&f.pool, &redis, f.a, event).await.unwrap(),
        RelayOutcome::Published { .. }
    ));
    let second = tokio::time::timeout(Duration::from_secs(2), subscription.on_message().next())
        .await
        .unwrap()
        .unwrap();
    let first: Value = serde_json::from_str(&first.get_payload::<String>().unwrap()).unwrap();
    let second: Value = serde_json::from_str(&second.get_payload::<String>().unwrap()).unwrap();
    assert_eq!(first, second);
    assert_eq!(first["event_id"], event);
    f.finish().await;
}

#[tokio::test]
async fn relay_holds_parent_authority_until_bounded_publication_finishes() {
    use crate::chat_outbox::{RelayOutcome, relay_chat_event};
    for authority in ["inbox", "message"] {
        let f = PgFixture::new().await;
        let conversation = f.seed_conversation(false).await;
        let response=f.request(false,"POST","/api/widget/messages",Some(&f.token),json!({"tenant_id":f.a,"conversation_id":conversation,"content":"ownership fenced through publication"})).await;
        let event = response.1["id"].as_str().unwrap().to_string();
        let redis_url = url::Url::parse(&redis_test_url()).unwrap();
        let upstream = (
            redis_url.host_str().unwrap().to_string(),
            redis_url.port().unwrap(),
        );
        let real_redis = crate::redis_pool::RedisPool::new(&redis_test_url()).unwrap();
        let mut subscription = real_redis.get_pubsub().await.unwrap();
        subscription
            .subscribe(format!("unified:chat:{}", f.a))
            .await
            .unwrap();
        // Delay a transparent connection to real Redis after the relay has read its
        // canonical rows; the proxy never fabricates Redis protocol responses.
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let redis = crate::redis_pool::RedisPool::new(&format!(
            "redis://{}",
            listener.local_addr().unwrap()
        ))
        .unwrap();
        let (accepted_tx, accepted_rx) = tokio::sync::oneshot::channel();
        let (release_tx, release_rx) = tokio::sync::oneshot::channel();
        let proxy = tokio::spawn(async move {
            let (mut downstream, _) = listener.accept().await.unwrap();
            accepted_tx.send(()).unwrap();
            release_rx.await.unwrap();
            let mut upstream = tokio::net::TcpStream::connect(upstream).await.unwrap();
            let _ = tokio::io::copy_bidirectional(&mut downstream, &mut upstream).await;
        });
        let pool = f.pool.clone();
        let tenant = f.a;
        let relay =
            tokio::spawn(async move { relay_chat_event(&pool, &redis, tenant, &event).await });
        tokio::time::timeout(Duration::from_secs(2), accepted_rx)
            .await
            .unwrap()
            .unwrap();
        let mut transfer = f.admin.acquire().await.unwrap();
        let pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
            .fetch_one(&mut *transfer)
            .await
            .unwrap();
        let foreign = f.b;
        let inbox = f.ai;
        let message_id = Uuid::parse_str(response.1["id"].as_str().unwrap()).unwrap();
        let change = tokio::spawn(async move {
            sqlx::query(if authority == "inbox" {
                "UPDATE chat_inboxes SET tenant_id=$1 WHERE id=$2"
            } else {
                "UPDATE chat_messages SET tenant_id=$1 WHERE id=$2"
            })
            .bind(foreign)
            .bind(if authority == "inbox" {
                inbox
            } else {
                message_id
            })
            .execute(&mut *transfer)
            .await
            .unwrap()
        });
        let blocked=tokio::time::timeout(Duration::from_millis(200),async {
        loop {
            let waiting:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_locks WHERE pid=$1 AND NOT granted AND locktype IN ('tuple','transactionid'))").bind(pid).fetch_one(&f.admin).await.unwrap();
            if waiting {return true;}
            if change.is_finished() {return false;}
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    }).await.unwrap();
        release_tx.send(()).unwrap();
        let outcome = relay.await.unwrap().unwrap();
        tokio::time::timeout(Duration::from_secs(2), change)
            .await
            .unwrap()
            .unwrap();
        proxy.abort();
        f.finish().await;
        assert!(
            blocked,
            "{authority}: ownership transfer must wait for the in-flight bounded publication"
        );
        assert!(matches!(outcome, RelayOutcome::Published { .. }));
    }
}

#[tokio::test]
async fn relay_preserves_stricter_operator_statement_and_lock_timeouts() {
    use crate::chat_outbox::{RelayOutcome, relay_chat_event};
    let f = PgFixture::new().await;
    let conversation = f.seed_conversation(false).await;
    let response=f.request(false,"POST","/api/widget/messages",Some(&f.token),json!({"tenant_id":f.a,"conversation_id":conversation,"content":"keep stricter operator limits"})).await;
    let event = response.1["id"].as_str().unwrap();
    sqlx::raw_sql(&format!(
        "ALTER ROLE {} SET lock_timeout='100ms';ALTER ROLE {} SET statement_timeout='200ms';",
        f.role, f.role
    ))
    .execute(&f.admin)
    .await
    .unwrap();
    f.pool.acquire().await.unwrap().close().await.unwrap();
    sqlx::raw_sql("CREATE FUNCTION check_relay_limits() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF current_setting('lock_timeout')::interval>interval '100ms' OR current_setting('statement_timeout')::interval>interval '200ms' THEN RAISE EXCEPTION 'relay increased configured timeout'; END IF; RETURN NEW; END $$;CREATE TRIGGER check_relay_limits BEFORE UPDATE ON ohc_job_queue FOR EACH ROW EXECUTE FUNCTION check_relay_limits();").execute(&f.admin).await.unwrap();
    let unavailable = crate::redis_pool::RedisPool::new("redis://127.0.0.1:1").unwrap();
    let outcome = relay_chat_event(&f.pool, &unavailable, f.a, event).await;
    f.finish().await;
    assert_eq!(outcome.unwrap(), RelayOutcome::RetryScheduled);
}
