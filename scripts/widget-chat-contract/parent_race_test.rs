use super::*;

fn race_request(f: &PgFixture, operation: &str, conversation: Uuid) -> Request<Body> {
    let (method, path, body) = match operation {
        "conversation" => (
            "POST",
            "/api/widget/conversations".into(),
            f.conversation(false),
        ),
        "message" => (
            "POST",
            "/api/widget/messages".into(),
            json!({"tenant_id":f.a,"conversation_id":conversation,"content":"race-owned-message"}),
        ),
        "read" => (
            "GET",
            format!("/api/widget/{}/conversations/{conversation}/messages", f.a),
            Value::Null,
        ),
        _ => unreachable!(),
    };
    Request::builder()
        .method(method)
        .uri(path)
        .header("authorization", format!("Bearer {}", f.token))
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

// Observe PostgreSQL's actual lock wait or task completion, never infer ordering
// from a sleep. All gates are released before asserting the expected result.
async fn observe_wait<T>(
    pool: &PgPool,
    task: &tokio::task::JoinHandle<T>,
    pid: Option<i32>,
    role: &str,
    advisory: bool,
) -> bool {
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let waiting: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_locks l JOIN pg_stat_activity a ON a.pid=l.pid WHERE NOT l.granted AND (($1::integer IS NOT NULL AND a.pid=$1) OR ($1::integer IS NULL AND a.usename=$2)) AND (($3 AND l.locktype='advisory') OR (NOT $3 AND l.locktype IN ('tuple','transactionid'))))").bind(pid).bind(role).bind(advisory).fetch_one(pool).await.unwrap();
            if waiting { return true; }
            if task.is_finished() { return false; }
            tokio::task::yield_now().await;
        }
    }).await.expect("operation must either finish or enter an observed PostgreSQL lock wait")
}

#[tokio::test]
async fn admitted_operations_hold_both_parent_authorities_until_completion() {
    let mut evidence = vec![];
    for parent in ["inboxes", "contacts"] {
        for operation in ["conversation", "message", "read"] {
            let f = PgFixture::new().await;
            let conversation = f.seed_conversation(false).await;
            sqlx::query("INSERT INTO chat_messages(id,tenant_id,conversation_id,sender_type,content) VALUES($1,$2,$3,'contact','existing-private-message')").bind(Uuid::new_v4()).bind(f.a).bind(conversation).execute(&f.admin).await.unwrap();
            let key = Uuid::new_v4().as_u128() as i64;
            let gate_sql = if operation == "read" {
                format!(
                    "CREATE FUNCTION fixture_read_gate() RETURNS boolean LANGUAGE plpgsql VOLATILE AS $$BEGIN PERFORM pg_advisory_xact_lock({key});RETURN true;END$$; CREATE POLICY fixture_pause_read ON chat_messages AS RESTRICTIVE FOR SELECT USING(fixture_read_gate());"
                )
            } else {
                let table = if operation == "conversation" {
                    "chat_conversations"
                } else {
                    "chat_messages"
                };
                format!(
                    "CREATE FUNCTION fixture_insert_gate() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN PERFORM pg_advisory_xact_lock({key});RETURN NEW;END$$;CREATE TRIGGER fixture_pause_insert BEFORE INSERT ON {table} FOR EACH ROW EXECUTE FUNCTION fixture_insert_gate();"
                )
            };
            sqlx::raw_sql(&gate_sql).execute(&f.admin).await.unwrap();
            let mut gate = f.admin.begin().await.unwrap();
            sqlx::query("SELECT pg_advisory_xact_lock($1)")
                .bind(key)
                .execute(&mut *gate)
                .await
                .unwrap();
            let app = f.app(false);
            let request = race_request(&f, operation, conversation);
            let active = tokio::spawn(async move { app.oneshot(request).await.unwrap() });
            let admitted = observe_wait(&f.admin, &active, None, &f.role, true).await;
            let mut updater = f.admin.acquire().await.unwrap();
            let pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
                .fetch_one(&mut *updater)
                .await
                .unwrap();
            let other = f.b;
            let id = if parent == "inboxes" { f.ai } else { f.ac };
            let sql = format!("UPDATE chat_{parent} SET tenant_id=$1 WHERE id=$2");
            let changed = tokio::spawn(async move {
                sqlx::query(&sql)
                    .bind(other)
                    .bind(id)
                    .execute(&mut *updater)
                    .await
            });
            let blocked = observe_wait(&f.admin, &changed, Some(pid), &f.role, false).await;
            gate.commit().await.unwrap();
            let result = tokio::time::timeout(Duration::from_secs(5), active)
                .await
                .unwrap()
                .unwrap();
            let update = tokio::time::timeout(Duration::from_secs(5), changed)
                .await
                .unwrap()
                .unwrap();
            let later = f
                .request(
                    false,
                    "GET",
                    &format!("/api/widget/{}/conversations/{conversation}/messages", f.a),
                    Some(&f.token),
                    Value::Null,
                )
                .await
                .0;
            evidence.push((
                parent,
                operation,
                admitted,
                blocked,
                result.status(),
                update.is_ok(),
                later,
            ));
            f.finish().await;
        }
    }
    for (parent, operation, admitted, blocked, status, updated, later) in evidence {
        assert!(
            admitted,
            "{parent}/{operation}: actual operation did not reach fixture barrier"
        );
        assert!(
            blocked,
            "{parent}/{operation}: parent tenant changed while the admitted operation remained in flight"
        );
        assert_eq!(status, StatusCode::OK, "{parent}/{operation}");
        assert!(updated);
        assert_eq!(later, StatusCode::NOT_FOUND);
    }
}

