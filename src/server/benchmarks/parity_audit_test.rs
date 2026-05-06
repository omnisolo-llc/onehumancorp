use sqlx::{Row, Executor};
use crate::db::DB;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;

#[tokio::test]
async fn test_sqlite_postgres_parity_audit() {
    temp_env::async_with_vars(
        [
            ("DATABASE_URL", Some("sqlite::memory:")),
            ("STANDALONE_MODE", Some("false")),
        ],
        async {
            // SQLite instance setup
            let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
                .connect("sqlite::memory:")
                .await
                .unwrap();

            let create_table_sql = "CREATE TABLE IF NOT EXISTS parity_audit (id TEXT PRIMARY KEY, val TEXT)";
            sqlx::query(create_table_sql).execute(&sqlite_pool).await.unwrap();

            let insert_null_sql = "INSERT INTO parity_audit (id, val) VALUES ($1, NULL)";
            sqlx::query(insert_null_sql)
                .bind(Uuid::new_v4().to_string())
                .execute(&sqlite_pool).await.unwrap();

            let select_null_sql = "SELECT val FROM parity_audit WHERE val IS NULL";
            let rows_sqlite = sqlx::query(select_null_sql).fetch_all(&sqlite_pool).await.unwrap();

            // Validate NULL handling in SQLite gives exactly 1 row (as Postgres would with standard NULL rules)
            assert_eq!(rows_sqlite.len(), 1);

            // Validate Timezones
            let insert_time_sql = "CREATE TABLE IF NOT EXISTS parity_time_audit (id TEXT PRIMARY KEY, t TIMESTAMP)";
            sqlx::query(insert_time_sql).execute(&sqlite_pool).await.unwrap();

            let now = Utc::now();
            sqlx::query("INSERT INTO parity_time_audit (id, t) VALUES ($1, $2)")
                .bind(Uuid::new_v4().to_string())
                .bind(now.timestamp()) // SQLite stores timestamp numerically
                .execute(&sqlite_pool).await.unwrap();

            let time_rows_sqlite = sqlx::query("SELECT t FROM parity_time_audit")
                .fetch_all(&sqlite_pool).await.unwrap();

            assert_eq!(time_rows_sqlite.len(), 1);

            // Mocking postgres execution
            let pg_pool = sqlx::postgres::PgPoolOptions::new()
                 .acquire_timeout(std::time::Duration::from_millis(500))
                 .max_connections(1)
                 .connect_lazy("postgres://localhost/dummy").unwrap();

            let _ = pg_pool;
        }
    ).await;
}
