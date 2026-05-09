use sqlx::PgPool;
use std::time::Duration;
use crate::utils::auth_utils::set_org_context;
use sqlx::Row;

async fn setup_isolation_test_db() -> Option<PgPool> {
    if std::env::var("DATABASE_URL").is_err() {
        return None;
    }
    let database_url = std::env::var("DATABASE_URL").unwrap();
    if database_url.starts_with("sqlite") {
        return None;
    }

    let pool = sqlx::postgres::PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(30))
        .max_connections(10)
        .connect_lazy(&database_url)
        .ok()?;
    Some(pool)
}

#[tokio::test]
async fn test_rls_cross_tenant_isolation() {
    let pool = match setup_isolation_test_db().await {
        Some(p) => p,
        None => return,
    };

    let tenant_a = uuid::Uuid::new_v4().to_string();
    let tenant_b = uuid::Uuid::new_v4().to_string();

    // 1. Create data for Tenant A and Tenant B as superuser
    sqlx::query("INSERT INTO tenants (tenant_id, business_name) VALUES ($1, 'Tenant A') ON CONFLICT DO NOTHING")
        .bind(uuid::Uuid::parse_str(&tenant_a).unwrap())
        .execute(&pool).await.ok();
    sqlx::query("INSERT INTO tenants (tenant_id, business_name) VALUES ($1, 'Tenant B') ON CONFLICT DO NOTHING")
        .bind(uuid::Uuid::parse_str(&tenant_b).unwrap())
        .execute(&pool).await.ok();

    sqlx::query("INSERT INTO products (id, tenant_id, name) VALUES ('prod_a_sec', $1, 'Product A') ON CONFLICT DO NOTHING")
        .bind(uuid::Uuid::parse_str(&tenant_a).unwrap())
        .execute(&pool).await.ok();
    sqlx::query("INSERT INTO products (id, tenant_id, name) VALUES ('prod_b_sec', $1, 'Product B') ON CONFLICT DO NOTHING")
        .bind(uuid::Uuid::parse_str(&tenant_b).unwrap())
        .execute(&pool).await.ok();

    // 2. Set context to Tenant A and try to access Tenant B's data
    let mut conn = match pool.acquire().await {
        Ok(c) => c,
        Err(_) => return, // DB unavailable
    };
    set_org_context(&mut *conn, &tenant_a).await.unwrap();

    // Should see Product A
    let product_a = sqlx::query("SELECT name FROM products WHERE id = 'prod_a_sec'")
        .fetch_optional(&mut *conn).await.unwrap();
    assert!(product_a.is_some());

    // Should NOT see Product B (IDOR attempt)
    let product_b = sqlx::query("SELECT name FROM products WHERE id = 'prod_b_sec'")
        .fetch_optional(&mut *conn).await.unwrap();
    assert!(product_b.is_none(), "RLS Leak: Tenant A can see Tenant B's products");

    // 3. Try to update Tenant B's product as Tenant A
    let update_res = sqlx::query("UPDATE products SET name = 'Hacked' WHERE id = 'prod_b_sec'")
        .execute(&mut *conn).await.unwrap();
    assert_eq!(update_res.rows_affected(), 0, "RLS Leak: Tenant A can update Tenant B's products");

    // 4. Test "system" role bypass
    set_org_context(&mut *conn, "system").await.unwrap();
    // System should see both (assuming multitenant = false in test config or using bypassrls)
    if !crate::config::get().multitenant {
        let all_products = sqlx::query("SELECT count(*) FROM products WHERE id IN ('prod_a_sec', 'prod_b_sec')")
            .fetch_one(&mut *conn).await.unwrap();
        let count: i64 = all_products.get(0);
        assert!(count >= 2, "System bypass failed");
    }

    // Cleanup
    set_org_context(&mut *conn, "system").await.unwrap();
    let _ = sqlx::query("DELETE FROM products WHERE id IN ('prod_a_sec', 'prod_b_sec')").execute(&mut *conn).await;
    let _ = sqlx::query("DELETE FROM tenants WHERE tenant_id IN ($1, $2)")
        .bind(uuid::Uuid::parse_str(&tenant_a).unwrap())
        .bind(uuid::Uuid::parse_str(&tenant_b).unwrap())
        .execute(&mut *conn).await;
}

#[tokio::test]
async fn test_empty_org_id_multitenant_rejection() {
    // Only run if multitenant is forced or detected
    if !crate::config::get().multitenant {
        return;
    }

    let pool = match setup_isolation_test_db().await {
        Some(p) => p,
        None => return,
    };

    let mut conn = pool.acquire().await.unwrap();
    let res = set_org_context(&mut *conn, "").await;
    assert!(res.is_err(), "Security Failure: Empty org_id allowed in multitenant mode");
    assert!(res.unwrap_err().to_string().contains("CRITICAL SECURITY ERROR"));
}
