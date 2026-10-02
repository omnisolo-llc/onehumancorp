use super::*;
use sqlx::PgPool;
use std::sync::atomic::Ordering;
struct Fixture {
    agent: Arc<OnboardingAgent>,
    admin: PgPool,
    schema: String,
    application: String,
}
impl Fixture {
    async fn new() -> Self {
        let url =
            std::env::var("OHC_DRAFT_TEST_DATABASE_URL").expect("isolated PostgreSQL required");
        let admin = PgPool::connect(&url).await.unwrap();
        let schema = format!("draft_merge_{}", uuid::Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        let search = schema.clone();
        let application = schema.clone();
        let options = url
            .parse::<sqlx::postgres::PgConnectOptions>()
            .unwrap()
            .application_name(&application);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(8)
            .after_connect(move |connection, _| {
                let command = format!("SET search_path TO {search}");
                Box::pin(async move {
                    sqlx::query(&command).execute(connection).await?;
                    Ok(())
                })
            })
            .connect_with(options)
            .await
            .unwrap();
        let migration = include_str!("../../src/server/migrations/002_missing_tables.sql");
        let start = migration
            .find("CREATE TABLE IF NOT EXISTS onboarding_state (")
            .unwrap();
        let end = migration[start..].find("\n);").unwrap() + start + 3;
        sqlx::raw_sql(&migration[start..end])
            .execute(&pool)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!(
            "../../src/server/migrations/235_onboarding_preparation_receipt.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();
        INVALIDATIONS.store(0, Ordering::SeqCst);
        Self {
            agent: Arc::new(OnboardingAgent {
                hub: Arc::new(Hub { pool }),
            }),
            admin,
            schema,
            application,
        }
    }
    async fn stored(&self, tenant: &str, user: &str) -> (serde_json::Value, i32) {
        sqlx::query_as("SELECT state_json,current_step FROM onboarding_state WHERE tenant_id=$1 AND user_id=$2").bind(tenant).bind(user).fetch_one(&self.agent.hub.pool).await.unwrap()
    }
    async fn finish(self) {
        self.agent.hub.pool.close().await;
        sqlx::query(&format!("DROP SCHEMA {} CASCADE", self.schema))
            .execute(&self.admin)
            .await
            .unwrap();
        self.admin.close().await;
    }
}
#[tokio::test]
async fn concurrent_first_saves_preserve_both_permitted_fields() {
    let f = Fixture::new().await;
    let key = uuid::Uuid::new_v4().as_u128() as i64;
    sqlx::raw_sql(&format!("CREATE FUNCTION park_insert() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN PERFORM pg_advisory_xact_lock({key}); RETURN NEW; END$$;CREATE TRIGGER park_insert BEFORE INSERT ON onboarding_state FOR EACH ROW EXECUTE FUNCTION park_insert();")).execute(&f.agent.hub.pool).await.unwrap();
    let mut gate = f.admin.acquire().await.unwrap();
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(key)
        .execute(&mut *gate)
        .await
        .unwrap();
    let mut jobs = tokio::task::JoinSet::new();
    for (step, payload) in [
        (2, json!({"businessName":"Reviewed name"})),
        (4, json!({"location":"Reviewed location"})),
    ] {
        let agent = f.agent.clone();
        jobs.spawn(async move {
            agent
                .save_onboarding_state("tenant-a", "user-a", step, &payload)
                .await
        });
    }
    // Both actual INSERTs must reach the database barrier before releasing either.
    let reached=tokio::time::timeout(std::time::Duration::from_secs(5),async{loop{let waiting:i64=sqlx::query_scalar("SELECT count(*) FROM pg_stat_activity WHERE application_name=$1 AND wait_event='advisory'").bind(&f.application).fetch_one(&f.admin).await.unwrap();if waiting>=2{break}tokio::time::sleep(std::time::Duration::from_millis(5)).await;}}).await;
    sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(key)
        .execute(&mut *gate)
        .await
        .unwrap();
    drop(gate);
    assert!(
        reached.is_ok(),
        "both first writes must reach the real database barrier"
    );
    while let Some(result) = jobs.join_next().await {
        result.unwrap().unwrap();
    }
    let (state, step) = f.stored("tenant-a", "user-a").await;
    assert_eq!(
        state["businessName"], "Reviewed name",
        "the second first-write must merge the first committed field"
    );
    assert_eq!(state["location"], "Reviewed location");
    assert_eq!(step, 4);
    f.finish().await;
}
#[tokio::test]
async fn failed_first_write_leaves_no_placeholder_or_success_invalidation() {
    let f = Fixture::new().await;
    sqlx::raw_sql("CREATE FUNCTION reject_payload() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN IF NEW.state_json->>'businessName'='FAIL' THEN RAISE EXCEPTION 'forced draft failure';END IF;RETURN NEW;END$$;CREATE TRIGGER reject_payload BEFORE INSERT OR UPDATE ON onboarding_state FOR EACH ROW EXECUTE FUNCTION reject_payload();").execute(&f.agent.hub.pool).await.unwrap();
    assert!(
        f.agent
            .save_onboarding_state("tenant-a", "user-a", 2, &json!({"businessName":"FAIL"}))
            .await
            .is_err()
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM onboarding_state")
        .fetch_one(&f.agent.hub.pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    assert_eq!(INVALIDATIONS.load(Ordering::SeqCst), 0);
    f.finish().await;
}
#[tokio::test]
async fn existing_shallow_merge_and_step_semantics_are_preserved() {
    let f = Fixture::new().await;
    f.agent.save_onboarding_system_state("tenant-a","user-a",4,&json!({"status":"prepared","businessName":"Old","wizardState":{"businessName":"Nested old","location":"Old nested location"}})).await.unwrap();
    f.agent.save_onboarding_state("tenant-a","user-a",1,&json!({"location":"New","wizardState":{"businessName":"Nested new"},"status":"forged","apiKey":"do-not-store"})).await.unwrap();
    let (state, step) = f.stored("tenant-a", "user-a").await;
    assert_eq!(step, 4);
    assert_eq!(state["businessName"], "Old");
    assert_eq!(state["location"], "New");
    assert_eq!(state["status"], "prepared");
    assert_eq!(state["wizardState"], json!({"businessName":"Nested new"}));
    assert!(state.get("apiKey").is_none());
    f.finish().await;
}

#[tokio::test]
async fn concurrent_saves_keep_the_verified_tenant_and_user_keys_separate() {
    let f = Fixture::new().await;
    let mut jobs = tokio::task::JoinSet::new();
    for (tenant, user, label) in [
        ("tenant-a", "user-a", "A"),
        ("tenant-a", "user-b", "B"),
        ("tenant-b", "user-a", "C"),
    ] {
        let agent = f.agent.clone();
        jobs.spawn(async move{agent.save_onboarding_state(tenant,user,2,&json!({"businessName":label,"tenant_id":"forged-tenant","user_id":"forged-user"})).await});
    }
    while let Some(result) = jobs.join_next().await {
        result.unwrap().unwrap();
    }
    for (tenant, user, label) in [
        ("tenant-a", "user-a", "A"),
        ("tenant-a", "user-b", "B"),
        ("tenant-b", "user-a", "C"),
    ] {
        let (state, _) = f.stored(tenant, user).await;
        assert_eq!(state["businessName"], label);
        assert!(state.get("tenant_id").is_none());
        assert!(state.get("user_id").is_none());
    }
    let rows: i64 = sqlx::query_scalar("SELECT count(*) FROM onboarding_state")
        .fetch_one(&f.agent.hub.pool)
        .await
        .unwrap();
    assert_eq!(rows, 3);
    f.finish().await;
}
#[tokio::test]
async fn user_draft_cannot_replace_system_flags_or_the_separate_protected_receipt() {
    let f = Fixture::new().await;
    f.agent
        .save_onboarding_system_state(
            "tenant-a",
            "user-a",
            5,
            &json!({"status":"prepared","businessName":"Original"}),
        )
        .await
        .unwrap();
    // Opaque storage sentinel only; this is not a valid launch preparation.
    let receipt = json!({"protected_storage_nonce":"fixture-original"});
    sqlx::query("UPDATE onboarding_state SET preparation_receipt=$1")
        .bind(&receipt)
        .execute(&f.agent.hub.pool)
        .await
        .unwrap();
    f.agent.save_onboarding_state("tenant-a","user-a",1,&json!({"businessName":"Reviewed","status":"launched","preparation":{"forged":true},"preparation_receipt":{"forged":true},"api_key":"discard-me"})).await.unwrap();
    let (state, step) = f.stored("tenant-a", "user-a").await;
    assert_eq!(step, 5);
    assert_eq!(state["status"], "prepared");
    assert_eq!(state["businessName"], "Reviewed");
    assert!(state.get("preparation").is_none());
    assert!(state.get("api_key").is_none());
    let (stored, version): (serde_json::Value, i32) =
        sqlx::query_as("SELECT preparation_receipt,version FROM onboarding_state")
            .fetch_one(&f.agent.hub.pool)
            .await
            .unwrap();
    assert_eq!(stored, receipt);
    assert_eq!(
        version, 1,
        "this fix does not invent a draft revision contract"
    );
    f.finish().await;
}
#[tokio::test]
async fn rejected_first_save_can_retry_without_a_partial_step_or_field() {
    let f = Fixture::new().await;
    sqlx::raw_sql("CREATE FUNCTION reject_payload() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN IF NEW.state_json->>'businessName'='FAIL' THEN RAISE EXCEPTION 'forced draft failure';END IF;RETURN NEW;END$$;CREATE TRIGGER reject_payload BEFORE INSERT OR UPDATE ON onboarding_state FOR EACH ROW EXECUTE FUNCTION reject_payload();").execute(&f.agent.hub.pool).await.unwrap();
    assert!(
        f.agent
            .save_onboarding_state(
                "tenant-a",
                "user-a",
                9,
                &json!({"businessName":"FAIL","location":"Must not survive"})
            )
            .await
            .is_err()
    );
    f.agent
        .save_onboarding_state("tenant-a", "user-a", 1, &json!({"businessName":"Retried"}))
        .await
        .unwrap();
    let (state, step) = f.stored("tenant-a", "user-a").await;
    assert_eq!(step, 1);
    assert_eq!(state, json!({"businessName":"Retried"}));
    assert_eq!(INVALIDATIONS.load(Ordering::SeqCst), 2);
    f.finish().await;
}
