use std::sync::Arc;
use crate::db::DB;
use crate::orchestration::identity_resolution::IdentityResolver;
use uuid::Uuid;

#[tokio::test]
async fn test_resolve_existing_customer() {
    let tenant_id = format!("test_tenant_{}", Uuid::new_v4());
    let temp_db_path = format!("file:test_resolve_existing_customer_{}.db?mode=memory&cache=shared", Uuid::new_v4());
    std::env::set_var("OHC_DATABASE_URL", &temp_db_path);
    let db = Arc::new(DB::new().await.unwrap());

    // Explicitly run migrations to ensure table schema is there in SQLite memory
    crate::migrations::run_migrations(&db.pool).await.unwrap();

    let resolver = IdentityResolver::new(db.clone());
    let sender_id = "test_lead_existing_123@example.com";
    let source = "email";

    let new_id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'test')")
        .bind(&tenant_id)
        .execute(&db.pool)
        .await.unwrap();

    sqlx::query("INSERT INTO customers (id, tenant_id, name, email, phone) VALUES ($1, $2, 'Existing Customer', $3, NULL)")
        .bind(&new_id)
        .bind(&tenant_id)
        .bind(sender_id)
        .execute(&db.pool)
        .await.unwrap();

    let resolved_id = resolver.resolve_or_create_customer(&tenant_id, sender_id, source).await.unwrap_or_default();

    assert_eq!(resolved_id, new_id);
}

#[tokio::test]
async fn test_create_new_customer() {
    let tenant_id = format!("test_tenant_{}", Uuid::new_v4());
    let temp_db_path = format!("file:test_create_new_customer_{}.db?mode=memory&cache=shared", Uuid::new_v4());
    std::env::set_var("OHC_DATABASE_URL", &temp_db_path);
    let db = Arc::new(DB::new().await.unwrap());

    // Explicitly run migrations to ensure table schema is there in SQLite memory
    crate::migrations::run_migrations(&db.pool).await.unwrap();

    let resolver = IdentityResolver::new(db.clone());
    let sender_id = "new_lead_12345@example.com";
    let source = "whatsapp";

    sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'test')")
        .bind(&tenant_id)
        .execute(&db.pool)
        .await.unwrap();

    let lead_id = resolver.resolve_or_create_customer(&tenant_id, sender_id, source).await.unwrap_or_default();

    let row: Option<(String, Option<String>)> = sqlx::query_as("SELECT id, phone FROM customers WHERE id = $1")
        .bind(&lead_id)
        .fetch_optional(&db.pool)
        .await
        .unwrap_or(None);

    assert!(!lead_id.is_empty());
    assert!(row.is_some());
    let r = row.unwrap();
    assert_eq!(r.0, lead_id);
    assert_eq!(r.1.unwrap(), sender_id);
}

#[tokio::test]
async fn test_create_and_resolve_social_customer() {
    let tenant_id = format!("test_tenant_{}", Uuid::new_v4());
    let temp_db_path = format!("file:test_create_and_resolve_social_customer_{}.db?mode=memory&cache=shared", Uuid::new_v4());
    std::env::set_var("OHC_DATABASE_URL", &temp_db_path);
    let db = Arc::new(DB::new().await.unwrap());

    // Explicitly run migrations to ensure table schema is there in SQLite memory
    crate::migrations::run_migrations(&db.pool).await.unwrap();

    let resolver = IdentityResolver::new(db.clone());
    let sender_id = "insta_handle_123";
    let source = "instagram";

    sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'test')")
        .bind(&tenant_id)
        .execute(&db.pool)
        .await.unwrap();

    let lead_id = resolver.resolve_or_create_customer(&tenant_id, sender_id, source).await.unwrap_or_default();
    let resolved_id = resolver.resolve_or_create_customer(&tenant_id, sender_id, source).await.unwrap_or_default();

    assert!(!lead_id.is_empty());
    assert_eq!(lead_id, resolved_id);
}
