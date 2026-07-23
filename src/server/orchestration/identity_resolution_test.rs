use super::identity_resolution::IdentityResolver;
use crate::db::DB;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_resolve_existing_customer() {
    let tenant_id = format!("test_tenant_{}", Uuid::new_v4());
    let temp_db_path = format!("file:test_resolve_existing_customer_{}.db?mode=memory&cache=shared", Uuid::new_v4());
    let pool = sqlx::SqlitePool::connect(&temp_db_path).await.unwrap();
    let db = Arc::new(DB {
        pool: sqlx::PgPool::connect_lazy("postgres://dummy").unwrap(),
        store: crate::db::DbStore::Sqlite(pool.clone()),
    });

    sqlx::query("CREATE TABLE tenants (id TEXT, name TEXT)").execute(&pool).await.unwrap();
    sqlx::query("CREATE TABLE customers (id TEXT, tenant_id TEXT, name TEXT, email TEXT, phone TEXT, preferences JSON, embedding BLOB, profile_summary JSON)").execute(&pool).await.unwrap();
    sqlx::query("CREATE TABLE customer_identities (id TEXT, tenant_id TEXT, customer_id TEXT, channel TEXT, channel_identity TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, UNIQUE(tenant_id, channel, channel_identity))").execute(&pool).await.unwrap();

    let sender_id = "test_lead_existing_123@example.com";
    sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'test')")
        .bind(&tenant_id)
        .execute(&pool)
        .await
        .unwrap();

    let existing_customer_id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO customers (id, tenant_id, name, email) VALUES ($1, $2, 'Existing Customer', $3)")
        .bind(&existing_customer_id)
        .bind(&tenant_id)
        .bind(sender_id)
        .execute(&pool)
        .await
        .unwrap();

    let resolver = IdentityResolver::new(db.clone());
    let resolved_id = resolver.resolve_or_create_customer(&tenant_id, sender_id, "email").await.unwrap();

    assert_eq!(resolved_id, existing_customer_id, "Should resolve to existing customer ID based on email");
}

#[tokio::test]
async fn test_create_new_customer() {
    let tenant_id = format!("test_tenant_{}", Uuid::new_v4());
    let temp_db_path = format!("file:test_create_new_customer_{}.db?mode=memory&cache=shared", Uuid::new_v4());
    let pool = sqlx::SqlitePool::connect(&temp_db_path).await.unwrap();
    let db = Arc::new(DB {
        pool: sqlx::PgPool::connect_lazy("postgres://dummy").unwrap(),
        store: crate::db::DbStore::Sqlite(pool.clone()),
    });

    sqlx::query("CREATE TABLE tenants (id TEXT, name TEXT)").execute(&pool).await.unwrap();
    sqlx::query("CREATE TABLE customers (id TEXT, tenant_id TEXT, name TEXT, email TEXT, phone TEXT, preferences JSON, embedding BLOB, profile_summary JSON)").execute(&pool).await.unwrap();
    sqlx::query("CREATE TABLE customer_identities (id TEXT, tenant_id TEXT, customer_id TEXT, channel TEXT, channel_identity TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, UNIQUE(tenant_id, channel, channel_identity))").execute(&pool).await.unwrap();

    sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'test')")
        .bind(&tenant_id)
        .execute(&pool)
        .await
        .unwrap();

    let sender_id = "new_lead_456@example.com";
    let source = "email";
    let resolver = IdentityResolver::new(db.clone());
    let lead_id = resolver.resolve_or_create_customer(&tenant_id, sender_id, source).await.unwrap();

    // Verify customer was created
    let row: (String, Option<String>) = sqlx::query_as("SELECT name, email FROM customers WHERE id = $1")
        .bind(&lead_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(row.0, "Unknown Lead (email)");
    assert_eq!(row.1, Some(sender_id.to_string()));

    // Verify identity was created
    let identity_row: (String, String) = sqlx::query_as("SELECT channel, channel_identity FROM customer_identities WHERE customer_id = $1")
        .bind(&lead_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(identity_row.0, source);
    assert_eq!(identity_row.1, sender_id);
}

#[tokio::test]
async fn test_create_and_resolve_social_customer() {
    let tenant_id = format!("test_tenant_{}", Uuid::new_v4());
    let temp_db_path = format!("file:test_create_and_resolve_social_customer_{}.db?mode=memory&cache=shared", Uuid::new_v4());
    let pool = sqlx::SqlitePool::connect(&temp_db_path).await.unwrap();
    let db = Arc::new(DB {
        pool: sqlx::PgPool::connect_lazy("postgres://dummy").unwrap(),
        store: crate::db::DbStore::Sqlite(pool.clone()),
    });

    sqlx::query("CREATE TABLE tenants (id TEXT, name TEXT)").execute(&pool).await.unwrap();
    sqlx::query("CREATE TABLE customers (id TEXT, tenant_id TEXT, name TEXT, email TEXT, phone TEXT, preferences JSON, embedding BLOB, profile_summary JSON)").execute(&pool).await.unwrap();
    sqlx::query("CREATE TABLE customer_identities (id TEXT, tenant_id TEXT, customer_id TEXT, channel TEXT, channel_identity TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, UNIQUE(tenant_id, channel, channel_identity))").execute(&pool).await.unwrap();

    sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'test')")
        .bind(&tenant_id)
        .execute(&pool)
        .await
        .unwrap();

    let sender_id = "insta_user_789";
    let source = "instagram";
    let resolver = IdentityResolver::new(db.clone());
    let lead_id = resolver.resolve_or_create_customer(&tenant_id, sender_id, source).await.unwrap();

    // Check preferences JSON for social handle
    let row: (String, String) = sqlx::query_as("SELECT name, preferences FROM customers WHERE id = $1")
        .bind(&lead_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(row.0, "Unknown Lead (instagram)");
    assert!(row.1.contains("\"social_handle\":\"insta_user_789\""));

    // Verify identity was created
    let identity_row: (String, String) = sqlx::query_as("SELECT channel, channel_identity FROM customer_identities WHERE customer_id = $1")
        .bind(&lead_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(identity_row.0, source);
    assert_eq!(identity_row.1, sender_id);

    // Resolve again to test existing fetch
    let resolved_id = resolver.resolve_or_create_customer(&tenant_id, sender_id, source).await.unwrap();
    assert_eq!(resolved_id, lead_id, "Should resolve back to the same lead_id");
}
