use crate::api::crm::identity::{resolve_customer_identity, create_customer_identity, get_customer_context};
use crate::db::DbStore;

#[tokio::test]
async fn test_customer_identity_resolution() {
    let _ = tracing_subscriber::fmt::try_init();

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE tenants (id TEXT PRIMARY KEY);
         CREATE TABLE customers (id TEXT PRIMARY KEY, tenant_id TEXT, name TEXT, email TEXT);
         CREATE TABLE orders (id TEXT PRIMARY KEY, tenant_id TEXT, customer_id TEXT, status TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP);
         CREATE TABLE customer_identities (id TEXT PRIMARY KEY, tenant_id TEXT, customer_id TEXT, channel TEXT, identifier TEXT, UNIQUE(tenant_id, channel, identifier));"
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query("INSERT INTO tenants (id) VALUES ('tenant1')").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO customers (id, tenant_id, name, email) VALUES ('cust1', 'tenant1', 'Alice', 'alice@example.com')").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO orders (id, tenant_id, customer_id, status) VALUES ('ord1', 'tenant1', 'cust1', 'completed')").execute(&pool).await.unwrap();

    let db_store = DbStore::Sqlite(pool.clone());

    let identity = resolve_customer_identity(&db_store, "tenant1", "instagram", "alice_insta").await.unwrap();
    assert!(identity.is_none());

    create_customer_identity(&db_store, "tenant1", "cust1", "instagram", "alice_insta").await.unwrap();

    let identity = resolve_customer_identity(&db_store, "tenant1", "instagram", "alice_insta").await.unwrap();
    assert_eq!(identity.unwrap(), "cust1");

    let context = get_customer_context(&db_store, "tenant1", "cust1").await.unwrap();
    assert!(context.contains("Alice"));
    assert!(context.contains("alice@example.com"));
    assert!(context.contains("ord1"));
    assert!(context.contains("completed"));
}
