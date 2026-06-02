#[cfg(test)]
mod tests {
    use crate::db::{DbStore, DB};
    use sqlx::Row;
    use std::sync::Arc;
    use serde_json::json;

    async fn setup_sqlite_db() -> Arc<DB> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create SQLite memory pool");

        sqlx::query("CREATE TABLE parity_test (id TEXT PRIMARY KEY, data TEXT, val INTEGER, ts TIMESTAMP DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool).await.unwrap();

        // Dummy PG pool
        let pg_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://localhost/dummy").unwrap();

        Arc::new(DB { pool: pg_pool, store: DbStore::Sqlite(pool) })
    }

    async fn setup_postgres_db() -> Option<Arc<DB>> {
        let url = std::env::var("OHC_DATABASE_URL").ok()?;
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(&url).await.ok()?;

        let _ = sqlx::query("DROP TABLE IF EXISTS parity_test").execute(&pool).await;
        sqlx::query("CREATE TABLE parity_test (id TEXT PRIMARY KEY, data JSONB, val INTEGER, ts TIMESTAMP DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool).await.unwrap();

        Some(Arc::new(DB { pool, store: DbStore::Postgres }))
    }

    #[tokio::test]
    async fn test_database_parity_json_timestamp_null() {
        let sqlite_db = setup_sqlite_db().await;
        let pg_db = setup_postgres_db().await;

        let dbs = if let Some(pg) = pg_db {
            vec![sqlite_db, pg]
        } else {
            vec![sqlite_db]
        };

        for db in dbs {
            let id = "test_1";
            let data = json!({"key": "value", "nested": {"a": 1}});
            let val = 42;

            // Insert
            let query = format!("INSERT INTO parity_test (id, data, val) VALUES ({}, {}, {})", db.placeholder(1), db.placeholder(2), db.placeholder(3));

            match &db.store {
                DbStore::Sqlite(pool) => {
                    sqlx::query(&query).bind(id).bind(data.to_string()).bind(val).execute(pool).await.unwrap();
                },
                DbStore::Postgres => {
                    sqlx::query(&query).bind(id).bind(&data).bind(val).execute(&db.pool).await.unwrap();
                }
            }

            // Select
            let select_query = format!("SELECT id, data, val FROM parity_test WHERE id = {}", db.placeholder(1));

            let (res_id, res_val, res_data) = match &db.store {
                DbStore::Sqlite(pool) => {
                    let row = sqlx::query(&select_query).bind(id).fetch_one(pool).await.unwrap();
                    let s: String = row.get("data");
                    (row.get::<String, _>("id"), row.get::<i32, _>("val"), serde_json::from_str::<serde_json::Value>(&s).unwrap())
                },
                DbStore::Postgres => {
                    let row = sqlx::query(&select_query).bind(id).fetch_one(&db.pool).await.unwrap();
                    (row.get::<String, _>("id"), row.get::<i32, _>("val"), row.get::<serde_json::Value, _>("data"))
                }
            };

            assert_eq!(res_id, id);
            assert_eq!(res_val, val);
            assert_eq!(res_data, data);

            // NULL handling
            let null_id = "test_null";
            let null_query = format!("INSERT INTO parity_test (id, data, val) VALUES ({}, NULL, NULL)", db.placeholder(1));
            match &db.store {
                DbStore::Sqlite(pool) => {
                    sqlx::query(&null_query).bind(null_id).execute(pool).await.unwrap();
                },
                DbStore::Postgres => {
                    sqlx::query(&null_query).bind(null_id).execute(&db.pool).await.unwrap();
                }
            }

            let select_null = format!("SELECT data, val FROM parity_test WHERE id = {}", db.placeholder(1));
            let (n_val, is_data_null) = match &db.store {
                DbStore::Sqlite(pool) => {
                    let row = sqlx::query(&select_null).bind(null_id).fetch_one(pool).await.unwrap();
                    (row.get::<Option<i32>, _>("val"), row.get::<Option<String>, _>("data").is_none())
                },
                DbStore::Postgres => {
                    let row = sqlx::query(&select_null).bind(null_id).fetch_one(&db.pool).await.unwrap();
                    (row.get::<Option<i32>, _>("val"), row.get::<Option<serde_json::Value>, _>("data").is_none())
                }
            };

            assert!(n_val.is_none());
            assert!(is_data_null);
        }
    }
}
