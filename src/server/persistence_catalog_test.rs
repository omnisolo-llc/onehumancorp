#[path = "persistence/capabilities.rs"]
mod capabilities;
#[path = "persistence/catalog.rs"]
mod catalog;
#[path = "persistence/connection.rs"]
mod connection;
#[path = "persistence/entities.rs"]
mod entities;
#[path = "persistence/migration.rs"]
mod migration;

use catalog::{CatalogRepository, NewProduct};
use connection::AppDatabase;

#[tokio::test]
async fn catalog_repository_reads_only_real_tenant_rows() {
    let database = AppDatabase::connect("sqlite::memory:").await.unwrap();
    migration::migrate(&database).await.unwrap();
    let repository = CatalogRepository::new(database);

    repository
        .create_product(NewProduct {
            tenant_id: "tenant-a".to_owned(),
            title: "Database product".to_owned(),
            description: Some("Persisted by SeaORM".to_owned()),
            item_type: Some("Product".to_owned()),
            price_cents: 1250,
            inventory_count: 3,
        })
        .await
        .unwrap();
    repository
        .create_product(NewProduct {
            tenant_id: "tenant-b".to_owned(),
            title: "Other tenant".to_owned(),
            description: None,
            item_type: Some("Service".to_owned()),
            price_cents: 9900,
            inventory_count: 1,
        })
        .await
        .unwrap();

    let products = repository.list_products("tenant-a").await.unwrap();
    assert_eq!(products.len(), 1);
    assert_eq!(products[0].title, "Database product");
    assert_eq!(products[0].price_cents, 1250);
}
