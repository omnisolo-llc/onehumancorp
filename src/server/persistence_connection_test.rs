use server_lib::persistence::{AppDatabase, DatabaseBackend, DatabaseUrl};

#[tokio::test]
async fn backend_is_derived_from_the_real_connection() {
    let db = AppDatabase::connect("sqlite::memory:").await.unwrap();
    assert_eq!(db.backend(), DatabaseBackend::Sqlite);
    assert!(db.capabilities().transactions);
    assert!(!db.capabilities().pg_vector);
}

#[test]
fn database_url_debug_output_is_redacted() {
    let url = DatabaseUrl::new("mysql://app:secret@db/onehumancorp");
    assert_eq!(format!("{url:?}"), "DatabaseUrl(REDACTED)");
}
