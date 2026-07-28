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

    // Implement a robust retry loop for migrations to handle unique constraint violations
    // that occur when multiple test processes try to migrate simultaneously.
    let mut retry_count = 0;
    loop {
        use sqlx::Executor;
        let mut conn = admin_pool.acquire().await.map_err(|e| format!("acquire connection: {e}"))?;

        // Take a database-level advisory lock
        conn.execute("SELECT pg_advisory_lock(20240726)").await.map_err(|e| format!("acquire lock: {e}"))?;

        let res = MIGRATOR.run(&mut *conn).await;

        let _ = conn.execute("SELECT pg_advisory_unlock(20240726)").await;

        match res {
            Ok(_) => break,
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("duplicate key value violates unique constraint") || err_str.contains("already exists") {
                    retry_count += 1;
                    if retry_count > 10 {
                        return Err(format!("run src/server/migrations: {e}"));
                    }
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    continue;
                }
                return Err(format!("run src/server/migrations: {e}"));
            }
        }
    }

    sqlx::raw_sql(
        r#"
        DO $$
        BEGIN
            IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'ohc_application_role') THEN
                CREATE ROLE ohc_application_role;
            END IF;

            GRANT USAGE ON SCHEMA public TO ohc_application_role;
            GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO ohc_application_role;
            GRANT USAGE, SELECT, UPDATE ON ALL SEQUENCES IN SCHEMA public TO ohc_application_role;

            ALTER DEFAULT PRIVILEGES IN SCHEMA public
            GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO ohc_application_role;

            ALTER DEFAULT PRIVILEGES IN SCHEMA public
            GRANT USAGE, SELECT, UPDATE ON SEQUENCES TO ohc_application_role;
        END
        $$;
        "#,
    )
    .execute(&admin_pool)
    .await
    .map_err(|error| format!("setup application role: {error}"))?;

    Ok(())
}

pub(crate) async fn postgres_security_pool(max_connections: u32) -> Option<PgPool> {
    let database_url = std::env::var("OHC_DATABASE_URL").ok();
    let require_postgres = std::env::var("OHC_REQUIRE_POSTGRES_TESTS").ok();

    let decision = decide_postgres_test(database_url.as_deref(), require_postgres.as_deref());

    let url = match decision {
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
        .acquire_timeout(Duration::from_secs(5))
        .connect(&url)
        .await
        .unwrap_or_else(|error| panic!("Failed to connect to application database URL: {error}"));

    Some(pool)
}

pub(crate) async fn set_tenant_context(pool: &PgPool, tenant_id: &str) {
    sqlx::query("SELECT set_config('role', 'ohc_application_role', false), set_config('app.current_tenant', $1, false)")
        .bind(tenant_id)
        .execute(pool)
        .await
        .unwrap_or_else(|error| panic!("Failed to set tenant context to {tenant_id}: {error}"));
}
