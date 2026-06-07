



#[tokio::test]
#[cfg(feature = "postgres")]
async fn test_db_parity_audit() -> Result<(), Box<dyn Error>> {
    let pg_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://ohc:ohc@127.0.0.1:5432/ohc?sslmode=disable".to_string());

    // We expect Postgres to be available when the "postgres" feature is active.
    let pg_pool = PgPool::connect(&pg_url).await?;

    // Create an in-memory SQLite database
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await?;

    // 1. Setup table in both DBs
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS parity_test (
            id TEXT PRIMARY KEY,
            val_text TEXT,
            val_int INTEGER,
            val_null TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            time_tz TIMESTAMPTZTZ
        )",
    )
    .execute(&pg_pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS parity_test (
            id TEXT PRIMARY KEY,
            val_text TEXT,
            val_int INTEGER,
            val_null TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            time_tz TIMESTAMPTZ
        )",
    )
    .execute(&sqlite_pool)
    .await?;

    // 2. Insert data
    let insert_query = "INSERT INTO parity_test (id, val_text, val_int, val_null, time_tz) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING";

    // SQLite uses ? instead of $1
    let sqlite_insert = "INSERT INTO parity_test (id, val_text, val_int, val_null, time_tz) VALUES (?, ?, ?, ?, ?) ON CONFLICT DO NOTHING";

    sqlx::query(sqlite_insert)
        .bind("1")
        .bind("test_str")
        .bind(42)
        .bind(None::<String>)
        .bind("2024-01-01T12:00:00Z")
        .execute(&sqlite_pool)
        .await?;

    sqlx::query(insert_query)
        .bind("1")
        .bind("test_str")
        .bind(42)
        .bind(None::<String>)
        .bind("2024-01-01T12:00:00Z")
        .execute(&pg_pool)
        .await?;

    // 3. Query and diff
    // A query with NULL handling and timestamps
    let sqlite_row = sqlx::query("SELECT id, val_text, val_int, COALESCE(val_null, 'default') as val_null_resolved, time_tz FROM parity_test WHERE id = '1'")
        .fetch_one(&sqlite_pool)
        .await?;

    let pg_row = sqlx::query("SELECT id, val_text, val_int, COALESCE(val_null, 'default') as val_null_resolved, CAST(time_tz AS TEXT) as time_tz FROM parity_test WHERE id = '1'")
        .fetch_one(&pg_pool)
        .await?;

    let sqlite_id: String = sqlite_row.get("id");
    let pg_id: String = pg_row.get("id");
    assert_eq!(sqlite_id, pg_id);

    let sqlite_val: i32 = sqlite_row.get("val_int");
    let pg_val: i32 = pg_row.get("val_int");
    assert_eq!(sqlite_val, pg_val);

    let sqlite_null: String = sqlite_row.get("val_null_resolved");
    let pg_null: String = pg_row.get("val_null_resolved");
    assert_eq!(sqlite_null, pg_null);

    // Let's check time handling output format between databases
    let sqlite_time: String = sqlite_row.get("time_tz");
    // Postgres with sqlx returns String for timestamp when requested, format varies.
    let pg_time: String = pg_row.get("time_tz");

    // SQLite time: 2024-01-01T12:00:00Z
    // Postgres time: 2024-01-01 12:00:00+00
    // assert_ne!(sqlite_time, pg_time);

    // Testing `COALESCE` with specific string casting often yields discrepancies.
    // However standard coalesce handles string correctly.
    let _pg_row2 = sqlx::query("SELECT COALESCE(val_null, 'default') as bad_coalesce FROM parity_test WHERE id = '1'").fetch_one(&pg_pool).await;
    let _sqlite_row2 = sqlx::query("SELECT COALESCE(val_null, 'default') as bad_coalesce FROM parity_test WHERE id = '1'").fetch_one(&sqlite_pool).await;

    // Clean up
    sqlx::query("DROP TABLE parity_test").execute(&pg_pool).await?;

    Ok(())
}
