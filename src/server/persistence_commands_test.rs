#[path = "persistence/capabilities.rs"]
mod capabilities;
#[path = "persistence/commands.rs"]
mod commands;
#[path = "persistence/connection.rs"]
mod connection;
#[path = "persistence/entities.rs"]
mod entities;
#[path = "persistence/migration.rs"]
mod migration;

use sea_orm::EntityTrait;

use connection::AppDatabase;

#[tokio::test]
async fn migration_and_admin_bootstrap_are_idempotent_without_demo_rows() {
    let database = AppDatabase::connect("sqlite::memory:").await.unwrap();
    migration::migrate(&database).await.unwrap();
    migration::migrate(&database).await.unwrap();

    commands::bootstrap_admin(&database, "admin@ohc.test", "correct horse battery staple")
        .await
        .unwrap();
    commands::bootstrap_admin(&database, "admin@ohc.test", "correct horse battery staple")
        .await
        .unwrap();

    assert_eq!(
        entities::user::Entity::find()
            .count(database.connection())
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        entities::product::Entity::find()
            .count(database.connection())
            .await
            .unwrap(),
        0
    );
}
