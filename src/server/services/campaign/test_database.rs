//! Disposable PostgreSQL fixtures for the existing campaign service tests.
//! The caller must provide a dedicated test database; ambient app URLs are ignored.
use sqlx::{
    PgPool, Row,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::str::FromStr;
use uuid::Uuid;

pub(super) struct TestDatabase {
    pub pool: PgPool,
    pub tenant_id: String,
    admin: PgPool,
    schema: String,
    role: String,
}

impl TestDatabase {
    pub async fn new() -> Self {
        let url = std::env::var("OHC_CAMPAIGN_TEST_DATABASE_URL").expect(
            "Supply OHC_CAMPAIGN_TEST_DATABASE_URL for a disposable PostgreSQL test database",
        );
        let admin = PgPoolOptions::new()
            .max_connections(1)
            .connect(&url)
            .await
            .unwrap();
        let suffix = Uuid::new_v4().simple().to_string();
        let schema = format!("campaign_test_{suffix}");
        let role = format!("campaign_role_{suffix}");
        let password = Uuid::new_v4().simple().to_string();
        let mut connection = admin.acquire().await.unwrap();
        sqlx::raw_sql(&format!(
            "CREATE SCHEMA {schema}; CREATE ROLE {role} LOGIN PASSWORD '{password}' NOSUPERUSER NOBYPASSRLS; SET search_path TO {schema}"
        )).execute(&mut *connection).await.unwrap();

        // Reuse the exact canonical tenant table and complete campaign migration.
        // Unrelated vector/provider schemas are outside these service cases.
        let initial = include_str!("../../migrations/001_initial.sql");
        let start = initial
            .find("CREATE TABLE IF NOT EXISTS tenants (")
            .unwrap();
        let end = start + initial[start..].find("\n);").unwrap() + 3;
        sqlx::raw_sql(&initial[start..end])
            .execute(&mut *connection)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!("../../migrations/063_campaign_engine.sql"))
            .execute(&mut *connection)
            .await
            .unwrap();
        sqlx::raw_sql(&format!(
            "ALTER TABLE campaigns FORCE ROW LEVEL SECURITY;
             ALTER TABLE campaign_assets FORCE ROW LEVEL SECURITY;
             ALTER TABLE channel_executions FORCE ROW LEVEL SECURITY;
             ALTER TABLE promotion_codes FORCE ROW LEVEL SECURITY;
             GRANT USAGE ON SCHEMA {schema} TO {role};
             GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA {schema} TO {role};
             RESET search_path;"
        ))
        .execute(&mut *connection)
        .await
        .unwrap();
        drop(connection);

        let tenant_id = Uuid::new_v4().to_string();
        let options = PgConnectOptions::from_str(&url)
            .unwrap()
            .username(&role)
            .password(&password);
        let connection_schema = schema.clone();
        let connection_tenant = tenant_id.clone();
        let pool = PgPoolOptions::new().max_connections(1).after_connect(move |connection, _| {
            let schema = connection_schema.clone();
            let tenant = connection_tenant.clone();
            Box::pin(async move {
                sqlx::query("SELECT set_config('search_path', $1, false), set_config('app.current_tenant', $2, false)")
                    .bind(schema).bind(tenant).execute(connection).await?;
                Ok(())
            })
        }).connect_with(options).await.unwrap();
        let fixture = Self {
            pool,
            tenant_id,
            admin,
            schema,
            role,
        };
        fixture.assert_restricted_identity().await;
        fixture.insert_tenant(&fixture.tenant_id).await;
        fixture
    }

    pub async fn insert_tenant(&self, id: &str) {
        sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, $2)")
            .bind(id)
            .bind(format!("Test Tenant {id}"))
            .execute(&self.pool)
            .await
            .unwrap();
    }

    pub async fn assert_restricted_identity(&self) {
        let row = sqlx::query("SELECT current_user::text AS current_user, session_user::text AS session_user, rolsuper, rolbypassrls FROM pg_roles WHERE rolname=current_user")
            .fetch_one(&self.pool).await.unwrap();
        assert_eq!(row.get::<String, _>("current_user"), self.role);
        assert_eq!(row.get::<String, _>("session_user"), self.role);
        assert!(!row.get::<bool, _>("rolsuper"));
        assert!(!row.get::<bool, _>("rolbypassrls"));
    }

    pub async fn cleanup(self) {
        self.assert_restricted_identity().await;
        self.pool.close().await;
        sqlx::raw_sql(&format!(
            "DROP SCHEMA {} CASCADE; DROP ROLE {};",
            self.schema, self.role
        ))
        .execute(&self.admin)
        .await
        .unwrap();
        self.admin.close().await;
    }
}
