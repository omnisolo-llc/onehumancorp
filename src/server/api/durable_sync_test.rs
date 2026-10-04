//! These tests require an isolated PostgreSQL database, never a customer database.
use super::*;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;

async fn fixture() -> (sqlx::PgPool, String) {
    let url = std::env::var("OHC_SYNC_TEST_DATABASE_URL").expect("isolated test database required");
    let schema = format!("sync_test_{}", uuid::Uuid::new_v4().simple());
    let admin = PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
        .unwrap();
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&admin)
        .await
        .unwrap();
    let schema_copy = schema.clone();
    let pool = PgPoolOptions::new()
        .max_connections(12)
        .after_connect(move |connection, _| {
            let schema = schema_copy.clone();
            Box::pin(async move {
                sqlx::query(&format!("SET search_path TO {schema}"))
                    .execute(connection)
                    .await?;
                Ok(())
            })
        })
        .connect(&url)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!("durable_sync_test_schema.sql"))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!("../migrations/234_sync_durable_receipts.sql"))
        .execute(&pool)
        .await
        .unwrap();
    (pool, schema)
}

fn event(
    id: &str,
    entity: &str,
    kind: &str,
    action: &str,
    mut payload: serde_json::Value,
) -> SyncEvent {
    if kind != "audio_intent" && payload.get("expected_updated_at").is_none() {
        payload["expected_updated_at"] = json!("2026-01-01T00:00:00Z");
    }
    SyncEvent {
        id: id.into(),
        entity_id: entity.into(),
        entity_type: kind.into(),
        action_type: action.into(),
        payload,
        base_version: 1,
    }
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn mounted_product_commits_before_ack_and_changed_replay_reconciles() {
    let (pool, _) = fixture().await;
    sqlx::query("INSERT INTO products (id,tenant_id) VALUES ('p','tenant-a')")
        .execute(&pool)
        .await
        .unwrap();
    let e = event(
        "e",
        "p",
        "product",
        "ToggleSoldOut",
        json!({"is_sold_out":true,"expected_is_sold_out":false}),
    );
    let first = sync_events(&pool, "tenant-a", std::slice::from_ref(&e)).await;
    assert_eq!(first.outcomes[0].status, "acknowledged");
    assert_eq!(first.outcomes[0].route, EVENTS_ROUTE);
    let sold: bool = sqlx::query_scalar("SELECT is_sold_out FROM products WHERE id='p'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(sold);
    assert_eq!(
        sync_events(&pool, "tenant-a", std::slice::from_ref(&e))
            .await
            .outcomes[0]
            .status,
        "acknowledged"
    );
    let mut changed = e;
    changed.payload["is_sold_out"] = json!(false);
    assert_eq!(
        sync_events(&pool, "tenant-a", &[changed]).await.outcomes[0].status,
        "reconciliation"
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM sync_events")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn concurrent_replay_applies_order_and_downstream_task_once() {
    let (pool, _) = fixture().await;
    sqlx::query("INSERT INTO orders (id,tenant_id,status) VALUES ('o','tenant-a','Pending')")
        .execute(&pool)
        .await
        .unwrap();
    let e = event(
        "e",
        "o",
        "order",
        "UpdateStatus",
        json!({"status":"ReadyForPickup","expected_status":"Pending"}),
    );
    let mut handles = vec![];
    for _ in 0..8 {
        let p = pool.clone();
        let e = e.clone();
        handles.push(tokio::spawn(async move {
            sync_events(&p, "tenant-a", &[e]).await
        }));
    }
    for h in handles {
        assert_eq!(h.await.unwrap().outcomes[0].status, "acknowledged");
    }
    let tasks: i64 = sqlx::query_scalar("SELECT count(*) FROM department_tasks")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(tasks, 1);
    let receipts: i64 = sqlx::query_scalar("SELECT count(*) FROM sync_events")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(receipts, 1);
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn batch_unknown_and_tenant_miss_are_blocked_without_false_ack() {
    let (pool, _) = fixture().await;
    sqlx::query(
        "INSERT INTO products (id,tenant_id) VALUES ('p','tenant-a'),('foreign','tenant-b')",
    )
    .execute(&pool)
    .await
    .unwrap();
    let events = vec![
        event(
            "good",
            "p",
            "product",
            "ToggleSoldOut",
            json!({"is_sold_out":true,"expected_is_sold_out":false}),
        ),
        event("bad", "p", "product", "Unknown", json!({})),
        event(
            "foreign",
            "foreign",
            "product",
            "ToggleSoldOut",
            json!({"is_sold_out":true,"expected_is_sold_out":false}),
        ),
    ];
    let r = sync_events(&pool, "tenant-a", &events).await;
    assert_eq!(r.applied_count, 1);
    assert_eq!(r.failed_count, 2);
    assert_eq!(r.outcomes.len(), 3);
    assert_eq!(r.outcomes[1].id, "bad");
    assert_eq!(r.outcomes[1].status, "blocked");
    assert_eq!(r.outcomes[2].status, "blocked");
    let foreign: bool = sqlx::query_scalar("SELECT is_sold_out FROM products WHERE id='foreign'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!foreign);
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn downstream_failure_rolls_back_claim_and_order() {
    let (pool, _) = fixture().await;
    sqlx::query("INSERT INTO orders (id,tenant_id,status) VALUES ('o','tenant-a','Pending')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("ALTER TABLE department_tasks ADD CONSTRAINT reject_task CHECK (false)")
        .execute(&pool)
        .await
        .unwrap();
    let e = event(
        "e",
        "o",
        "order",
        "UpdateStatus",
        json!({"status":"ReadyForPickup","expected_status":"Pending"}),
    );
    let r = sync_events(&pool, "tenant-a", &[e]).await;
    assert_eq!(r.outcomes[0].status, "blocked");
    let status: String = sqlx::query_scalar("SELECT status FROM orders WHERE id='o'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status, "Pending");
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM sync_events")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn stale_event_and_replay_stay_reconciliation_without_duplicate_alerts() {
    let (pool, _) = fixture().await;
    sqlx::query(
        "INSERT INTO orders (id,tenant_id,status) VALUES ('o','tenant-a','ReadyForPickup')",
    )
    .execute(&pool)
    .await
    .unwrap();
    let e = event(
        "e",
        "o",
        "order",
        "UpdateStatus",
        json!({"status":"Preparing","expected_status":"Pending"}),
    );
    for _ in 0..2 {
        assert_eq!(
            sync_events(&pool, "tenant-a", std::slice::from_ref(&e))
                .await
                .outcomes[0]
                .status,
            "reconciliation"
        );
    }
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM sync_conflict_queue")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn offline_quote_ack_requires_committed_task_and_immutable_replay() {
    let (pool, _) = fixture().await;
    let mutation = OfflineMutation {
        transaction_id: "m".into(),
        timestamp: None,
        product_id: "".into(),
        quantity_deducted: 0,
        amount: None,
        payment_method: None,
        payment_intent_id: None,
        currency: None,
        mutation_type: Some("draft_quote".into()),
        payload: Some("prepare a quote".into()),
        client_mutation_id: Some("m".into()),
    };
    for _ in 0..2 {
        assert_eq!(
            sync_mutations(&pool, "tenant-a", std::slice::from_ref(&mutation))
                .await
                .outcomes[0]
                .status,
            "acknowledged"
        );
    }
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM department_tasks")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
    let marker: i64 = sqlx::query_scalar("SELECT count(*) FROM applied_client_mutations")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(marker, 1);
    let mut changed = mutation;
    changed.payload = Some("changed".into());
    assert_eq!(
        sync_mutations(&pool, "tenant-a", &[changed]).await.outcomes[0].status,
        "reconciliation"
    );
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn commit_failure_never_exposes_acknowledgment_or_cache_invalidation() {
    let (pool, _) = fixture().await;
    sqlx::query("INSERT INTO products (id,tenant_id) VALUES ('p','tenant-a')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql("CREATE FUNCTION reject_receipt_commit() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'injected deferred commit failure'; END $$; CREATE CONSTRAINT TRIGGER reject_receipt AFTER INSERT ON sync_events DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_receipt_commit();").execute(&pool).await.unwrap();
    let e = event(
        "e",
        "p",
        "product",
        "ToggleSoldOut",
        json!({"is_sold_out":true,"expected_is_sold_out":false}),
    );
    let response = sync_events(&pool, "tenant-a", &[e]).await;
    assert_eq!(response.outcomes[0].status, "blocked");
    assert_eq!(response.applied_count, 0);
    assert!(response.committed_products.is_empty());
    let sold: bool = sqlx::query_scalar("SELECT is_sold_out FROM products WHERE id='p'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!sold);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM sync_events")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn appointment_notes_require_frozen_expected_state_and_support_explicit_clear() {
    let (pool, _) = fixture().await;
    use sea_orm::{ConnectionTrait, Schema};
    let orm = sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone());
    let backend = sea_orm::DatabaseBackend::Postgres;
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
    let store = std::sync::Arc::new(server_auth::Store::with_portable_repo(std::sync::Arc::new(
        server_auth::seaorm_store::SeaOrmAuthRepository::new(orm),
    )));
    let user = server_auth::User {
        id: "owner".into(),
        username: "owner".into(),
        email: "owner@example.test".into(),
        password_hash: String::new(),
        roles: vec!["ADMIN".into()],
        active: true,
        organization_id: Some("tenant-a".into()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        oidc_subject: None,
    };
    sqlx::query("INSERT INTO users(id,username,email,password_hash,active,tenant_id,created_at,updated_at)VALUES('owner','owner','owner@example.test','',true,'tenant-a',now(),now())").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position)VALUES('owner','ADMIN','tenant-a',0)").execute(&pool).await.unwrap();
    let token = store.issue_token(&user).unwrap();
    let claims = store.validate_token(&token).await.unwrap();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("authorization", format!("Bearer {token}").parse().unwrap());
    let state = super::super::SyncEventsState {
        pool: pool.clone(),
        access: crate::api::field_ops::records::FieldAccess {
            pool: Some(pool.clone()),
            store,
        },
    };
    sqlx::query("INSERT INTO appointments (id,tenant_id,status,notes) VALUES ('a','tenant-a','Scheduled','original')").execute(&pool).await.unwrap();
    let stale = event(
        "stale",
        "a",
        "appointment",
        "UpdateStatus",
        json!({"status":"Completed","expected_status":"Scheduled","notes":null,"expected_notes":"old"}),
    );
    assert_eq!(
        sync_authorized_events(&state, &claims, &headers, "tenant-a", &[stale])
            .await
            .outcomes[0]
            .status,
        "reconciliation"
    );
    let clear = event(
        "clear",
        "a",
        "appointment",
        "UpdateStatus",
        json!({"status":"Completed","expected_status":"Scheduled","notes":null,"expected_notes":"original"}),
    );
    assert_eq!(
        sync_authorized_events(&state, &claims, &headers, "tenant-a", &[clear])
            .await
            .outcomes[0]
            .status,
        "acknowledged"
    );
    let notes: Option<String> = sqlx::query_scalar("SELECT notes FROM appointments WHERE id='a'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(notes, None);
    let task: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM department_tasks WHERE event_type='job.completed'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(task, 1);
    let audio = event(
        "audio",
        "audio",
        "audio_intent",
        "ProcessVoiceCommand",
        json!({"audio_data":"untranscribed"}),
    );
    assert_eq!(
        sync_events(&pool, "tenant-a", &[audio]).await.outcomes[0].status,
        "blocked"
    );
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn distinct_events_cannot_overwrite_stale_state_but_frozen_next_edit_can_commit() {
    let (pool, _) = fixture().await;
    sqlx::query("INSERT INTO orders (id,tenant_id,status) VALUES ('o','tenant-a','Pending')")
        .execute(&pool)
        .await
        .unwrap();
    let first = event(
        "first",
        "o",
        "order",
        "UpdateStatus",
        json!({"status":"Preparing","expected_status":"Pending"}),
    );
    let stale = event(
        "stale",
        "o",
        "order",
        "UpdateStatus",
        json!({"status":"Cancelled","expected_status":"Pending"}),
    );
    assert_eq!(
        sync_events(&pool, "tenant-a", &[first]).await.outcomes[0].status,
        "acknowledged"
    );
    assert_eq!(
        sync_events(&pool, "tenant-a", &[stale]).await.outcomes[0].status,
        "reconciliation"
    );
    let mut next = event(
        "next",
        "o",
        "order",
        "UpdateStatus",
        json!({"status":"ReadyForPickup","expected_status":"Preparing"}),
    );
    next.base_version = 2;
    let observed: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT updated_at FROM orders WHERE id='o'")
            .fetch_one(&pool)
            .await
            .unwrap();
    next.payload["expected_updated_at"] = json!(observed.to_rfc3339());
    assert_eq!(
        sync_events(&pool, "tenant-a", &[next]).await.outcomes[0].status,
        "acknowledged"
    );
    let version: i64 = sqlx::query_scalar("SELECT max(result_version) FROM sync_events")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(version, 3);
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn operation_intent_replay_is_bound_to_payload_and_legacy_receipts_are_not_upgraded() {
    let (pool, _) = fixture().await;
    let intent = OperationIntent {
        id: "intent".into(),
        action_type: "agent_intent".into(),
        payload: json!({"message":"prepare"}),
        timestamp: None,
    };
    for _ in 0..2 {
        assert_eq!(
            sync_intents(&pool, "tenant-a", std::slice::from_ref(&intent))
                .await
                .outcomes[0]
                .status,
            "acknowledged"
        );
    }
    let mut changed = intent;
    changed.payload = json!({"message":"changed"});
    assert_eq!(
        sync_intents(&pool, "tenant-a", &[changed]).await.outcomes[0].status,
        "reconciliation"
    );
    sqlx::query("INSERT INTO sync_events (id,tenant_id,action_type,payload) VALUES ('legacy','tenant-a','ToggleSoldOut','{}')").execute(&pool).await.unwrap();
    let old = event(
        "legacy",
        "p",
        "product",
        "ToggleSoldOut",
        json!({"is_sold_out":true,"expected_is_sold_out":false}),
    );
    assert_eq!(
        sync_events(&pool, "tenant-a", &[old]).await.outcomes[0].status,
        "reconciliation"
    );
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn forced_rls_scopes_effects_receipts_and_pool_reuse() {
    let (admin, schema) = fixture().await;
    sqlx::query(
        "INSERT INTO products (id,tenant_id) VALUES ('p','tenant-a'),('foreign','tenant-b')",
    )
    .execute(&admin)
    .await
    .unwrap();
    let role = format!("sync_role_{}", uuid::Uuid::new_v4().simple());
    // Disposable test credentials must also work on CI's password-authenticated
    // PostgreSQL host. Never depend on the local cluster using trust auth.
    let password = uuid::Uuid::new_v4().simple().to_string();
    sqlx::raw_sql(&format!("CREATE ROLE {role} LOGIN PASSWORD '{password}' NOSUPERUSER NOBYPASSRLS; GRANT USAGE ON SCHEMA {schema} TO {role}; GRANT ALL ON ALL TABLES IN SCHEMA {schema} TO {role};")).execute(&admin).await.unwrap();
    for table in [
        "sync_events",
        "products",
        "department_tasks",
        "sync_conflict_queue",
    ] {
        sqlx::raw_sql(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY; ALTER TABLE {table} FORCE ROW LEVEL SECURITY; CREATE POLICY sync_scope ON {table} USING (tenant_id=current_setting('app.current_tenant',true)) WITH CHECK (tenant_id=current_setting('app.current_tenant',true));")).execute(&admin).await.unwrap();
    }
    let options: sqlx::postgres::PgConnectOptions = std::env::var("OHC_SYNC_TEST_DATABASE_URL")
        .unwrap()
        .parse()
        .unwrap();
    let restricted = PgPoolOptions::new()
        .max_connections(1)
        .after_connect(move |c, _| {
            let schema = schema.clone();
            Box::pin(async move {
                sqlx::query(&format!("SET search_path TO {schema}"))
                    .execute(c)
                    .await?;
                Ok(())
            })
        })
        .connect_with(options.username(&role).password(&password))
        .await
        .unwrap();
    let own = event(
        "same",
        "p",
        "product",
        "ToggleSoldOut",
        json!({"is_sold_out":true,"expected_is_sold_out":false}),
    );
    for _ in 0..2 {
        assert_eq!(
            sync_events(&restricted, "tenant-a", std::slice::from_ref(&own))
                .await
                .outcomes[0]
                .status,
            "acknowledged"
        );
    }
    let foreign = event(
        "foreign",
        "foreign",
        "product",
        "ToggleSoldOut",
        json!({"is_sold_out":true,"expected_is_sold_out":false}),
    );
    assert_eq!(
        sync_events(&restricted, "tenant-a", std::slice::from_ref(&foreign))
            .await
            .outcomes[0]
            .status,
        "blocked"
    );
    assert_eq!(
        sync_events(&restricted, "tenant-b", &[foreign])
            .await
            .outcomes[0]
            .status,
        "acknowledged"
    );
    let scope: Option<String> =
        sqlx::query_scalar("SELECT current_setting('app.current_tenant',true)")
            .fetch_one(&restricted)
            .await
            .unwrap();
    assert!(scope.is_none_or(|s| s.is_empty()));
    let visible: i64 = sqlx::query_scalar("SELECT count(*) FROM sync_events")
        .fetch_one(&restricted)
        .await
        .unwrap();
    assert_eq!(visible, 0);
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn stale_version_cannot_pass_after_state_changes_away_and_back() {
    let (pool, _) = fixture().await;
    sqlx::query("INSERT INTO products(id,tenant_id) VALUES ('p','tenant-a')")
        .execute(&pool)
        .await
        .unwrap();
    let first = event(
        "first",
        "p",
        "product",
        "ToggleSoldOut",
        json!({"is_sold_out":true,"expected_is_sold_out":false}),
    );
    assert_eq!(
        sync_events(&pool, "tenant-a", &[first]).await.outcomes[0].status,
        "acknowledged"
    );
    let mut back = event(
        "back",
        "p",
        "product",
        "ToggleSoldOut",
        json!({"is_sold_out":false,"expected_is_sold_out":true}),
    );
    back.base_version = 2;
    let observed: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT updated_at FROM products WHERE id='p'")
            .fetch_one(&pool)
            .await
            .unwrap();
    back.payload["expected_updated_at"] = json!(observed.to_rfc3339());
    assert_eq!(
        sync_events(&pool, "tenant-a", &[back]).await.outcomes[0].status,
        "acknowledged"
    );
    let stale = event(
        "stale",
        "p",
        "product",
        "ToggleSoldOut",
        json!({"is_sold_out":true,"expected_is_sold_out":false}),
    );
    assert_eq!(
        sync_events(&pool, "tenant-a", &[stale]).await.outcomes[0].status,
        "reconciliation"
    );
    let sold: bool = sqlx::query_scalar("SELECT is_sold_out FROM products WHERE id='p'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!sold);
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn unknown_version_requires_frozen_server_timestamp() {
    let (pool, _) = fixture().await;
    sqlx::query("INSERT INTO products(id,tenant_id) VALUES ('p','tenant-a')")
        .execute(&pool)
        .await
        .unwrap();
    let mut unknown = event(
        "unknown",
        "p",
        "product",
        "ToggleSoldOut",
        json!({"is_sold_out":true,"expected_is_sold_out":false}),
    );
    unknown.base_version = 0;
    unknown
        .payload
        .as_object_mut()
        .unwrap()
        .remove("expected_updated_at");
    assert_eq!(
        sync_events(&pool, "tenant-a", &[unknown]).await.outcomes[0].status,
        "reconciliation"
    );
    let observed: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT updated_at FROM products WHERE id='p'")
            .fetch_one(&pool)
            .await
            .unwrap();
    let mut known = event(
        "observed",
        "p",
        "product",
        "ToggleSoldOut",
        json!({"is_sold_out":true,"expected_is_sold_out":false,"expected_updated_at":observed.to_rfc3339()}),
    );
    known.base_version = 0;
    let response = sync_events(&pool, "tenant-a", std::slice::from_ref(&known)).await;
    assert_eq!(response.outcomes[0].status, "acknowledged");
    assert_eq!(response.outcomes[0].result_version, Some(2));
    let replay = sync_events(&pool, "tenant-a", &[known]).await;
    assert_eq!(replay.outcomes[0].result_version, Some(2));
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn non_sync_aba_updates_advance_observed_token_even_without_timestamp_assignment() {
    let (pool, _) = fixture().await;
    sqlx::query("INSERT INTO products(id,tenant_id) VALUES ('p','tenant-a')")
        .execute(&pool)
        .await
        .unwrap();
    let stale = event(
        "stale",
        "p",
        "product",
        "ToggleSoldOut",
        json!({"is_sold_out":true,"expected_is_sold_out":false}),
    );
    sqlx::query("UPDATE products SET is_sold_out=true WHERE id='p'")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("UPDATE products SET is_sold_out=false WHERE id='p'")
        .execute(&pool)
        .await
        .unwrap();
    let response = sync_events(&pool, "tenant-a", &[stale]).await;
    assert_eq!(response.outcomes[0].status, "reconciliation");
    let sold: bool = sqlx::query_scalar("SELECT is_sold_out FROM products WHERE id='p'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!sold);
}

#[tokio::test]
#[ignore = "requires OHC_SYNC_TEST_DATABASE_URL"]
async fn observed_token_advances_even_when_clock_is_behind_previous_value() {
    let (pool, _) = fixture().await;
    sqlx::query("INSERT INTO products(id,tenant_id,updated_at) VALUES ('p','tenant-a','2099-01-01T00:00:00Z')").execute(&pool).await.unwrap();
    sqlx::query("UPDATE products SET is_sold_out=true WHERE id='p'")
        .execute(&pool)
        .await
        .unwrap();
    let advanced: bool = sqlx::query_scalar(
        "SELECT updated_at>'2099-01-01T00:00:00Z'::timestamptz FROM products WHERE id='p'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(advanced);
}
