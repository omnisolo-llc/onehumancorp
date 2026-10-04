use super::*;
const ROUTE: &str = "/api/v1/sync/events";
fn event(id: &str, status: &str) -> Value {
    json!({"id":id,"entity_id":"a","entity_type":"appointment","action_type":"UpdateStatus","base_version":1,"payload":{"status":status,"expected_status":"Scheduled","expected_updated_at":"2026-10-03T00:00:00Z"}})
}
async fn send(f: &Fixture, token: &str, events: Vec<Value>) -> (StatusCode, Value) {
    f.request("POST", ROUTE, token, json!({"events":events}), None)
        .await
}
async fn counts(f: &Fixture) -> (i64, i64, i64) {
    sqlx::query_as("SELECT (SELECT count(*) FROM sync_events),(SELECT count(*) FROM sync_conflict_queue),(SELECT count(*) FROM department_tasks)").fetch_one(&f.admin).await.unwrap()
}
#[tokio::test]
async fn offline_owner_schedule_notes_and_exact_replay_commit_once() {
    let f = Fixture::new(true).await;
    let mut e = event("complete", "Completed");
    e["payload"]["notes"] = json!(null);
    e["payload"]["expected_notes"] = json!("stored");
    e["payload"]["scheduled_start_time"] = json!("2026-10-04T10:15:00Z");
    for _ in 0..2 {
        let r = send(&f, &f.token, vec![e.clone()]).await;
        assert_eq!(r.0, StatusCode::OK, "{r:?}");
        assert_eq!(r.1["outcomes"][0]["status"], "acknowledged", "{r:?}");
        assert_eq!(r.1["outcomes"][0]["result_version"], 2);
    }
    let row:(String,Option<String>,DateTime<Utc>,DateTime<Utc>)=sqlx::query_as("SELECT status,notes,scheduled_start_time,scheduled_end_time FROM appointments WHERE id='a'").fetch_one(&f.admin).await.unwrap();
    assert_eq!(row.0, "Completed");
    assert_eq!(row.1, None);
    assert_eq!(row.2.to_rfc3339(), "2026-10-04T10:15:00+00:00");
    assert_eq!(row.3.to_rfc3339(), "2026-10-04T11:00:00+00:00");
    assert_eq!(counts(&f).await, (1, 0, 1));
    e["payload"]["notes"] = json!("changed");
    assert_eq!(
        send(&f, &f.token, vec![e]).await.1["outcomes"][0]["status"],
        "reconciliation"
    );
    assert_eq!(counts(&f).await, (1, 0, 1));
    f.finish().await;
}
#[tokio::test]
async fn offline_member_foreign_missing_and_changed_owner_never_write() {
    let f = Fixture::new(true).await;
    for token in [&f.member, &f.foreign] {
        let r = send(&f, token, vec![event("denied", "Completed")]).await;
        assert_ne!(r.1["outcomes"][0]["status"], "acknowledged", "{r:?}");
    }
    let mut missing = event("missing", "Completed");
    missing["entity_id"] = json!("missing");
    assert_eq!(
        send(&f, &f.token, vec![missing]).await.1["outcomes"][0]["status"],
        "blocked"
    );
    sqlx::query("DELETE FROM identity_user_roles WHERE user_id='owner-a'")
        .execute(&f.admin)
        .await
        .unwrap();
    let r = send(&f, &f.token, vec![event("downgraded", "Completed")]).await;
    assert_ne!(r.1["outcomes"][0]["status"], "acknowledged", "{r:?}");
    assert_eq!(counts(&f).await, (0, 0, 0));
    f.finish().await;
}
#[tokio::test]
async fn offline_terminal_reopen_and_invalid_status_are_blocked() {
    for current in ["Completed", "completed", "Cancelled", "canceled", "done"] {
        let f = Fixture::new(true).await;
        sqlx::query("UPDATE appointments SET status=$1 WHERE id='a'")
            .bind(current)
            .execute(&f.admin)
            .await
            .unwrap();
        let timestamp: DateTime<Utc> =
            sqlx::query_scalar("SELECT updated_at FROM appointments WHERE id='a'")
                .fetch_one(&f.admin)
                .await
                .unwrap();
        let mut e = event("reopen", "Scheduled");
        e["payload"]["expected_status"] = json!(current);
        e["payload"]["expected_updated_at"] = json!(timestamp);
        assert_eq!(
            send(&f, &f.token, vec![e]).await.1["outcomes"][0]["status"],
            "blocked"
        );
        assert_eq!(counts(&f).await, (0, 0, 0));
        f.finish().await;
    }
    let f = Fixture::new(true).await;
    assert_eq!(
        send(&f, &f.token, vec![event("invalid", "arbitrary")])
            .await
            .1["outcomes"][0]["status"],
        "blocked"
    );
    assert_eq!(counts(&f).await, (0, 0, 0));
    f.finish().await;
}
#[tokio::test]
async fn offline_invalid_merged_schedule_and_invalid_types_have_no_receipt() {
    for value in [
        json!("2026-10-04T12:00:00Z"),
        json!("not a timestamp"),
        json!(10),
    ] {
        let f = Fixture::new(true).await;
        let mut e = event("schedule", "Confirmed");
        e["payload"]["scheduled_start_time"] = value;
        assert_eq!(
            send(&f, &f.token, vec![e]).await.1["outcomes"][0]["status"],
            "blocked"
        );
        assert_eq!(counts(&f).await, (0, 0, 0));
        f.finish().await;
    }
}
#[tokio::test]
async fn offline_stale_state_has_one_durable_reconciliation() {
    let f = Fixture::new(true).await;
    let mut e = event("stale", "Completed");
    e["payload"]["expected_notes"] = json!("old");
    e["payload"]["notes"] = json!("new");
    for _ in 0..2 {
        assert_eq!(
            send(&f, &f.token, vec![e.clone()]).await.1["outcomes"][0]["status"],
            "reconciliation"
        );
    }
    assert_eq!(counts(&f).await, (1, 1, 1));
    f.finish().await;
}
#[tokio::test]
async fn offline_deferred_commit_failure_rolls_back_every_effect() {
    let f = Fixture::new(true).await;
    sqlx::raw_sql("CREATE FUNCTION reject_offline_commit() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN RAISE EXCEPTION 'synthetic deferred failure';END$$;CREATE CONSTRAINT TRIGGER reject_offline_commit AFTER UPDATE ON appointments DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_offline_commit();").execute(&f.admin).await.unwrap();
    let r = send(&f, &f.token, vec![event("deferred", "Completed")]).await;
    assert_eq!(r.1["outcomes"][0]["status"], "blocked", "{r:?}");
    assert_eq!(counts(&f).await, (0, 0, 0));
    assert_eq!(
        sqlx::query_scalar::<_, String>("SELECT status FROM appointments WHERE id='a'")
            .fetch_one(&f.admin)
            .await
            .unwrap(),
        "Scheduled"
    );
    f.finish().await;
}
#[tokio::test]
async fn offline_concurrent_updates_commit_one_and_reconcile_one() {
    let f = Fixture::new(true).await;
    let (one, two) = tokio::join!(
        send(&f, &f.token, vec![event("one", "Completed")]),
        send(&f, &f.token, vec![event("two", "Confirmed")])
    );
    let mut statuses = vec![
        one.1["outcomes"][0]["status"].as_str().unwrap().to_owned(),
        two.1["outcomes"][0]["status"].as_str().unwrap().to_owned(),
    ];
    statuses.sort();
    assert_eq!(statuses, vec!["acknowledged", "reconciliation"]);
    assert_eq!(counts(&f).await, (2, 1, 2));
    f.finish().await;
}
#[tokio::test]
async fn offline_case_normalization_and_null_schedule_preserve_stored_state() {
    let f = Fixture::new(true).await;
    sqlx::query("UPDATE appointments SET status='completed' WHERE id='a'")
        .execute(&f.admin)
        .await
        .unwrap();
    let observed: DateTime<Utc> =
        sqlx::query_scalar("SELECT updated_at FROM appointments WHERE id='a'")
            .fetch_one(&f.admin)
            .await
            .unwrap();
    let mut e = event("normalize", "Completed");
    e["payload"]["expected_status"] = json!("Completed");
    e["payload"]["expected_updated_at"] = json!(observed);
    e["payload"]["scheduled_start_time"] = Value::Null;
    assert_eq!(
        send(&f, &f.token, vec![e]).await.1["outcomes"][0]["status"],
        "acknowledged"
    );
    let row: (String, Option<String>, DateTime<Utc>) =
        sqlx::query_as("SELECT status,notes,scheduled_start_time FROM appointments WHERE id='a'")
            .fetch_one(&f.admin)
            .await
            .unwrap();
    assert_eq!(row.0, "Completed");
    assert_eq!(row.1.as_deref(), Some("stored"));
    assert_eq!(row.2.to_rfc3339(), "2026-10-04T10:00:00+00:00");
    assert_eq!(counts(&f).await, (1, 0, 0));
    f.finish().await;
}
#[tokio::test]
async fn offline_alternate_receipt_schema_cannot_gain_canonical_authority() {
    let f = Fixture::new(true).await;
    let alternate = format!("alternate_{}", Uuid::new_v4().simple());
    sqlx::raw_sql(&format!("CREATE SCHEMA {alternate};CREATE TABLE {alternate}.sync_events (LIKE {}.sync_events INCLUDING ALL);GRANT USAGE ON SCHEMA {alternate} TO {};GRANT ALL ON {alternate}.sync_events TO {};",f.schema,f.role,f.role)).execute(&f.admin).await.unwrap();
    let raw = std::env::var("OHC_FIELD_TEST_DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .connect_with(
            target(&raw).options([("search_path", format!("{alternate},{}", f.schema).as_str())]),
        )
        .await
        .unwrap();
    let app = crate::actual_mount(
        Arc::new(crate::db::DB { pool: pool.clone() }),
        f.auth.clone(),
    )
    .await;
    let r = app
        .oneshot(
            Request::post(ROUTE)
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", f.token))
                .body(Body::from(
                    json!({"events":[event("alternate","Completed")]}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let value: Value =
        serde_json::from_slice(&to_bytes(r.into_body(), 1048576).await.unwrap()).unwrap();
    assert_eq!(value["outcomes"][0]["status"], "blocked");
    assert_eq!(
        value["outcomes"][0]["reason"],
        "canonical_authority_unavailable"
    );
    assert_eq!(counts(&f).await, (0, 0, 0));
    pool.close().await;
    sqlx::query(&format!("DROP SCHEMA {alternate} CASCADE"))
        .execute(&f.admin)
        .await
        .unwrap();
    f.finish().await;
}
#[tokio::test]
async fn offline_revocation_while_update_blocked_prevents_receipt_and_effect() {
    let f = Fixture::new(true).await;
    sqlx::raw_sql("CREATE FUNCTION hold_offline_update() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN PERFORM pg_advisory_xact_lock(71893421);RETURN NEW;END$$;CREATE TRIGGER hold_offline_update BEFORE UPDATE ON appointments FOR EACH ROW EXECUTE FUNCTION hold_offline_update();").execute(&f.admin).await.unwrap();
    let app = crate::actual_mount(
        Arc::new(crate::db::DB {
            pool: f.pool.clone(),
        }),
        f.auth.clone(),
    )
    .await;
    let mut gate = f.admin.begin().await.unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock(71893421)")
        .execute(&mut *gate)
        .await
        .unwrap();
    let token = f.token.clone();
    let pending = tokio::spawn(async move {
        app.oneshot(
            Request::post(ROUTE)
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::from(
                    json!({"events":[event("revoke-race","Completed")]}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap()
    });
    tokio::time::timeout(Duration::from_millis(800),async {loop {
        let waiting:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_locks WHERE locktype='advisory' AND objid=71893421 AND NOT granted)").fetch_one(&f.admin).await.unwrap();if waiting {break;}tokio::time::sleep(Duration::from_millis(5)).await;
    }}).await.expect("request must reach guarded update before revocation");
    let claims = f.auth.validate_token(&f.token).await.unwrap();
    f.auth
        .revoke_token(
            claims.jti,
            DateTime::from_timestamp(claims.exp, 0).unwrap(),
            "tenant-a",
        )
        .await
        .unwrap();
    gate.commit().await.unwrap();
    let r = pending.await.unwrap();
    let value: Value =
        serde_json::from_slice(&to_bytes(r.into_body(), 1048576).await.unwrap()).unwrap();
    assert_eq!(value["outcomes"][0]["status"], "blocked");
    assert_eq!(
        value["outcomes"][0]["reason"],
        "current_owner_authority_required"
    );
    assert_eq!(counts(&f).await, (0, 0, 0));
    f.finish().await;
}
#[tokio::test]
async fn offline_expiry_during_update_prevents_commit() {
    let f = Fixture::new(true).await;
    let mut claims = f.auth.validate_token(&f.token).await.unwrap();
    claims.exp = Utc::now().timestamp() + 2;
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_bytes()),
    )
    .unwrap();
    sqlx::raw_sql("CREATE FUNCTION delay_offline_update() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN PERFORM pg_sleep(2.1);RETURN NEW;END$$;CREATE TRIGGER delay_offline_update BEFORE UPDATE ON appointments FOR EACH ROW EXECUTE FUNCTION delay_offline_update();").execute(&f.admin).await.unwrap();
    let r = send(&f, &token, vec![event("expiry-race", "Completed")]).await;
    assert_eq!(r.1["outcomes"][0]["status"], "blocked", "{r:?}");
    assert_eq!(
        r.1["outcomes"][0]["reason"],
        "current_owner_authority_required"
    );
    assert_eq!(counts(&f).await, (0, 0, 0));
    f.finish().await;
}
#[tokio::test]
async fn offline_sqlite_identity_preserves_local_authority_but_cannot_mutate_pg_route() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    let orm = sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
    let backend = sea_orm::DatabaseBackend::Sqlite;
    for statement in [
        Schema::new(backend)
            .create_table_from_entity(server_auth::seaorm_store::entities::user::Entity),
        Schema::new(backend).create_table_from_entity(
            server_auth::seaorm_store::entities::identity_user_role::Entity,
        ),
        Schema::new(backend)
            .create_table_from_entity(server_auth::seaorm_store::entities::revoked_token::Entity),
    ] {
        orm.execute(backend.build(&statement)).await.unwrap();
    }
    sqlx::raw_sql("INSERT INTO users(id,username,email,password_hash,active,tenant_id,created_at,updated_at)VALUES('local-owner','local-owner','local-owner@example.test','',1,'local-tenant',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP);INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position)VALUES('local-owner','ADMIN','local-tenant',0);CREATE TABLE appointments(id TEXT PRIMARY KEY,tenant_id TEXT,status TEXT);INSERT INTO appointments VALUES('a','local-tenant','Scheduled');").execute(&pool).await.unwrap();
    let auth = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
        server_auth::seaorm_store::SeaOrmAuthRepository::new(orm),
    )));
    let user = auth.get_user("local-owner", "local-tenant").await.unwrap();
    let token = auth.issue_token(&user).unwrap();
    let claims = auth.validate_token(&token).await.unwrap();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("authorization", format!("Bearer {token}").parse().unwrap());
    let pg = PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(20))
        .connect_lazy("postgres://fixture@127.0.0.1:1/unused")
        .unwrap();
    let app = crate::actual_mount(Arc::new(crate::db::DB { pool: pg }), auth.clone()).await;
    let r = app
        .clone()
        .oneshot(
            Request::post(ROUTE)
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::from(
                    json!({"events":[event("local","Completed")]}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(r.status(), StatusCode::OK);
    let value: Value =
        serde_json::from_slice(&to_bytes(r.into_body(), 1048576).await.unwrap()).unwrap();
    assert_eq!(value["outcomes"][0]["status"], "blocked");
    assert_eq!(
        value["outcomes"][0]["reason"],
        "canonical_authority_unavailable"
    );
    assert_eq!(
        sqlx::query_scalar::<_, String>("SELECT status FROM appointments WHERE id='a'")
            .fetch_one(&pool)
            .await
            .unwrap(),
        "Scheduled"
    );
    for (route, body) in [
        (
            "/api/v1/sync/offline",
            json!({"mutations":[{"transaction_id":"local-quote","product_id":"draft_quote","quantity_deducted":0,"mutation_type":"draft_quote","payload":"Prepare quote"}]}),
        ),
        (
            "/api/v1/sync/operation-intents",
            json!({"intents":[{"id":"local-intent","action_type":"draft_quote","payload":{"notes":"Prepare quote"}}]}),
        ),
    ] {
        let r = app
            .clone()
            .oneshot(
                Request::post(route)
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {token}"))
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(r.status(), StatusCode::OK);
        let value: Value =
            serde_json::from_slice(&to_bytes(r.into_body(), 1048576).await.unwrap()).unwrap();
        assert_eq!(value["outcomes"][0]["status"], "blocked");
        assert_eq!(
            value["outcomes"][0]["reason"],
            "canonical_authority_unavailable"
        );
    }
    // The existing SQLite capability remains usable for supported local writes.
    let authority =
        server_auth::commit_authority::CanonicalSqliteAuthority::bind(auth, &pool).unwrap();
    let owner = authority.authorize(&claims, &headers).await.unwrap();
    let mut tx = owner.begin().await.unwrap();
    sqlx::query(
        "UPDATE appointments SET status='Completed' WHERE id='a' AND tenant_id='local-tenant'",
    )
    .execute(tx.connection())
    .await
    .unwrap();
    tx.commit().await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, String>("SELECT status FROM appointments WHERE id='a'")
            .fetch_one(&pool)
            .await
            .unwrap(),
        "Completed"
    );
    pool.close().await;
}
#[tokio::test]
async fn offline_revoked_admission_and_exact_replay_cannot_acknowledge() {
    let f = Fixture::new(true).await;
    let e = event("replay-revoked", "Completed");
    assert_eq!(
        send(&f, &f.token, vec![e.clone()]).await.1["outcomes"][0]["status"],
        "acknowledged"
    );
    let claims = f.auth.validate_token(&f.token).await.unwrap();
    f.auth
        .revoke_token(
            claims.jti,
            DateTime::from_timestamp(claims.exp, 0).unwrap(),
            "tenant-a",
        )
        .await
        .unwrap();
    let r = send(&f, &f.token, vec![e]).await;
    assert_eq!(r.0, StatusCode::UNAUTHORIZED);
    assert_eq!(counts(&f).await, (1, 0, 1));
    f.finish().await;
}
#[tokio::test]
async fn offline_mixed_inventory_preserves_effect_when_appointment_is_blocked() {
    let f = Fixture::new(true).await;
    sqlx::query("INSERT INTO products(id,tenant_id)VALUES('p','tenant-a')")
        .execute(&f.admin)
        .await
        .unwrap();
    let product = json!({"id":"product","entity_id":"p","entity_type":"product","action_type":"ToggleSoldOut","base_version":1,"payload":{"is_sold_out":true,"expected_is_sold_out":false,"expected_updated_at":"2026-01-01T00:00:00Z"}});
    let r = send(
        &f,
        &f.member,
        vec![event("not-owner", "Completed"), product],
    )
    .await;
    assert_eq!(r.1["outcomes"][0]["status"], "blocked");
    assert_eq!(r.1["outcomes"][1]["status"], "acknowledged");
    assert_eq!(r.1["success"], false);
    assert_eq!(r.1["applied_count"], 1);
    assert_eq!(r.1["failed_count"], 1);
    assert!(
        sqlx::query_scalar::<_, bool>("SELECT is_sold_out FROM products WHERE id='p'")
            .fetch_one(&f.admin)
            .await
            .unwrap()
    );
    assert_eq!(counts(&f).await, (1, 0, 0));
    f.finish().await;
}
