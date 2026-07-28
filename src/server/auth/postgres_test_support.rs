use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tokio::sync::OnceCell;

static POSTGRES_SETUP: OnceCell<Result<(), String>> = OnceCell::const_new();
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../migrations");

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PostgresTestDecision {
    Run(String),
    Skip(String),
}

pub(crate) fn decide_postgres_test(
    database_url: Option<&str>,
    require_postgres: Option<&str>,
) -> Result<PostgresTestDecision, String> {
    let unavailable_reason = match database_url {
        None | Some("") => Some("OHC_DATABASE_URL is not set"),
        Some(url) if !url.starts_with("postgres://") && !url.starts_with("postgresql://") => {
            Some("OHC_DATABASE_URL is not a PostgreSQL URL")
        }
        Some(_) => None,
    };

    if let Some(reason) = unavailable_reason {
        return if require_postgres == Some("1") {
            Err(format!(
                "OHC_REQUIRE_POSTGRES_TESTS=1 requires PostgreSQL security tests to execute: {reason}"
            ))
        } else {
            Ok(PostgresTestDecision::Skip(reason.to_string()))
        };
    }

    Ok(PostgresTestDecision::Run(database_url.unwrap().to_string()))
}

async fn initialize_postgres(admin_url: &str) -> Result<(), String> {
    let admin_pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(10))
        .connect(admin_url)
        .await
        .map_err(|error| format!("connect to OHC_POSTGRES_ADMIN_URL: {error}"))?;

    sqlx::query("CREATE EXTENSION IF NOT EXISTS vector")
        .execute(&admin_pool)
        .await
        .map_err(|error| format!("create vector extension: {error}"))?;
    sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"")
        .execute(&admin_pool)
        .await
        .map_err(|error| format!("create uuid-ossp extension: {error}"))?;

    // To prevent concurrent test suites from running migrations simultaneously
    // we use a PostgreSQL advisory lock on a fixed unique ID
    let lock_id = 998877665544i64; // Unique fixed ID for migrations
    sqlx::query(&format!("SELECT pg_advisory_lock({})", lock_id))
        .execute(&admin_pool)
        .await
        .map_err(|error| format!("acquire migration lock: {error}"))?;

    let result = MIGRATOR.run(&admin_pool).await;

    // Always unlock
    let _ = sqlx::query(&format!("SELECT pg_advisory_unlock({})", lock_id))
        .execute(&admin_pool)
        .await;

    result.map_err(|error| format!("run src/server/migrations: {error}"))?;

    let role_name = format!("ohc_security_test_{}", std::process::id());
    sqlx::raw_sql(
        &format!(r#"
        DO $$
        BEGIN
            IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = '{0}') THEN
                CREATE ROLE {0} LOGIN PASSWORD '{0}';
            END IF;
            EXECUTE format(
                'GRANT CONNECT ON DATABASE %I TO {0}',
                current_database()
            );
        END
        $$;
        ALTER ROLE {0} NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT NOBYPASSRLS;
        GRANT ohc_bypassrls TO {0};
        GRANT USAGE ON SCHEMA public TO {0};
        GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO {0};
        GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO {0};
        ALTER DEFAULT PRIVILEGES IN SCHEMA public
            GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO {0};
        ALTER DEFAULT PRIVILEGES IN SCHEMA public
            GRANT USAGE, SELECT ON SEQUENCES TO {0};
        "#, role_name)
    )
    .execute(&admin_pool)
    .await
    .map_err(|error| format!("provision non-superuser test role: {error}"))?;

    admin_pool.close().await;
    Ok(())
}

