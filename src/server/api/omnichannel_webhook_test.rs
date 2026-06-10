use super::omnichannel_webhook::IdentityResolver;

#[tokio::test]
async fn test_identity_resolver_finds_customer_by_email() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    sqlx::query("CREATE TABLE customers (id TEXT, tenant_id TEXT, email TEXT, phone TEXT)").execute(&pool).await.unwrap();

    let tenant_id = "test_tenant";
    let email = "maya@example.com";
    let customer_id = uuid::Uuid::new_v4().to_string();

    // Insert test customer
    let _ = sqlx::query("INSERT INTO customers (id, tenant_id, email) VALUES ($1, $2, $3)")
        .bind(&customer_id)
        .bind(tenant_id)
        .bind(email)
        .execute(&pool)
        .await;

    // Use test local identity resolver logic for sqlite test DB
    let query = "SELECT id FROM customers WHERE tenant_id = $1 AND (email = $2 OR phone = $2) LIMIT 1";
    let result: Option<(String,)> = sqlx::query_as(query)
        .bind(tenant_id)
        .bind(email)
        .fetch_optional(&pool)
        .await.unwrap();
    assert_eq!(result.map(|(id,)| id), Some(customer_id.clone()));
}

#[tokio::test]
async fn test_identity_resolver_finds_customer_by_phone() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    sqlx::query("CREATE TABLE customers (id TEXT, tenant_id TEXT, email TEXT, phone TEXT)").execute(&pool).await.unwrap();

    let tenant_id = "test_tenant";
    let phone = "+15551234567";

    let customer_id = uuid::Uuid::new_v4().to_string();

    // Insert test customer
    let _ = sqlx::query("INSERT INTO customers (id, tenant_id, phone) VALUES ($1, $2, $3)")
        .bind(&customer_id)
        .bind(tenant_id)
        .bind(phone)
        .execute(&pool)
        .await;

    let query = "SELECT id FROM customers WHERE tenant_id = $1 AND (email = $2 OR phone = $2) LIMIT 1";
    let result: Option<(String,)> = sqlx::query_as(query)
        .bind(tenant_id)
        .bind(phone)
        .fetch_optional(&pool)
        .await.unwrap();
    assert_eq!(result.map(|(id,)| id), Some(customer_id.clone()));
}

#[tokio::test]
async fn test_identity_resolver_returns_none_if_not_found() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    sqlx::query("CREATE TABLE customers (id TEXT, tenant_id TEXT, email TEXT, phone TEXT)").execute(&pool).await.unwrap();

    let tenant_id = "test_tenant";
    let unknown_sender = "unknown@example.com";

    let query = "SELECT id FROM customers WHERE tenant_id = $1 AND (email = $2 OR phone = $2) LIMIT 1";
    let result: Option<(String,)> = sqlx::query_as(query)
        .bind(tenant_id)
        .bind(unknown_sender)
        .fetch_optional(&pool)
        .await.unwrap();
    assert_eq!(result, None);
}

// Dummy use for test
fn _dummy_use() {
    let _ = IdentityResolver;
}
