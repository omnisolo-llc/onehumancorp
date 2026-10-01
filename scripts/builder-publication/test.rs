use super::*;
use crate::boundaries::edge::{BINDING, OBSERVED};

struct Fixture {
    admin: PgPool,
    pool: PgPool,
    schema: String,
    role: String,
    tenant: Uuid,
    site: Uuid,
}
impl Fixture {
    async fn new() -> Self {
        let url = std::env::var("OHC_BUILDER_TEST_DATABASE_URL")
            .expect("explicit disposable PostgreSQL required");
        let admin = PgPool::connect(&url).await.unwrap();
        let schema = format!("builder_publication_{}", Uuid::new_v4().simple());
        let role = format!("builder_role_{}", Uuid::new_v4().simple());
        let password = Uuid::new_v4().simple().to_string();
        sqlx::raw_sql(&format!("CREATE SCHEMA {schema};CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS PASSWORD '{password}';"))
            .execute(&admin).await.unwrap();
        let scoped_admin = sqlx::postgres::PgPoolOptions::new()
            .after_connect({
                let schema = schema.clone();
                move |conn, _| {
                    let q = format!("SET search_path TO {schema}");
                    Box::pin(async move {
                        sqlx::query(&q).execute(conn).await?;
                        Ok(())
                    })
                }
            })
            .connect(&url)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!("../../src/server/migrations/009_builder.sql"))
            .execute(&scoped_admin)
            .await
            .unwrap();
        let migration =
            include_str!("../../src/server/migrations/229_quotes_and_builder_parity.sql");
        let add_column = migration
            .lines()
            .find(|line| line.starts_with("ALTER TABLE builder_sites ADD COLUMN"))
            .unwrap();
        sqlx::raw_sql(add_column)
            .execute(&scoped_admin)
            .await
            .unwrap();
        sqlx::raw_sql(&format!("GRANT USAGE ON SCHEMA {schema} TO {role};GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {schema} TO {role};"))
            .execute(&admin).await.unwrap();
        let tenant = Uuid::new_v4();
        let site = Uuid::new_v4();
        sqlx::query("INSERT INTO builder_sites(id,tenant_id) VALUES($1,$2)")
            .bind(site)
            .bind(tenant)
            .execute(&scoped_admin)
            .await
            .unwrap();
        sqlx::query("INSERT INTO builder_pages(id,tenant_id,site_id,path,title,seo_metadata) VALUES($1,$2,$3,'/','Owner page','{\"name\":\"Owner title\"}')")
            .bind(Uuid::new_v4()).bind(tenant).bind(site).execute(&scoped_admin).await.unwrap();
        let options = url
            .parse::<sqlx::postgres::PgConnectOptions>()
            .unwrap()
            .username(&role)
            .password(&password);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .after_connect({
                let schema = schema.clone();
                move |conn, _| {
                    let q = format!("SET search_path TO {schema}");
                    Box::pin(async move {
                        sqlx::query(&q).execute(conn).await?;
                        Ok(())
                    })
                }
            })
            .connect_with(options)
            .await
            .unwrap();
        let identities: (String, String, bool, bool) = sqlx::query_as("SELECT current_user::text,session_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&pool).await.unwrap();
        assert_eq!(identities, (role.clone(), role.clone(), false, false));
        OBSERVED.lock().unwrap().clear();
        *BINDING.lock().unwrap() = Some((scoped_admin.clone(), site));
        admin.close().await;
        Self {
            admin: scoped_admin,
            pool,
            schema,
            role,
            tenant,
            site,
        }
    }
    async fn published(&self) -> bool {
        sqlx::query_scalar("SELECT published_at IS NOT NULL FROM builder_sites WHERE id=$1")
            .bind(self.site)
            .fetch_one(&self.admin)
            .await
            .unwrap()
    }
    async fn finish(self) {
        *BINDING.lock().unwrap() = None;
        self.pool.close().await;
        sqlx::raw_sql(&format!(
            "DROP SCHEMA {} CASCADE;DROP ROLE {};",
            self.schema, self.role
        ))
        .execute(&self.admin)
        .await
        .unwrap();
        self.admin.close().await;
    }
}

#[tokio::test]
async fn publication_commits_before_cache_invalidation() {
    let f = Fixture::new().await;
    let mut listener = sqlx::postgres::PgListener::connect_with(&f.admin)
        .await
        .unwrap();
    listener.listen("edge_cache_invalidation").await.unwrap();
    let result = execute_publish_site_job(&f.pool, f.tenant, f.site).await;
    let committed = f.published().await;
    let observed = OBSERVED.lock().unwrap().clone();
    let notification =
        tokio::time::timeout(std::time::Duration::from_secs(2), listener.recv()).await;
    let expected_key = format!("edge_site_{}_{}", f.tenant, f.site);
    drop(listener);
    f.finish().await;
    assert!(result.is_ok(), "{result:?}");
    assert!(
        committed,
        "successful publication must persist its published_at marker"
    );
    assert_eq!(
        observed,
        vec![true],
        "cache effects must observe the committed site"
    );
    assert_eq!(notification.unwrap().unwrap().payload(), expected_key);
}

#[tokio::test]
async fn failed_commit_never_invalidates_cache_or_reports_success() {
    let f = Fixture::new().await;
    sqlx::raw_sql("CREATE FUNCTION reject_publication() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'fixture deferred commit rejection'; END $$; CREATE CONSTRAINT TRIGGER reject_publication AFTER UPDATE ON builder_sites DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_publication();")
        .execute(&f.admin).await.unwrap();
    let mut listener = sqlx::postgres::PgListener::connect_with(&f.admin)
        .await
        .unwrap();
    listener.listen("edge_cache_invalidation").await.unwrap();
    let result = execute_publish_site_job(&f.pool, f.tenant, f.site).await;
    let committed = f.published().await;
    let observed = OBSERVED.lock().unwrap().clone();
    let notification =
        tokio::time::timeout(std::time::Duration::from_millis(100), listener.recv()).await;
    drop(listener);
    f.finish().await;
    assert!(
        result.is_err(),
        "deferred commit rejection cannot report publication success"
    );
    assert!(!committed);
    assert!(
        observed.is_empty(),
        "rollback must not invalidate edge cache"
    );
    assert!(
        notification.is_err(),
        "rollback must not deliver PostgreSQL invalidation"
    );
}

#[tokio::test]
async fn foreign_site_cannot_publish_or_invalidate() {
    let f = Fixture::new().await;
    let result = execute_publish_site_job(&f.pool, Uuid::new_v4(), f.site).await;
    let committed = f.published().await;
    let observed = OBSERVED.lock().unwrap().clone();
    f.finish().await;
    assert!(
        result.is_err(),
        "zero owned rows cannot report publication success"
    );
    assert!(!committed);
    assert!(observed.is_empty());
}