pub(crate) async fn postgres_security_pool(max_connections: u32) -> Option<PgPool> {
    let database_url = std::env::var("OHC_DATABASE_URL").ok();
    let require_postgres = std::env::var("OHC_REQUIRE_POSTGRES_TESTS").ok();
    let database_url =
        match decide_postgres_test(database_url.as_deref(), require_postgres.as_deref()) {
            Ok(PostgresTestDecision::Run(url)) => url,
            Ok(PostgresTestDecision::Skip(reason)) => {
                eprintln!("SKIPPED postgres security test: {reason}");
                return None;
            }
            Err(error) => panic!("{error}"),
        };

    let admin_url = std::env::var("OHC_POSTGRES_ADMIN_URL").unwrap_or_else(|_| {
        panic!(
            "PostgreSQL security tests require OHC_POSTGRES_ADMIN_URL for extension, migration, and application-role setup"
        )
    });
    POSTGRES_SETUP
        .get_or_init(|| initialize_postgres(&admin_url))
        .await
        .as_ref()
        .unwrap_or_else(|error| panic!("PostgreSQL security test setup failed: {error}"));

    let role_name = format!("ohc_security_test_{}", std::process::id());
    // Parse it manually since we don't have url crate
    let split_url: Vec<&str> = database_url.split("@").collect();
    let after_at = split_url[1];
    let before_at = split_url[0];
    let split_before: Vec<&str> = before_at.split("://").collect();
    let prefix = split_before[0];
    let new_url = format!("{}://{}:{}@{}", prefix, role_name, role_name, after_at);

    let pool = PgPoolOptions::new()
        .before_acquire(|conn, _meta| {
            Box::pin(async move {
                use sqlx::Executor;
                conn.execute("SELECT set_config('role', 'none', false), set_config('app.current_tenant', '', false)").await?;
                Ok(true)
            })
        })
        .after_release(|conn, _meta| {
            Box::pin(async move {
                use sqlx::Executor;
                conn.execute("SELECT set_config('role', 'none', false), set_config('app.current_tenant', '', false)").await?;
                Ok(true)
            })
        })
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&new_url)
        .await
        .unwrap_or_else(|error| {
            panic!("connect through OHC_DATABASE_URL application role: {error}")
        });

    let (
        session_user,
        current_user,
        is_superuser,
        inherits_roles,
        bypasses_rls,
        bypass_member,
        row_security,
    ): (String, String, bool, bool, bool, bool, String) = sqlx::query_as(
        "SELECT session_user::text, current_user::text, rolsuper, rolinherit, rolbypassrls, pg_has_role(current_user, 'ohc_bypassrls', 'MEMBER'), current_setting('row_security') FROM pg_roles WHERE rolname = current_user",
    )
    .fetch_one(&pool)
    .await
    .unwrap_or_else(|error| panic!("verify PostgreSQL application role: {error}"));
    assert_eq!(session_user, role_name, "unexpected session user");
    assert_eq!(
        current_user, session_user,
        "pool must start as its login role"
    );
    assert!(
        !is_superuser,
        "security tests must not run as superuser ({current_user})"
    );
    assert!(
        !bypasses_rls,
        "security tests must use NOBYPASSRLS ({current_user})"
    );
    assert!(
        !inherits_roles,
        "security tests must require explicit SET ROLE ({current_user})"
    );
    assert!(
        bypass_member,
        "security application role must be able to SET ROLE ohc_bypassrls"
    );
    assert_eq!(row_security, "on", "security tests require row_security=on");

    Some(pool)
}

#[cfg(test)]
mod tests {
    use super::{PostgresTestDecision, decide_postgres_test};

    #[test]
    fn local_missing_database_url_is_an_explicit_skip() {
        assert_eq!(
            decide_postgres_test(None, None),
            Ok(PostgresTestDecision::Skip(
                "OHC_DATABASE_URL is not set".to_string()
            ))
        );
    }

    #[test]
    fn local_sqlite_database_url_is_an_explicit_skip() {
        assert_eq!(
            decide_postgres_test(Some("sqlite://ohc.db"), None),
            Ok(PostgresTestDecision::Skip(
                "OHC_DATABASE_URL is not a PostgreSQL URL".to_string()
            ))
        );
    }

    #[test]
    fn required_lane_rejects_missing_or_non_postgres_database_url() {
        let missing = decide_postgres_test(None, Some("1")).unwrap_err();
        assert!(missing.contains("OHC_REQUIRE_POSTGRES_TESTS=1"));
        assert!(missing.contains("OHC_DATABASE_URL is not set"));

        let sqlite = decide_postgres_test(Some("sqlite://ohc.db"), Some("1")).unwrap_err();
        assert!(sqlite.contains("OHC_REQUIRE_POSTGRES_TESTS=1"));
        assert!(sqlite.contains("not a PostgreSQL URL"));
    }

    #[test]
    fn required_lane_runs_with_postgres_database_url() {
        assert_eq!(
            decide_postgres_test(
                Some("postgresql://ohc_test:password@127.0.0.1:5432/ohc_test"),
                Some("1")
            ),
            Ok(PostgresTestDecision::Run(
                "postgresql://ohc_test:password@127.0.0.1:5432/ohc_test".to_string()
            ))
        );
    }
}
