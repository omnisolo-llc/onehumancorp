use std::sync::Arc;
use crate::db::{DB, DbStore, create_dummy_pg_pool, create_sqlite_pool_for_test};
use crate::orchestration::identity_resolution::IdentityResolver;
use uuid::Uuid;

async fn setup_db() -> Arc<DB> {
    let sqlite_pool = create_sqlite_pool_for_test().await;
    let dummy_pg_pool = create_dummy_pg_pool().await;

    let db = Arc::new(DB {
        pool: dummy_pg_pool,
        store: DbStore::Sqlite(sqlite_pool),
    });

    db.run_migrations().await.unwrap();
    db
}

#[tokio::test]
async fn test_resolve_existing_customer() {
    let tenant_id = format!("test_tenant_{}", Uuid::new_v4());
    let db = setup_db().await;

    let resolver = IdentityResolver::new(db.clone());
    let sender_id = "test_lead_existing_123@example.com";
    let source = "email";

    let new_id = Uuid::new_v4().to_string();

    if let DbStore::Sqlite(pool) = &db.store {
        sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'test')")
            .bind(&tenant_id)
            .execute(pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO customers (id, tenant_id, name, email, phone) VALUES ($1, $2, 'Existing Customer', $3, NULL)")
            .bind(&new_id)
            .bind(&tenant_id)
            .bind(sender_id)
            .execute(pool)
            .await
            .unwrap();
    }

    let resolved_id = resolver.resolve_or_create_customer(&tenant_id, sender_id, source).await.unwrap_or_default();

    assert_eq!(resolved_id, new_id);
}

#[tokio::test]
async fn test_create_new_customer() {
    let tenant_id = format!("test_tenant_{}", Uuid::new_v4());
    let db = setup_db().await;

    let resolver = IdentityResolver::new(db.clone());
    let sender_id = "new_lead_12345@example.com";
    let source = "whatsapp";

    if let DbStore::Sqlite(pool) = &db.store {
        sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'test')")
            .bind(&tenant_id)
            .execute(pool)
            .await
            .unwrap();
    }

    let lead_id = resolver.resolve_or_create_customer(&tenant_id, sender_id, source).await.unwrap_or_default();

    if let DbStore::Sqlite(pool) = &db.store {
        let row: Option<(String, Option<String>)> = sqlx::query_as("SELECT id, phone FROM customers WHERE id = $1")
            .bind(&lead_id)
            .fetch_optional(pool)
            .await
            .unwrap_or(None);

        assert!(!lead_id.is_empty());
        assert!(row.is_some());
        let r = row.unwrap();
        assert_eq!(r.0, lead_id);
        assert_eq!(r.1.unwrap(), sender_id);
    }
}

#[tokio::test]
async fn test_create_and_resolve_social_customer() {
    let tenant_id = format!("test_tenant_{}", Uuid::new_v4());
    let db = setup_db().await;

    let resolver = IdentityResolver::new(db.clone());
    let sender_id = "insta_handle_123";
    let source = "instagram";

    if let DbStore::Sqlite(pool) = &db.store {
        sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'test')")
            .bind(&tenant_id)
            .execute(pool)
            .await
            .unwrap();
    }

    let lead_id = resolver.resolve_or_create_customer(&tenant_id, sender_id, source).await.unwrap_or_default();
    let resolved_id = resolver.resolve_or_create_customer(&tenant_id, sender_id, source).await.unwrap_or_default();

    assert!(!lead_id.is_empty());
    assert_eq!(lead_id, resolved_id);
}

#[tokio::test]
async fn test_create_and_resolve_alias() {
    let tenant_id = format!("test_tenant_{}", Uuid::new_v4());
    let db = setup_db().await;

    let resolver = IdentityResolver::new(db.clone());
    let sender_id = "test_alias_123";
    let source = "whatsapp";

    if let DbStore::Sqlite(pool) = &db.store {
        sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'test')")
            .bind(&tenant_id)
            .execute(pool)
            .await
            .unwrap();
    }

    let lead_id = resolver.resolve_or_create_customer(&tenant_id, sender_id, source).await.unwrap_or_default();
    let resolved_id = resolver.resolve_or_create_customer(&tenant_id, sender_id, source).await.unwrap_or_default();

    assert!(!lead_id.is_empty());
    assert_eq!(lead_id, resolved_id);

    // Test alternative channel
    let alt_sender = "alt_channel_id";
    let alt_source = "instagram";
    let alt_lead_id = resolver.resolve_or_create_customer(&tenant_id, alt_sender, alt_source).await.unwrap_or_default();
    assert_ne!(lead_id, alt_lead_id);
}
