use sqlx::{PgPool, Result};

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    let queries = vec![
        r#"
        CREATE TABLE IF NOT EXISTS tenants (
            tenant_id VARCHAR(255) PRIMARY KEY,
            name VARCHAR(255) NOT NULL
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS contacts (
            contact_id VARCHAR(255) PRIMARY KEY,
            tenant_id VARCHAR(255) NOT NULL,
            name VARCHAR(255) NOT NULL
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS inboxes (
            inbox_id VARCHAR(255) PRIMARY KEY,
            tenant_id VARCHAR(255) NOT NULL,
            name VARCHAR(255) NOT NULL
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS conversations (
            conversation_id VARCHAR(255) PRIMARY KEY,
            tenant_id VARCHAR(255) NOT NULL,
            inbox_id VARCHAR(255) NOT NULL,
            contact_id VARCHAR(255) NOT NULL
        )
        "#,
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            message_id VARCHAR(255) PRIMARY KEY,
            tenant_id VARCHAR(255) NOT NULL,
            conversation_id VARCHAR(255) NOT NULL,
            content TEXT NOT NULL
        )
        "#,
    ];

    for q in queries {
        sqlx::query(q).execute(pool).await?;
    }

    // Enable RLS for each table
    let rls_queries = vec![
        "ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;",
        "ALTER TABLE contacts ENABLE ROW LEVEL SECURITY;",
        "ALTER TABLE inboxes ENABLE ROW LEVEL SECURITY;",
        "ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;",
        "ALTER TABLE messages ENABLE ROW LEVEL SECURITY;",

        "CREATE POLICY tenant_isolation_policy ON tenants USING (tenant_id = current_setting('app.current_tenant', true));",
        "CREATE POLICY contact_isolation_policy ON contacts USING (tenant_id = current_setting('app.current_tenant', true));",
        "CREATE POLICY inbox_isolation_policy ON inboxes USING (tenant_id = current_setting('app.current_tenant', true));",
        "CREATE POLICY conv_isolation_policy ON conversations USING (tenant_id = current_setting('app.current_tenant', true));",
        "CREATE POLICY msg_isolation_policy ON messages USING (tenant_id = current_setting('app.current_tenant', true));",
    ];

    for q in rls_queries {
        let _ = sqlx::query(q).execute(pool).await; // Ignore errors if policies already exist
    }

    Ok(())
}