#[tokio::test]
async fn parent_reassignment_winning_the_lock_refuses_every_later_operation() {
    let mut evidence = vec![];
    for parent in ["inboxes", "contacts"] {
        for operation in ["conversation", "message", "read"] {
            let f = PgFixture::new().await;
            let conversation = f.seed_conversation(false).await;
            sqlx::query("INSERT INTO chat_messages(id,tenant_id,conversation_id,sender_type,content) VALUES($1,$2,$3,'contact','private-before-revocation')").bind(Uuid::new_v4()).bind(f.a).bind(conversation).execute(&f.admin).await.unwrap();
            let before = f.counts().await;
            let mut updater = f.admin.begin().await.unwrap();
            let id = if parent == "inboxes" { f.ai } else { f.ac };
            sqlx::query(&format!(
                "UPDATE chat_{parent} SET tenant_id=$1 WHERE id=$2"
            ))
            .bind(f.b)
            .bind(id)
            .execute(&mut *updater)
            .await
            .unwrap();
            let app = f.app(false);
            let request = race_request(&f, operation, conversation);
            let active = tokio::spawn(async move { app.oneshot(request).await.unwrap() });
            let blocked = observe_wait(&f.admin, &active, None, &f.role, false).await;
            updater.commit().await.unwrap();
            let response = tokio::time::timeout(Duration::from_secs(5), active)
                .await
                .unwrap()
                .unwrap();
            let after = f.counts().await;
            evidence.push((parent, operation, blocked, response.status(), before, after));
            f.finish().await;
        }
    }
    for (parent, operation, blocked, status, before, after) in evidence {
        assert!(
            blocked,
            "{parent}/{operation}: operation admitted from an old parent snapshot"
        );
        assert_eq!(status, StatusCode::NOT_FOUND, "{parent}/{operation}");
        assert_eq!(
            before, after,
            "{parent}/{operation}: refused operation wrote a row"
        );
    }
}

#[tokio::test]
async fn contended_parent_wait_times_out_without_writing_or_poisoning_the_pool() {
    let f = PgFixture::new().await;
    let conversation = f.seed_conversation(false).await;
    let mut blocker = f.admin.begin().await.unwrap();
    sqlx::query("SELECT id FROM chat_inboxes WHERE id=$1 FOR UPDATE")
        .bind(f.ai)
        .fetch_one(&mut *blocker)
        .await
        .unwrap();
    let app = f.app(false);
    let request = race_request(&f, "message", conversation);
    let mut active = tokio::spawn(async move { app.oneshot(request).await.unwrap() });
    let waited = observe_wait(&f.admin, &active, None, &f.role, false).await;
    let outcome = tokio::time::timeout(Duration::from_millis(4_500), &mut active).await;
    let bounded = outcome.is_ok();
    blocker.rollback().await.unwrap();
    let response = match outcome {
        Ok(result) => result.unwrap(),
        Err(_) => active.await.unwrap(),
    };
    let counts = f.counts().await;
    // A stricter real role default must not be lengthened by the application cap.
    sqlx::raw_sql(&format!(
        "ALTER ROLE {} SET lock_timeout='100ms'; ALTER ROLE {} SET statement_timeout='200ms';",
        f.role, f.role
    ))
    .execute(&f.admin)
    .await
    .unwrap();
    f.pool.acquire().await.unwrap().close().await.unwrap();
    let mut shorter_blocker = f.admin.begin().await.unwrap();
    sqlx::query("SELECT id FROM chat_inboxes WHERE id=$1 FOR UPDATE")
        .bind(f.ai)
        .fetch_one(&mut *shorter_blocker)
        .await
        .unwrap();
    let app = f.app(false);
    let request = race_request(&f, "message", conversation);
    let mut shorter = tokio::spawn(async move { app.oneshot(request).await.unwrap() });
    let short_result = tokio::time::timeout(Duration::from_millis(1_000), &mut shorter).await;
    let preserved_shorter = short_result.is_ok();
    shorter_blocker.rollback().await.unwrap();
    let short_response = match short_result {
        Ok(result) => result.unwrap(),
        Err(_) => shorter.await.unwrap(),
    };
    let next=f.request(false,"POST","/api/widget/messages",Some(&f.token),json!({"tenant_id":f.a,"conversation_id":conversation,"content":"explicit-independent-followup"})).await;
    f.finish().await;
    assert!(
        waited,
        "the regression must reach a real parent row-lock wait"
    );
    assert!(
        bounded,
        "lock waits must end before the application deadline even while the blocker remains held"
    );
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(counts, (1, 0));
    assert!(
        preserved_shorter,
        "stricter configured timeouts must not be increased"
    );
    assert_eq!(short_response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(next.0, StatusCode::OK);
}
