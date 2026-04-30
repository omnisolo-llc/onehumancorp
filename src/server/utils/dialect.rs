pub enum DatabaseKind {
    Postgres,
    Sqlite,
}

pub fn get_database_kind(url: &str) -> DatabaseKind {
    if url.starts_with("postgres") {
        DatabaseKind::Postgres
    } else {
        DatabaseKind::Sqlite
    }
}

pub fn dialect_query(query: &str, kind: DatabaseKind) -> String {
    match kind {
        DatabaseKind::Postgres => query.to_string(),
        DatabaseKind::Sqlite => {
            let mut result = query.to_string();
            let mut i = 1;
            while result.contains(&format!("${}", i)) {
                result = result.replace(&format!("${}", i), "?");
                i += 1;
            }
            result
        }
    }
}


pub enum PoolType {
    Pg(sqlx::PgPool),
    Sqlite(sqlx::SqlitePool),
}

impl PoolType {
    pub async fn connect(url: &str) -> Result<Self, String> {
        match get_database_kind(url) {
            DatabaseKind::Postgres => {
                let pool = sqlx::PgPool::connect(url).await.map_err(|e| e.to_string())?;
                Ok(PoolType::Pg(pool))
            }
            DatabaseKind::Sqlite => {
                use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
                use std::str::FromStr;
                let mut options = SqliteConnectOptions::from_str(url).map_err(|e| e.to_string())?;

                if let Ok(key) = std::env::var("SQLCIPHER_KEY") {
                    options = options.pragma("key", key);
                }

                let pool = SqlitePoolOptions::new()
                    .connect_with(options)
                    .await
                    .map_err(|e| e.to_string())?;

                Ok(PoolType::Sqlite(pool))
            }
        }
    }
}
