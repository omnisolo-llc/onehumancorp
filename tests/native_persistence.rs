//! Small integration target: exercise the production migration implementation
//! without compiling the monolithic backend's entire unit-test module graph.
use server_lib::db::{DB, DbStore};

#[tokio::test]
async fn startup_migrations_preserve_existing_sqlite_tasks() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("isolated SQLite database");
    let db = DB {
        pool: sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://unused:unused@127.0.0.1:1/unused")
            .expect("unused lazy PostgreSQL placeholder"),
        store: DbStore::Sqlite(pool.clone()),
    };
    db.run_migrations().await.expect("first startup");
    sqlx::query("INSERT INTO shared_tasks (id, organization_id, title, description, status) VALUES ('task-a', 'tenant-a', 'Owner work A', 'Must survive restart', 'IN_PROGRESS'), ('task-b', 'tenant-b', 'Owner work B', 'Also preserved', 'PENDING')")
        .execute(&pool).await.unwrap();
    for _ in 0..2 {
        db.run_migrations().await.expect("repeated startup");
    }
    let rows: Vec<(String, String, String)> =
        sqlx::query_as("SELECT organization_id, title, status FROM shared_tasks ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(
        rows,
        vec![
            (
                "tenant-a".into(),
                "Owner work A".into(),
                "IN_PROGRESS".into()
            ),
            ("tenant-b".into(), "Owner work B".into(), "PENDING".into()),
        ]
    );
    pool.close().await;
}
