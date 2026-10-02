//! Prototype authority proof: real restricted PostgreSQL, no provider/runtime.
use super::PgFixture;
use sqlx::{Executor, Postgres, Transaction};
const SQL: &str = include_str!("../../src/server/persistence/agent_definition_authority_pg.sql");
const GATE: i64 = 57129048260865031;
async fn fixture() -> PgFixture {
    let pg = PgFixture::new().await;
    let mut c = pg.admin.acquire().await.unwrap();
    sqlx::raw_sql("INSERT INTO tenants(id,name) VALUES('a','a'),('b','b'); INSERT INTO users(id,tenant_id,username,email) VALUES('u','a','original','u@example.test'),('v','b','foreign','v@example.test'); INSERT INTO identity_user_roles VALUES('u','OWNER','a',0);").execute(&mut *c).await.unwrap();
    drop(c);
    pg
}
async fn tx(pg: &PgFixture, tenant: &str) -> Transaction<'static, Postgres> {
    let mut tx = pg.pool.begin().await.unwrap();
    server_common::auth_utils::set_org_context(&mut *tx, tenant)
        .await
        .unwrap();
    tx
}
async fn key(pg: &PgFixture, user: &str, tenant: &str) -> String {
    let mut t = tx(pg, tenant).await;
    sqlx::query_scalar("SELECT marketplace_authority_key FROM users WHERE id=$1")
        .bind(user)
        .fetch_one(&mut *t)
        .await
        .unwrap()
}
async fn eligible(pg: &PgFixture, key: &str) -> bool {
    sqlx::query_scalar("SELECT eligible FROM agent_definition_authorities WHERE authority_key=$1")
        .bind(key)
        .fetch_one(&pg.pool)
        .await
        .unwrap()
}
#[tokio::test]
async fn restricted_authority_projection_denies_direct_forgery_and_allows_fk_cascades() {
    let pg = fixture().await;
    let a = key(&pg, "u", "a").await;
    let b = key(&pg, "v", "b").await;
    assert!(eligible(&pg, &a).await);
    assert!(!eligible(&pg, &b).await);
    for sql in [
        format!("INSERT INTO agent_definition_authorities VALUES('{a}',FALSE)"),
        "INSERT INTO agent_definition_authorities VALUES('unbound',TRUE)".to_string(),
        format!("UPDATE agent_definition_authorities SET eligible=TRUE WHERE authority_key='{b}'"),
        format!("DELETE FROM agent_definition_authorities WHERE authority_key='{a}'"),
        format!("UPDATE users SET marketplace_authority_key='{a}' WHERE id='v'"),
        "UPDATE users SET marketplace_eligible=TRUE,username='forged' WHERE id='v'".into(),
    ] {
        let mut t = tx(&pg, "b").await;
        assert!(
            sqlx::query(&sql).execute(&mut *t).await.is_err(),
            "forgery admitted: {sql}"
        );
        t.rollback().await.unwrap();
    }
    // A caller-created temp role table must not shadow the pinned trigger schema.
    let mut t = tx(&pg, "b").await;
    t.execute("CREATE TEMP TABLE identity_user_roles(user_id TEXT,role_name TEXT,tenant_id TEXT); INSERT INTO identity_user_roles VALUES('v','OWNER','b'); UPDATE users SET username='legitimate' WHERE id='v'").await.unwrap();
    t.commit().await.unwrap();
    assert!(!eligible(&pg, &b).await);
    let mut t = tx(&pg, "a").await;
    sqlx::query("UPDATE users SET active=FALSE WHERE id='u'")
        .execute(&mut *t)
        .await
        .unwrap();
    t.commit().await.unwrap();
    assert!(!eligible(&pg, &a).await);
    let mut t = tx(&pg, "a").await;
    sqlx::query("UPDATE users SET active=TRUE WHERE id='u'")
        .execute(&mut *t)
        .await
        .unwrap();
    t.commit().await.unwrap();
    assert!(eligible(&pg, &a).await);
    pg.close().await;
}
#[tokio::test]
async fn canonical_role_changes_rollback_and_idempotent_bootstrap_preserve_authority() {
    let pg = fixture().await;
    let a = key(&pg, "u", "a").await;
    let mut t = tx(&pg, "a").await;
    sqlx::query("DELETE FROM identity_user_roles WHERE user_id='u'")
        .execute(&mut *t)
        .await
        .unwrap();
    let within: bool = sqlx::query_scalar(
        "SELECT eligible FROM agent_definition_authorities WHERE authority_key=$1",
    )
    .bind(&a)
    .fetch_one(&mut *t)
    .await
    .unwrap();
    assert!(!within);
    t.rollback().await.unwrap();
    assert!(eligible(&pg, &a).await);
    let mut t = tx(&pg, "a").await;
    t.execute("DELETE FROM identity_user_roles WHERE user_id='u'; INSERT INTO identity_user_roles VALUES('u','aDmIn','a',0)").await.unwrap();
    t.commit().await.unwrap();
    assert!(eligible(&pg, &a).await);
    let mut t = tx(&pg, "a").await;
    t.execute("UPDATE identity_user_roles SET role_name='ADMİN' WHERE user_id='u'; UPDATE users SET roles=ARRAY['ADMIN'],username='still revoked' WHERE id='u'").await.unwrap();
    t.commit().await.unwrap();
    assert!(!eligible(&pg, &a).await);
    let mut c = pg.admin.acquire().await.unwrap();
    c.execute(format!("SET search_path={}", pg.schema).as_str())
        .await
        .unwrap();
    sqlx::raw_sql(SQL).execute(&mut *c).await.unwrap();
    drop(c);
    assert_eq!(key(&pg, "u", "a").await, a);
    assert!(!eligible(&pg, &a).await);
    pg.close().await;
}
#[tokio::test]
async fn deletion_withdraws_projection_and_recreated_user_cannot_adopt_stale_key() {
    let pg = fixture().await;
    let old = key(&pg, "u", "a").await;
    let mut t = tx(&pg, "a").await;
    t.execute("DELETE FROM users WHERE id='u'").await.unwrap();
    t.commit().await.unwrap();
    let n: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM agent_definition_authorities WHERE authority_key=$1",
    )
    .bind(&old)
    .fetch_one(&pg.pool)
    .await
    .unwrap();
    assert_eq!(n, 0);
    let mut t = tx(&pg, "a").await;
    t.execute("INSERT INTO users(id,tenant_id,username,email) VALUES('u','a','recreated','u@example.test')")
        .await
        .unwrap();
    t.commit().await.unwrap();
    let new = key(&pg, "u", "a").await;
    assert_ne!(old, new);
    assert!(!eligible(&pg, &new).await);
    let mut t = tx(&pg, "a").await;
    assert!(
        sqlx::query("UPDATE users SET marketplace_authority_key=$1 WHERE id='u'")
            .bind(old)
            .execute(&mut *t)
            .await
            .is_err()
    );
    t.rollback().await.unwrap();
    pg.close().await;
}
#[tokio::test]
async fn shared_admission_gate_orders_committed_role_revocation_both_ways() {
    let pg = fixture().await;
    let key = key(&pg, "u", "a").await;
    let mut admission = tx(&pg, "b").await;
    sqlx::query("SELECT pg_advisory_xact_lock_shared($1)")
        .bind(GATE)
        .execute(&mut *admission)
        .await
        .unwrap();
    let before: bool = sqlx::query_scalar(
        "SELECT eligible FROM agent_definition_authorities WHERE authority_key=$1",
    )
    .bind(&key)
    .fetch_one(&mut *admission)
    .await
    .unwrap();
    assert!(before);
    let pool = pg.pool.clone();
    let revocation = tokio::spawn(async move {
        let mut t = pool.begin().await.unwrap();
        server_common::auth_utils::set_org_context(&mut *t, "a")
            .await
            .unwrap();
        t.execute("DELETE FROM identity_user_roles WHERE user_id='u'")
            .await
            .unwrap();
        t.commit().await.unwrap();
    });
    tokio::time::timeout(std::time::Duration::from_secs(5),async {loop{let waiting:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_locks l JOIN pg_stat_activity a ON a.pid=l.pid WHERE a.usename=$1 AND l.locktype='advisory' AND NOT l.granted)").bind(&pg.role).fetch_one(&pg.admin).await.unwrap(); if waiting {break} tokio::task::yield_now().await;}}).await.unwrap();
    assert!(!revocation.is_finished());
    admission.commit().await.unwrap();
    revocation.await.unwrap();
    assert!(!eligible(&pg, &key).await);
    let mut admission = tx(&pg, "b").await;
    sqlx::query("SELECT pg_advisory_xact_lock_shared($1)")
        .bind(GATE)
        .execute(&mut *admission)
        .await
        .unwrap();
    let after: bool = sqlx::query_scalar(
        "SELECT eligible FROM agent_definition_authorities WHERE authority_key=$1",
    )
    .bind(&key)
    .fetch_one(&mut *admission)
    .await
    .unwrap();
    assert!(!after);
    admission.rollback().await.unwrap();
    pg.close().await;
}
#[tokio::test]
async fn preexisting_user_lock_can_deadlock_and_is_explicitly_rejected() {
    let pg = fixture().await;
    let mut writer = tx(&pg, "a").await;
    writer
        .execute("SELECT id FROM users WHERE id='u' FOR UPDATE")
        .await
        .unwrap();
    let mut admission = tx(&pg, "a").await;
    sqlx::query("SELECT pg_advisory_xact_lock_shared($1)")
        .bind(GATE)
        .execute(&mut *admission)
        .await
        .unwrap();
    let wait_row = tokio::spawn(async move {
        let result = admission
            .execute("SELECT id FROM users WHERE id='u' FOR SHARE")
            .await;
        admission.rollback().await.unwrap();
        result.map(|_| ()).map_err(|e| {
            e.as_database_error()
                .and_then(|e| e.code())
                .map(|s| s.into_owned())
        })
    });
    tokio::time::timeout(std::time::Duration::from_secs(5),async{loop{let waiting:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE usename=$1 AND wait_event_type='Lock' AND query LIKE 'SELECT id FROM users%')").bind(&pg.role).fetch_one(&pg.admin).await.unwrap(); if waiting{break} tokio::task::yield_now().await;}}).await.unwrap();
    let write = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        writer.execute("UPDATE users SET username='changed' WHERE id='u'"),
    )
    .await
    .unwrap();
    writer.rollback().await.unwrap();
    let read = wait_row.await.unwrap();
    let write = write.map(|_| ()).map_err(|e| {
        e.as_database_error()
            .and_then(|e| e.code())
            .map(|s| s.into_owned())
    });
    assert_eq!(
        [write, read]
            .iter()
            .filter(|v| matches!(v,Err(Some(code)) if code=="40P01"))
            .count(),
        1
    );
    pg.close().await;
}
