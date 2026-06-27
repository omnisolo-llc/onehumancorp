use std::sync::Arc;
use chrono::{Utc, Duration};
use uuid::Uuid;
use crate::db::{DB, DbStore, create_sqlite_pool_for_test};
use crate::domain::repository::ucal_repo::UcalRepository;
use crate::domain::repository::models::UcalResource;

#[tokio::test]
async fn test_ucal_repo_integration() {
    let pool = create_sqlite_pool_for_test().await;

    sqlx::query(
        "CREATE TABLE tenants (id TEXT PRIMARY KEY);
         CREATE TABLE ucal_resources (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            name TEXT NOT NULL,
            resource_type TEXT NOT NULL,
            base_capacity INTEGER NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
         );
         CREATE TABLE ucal_ledger (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            resource_id TEXT NOT NULL,
            start_time TIMESTAMP NOT NULL,
            end_time TIMESTAMP NOT NULL,
            consumed_units INTEGER NOT NULL,
            total_units_at_time INTEGER,
            status TEXT NOT NULL,
            reference_id TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
         );"
    ).execute(&pool).await.unwrap();

    let db = Arc::new(DB {
        pool: crate::db::create_dummy_pg_pool().await,
        store: DbStore::Sqlite(pool),
    });

    let repo = UcalRepository::new(db);
    let tenant_id = "test-tenant";

    let resource = UcalResource {
        id: Uuid::new_v4(),
        tenant_id: tenant_id.to_string(),
        name: "Test Resource".to_string(),
        resource_type: "STAFF".to_string(),
        base_capacity: 5,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    repo.create_resource(resource.clone()).await.unwrap();
    let resources = repo.get_resources(tenant_id).await.unwrap();
    assert_eq!(resources.len(), 1);
    assert_eq!(resources[0].name, "Test Resource");
}
