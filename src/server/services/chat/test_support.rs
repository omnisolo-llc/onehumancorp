//! Real, isolated PostgreSQL prerequisites for the native chat service tests.
use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::path::PathBuf;
use uuid::Uuid;

pub(super) fn test_database_url(raw: Option<&str>) -> Result<PgConnectOptions, &'static str> {
    const ERROR: &str = "OHC_CHAT_TEST_DATABASE_URL must select an explicit loopback PostgreSQL ohc_*_test database without URL options";
    let raw = raw.ok_or(ERROR)?;
    if raw.chars().any(char::is_control) {
        return Err(ERROR);
    }
    let parsed = url::Url::parse(raw).map_err(|_| ERROR)?;
    let host = parsed
        .host_str()
        .ok_or(ERROR)?
        .trim_start_matches('[')
        .trim_end_matches(']');
    let ip: std::net::IpAddr = host.parse().map_err(|_| ERROR)?;
    let name = parsed.path().strip_prefix('/').ok_or(ERROR)?;
    if !matches!(parsed.scheme(), "postgres" | "postgresql")
        || !ip.is_loopback()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
        || !name.starts_with("ohc_")
        || !name.ends_with("_test")
        || name.len() <= 9
        || name.len() > 63
        || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        return Err(ERROR);
    }
    raw.parse().map_err(|_| ERROR)
}

fn repository_root() -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|path| path.join("src/server/services/chat/service.rs").is_file())
        .expect("chat tests must use an actual repository checkout")
        .to_path_buf()
}

pub(super) async fn fixture_migrator(content_type: bool) -> sqlx::migrate::Migrator {
    let source = sqlx::migrate::Migrator::new(repository_root().join("src/server/migrations"))
        .await
        .expect("active migrations must load");
    let versions: &[i64] = if content_type {
        &[233, 1009, 1021, 1024]
    } else {
        &[233, 1009, 1021]
    };
    let migrations = versions
        .iter()
        .map(|version| {
            source
                .iter()
                .find(|m| m.version == *version)
                .unwrap_or_else(|| panic!("chat migration {version} must be in the active source"))
                .clone()
        })
        .collect();
    sqlx::migrate::Migrator {
        migrations: std::borrow::Cow::Owned(migrations),
        ..sqlx::migrate::Migrator::DEFAULT
    }
}

pub(super) struct ChatFixture {
    pub(super) admin: PgPool,
    pub(super) scoped: PgPool,
    schema: String,
    role: String,
}
impl ChatFixture {
    pub(super) async fn new() -> Self {
        Self::with_content_type(true).await
    }
    pub(super) async fn legacy() -> Self {
        Self::with_content_type(false).await
    }
    async fn with_content_type(content_type: bool) -> Self {
        let value = std::env::var("OHC_CHAT_TEST_DATABASE_URL").ok();
        let options = test_database_url(value.as_deref())
            .expect("missing or unsafe disposable chat database prerequisite");
        // Load the actual production source directory, not the historical duplicate tree.
        let migrations = fixture_migrator(content_type).await;
        let schema = format!("chat_case_{}", Uuid::new_v4().simple());
        let role = format!("chat_member_{}", Uuid::new_v4().simple());
        let password = Uuid::new_v4().simple().to_string();
        let admin = PgPoolOptions::new()
            .max_connections(2)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect_with(options.clone().options([("search_path", schema.as_str())]))
            .await
            .expect("owned chat fixture database must be available");
        let encoding: String = sqlx::query_scalar("SHOW server_encoding")
            .fetch_one(&admin)
            .await
            .unwrap();
        assert_eq!(
            encoding, "UTF8",
            "the disposable fixture must preserve Unicode data"
        );
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        migrations
            .run(&admin)
            .await
            .expect("actual chat migrations must execute in SQLx order");
        sqlx::query(&format!("CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE PASSWORD '{password}'")).execute(&admin).await.unwrap();
        sqlx::raw_sql(&format!("GRANT USAGE ON SCHEMA {schema} TO {role}; GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {schema} TO {role};")).execute(&admin).await.unwrap();
        let scoped = PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect_with(
                options
                    .username(&role)
                    .password(&password)
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        let privilege:(String,bool,bool)=sqlx::query_as("SELECT current_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&scoped).await.unwrap();
        assert_eq!(privilege, (role.clone(), false, false));
        Self {
            admin,
            scoped,
            schema,
            role,
        }
    }
    pub(super) async fn count_as(&self, tenant: Uuid, table: &str, id: Uuid) -> i64 {
        assert!(
            [
                "chat_inboxes",
                "chat_channels",
                "chat_contacts",
                "chat_conversations",
                "chat_messages"
            ]
            .contains(&table)
        );
        let mut tx = self.scoped.begin().await.unwrap();
        sqlx::query("SELECT set_config('app.current_tenant_id',$1,true)")
            .bind(tenant.to_string())
            .execute(&mut *tx)
            .await
            .unwrap();
        let count = sqlx::query_scalar(&format!("SELECT COUNT(*) FROM {table} WHERE id=$1"))
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .expect("RLS assertion query must succeed; SQL errors are not zero rows");
        tx.commit().await.unwrap();
        count
    }
    pub(super) async fn finish(self) {
        self.scoped.close().await;
        sqlx::query(&format!("DROP SCHEMA {} CASCADE", self.schema))
            .execute(&self.admin)
            .await
            .unwrap();
        sqlx::query(&format!("DROP ROLE {}", self.role))
            .execute(&self.admin)
            .await
            .unwrap();
        self.admin.close().await;
    }
}
