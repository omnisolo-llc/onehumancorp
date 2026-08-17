#[path = "persistence/capabilities.rs"]
mod capabilities;
#[path = "persistence/connection.rs"]
mod connection;

use capabilities::DatabaseBackend;
use connection::{AppDatabase, DatabaseUrl};
use sea_orm::{ConnectionTrait, Statement};

#[tokio::test]
async fn backend_is_derived_from_the_real_connection() {
    let db = AppDatabase::connect("sqlite::memory:").await.unwrap();
    assert_eq!(db.backend(), DatabaseBackend::Sqlite);
    assert!(db.capabilities().transactions);
    assert!(!db.capabilities().pg_vector);
}

#[tokio::test]
async fn sqlcipher_key_encrypts_an_on_disk_sqlite_database() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("portable.db");
    std::fs::File::create(&path).unwrap();
    let url = format!("sqlite://{}", path.display());

    let database = AppDatabase::connect_with_sqlcipher_key(
        &url,
        "portable-test-key-with-'quote-that-must-not-appear-in-the-url",
    )
    .await
    .unwrap();
    database
        .connection()
        .execute(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            "CREATE TABLE encrypted_probe (value TEXT NOT NULL)".to_owned(),
        ))
        .await
        .unwrap();
    database.connection().clone().close().await.unwrap();

    let unkeyed = AppDatabase::connect(&url).await.unwrap();
    let error = unkeyed
        .connection()
        .query_one(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            "SELECT value FROM encrypted_probe".to_owned(),
        ))
        .await
        .unwrap_err();
    assert!(
        error.to_string().contains("file is not a database"),
        "{error}"
    );
    unkeyed.connection().clone().close().await.unwrap();

    let keyed = AppDatabase::connect_with_sqlcipher_key(
        &url,
        "portable-test-key-with-'quote-that-must-not-appear-in-the-url",
    )
    .await
    .unwrap();
    keyed
        .connection()
        .query_one(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            "SELECT value FROM encrypted_probe".to_owned(),
        ))
        .await
        .unwrap();
}

#[test]
fn database_url_debug_output_is_redacted() {
    let url = DatabaseUrl::new("mysql://app:secret@db/onehumancorp");
    assert_eq!(format!("{url:?}"), "DatabaseUrl(REDACTED)");
}
