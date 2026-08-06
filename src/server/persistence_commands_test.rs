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

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, PaginatorTrait, Set, Statement};

use connection::AppDatabase;

async fn insert_user(database: &AppDatabase, id: &str, tenant_id: &str, email: &str) {
    let now = Utc::now();
    entities::user::ActiveModel {
        id: Set(id.to_string()),
        username: Set(id.to_string()),
        email: Set(email.to_string()),
        password_hash: Set("unused".to_string()),
        active: Set(true),
        tenant_id: Set(tenant_id.to_string()),
        oidc_subject: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    }
    .insert(database.connection())
    .await
    .unwrap();
    database
        .connection()
        .execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Sqlite,
            "UPDATE users SET roles = ? WHERE id = ?",
            [serde_json::json!(["ADMIN"]).into(), id.into()],
        ))
        .await
        .unwrap();
}

#[tokio::test]
async fn migration_backfills_normalized_identity_email_claims() {
    let database = AppDatabase::connect("sqlite::memory:").await.unwrap();
    migration::migrate(&database).await.unwrap();
    insert_user(&database, "existing-user", "tenant-a", "Admin@Example.Test").await;

    migration::migrate(&database).await.unwrap();

    let claim = server_auth::seaorm_store::entities::identity_email_claim::Entity::find_by_id(
        "admin@example.test",
    )
    .one(database.connection())
    .await
    .unwrap()
    .unwrap();
    assert_eq!(claim.user_id, "existing-user");
}

#[tokio::test]
async fn migration_backfills_portable_roles_from_existing_json_users() {
    let database = AppDatabase::connect("sqlite::memory:").await.unwrap();
    migration::migrate(&database).await.unwrap();
    insert_user(&database, "existing-user", "tenant-a", "roles@example.test").await;

    migration::migrate(&database).await.unwrap();

    let rows = database
        .connection()
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            "SELECT role_name FROM identity_user_roles WHERE user_id = 'existing-user' ORDER BY role_name"
                .to_string(),
        ))
        .await
        .unwrap();
    let roles = rows
        .into_iter()
        .map(|row| row.try_get::<String>("", "role_name").unwrap())
        .collect::<Vec<_>>();
    assert_eq!(roles, vec!["ADMIN"]);
}

#[test]
fn postgres_portable_schema_retains_the_legacy_text_array_contract() {
    let migration_source = include_str!("persistence/migration.rs");
    let persistence_entity_source = include_str!("persistence/entities.rs");

    assert!(migration_source.contains("roles TEXT[] DEFAULT '{}'"));
    assert!(!persistence_entity_source.contains("pub roles: Json"));
    assert!(migration_source.contains("identity_user_roles"));
}

#[tokio::test]
async fn migration_rejects_colliding_normalized_identity_emails_without_choosing_an_owner() {
    let database = AppDatabase::connect("sqlite::memory:").await.unwrap();
    migration::migrate(&database).await.unwrap();
    insert_user(
        &database,
        "first-user",
        "tenant-a",
        "Same.Person@Example.Test",
    )
    .await;
    insert_user(
        &database,
        "second-user",
        "tenant-b",
        "same.person@example.test",
    )
    .await;

    let error = migration::migrate(&database).await.unwrap_err().to_string();

    assert!(error.contains("same.person@example.test"), "{error}");
    assert_eq!(
        server_auth::seaorm_store::entities::identity_email_claim::Entity::find()
            .count(database.connection())
            .await
            .unwrap(),
        0
    );
}

#[tokio::test]
async fn migration_and_admin_bootstrap_are_idempotent_without_demo_rows() {
    let database = AppDatabase::connect("sqlite::memory:").await.unwrap();
    migration::migrate(&database).await.unwrap();
    migration::migrate(&database).await.unwrap();

    commands::bootstrap_admin(&database, "admin@ohc.test", "correct horse battery staple")
        .await
        .unwrap();
    commands::bootstrap_admin(
        &database,
        "admin@ohc.test",
        "replacement horse battery staple",
    )
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

    let admin = entities::user::Entity::find()
        .one(database.connection())
        .await
        .unwrap()
        .unwrap();
    assert!(bcrypt::verify("replacement horse battery staple", &admin.password_hash).unwrap());
    assert!(!bcrypt::verify("correct horse battery staple", &admin.password_hash).unwrap());
    let claim = server_auth::seaorm_store::entities::identity_email_claim::Entity::find_by_id(
        "admin@ohc.test",
    )
    .one(database.connection())
    .await
    .unwrap()
    .unwrap();
    assert_eq!(claim.user_id, admin.id);

    let verification = commands::verify(&database).await.unwrap();
    assert_eq!(verification.migrations, 3);
    assert_eq!(verification.users, 1);
    assert_eq!(verification.products, 0);
}

#[tokio::test]
async fn bootstrap_admin_rejects_an_identity_email_claim_owned_by_another_user() {
    let database = AppDatabase::connect("sqlite::memory:").await.unwrap();
    migration::migrate(&database).await.unwrap();
    insert_user(
        &database,
        "existing-owner",
        "tenant-other",
        "admin@ohc.test",
    )
    .await;
    migration::migrate(&database).await.unwrap();

    let error =
        commands::bootstrap_admin(&database, "admin@ohc.test", "correct horse battery staple")
            .await
            .unwrap_err()
            .to_string();

    assert!(error.contains("admin@ohc.test"), "{error}");
    assert_eq!(
        entities::user::Entity::find()
            .count(database.connection())
            .await
            .unwrap(),
        1
    );
    let claim = server_auth::seaorm_store::entities::identity_email_claim::Entity::find_by_id(
        "admin@ohc.test",
    )
    .one(database.connection())
    .await
    .unwrap()
    .unwrap();
    assert_eq!(claim.user_id, "existing-owner");
}
