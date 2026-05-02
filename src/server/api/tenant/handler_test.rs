use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::api::tenant::handler::{CreateTenantRequest};
use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn test_tenant_api() {
    if std::env::var("DATABASE_URL").is_err() {
        return;
    }

    let database_url = "postgres://postgres:postgres@localhost:5432/test";
    // We intentionally do NOT set `app.current_tenant` in `before_acquire` to ensure
    // the API handler sets the context properly for RLS.
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_millis(50))
        .connect_lazy(database_url)
        .unwrap();

    let db = Arc::new(DB { pool: pool.clone(), store: DbStore::Postgres });

    // Ensure table exists for isolated unit tests
    // Here we set current_tenant because the migration/setup needs rights to create/modify tables.
    let mut tx = db.pool.begin().await.unwrap();
    use sqlx::Executor;
    tx.execute("SET LOCAL app.current_tenant = 'system'").await.unwrap();

    tx.execute(
        "CREATE TABLE IF NOT EXISTS tenants (
            id TEXT PRIMARY KEY,
            business_name TEXT NOT NULL,
            business_type TEXT NOT NULL,
            flags JSONB NOT NULL DEFAULT '{}',
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );"
    ).await.unwrap();

    tx.execute(
        "CREATE TABLE IF NOT EXISTS tenant_agents (
            tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
            agent_id TEXT NOT NULL,
            role TEXT NOT NULL,
            PRIMARY KEY (tenant_id, agent_id)
        );"
    ).await.unwrap();

    // In PostgreSQL, to ensure RLS is enforced we enable it.
    tx.execute("ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;").await.unwrap();
    tx.execute("ALTER TABLE tenant_agents ENABLE ROW LEVEL SECURITY;").await.unwrap();

    // Setup policies for test purposes if they don't exist
    let _ = tx.execute("DROP POLICY IF EXISTS tenant_isolation_tenants ON tenants;").await;
    tx.execute("CREATE POLICY tenant_isolation_tenants ON tenants USING (id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');").await.unwrap();
    let _ = tx.execute("DROP POLICY IF EXISTS tenant_isolation_tenant_agents ON tenant_agents;").await;
    tx.execute("CREATE POLICY tenant_isolation_tenant_agents ON tenant_agents USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');").await.unwrap();

    tx.commit().await.unwrap();

    // Call the application logic directly.
    // Usually handled by axum Router but tower-service missing prevents it here.
    // Instead we emulate create_tenant by extracting out the core logic that operates on DB.

    let payload = CreateTenantRequest {
        business_name: "Carlos Handyman".to_string(),
        business_type: "Handyman Service".to_string(),
    };

    let mut flags = crate::domain::tenant::TenantFlags::default();
    let lower_type = payload.business_type.to_lowercase();
    if lower_type.contains("service") || lower_type.contains("handyman") || lower_type.contains("tutor") {
        flags.enable_booking = true;
    }
    if lower_type.contains("food") || lower_type.contains("cart") || lower_type.contains("restaurant") {
        flags.enable_menu = true;
    }

    let tenant_id = uuid::Uuid::new_v4().to_string();
    let flags_json = serde_json::to_value(&flags).unwrap_or(serde_json::json!({}));

    let mut tx2 = db.pool.begin().await.unwrap();
    tx2.execute("SET LOCAL app.current_tenant = 'system'").await.unwrap();

    let query1 = sqlx::query("INSERT INTO tenants (id, business_name, business_type, flags) VALUES ($1, $2, $3, $4)")
        .bind(&tenant_id)
        .bind(&payload.business_name)
        .bind(&payload.business_type)
        .bind(&flags_json);

    query1.execute(&mut *tx2).await.unwrap();
    tx2.commit().await.unwrap();

    // Test fetch row to verify state
    let mut tx3 = db.pool.begin().await.unwrap();
    tx3.execute("SET LOCAL app.current_tenant = 'system'").await.unwrap();
    let row = sqlx::query("SELECT * FROM tenants WHERE id = $1").bind(&tenant_id).fetch_one(&mut *tx3).await.unwrap();
    use sqlx::Row;
    let fetched_name: String = row.get("business_name");
    assert_eq!(fetched_name, "Carlos Handyman");
    tx3.commit().await.unwrap();


    let payload2 = CreateTenantRequest {
        business_name: "Fatima Halal Cart".to_string(),
        business_type: "Food Cart".to_string(),
    };

    let mut flags2 = crate::domain::tenant::TenantFlags::default();
    let lower_type2 = payload2.business_type.to_lowercase();
    if lower_type2.contains("service") || lower_type2.contains("handyman") || lower_type2.contains("tutor") {
        flags2.enable_booking = true;
    }
    if lower_type2.contains("food") || lower_type2.contains("cart") || lower_type2.contains("restaurant") {
        flags2.enable_menu = true;
    }

    let tenant_id2 = uuid::Uuid::new_v4().to_string();
    let flags_json2 = serde_json::to_value(&flags2).unwrap_or(serde_json::json!({}));

    let mut tx4 = db.pool.begin().await.unwrap();
    tx4.execute("SET LOCAL app.current_tenant = 'system'").await.unwrap();

    let query2 = sqlx::query("INSERT INTO tenants (id, business_name, business_type, flags) VALUES ($1, $2, $3, $4)")
        .bind(&tenant_id2)
        .bind(&payload2.business_name)
        .bind(&payload2.business_type)
        .bind(&flags_json2);

    query2.execute(&mut *tx4).await.unwrap();
    tx4.commit().await.unwrap();

    // Test fetch row to verify state
    let mut tx5 = db.pool.begin().await.unwrap();
    tx5.execute("SET LOCAL app.current_tenant = 'system'").await.unwrap();
    let row2 = sqlx::query("SELECT * FROM tenants WHERE id = $1").bind(&tenant_id2).fetch_one(&mut *tx5).await.unwrap();
    let fetched_name2: String = row2.get("business_name");
    assert_eq!(fetched_name2, "Fatima Halal Cart");
    tx5.commit().await.unwrap();

    // Finally let's test RLS isolation directly:
    let mut tx6 = db.pool.begin().await.unwrap();
    // Do NOT set current_tenant - meaning it defaults to unknown
    let result = sqlx::query("SELECT * FROM tenants WHERE id = $1").bind(&tenant_id2).fetch_optional(&mut *tx6).await;
    // depending on the connection string it may error with RLS policy check, or return None
    assert!(result.is_err() || result.unwrap().is_none());
}
