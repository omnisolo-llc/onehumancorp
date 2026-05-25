use super::*;
use sqlx::Executor;
use sqlx::postgres::PgPoolOptions;
use std::env;
use uuid::Uuid;

#[tokio::test]
async fn test_tenant_isolation_rls() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let mut tx = pool.begin().await?;

    tx.execute("
        CREATE TABLE IF NOT EXISTS tenant (
            id UUID PRIMARY KEY,
            business_name TEXT NOT NULL,
            owner_email TEXT NOT NULL,
            subscription_tier TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS product (
            id UUID PRIMARY KEY,
            tenant_id UUID REFERENCES tenant(id) ON DELETE CASCADE,
            type TEXT NOT NULL,
            title TEXT NOT NULL,
            price DECIMAL NOT NULL,
            stock_level INT NOT NULL,
            is_active BOOLEAN NOT NULL DEFAULT true
        );

        ALTER TABLE tenant ENABLE ROW LEVEL SECURITY;
        ALTER TABLE product ENABLE ROW LEVEL SECURITY;

        DO $$
        BEGIN
            IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_tenant' AND tablename = 'tenant') THEN
                CREATE POLICY tenant_isolation_tenant ON tenant USING (id::text = current_setting('app.current_tenant', true));
            END IF;
            IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_product' AND tablename = 'product') THEN
                CREATE POLICY tenant_isolation_product ON product USING (tenant_id::text = current_setting('app.current_tenant', true));
            END IF;
        END
        $$;
    ").await?;

    let tenant1_id = Uuid::new_v4();
    let tenant2_id = Uuid::new_v4();

    tx.execute(format!("SET LOCAL app.current_tenant = '{}';", tenant1_id).as_str()).await?;
    tx.execute(sqlx::query("INSERT INTO tenant (id, business_name, owner_email, subscription_tier) VALUES ($1, 'Business 1', 'owner1@test.com', 'free')").bind(tenant1_id)).await?;
    tx.execute(sqlx::query("INSERT INTO product (id, tenant_id, type, title, price, stock_level) VALUES ($1, $2, 'physical', 'Product 1', 10.0, 100)").bind(Uuid::new_v4()).bind(tenant1_id)).await?;

    tx.execute(format!("SET LOCAL app.current_tenant = '{}';", tenant2_id).as_str()).await?;
    tx.execute(sqlx::query("INSERT INTO tenant (id, business_name, owner_email, subscription_tier) VALUES ($1, 'Business 2', 'owner2@test.com', 'pro')").bind(tenant2_id)).await?;
    tx.execute(sqlx::query("INSERT INTO product (id, tenant_id, type, title, price, stock_level) VALUES ($1, $2, 'physical', 'Product 2', 20.0, 200)").bind(Uuid::new_v4()).bind(tenant2_id)).await?;

    tx.execute("ALTER TABLE tenant FORCE ROW LEVEL SECURITY;").await?;
    tx.execute("ALTER TABLE product FORCE ROW LEVEL SECURITY;").await?;

    tx.execute(format!("SET LOCAL app.current_tenant = '{}';", tenant1_id).as_str()).await?;

    let products: Vec<Product> = sqlx::query_as::<_, Product>("SELECT * FROM product")
        .fetch_all(&mut *tx)
        .await?;

    assert_eq!(products.len(), 1, "Should only fetch 1 product for tenant 1");
    assert_eq!(products[0].tenant_id, tenant1_id);
    assert_eq!(products[0].title, "Product 1");

    tx.execute(format!("SET LOCAL app.current_tenant = '{}';", tenant2_id).as_str()).await?;

    let products2: Vec<Product> = sqlx::query_as::<_, Product>("SELECT * FROM product")
        .fetch_all(&mut *tx)
        .await?;

    assert_eq!(products2.len(), 1, "Should only fetch 1 product for tenant 2");
    assert_eq!(products2[0].tenant_id, tenant2_id);
    assert_eq!(products2[0].title, "Product 2");

    tx.execute("RESET app.current_tenant;").await?;

    let products_none: Vec<Product> = sqlx::query_as::<_, Product>("SELECT * FROM product")
        .fetch_all(&mut *tx)
        .await?;

    assert_eq!(products_none.len(), 0, "Should fetch 0 products when tenant context is missing");

    tx.rollback().await?;
    Ok(())
}
