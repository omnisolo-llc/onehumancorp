#![allow(dead_code)]
#[path = "../../src/server/persistence/capabilities.rs"]
mod capabilities;
#[path = "../../src/server/persistence/catalog.rs"]
mod catalog;
#[path = "../../src/server/api/catalog_edit.rs"]
mod catalog_edit;
#[path = "../../src/server/persistence/connection.rs"]
mod connection;
#[path = "../../src/server/persistence/entities.rs"]
mod entities;
#[cfg(test)]
mod sqlite_tests {
    use super::*;
    use sea_orm::{ConnectionTrait, Schema};
    #[tokio::test]
    async fn standalone_edits_only_owned_product_and_preserves_stock_metadata_and_type() {
        let db = connection::AppDatabase::connect("sqlite::memory:")
            .await
            .unwrap();
        let backend = db.connection().get_database_backend();
        let table = Schema::new(backend).create_table_from_entity(entities::product::Entity);
        db.connection()
            .execute(backend.build(&table))
            .await
            .unwrap();
        let repo = catalog::CatalogRepository::new(db);
        let original = repo
            .create_product(catalog::NewProduct {
                tenant_id: "tenant-a".into(),
                title: "Original".into(),
                description: Some("Original description".into()),
                item_type: Some("Service".into()),
                price_cents: 5000,
                inventory_count: 8,
                metadata: Some(serde_json::json!({"image_url":"/real.png"})),
            })
            .await
            .unwrap();
        assert!(
            !repo
                .update_product("tenant-b", &original.id, "Private change", "No", 1)
                .await
                .unwrap()
        );
        assert!(
            repo.update_product("tenant-a", &original.id, "Edited", "Saved", 4501)
                .await
                .unwrap()
        );
        let rows = repo.list_products("tenant-a").await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].title, "Edited");
        assert_eq!(rows[0].price_cents, 4501);
        assert_eq!(rows[0].inventory_count, 8);
        assert_eq!(rows[0].item_type, original.item_type);
        assert_eq!(rows[0].metadata, original.metadata);
        assert!(
            !repo
                .update_product("tenant-a", "missing", "Missing", "No", 1)
                .await
                .unwrap()
        );
    }
}
